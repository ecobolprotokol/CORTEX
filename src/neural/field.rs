use super::column::Column;
use crate::types::ids::ColumnId;

#[derive(Debug, Clone)]
pub struct Field {
    pub columns: Vec<Column>,
    pub column_ids: Vec<ColumnId>,
    pub average_activation: f32,
    pub coherence: f32,
}

impl Field {
    pub fn new(column_count: u32, cells_per_column: u32) -> Self {
        let columns = (0..column_count)
            .map(|_| Column::new(cells_per_column))
            .collect();
        let column_ids = (0..column_count)
            .map(|i| ColumnId::from(i as u64))
            .collect();
        Self {
            columns,
            column_ids,
            average_activation: 0.0,
            coherence: 0.0,
        }
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn total_cells(&self) -> usize {
        self.columns.iter().map(|c| c.cell_count()).sum()
    }

    pub fn set_column_activations(&mut self, column_index: usize, activations: &[f32]) {
        if let Some(column) = self.columns.get_mut(column_index) {
            column.set_activations(activations);
        }
    }

    pub fn compete(&mut self, sparsity: f32) -> Vec<crate::types::ids::CellId> {
        let mut all_active = Vec::new();
        for column in &mut self.columns {
            let active = column.compete(sparsity);
            all_active.extend(active);
        }
        all_active
    }

    pub fn enforce_sparsity(&mut self, max_active: usize) {
        let total_active: usize = self.columns.iter().map(|c| c.active_cell_count()).sum();

        if total_active > max_active {
            let excess = total_active - max_active;
            let mut removed = 0;
            for column in &mut self.columns {
                if removed >= excess {
                    break;
                }
                while column.active_cell_count() > 1 && removed < excess {
                    if let Some(last_id) = column.active_cells.pop() {
                        if let Some(cell) = column.cells.iter_mut().find(|c| c.id == last_id) {
                            cell.inhibit();
                        }
                        removed += 1;
                    }
                }
            }
        }

        self.compute_average_activation();
    }

    fn compute_average_activation(&mut self) {
        let total: f32 = self
            .columns
            .iter()
            .flat_map(|c| c.cells.iter())
            .map(|c| c.activation)
            .sum();
        let count = self.total_cells() as f32;
        self.average_activation = if count > 0.0 { total / count } else { 0.0 };
    }

    pub fn compute_coherence(&mut self) -> f32 {
        if self.columns.is_empty() {
            self.coherence = 0.0;
            return 0.0;
        }

        let avg_per_column: Vec<f32> = self
            .columns
            .iter()
            .map(|c| c.average_activation())
            .collect();

        let global_mean = if !avg_per_column.is_empty() {
            avg_per_column.iter().sum::<f32>() / avg_per_column.len() as f32
        } else {
            0.0
        };

        let variance = if !avg_per_column.is_empty() {
            avg_per_column
                .iter()
                .map(|&v| (v - global_mean).powi(2))
                .sum::<f32>()
                / avg_per_column.len() as f32
        } else {
            0.0
        };

        self.coherence = (1.0 - variance.sqrt()).clamp(0.0, 1.0);
        self.coherence
    }

    pub fn integrate_columns(&mut self) -> Vec<f32> {
        self.compute_average_activation();
        self.columns
            .iter()
            .map(|c| c.average_activation())
            .collect()
    }

    pub fn total_active_cells(&self) -> usize {
        self.columns.iter().map(|c| c.active_cell_count()).sum()
    }

    pub fn active_column_ids(&self) -> Vec<ColumnId> {
        self.columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.active_cell_count() > 0)
            .map(|(i, _)| self.column_ids[i])
            .collect()
    }

    pub fn active_cell_ids(&self) -> Vec<crate::types::ids::CellId> {
        self.columns
            .iter()
            .flat_map(|c| c.active_cells.clone())
            .collect()
    }

    pub fn reset(&mut self) {
        for column in &mut self.columns {
            column.reset();
        }
        self.average_activation = 0.0;
        self.coherence = 0.0;
    }
}
