use crate::types::*;
use crate::neural::TemporalEncoding;

pub fn encode(state: &NeuralState) -> TemporalEncoding {
    let current: Vec<Scalar> = state.fields.iter().map(|f| f.average_activation).collect();

    let sequence = current.clone();

    let transition = if state.temporal_buffer.len() >= 2 {
        let prev = &state.temporal_buffer[state.temporal_buffer.len() - 2];
        let curr = state.temporal_buffer.last().unwrap();
        let prev_avg = prev.average_activation;
        let curr_avg = curr.average_activation;
        vec![curr_avg - prev_avg]
    } else if state.temporal_buffer.len() == 1 {
        let prev_avg = state.temporal_buffer[0].average_activation;
        let curr_avg = current.first().copied().unwrap_or(0.0);
        vec![curr_avg - prev_avg]
    } else {
        vec![0.0]
    };

    let recurrence = compute_recurrence(&state.temporal_buffer);

    let dependency = if state.temporal_buffer.len() >= 3 {
        let len = state.temporal_buffer.len();
        let a = state.temporal_buffer[len - 3].average_activation;
        let b = state.temporal_buffer[len - 2].average_activation;
        let c = state.temporal_buffer[len - 1].average_activation;
        vec![
            b - a,
            c - b,
            (c - a).abs(),
        ]
    } else {
        vec![0.0; 3]
    };

    TemporalEncoding {
        sequence,
        transition,
        recurrence,
        dependency,
    }
}

fn compute_recurrence(buffer: &[NeuralField]) -> Scalar {
    if buffer.len() < 2 {
        return 0.0;
    }
    let len = buffer.len();
    let curr = buffer[len - 1].average_activation;
    let prev = buffer[len - 2].average_activation;
    let similarity = 1.0 - (curr - prev).abs();

    if buffer.len() >= 4 {
        let older = buffer[len - 4].average_activation;
        let trend = (curr - older).abs();
        similarity * 0.7 + (1.0 - trend) * 0.3
    } else {
        similarity
    }
}

pub fn update_temporal_buffer(state: &mut NeuralState) {
    let snapshot = NeuralField {
        id: FieldId(0),
        columns: Vec::new(),
        average_activation: state.fields.iter().map(|f| f.average_activation).sum::<Scalar>()
            / state.fields.len().max(1) as Scalar,
        coherence: 0.0,
    };
    state.temporal_buffer.push(snapshot);
    if state.temporal_buffer.len() > 10 {
        state.temporal_buffer.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        let state = NeuralState {
            fields: Vec::new(),
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: Vec::new(),
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.sequence.is_empty());
        assert_eq!(encoding.recurrence, 0.0);
    }

    #[test]
    fn test_encode_with_history() {
        let state = NeuralState {
            fields: vec![NeuralField {
                id: FieldId(0),
                columns: Vec::new(),
                average_activation: 0.5,
                coherence: 0.5,
            }],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![
                NeuralField { id: FieldId(0), columns: Vec::new(), average_activation: 0.3, coherence: 0.0 },
                NeuralField { id: FieldId(0), columns: Vec::new(), average_activation: 0.5, coherence: 0.0 },
            ],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(!encoding.transition.is_empty());
        assert!(encoding.recurrence > 0.0);
    }
}
