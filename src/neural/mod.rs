pub mod cell;
pub mod column;
pub mod field;
pub mod temporal;
pub mod plasticity;

use crate::config::ModelConfig;
use crate::error::Result;
use crate::types::*;

pub trait NeuralCore {
    fn process(&mut self, input: &LanguageState, context: &ContextState) -> Result<NeuralRepresentation>;
    fn predict(&self) -> Result<Option<Prediction>>;
    fn state(&self) -> &NeuralState;
}

#[derive(Debug, Clone)]
pub struct NeuralRepresentation {
    pub active_cells: Vec<CellId>,
    pub active_columns: Vec<ColumnId>,
    pub field_activations: Vec<Scalar>,
    pub temporal_encoding: TemporalEncoding,
    pub prediction: Option<Prediction>,
    pub confidence: ConfidenceState,
}

#[derive(Debug, Clone, Default)]
pub struct TemporalEncoding {
    pub sequence: Vec<Scalar>,
    pub transition: Vec<Scalar>,
    pub recurrence: Scalar,
    pub dependency: Vec<Scalar>,
    pub coherence: Scalar,
    pub anomaly_score: Scalar,
    pub short_term: Vec<Scalar>,
    pub long_term: Vec<Scalar>,
}

pub struct NeuralCoreImpl {
    config: ModelConfig,
    state: NeuralState,
}

impl NeuralCoreImpl {
    pub fn new(config: &ModelConfig) -> Result<Self> {
        let cells_per_column = config.cells / config.columns;
        let mut fields = Vec::new();
        let field_count = (config.cells / config.columns).max(1);
        for fi in 0..field_count {
            let mut columns = Vec::new();
            for ci in 0..config.columns {
                let mut cells = Vec::new();
                for cell_i in 0..cells_per_column {
                    let cell_id = CellId((fi * config.columns * cells_per_column + ci * cells_per_column + cell_i) as u64);
                    cells.push(Cell {
                        id: cell_id,
                        state: CellState::Resting,
                        activation: 0.0,
                        prediction_vector: vec![0.0; config.dimension as usize],
                        refractory_steps: 0,
                        adaptation_level: 0.0,
                        burst_counter: 0,
                        eligibility_trace: 0.0,
                    });
                }
                columns.push(Column {
                    id: ColumnId((fi * config.columns + ci) as u64),
                    cells,
                    active_cells: Vec::new(),
                    activation_threshold: 0.5,
                    learned_pattern: Vec::new(),
                });
            }
            fields.push(NeuralField {
                id: FieldId(fi as u64),
                columns,
                average_activation: 0.0,
                coherence: 0.0,
            });
        }
        Ok(Self {
            config: config.clone(),
            state: NeuralState {
                fields,
                active_cells: Vec::new(),
                active_columns: Vec::new(),
                temporal_buffer: Vec::new(),
                prediction: None,
            },
        })
    }

    fn map_language_to_cells(&mut self, input: &LanguageState) {
        let mut input_hash: u64 = 0;
        for symbol in &input.symbols {
            for byte in symbol.text.bytes() {
                input_hash = input_hash.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }

        let strength = if input.symbols.is_empty() { 0.0 } else { 1.0 / input.symbols.len() as Scalar };

        for (si, symbol) in input.symbols.iter().enumerate() {
            let symbol_hash = input_hash.wrapping_add(si as u64);
            let target_field = (symbol_hash % self.state.fields.len() as u64) as usize;
            let target_column = (symbol_hash.wrapping_div(self.state.fields.len() as u64) % self.config.columns as u64) as usize;

            if let Some(field) = self.state.fields.get_mut(target_field) {
                if let Some(column) = field.columns.get_mut(target_column) {
                    column::inject_input(column, symbol_hash, strength);
                }
            }

            let secondary_field = ((symbol_hash >> 16) % self.state.fields.len() as u64) as usize;
            let secondary_column = ((symbol_hash >> 16).wrapping_div(self.state.fields.len() as u64) % self.config.columns as u64) as usize;
            if let Some(field) = self.state.fields.get_mut(secondary_field) {
                if let Some(column) = field.columns.get_mut(secondary_column) {
                    column::inject_input(column, symbol_hash.wrapping_add(1), strength * 0.5);
                }
            }
        }
    }
}

impl NeuralCore for NeuralCoreImpl {
    fn process(&mut self, input: &LanguageState, _context: &ContextState) -> Result<NeuralRepresentation> {
        for field in &mut self.state.fields {
            for column in &mut field.columns {
                cell::tick_all(&mut column.cells);
            }
        }

        self.map_language_to_cells(input);

        let mut all_active_cells = Vec::new();
        let mut all_active_columns = Vec::new();
        let mut field_activations = Vec::new();

        for field in &mut self.state.fields {
            for column in &mut field.columns {
                column::compete(column, self.config.sparsity_ratio);
            }

            column::apply_lateral_inhibition(&mut field.columns, 0.3);

            let mut field_active = 0;
            let mut field_total = 0;
            for column in &mut field.columns {
                field_active += column.active_cells.len();
                field_total += column.cells.len();
                all_active_cells.extend(column.active_cells.iter().cloned());
                all_active_columns.push(column.id);
                column::homeostatic_adjust(column, 0.1);
            }

            field.average_activation = if field_total > 0 {
                field_active as Scalar / field_total as Scalar
            } else {
                0.0
            };
            field.coherence = field.average_activation;
            field_activations.push(field.average_activation);
        }

        let temporal = temporal::encode(&self.state);
        let prediction = cell::predict_from_state(&self.state, &temporal);

        self.state.active_cells = all_active_cells.clone();
        self.state.active_columns = all_active_columns.clone();

        let avg_activation = if field_activations.is_empty() {
            0.0
        } else {
            field_activations.iter().sum::<Scalar>() / field_activations.len() as Scalar
        };

        self.state.prediction = prediction.clone();

        Ok(NeuralRepresentation {
            active_cells: all_active_cells,
            active_columns: all_active_columns,
            field_activations: field_activations.clone(),
            temporal_encoding: temporal,
            prediction,
            confidence: ConfidenceState {
                belief: avg_activation,
                evidence_strength: 0.5,
                source_quality: 0.5,
                consistency: avg_activation,
                uncertainty: (1.0 - avg_activation).max(0.0),
                prediction_reliability: 0.0,
                verification_status: VerificationStatus::Inferred,
            },
        })
    }

    fn predict(&self) -> Result<Option<Prediction>> {
        Ok(self.state.prediction.clone())
    }

    fn state(&self) -> &NeuralState {
        &self.state
    }
}
