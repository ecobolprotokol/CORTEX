use crate::types::*;

const WEIGHT_DECAY: Scalar = 0.001;
const LEARNING_RATE_DECAY: Scalar = 0.999;
const MIN_LEARNING_RATE: Scalar = 0.0001;
const PLASTICITY_THRESHOLD: Scalar = 0.5;
const HIGH_PLASTICITY: Scalar = 0.8;
const LOW_PLASTICITY: Scalar = 0.1;

pub fn compute_weight_update(
    current_weight: Scalar,
    activation: Scalar,
    confidence: Scalar,
    error: Scalar,
    learning_rate: Scalar,
    plasticity: Scalar,
) -> Scalar {
    let stability_plasticity = compute_stability_plasticity(current_weight, plasticity);
    let delta = learning_rate * activation * confidence * error * stability_plasticity;
    let decay = -WEIGHT_DECAY * current_weight;
    (current_weight + delta + decay).clamp(-1.0, 1.0)
}

pub fn compute_weight_update_with_eligibility(
    current_weight: Scalar,
    activation: Scalar,
    confidence: Scalar,
    error: Scalar,
    learning_rate: Scalar,
    plasticity: Scalar,
    eligibility: Scalar,
) -> Scalar {
    let stability_plasticity = compute_stability_plasticity(current_weight, plasticity);
    let delta = learning_rate * activation * confidence * error * eligibility * stability_plasticity;
    let decay = -WEIGHT_DECAY * current_weight;
    (current_weight + delta + decay).clamp(-1.0, 1.0)
}

fn compute_stability_plasticity(weight: Scalar, base_plasticity: Scalar) -> Scalar {
    let stability = weight.abs();
    if stability > PLASTICITY_THRESHOLD {
        LOW_PLASTICITY + (base_plasticity - LOW_PLASTICITY) * (1.0 - stability)
    } else {
        HIGH_PLASTICITY * base_plasticity + LOW_PLASTICITY * (1.0 - base_plasticity)
    }
}

pub fn learning_rate_with_decay(base_lr: Scalar, experience_count: u64) -> Scalar {
    (base_lr * LEARNING_RATE_DECAY.powi(experience_count as i32)).max(MIN_LEARNING_RATE)
}

pub fn update_eligibility_trace(current_trace: Scalar, activation: Scalar) -> Scalar {
    (current_trace * 0.95 + activation).min(1.0)
}

pub fn enforce_stability(delta: Scalar, max_change: Scalar) -> Scalar {
    delta.clamp(-max_change, max_change)
}

pub fn apply_weight_updates(
    column: &mut Column,
    error_signal: Scalar,
    learning_rate: Scalar,
    plasticity: Scalar,
    experience_count: u64,
) {
    let effective_lr = learning_rate_with_decay(learning_rate, experience_count);

    let active_ids: Vec<CellId> = column.cells.iter()
        .filter(|c| c.state == CellState::Active)
        .map(|c| c.id)
        .collect();

    let pattern_len = column.learned_pattern.len();

    for cell in &mut column.cells {
        let eligibility = cell.eligibility_trace;
        let activation = cell.activation;

        if active_ids.contains(&cell.id) {
            let confidence = activation;
            let new_weight = compute_weight_update_with_eligibility(
                cell.activation,
                activation,
                confidence,
                error_signal,
                effective_lr,
                plasticity,
                eligibility,
            );
            cell.activation = new_weight;

            if pattern_len > 0 {
                let idx = (cell.id.0 as usize) % pattern_len;
                let old = column.learned_pattern[idx];
                column.learned_pattern[idx] = old * 0.95 + activation * 0.05;
            }
        } else {
            cell.activation *= 1.0 - WEIGHT_DECAY;
            cell.eligibility_trace = update_eligibility_trace(eligibility, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_update_basic() {
        let w = compute_weight_update(0.5, 1.0, 1.0, 0.1, 0.01, 1.0);
        assert!(w > 0.5);
        assert!(w <= 1.0);
    }

    #[test]
    fn test_weight_update_bounds() {
        let w = compute_weight_update(0.99, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert!(w <= 1.0);
        let w2 = compute_weight_update(-0.99, 1.0, 1.0, -1.0, 1.0, 1.0);
        assert!(w2 >= -1.0);
    }

    #[test]
    fn test_enforce_stability() {
        assert_eq!(enforce_stability(0.5, 0.3), 0.3);
        assert_eq!(enforce_stability(-0.5, 0.3), -0.3);
        assert_eq!(enforce_stability(0.1, 0.3), 0.1);
    }

    #[test]
    fn test_learning_rate_decay() {
        let lr0 = learning_rate_with_decay(0.01, 0);
        let lr100 = learning_rate_with_decay(0.01, 100);
        let lr1000 = learning_rate_with_decay(0.01, 1000);
        assert!(lr0 >= lr100);
        assert!(lr100 >= lr1000);
        assert!(lr1000 >= MIN_LEARNING_RATE);
    }

    #[test]
    fn test_weight_update_with_eligibility() {
        let w1 = compute_weight_update(0.5, 1.0, 1.0, 0.1, 0.01, 1.0);
        let w2 = compute_weight_update_with_eligibility(0.5, 1.0, 1.0, 0.1, 0.01, 1.0, 1.0);
        assert!((w1 - w2).abs() < 0.001);

        let w3 = compute_weight_update_with_eligibility(0.5, 1.0, 1.0, 0.5, 0.1, 1.0, 0.5);
        assert!(w3 > 0.5);
        let w4 = compute_weight_update_with_eligibility(0.5, 1.0, 1.0, 0.5, 0.1, 1.0, 1.0);
        assert!(w4 > w3);
    }

    #[test]
    fn test_stability_plasticity() {
        let p = compute_stability_plasticity(0.1, 0.5);
        assert!(p > 0.4);
        let p2 = compute_stability_plasticity(0.9, 0.5);
        assert!(p2 < p);
    }

    #[test]
    fn test_apply_weight_updates() {
        let mut column = Column {
            id: ColumnId(0),
            cells: vec![
                Cell {
                    id: CellId(0),
                    state: CellState::Active,
                    activation: 0.5,
                    prediction_vector: vec![],
                    refractory_steps: 0,
                    adaptation_level: 0.0,
                    burst_counter: 0,
                    eligibility_trace: 0.8,
                },
                Cell {
                    id: CellId(1),
                    state: CellState::Inhibited,
                    activation: 0.3,
                    prediction_vector: vec![],
                    refractory_steps: 0,
                    adaptation_level: 0.0,
                    burst_counter: 0,
                    eligibility_trace: 0.2,
                },
            ],
            active_cells: vec![CellId(0)],
            activation_threshold: 0.5,
            learned_pattern: vec![0.5, 0.3],
        };

        let old_active = column.cells[0].activation;
        let old_inactive = column.cells[1].activation;

        apply_weight_updates(&mut column, 0.1, 0.01, 1.0, 0);

        assert!(column.cells[0].activation != old_active || column.cells[1].activation < old_inactive);
    }

    #[test]
    fn test_eligibility_trace_update() {
        let t0 = update_eligibility_trace(0.0, 1.0);
        assert!(t0 > 0.9);
        let t1 = update_eligibility_trace(t0, 0.0);
        assert!(t1 < t0);
    }
}
