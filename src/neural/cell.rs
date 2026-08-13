use crate::types::*;
use crate::neural::TemporalEncoding;

const REFRACTORY_PERIOD: u32 = 3;
const ADAPTATION_RATE: Scalar = 0.05;
const ADAPTATION_RECOVERY: Scalar = 0.02;
const BURST_THRESHOLD: u32 = 3;
const ELIGIBILITY_DECAY: Scalar = 0.9;

pub fn activate(cell: &mut Cell, input: Scalar) {
    if cell.refractory_steps > 0 {
        return;
    }

    let adapted_input = input * (1.0 - cell.adaptation_level).max(0.1);
    cell.activation = (cell.activation + adapted_input).clamp(0.0, 1.0);

    if cell.activation > 0.5 {
        cell.state = CellState::Active;
        cell.refractory_steps = REFRACTORY_PERIOD;
        cell.burst_counter += 1;
        cell.adaptation_level = (cell.adaptation_level + ADAPTATION_RATE).min(1.0);
    } else if cell.activation < 0.1 {
        cell.state = CellState::Resting;
        cell.burst_counter = 0;
        cell.adaptation_level = (cell.adaptation_level - ADAPTATION_RECOVERY).max(0.0);
    }

    cell.eligibility_trace = (cell.eligibility_trace + cell.activation).min(1.0);
}

pub fn inhibit(cell: &mut Cell) {
    cell.activation = (cell.activation * 0.1).max(0.0);
    cell.state = CellState::Inhibited;
    cell.burst_counter = 0;
}

pub fn tick_refractory(cell: &mut Cell) {
    if cell.refractory_steps > 0 {
        cell.refractory_steps -= 1;
    }
    cell.eligibility_trace = (cell.eligibility_trace * ELIGIBILITY_DECAY).max(0.0);
}

pub fn tick_all(cells: &mut [Cell]) {
    for cell in cells.iter_mut() {
        tick_refractory(cell);
    }
}

pub fn burst_factor(cell: &Cell) -> Scalar {
    if cell.burst_counter >= BURST_THRESHOLD {
        let excess = (cell.burst_counter - BURST_THRESHOLD) as Scalar;
        (1.0 + excess * 0.2).min(2.0)
    } else {
        1.0
    }
}

pub fn adaptation_factor(cell: &Cell) -> Scalar {
    (1.0 - cell.adaptation_level).max(0.1)
}

pub fn predict_from_state(state: &NeuralState, temporal: &TemporalEncoding) -> Option<Prediction> {
    if state.active_cells.is_empty() {
        return None;
    }
    let predicted_state: Vec<Scalar> = state.fields.iter()
        .flat_map(|f| {
            f.columns.iter().flat_map(|c| {
                c.cells.iter().map(|cell| {
                    let base = cell.activation;
                    let burst = burst_factor(cell);
                    let adapt = adaptation_factor(cell);
                    let temporal_factor = temporal.recurrence * 0.3;
                    let predicted = (base * burst * adapt + temporal_factor).clamp(0.0, 1.0);
                    predicted
                })
            })
        })
        .take(64)
        .collect();

    if predicted_state.is_empty() {
        return None;
    }

    let confidence = predicted_state.iter().map(|x| x.abs()).sum::<Scalar>() / predicted_state.len() as Scalar;

    Some(Prediction {
        target: PredictionTarget::NextState,
        predicted_state,
        confidence: confidence.clamp(0.0, 1.0),
        timestamp: Timestamp::now(),
        context: ContextState::initial(),
        resolved: false,
        actual: None,
        error: None,
    })
}

pub fn compute_prediction_error(predicted: &[Scalar], actual: &[Scalar]) -> Scalar {
    if predicted.is_empty() || actual.is_empty() {
        return 1.0;
    }
    let len = predicted.len().min(actual.len());
    let mut sum_sq = 0.0;
    for i in 0..len {
        let diff = predicted[i] - actual[i];
        sum_sq += diff * diff;
    }
    (sum_sq / len as Scalar).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cell(id: u64, activation: Scalar) -> Cell {
        Cell {
            id: CellId(id),
            state: CellState::Resting,
            activation,
            prediction_vector: vec![0.0; 10],
            refractory_steps: 0,
            adaptation_level: 0.0,
            burst_counter: 0,
            eligibility_trace: 0.0,
        }
    }

    #[test]
    fn test_activate() {
        let mut cell = make_cell(1, 0.0);
        activate(&mut cell, 0.6);
        assert_eq!(cell.state, CellState::Active);
        assert!((cell.activation - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_activate_refractory() {
        let mut cell = make_cell(1, 0.6);
        cell.refractory_steps = 2;
        cell.state = CellState::Active;
        activate(&mut cell, 0.5);
        assert_eq!(cell.refractory_steps, 2);
        assert!((cell.activation - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_inhibit() {
        let mut cell = make_cell(1, 0.8);
        cell.state = CellState::Active;
        cell.burst_counter = 5;
        inhibit(&mut cell);
        assert_eq!(cell.state, CellState::Inhibited);
        assert!(cell.activation < 0.1);
        assert_eq!(cell.burst_counter, 0);
    }

    #[test]
    fn test_tick_refractory() {
        let mut cell = make_cell(1, 0.0);
        cell.refractory_steps = 3;
        tick_refractory(&mut cell);
        assert_eq!(cell.refractory_steps, 2);
        tick_refractory(&mut cell);
        assert_eq!(cell.refractory_steps, 1);
        tick_refractory(&mut cell);
        assert_eq!(cell.refractory_steps, 0);
        tick_refractory(&mut cell);
        assert_eq!(cell.refractory_steps, 0);
    }

    #[test]
    fn test_tick_all() {
        let mut cells = vec![
            make_cell(1, 0.0),
            make_cell(2, 0.0),
            make_cell(3, 0.0),
        ];
        cells[0].refractory_steps = 5;
        cells[1].refractory_steps = 1;
        cells[2].refractory_steps = 0;
        tick_all(&mut cells);
        assert_eq!(cells[0].refractory_steps, 4);
        assert_eq!(cells[1].refractory_steps, 0);
        assert_eq!(cells[2].refractory_steps, 0);
    }

    #[test]
    fn test_burst_factor() {
        let mut cell = make_cell(1, 0.0);
        assert_eq!(burst_factor(&cell), 1.0);
        cell.burst_counter = 3;
        assert!((burst_factor(&cell) - 1.0).abs() < 0.001);
        cell.burst_counter = 5;
        assert!(burst_factor(&cell) > 1.0);
    }

    #[test]
    fn test_adaptation_factor() {
        let mut cell = make_cell(1, 0.0);
        assert!((adaptation_factor(&cell) - 1.0).abs() < 0.001);
        cell.adaptation_level = 0.5;
        assert!((adaptation_factor(&cell) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_eligibility_trace() {
        let mut cell = make_cell(1, 0.0);
        cell.eligibility_trace = 0.0;
        activate(&mut cell, 0.8);
        assert!(cell.eligibility_trace > 0.0);
        let trace_before = cell.eligibility_trace;
        tick_refractory(&mut cell);
        assert!(cell.eligibility_trace < trace_before);
    }

    #[test]
    fn test_prediction_error() {
        let predicted = vec![0.5, 0.5, 0.5];
        let actual = vec![0.5, 0.5, 0.5];
        assert!(compute_prediction_error(&predicted, &actual) < 0.001);

        let actual2 = vec![0.0, 0.0, 0.0];
        let err = compute_prediction_error(&predicted, &actual2);
        assert!(err > 0.4);
    }
}
