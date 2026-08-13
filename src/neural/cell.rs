use crate::types::*;
use crate::neural::TemporalEncoding;

pub fn activate(cell: &mut Cell, input: Scalar) {
    cell.activation = (cell.activation + input).clamp(0.0, 1.0);
    if cell.activation > 0.5 {
        cell.state = CellState::Active;
    } else if cell.activation < 0.1 {
        cell.state = CellState::Resting;
    }
}

pub fn inhibit(cell: &mut Cell) {
    cell.activation = (cell.activation * 0.1).max(0.0);
    cell.state = CellState::Inhibited;
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
                    let temporal_factor = temporal.recurrence * 0.3;
                    let predicted = (base + temporal_factor).clamp(0.0, 1.0);
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

    #[test]
    fn test_activate() {
        let mut cell = Cell {
            id: CellId(1),
            state: CellState::Resting,
            activation: 0.0,
            prediction_vector: vec![0.0; 10],
        };
        activate(&mut cell, 0.6);
        assert_eq!(cell.state, CellState::Active);
        assert!((cell.activation - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_inhibit() {
        let mut cell = Cell {
            id: CellId(1),
            state: CellState::Active,
            activation: 0.8,
            prediction_vector: vec![0.0; 10],
        };
        inhibit(&mut cell);
        assert_eq!(cell.state, CellState::Inhibited);
        assert!(cell.activation < 0.1);
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
