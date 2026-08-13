use crate::types::*;
use crate::neural::TemporalEncoding;

const SLIDING_WINDOW: usize = 10;
const SHORT_TERM_WINDOW: usize = 3;
const LONG_TERM_WINDOW: usize = 10;

pub fn encode(state: &NeuralState) -> TemporalEncoding {
    let window = sliding_window(state, SLIDING_WINDOW);
    let current_pattern = extract_pattern(state);

    let sequence = current_pattern.clone();
    let transition = compute_transitions(&window);
    let recurrence = compute_recurrence(&window);
    let dependency = compute_dependencies(&window);
    let coherence = compute_coherence(&window);
    let anomaly_score = compute_anomaly(&window, &current_pattern);
    let short_term = compute_short_term(&window);
    let long_term = compute_long_term(&window);

    TemporalEncoding {
        sequence,
        transition,
        recurrence,
        dependency,
        coherence,
        anomaly_score,
        short_term,
        long_term,
    }
}

fn sliding_window<'a>(state: &'a NeuralState, max_size: usize) -> Vec<&'a NeuralField> {
    let start = state.temporal_buffer.len().saturating_sub(max_size);
    state.temporal_buffer[start..].iter().collect()
}

fn extract_pattern(state: &NeuralState) -> Vec<Scalar> {
    state.fields.iter().map(|f| f.average_activation).collect()
}

fn compute_transitions(window: &[&NeuralField]) -> Vec<Scalar> {
    if window.len() < 2 {
        return vec![0.0; 3];
    }

    let len = window.len();
    let prev = window[len - 2].average_activation;
    let curr = window[len - 1].average_activation;
    let step1 = curr - prev;

    let step2 = if len >= 4 {
        let older = window[len - 4].average_activation;
        curr - older
    } else {
        0.0
    };

    let step3 = if len >= 3 {
        let first = window[0].average_activation;
        (curr - first) / len as Scalar
    } else {
        0.0
    };

    vec![step1, step2, step3]
}

fn compute_coherence(window: &[&NeuralField]) -> Scalar {
    if window.len() < 2 {
        return 1.0;
    }

    let values: Vec<Scalar> = window.iter().map(|f| f.average_activation).collect();
    let mean = values.iter().sum::<Scalar>() / values.len() as Scalar;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<Scalar>() / values.len() as Scalar;
    let std = variance.sqrt();

    (1.0 - std * 2.0).max(0.0)
}

fn compute_anomaly(window: &[&NeuralField], current: &[Scalar]) -> Scalar {
    if window.len() < 3 || current.is_empty() {
        return 0.0;
    }

    let recent_avg: Scalar = window.iter().rev().take(3).map(|f| f.average_activation).sum::<Scalar>() / 3.0;
    let current_avg: Scalar = current.iter().sum::<Scalar>() / current.len() as Scalar;

    (current_avg - recent_avg).abs()
}

fn compute_dependencies(window: &[&NeuralField]) -> Vec<Scalar> {
    if window.len() < 3 {
        return vec![0.0; 3];
    }

    let len = window.len();
    let a = window[len - 3].average_activation;
    let b = window[len - 2].average_activation;
    let c = window[len - 1].average_activation;

    vec![
        b - a,
        c - b,
        (c - a).abs(),
    ]
}

fn compute_recurrence(window: &[&NeuralField]) -> Scalar {
    if window.len() < 2 {
        return 0.0;
    }
    let len = window.len();
    let curr = window[len - 1].average_activation;
    let prev = window[len - 2].average_activation;
    let similarity = 1.0 - (curr - prev).abs();

    if window.len() >= 4 {
        let older = window[len - 4].average_activation;
        let trend = (curr - older).abs();
        similarity * 0.7 + (1.0 - trend) * 0.3
    } else {
        similarity
    }
}

fn compute_short_term(window: &[&NeuralField]) -> Vec<Scalar> {
    let n = window.len().min(SHORT_TERM_WINDOW);
    if n == 0 {
        return Vec::new();
    }
    window.iter().rev().take(n).map(|f| f.average_activation).collect()
}

fn compute_long_term(window: &[&NeuralField]) -> Vec<Scalar> {
    let n = window.len().min(LONG_TERM_WINDOW);
    if n == 0 {
        return Vec::new();
    }
    window.iter().rev().take(n).map(|f| f.average_activation).collect()
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
    if state.temporal_buffer.len() > SLIDING_WINDOW {
        state.temporal_buffer.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_field(activation: Scalar) -> NeuralField {
        NeuralField {
            id: FieldId(0),
            columns: Vec::new(),
            average_activation: activation,
            coherence: 0.0,
        }
    }

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
        assert_eq!(encoding.coherence, 1.0);
        assert_eq!(encoding.anomaly_score, 0.0);
    }

    #[test]
    fn test_encode_with_history() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.3), make_field(0.5)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(!encoding.transition.is_empty());
        assert!(encoding.recurrence > 0.0);
    }

    #[test]
    fn test_coherence_stable() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.5), make_field(0.5), make_field(0.5)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.coherence > 0.9);
    }

    #[test]
    fn test_coherence_unstable() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.0), make_field(1.0), make_field(0.0)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.coherence < 0.5);
    }

    #[test]
    fn test_anomaly_detection() {
        let state = NeuralState {
            fields: vec![make_field(0.9)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.1), make_field(0.1), make_field(0.1)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.anomaly_score > 0.5);
    }

    #[test]
    fn test_no_anomaly() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.5), make_field(0.5), make_field(0.5)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.anomaly_score < 0.01);
    }

    #[test]
    fn test_short_term_long_term() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![
                make_field(0.1), make_field(0.2), make_field(0.3),
                make_field(0.4), make_field(0.5),
            ],
            prediction: None,
        };
        let encoding = encode(&state);
        assert!(encoding.short_term.len() <= 3);
        assert!(encoding.long_term.len() <= 10);
        assert!(!encoding.short_term.is_empty());
    }

    #[test]
    fn test_update_temporal_buffer() {
        let mut state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: Vec::new(),
            prediction: None,
        };
        for _ in 0..15 {
            update_temporal_buffer(&mut state);
        }
        assert!(state.temporal_buffer.len() <= SLIDING_WINDOW);
    }

    #[test]
    fn test_transitions() {
        let state = NeuralState {
            fields: vec![make_field(0.5)],
            active_cells: Vec::new(),
            active_columns: Vec::new(),
            temporal_buffer: vec![make_field(0.1), make_field(0.3), make_field(0.5)],
            prediction: None,
        };
        let encoding = encode(&state);
        assert_eq!(encoding.transition.len(), 3);
        assert!((encoding.transition[0] - 0.2).abs() < 0.001);
    }
}
