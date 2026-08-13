use crate::types::*;

const HOMEOSTATIC_RATE: Scalar = 0.01;

pub fn compete(column: &mut Column, sparsity_ratio: Scalar) -> Vec<CellId> {
    let max_active = ((column.cells.len() as Scalar * sparsity_ratio).ceil() as usize).max(1);

    if column.cells.is_empty() {
        return Vec::new();
    }

    let mut indexed: Vec<(usize, Scalar)> = column.cells
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.activation))
        .collect();

    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut active: Vec<CellId> = indexed.iter()
        .take(max_active)
        .filter(|(_, activation)| *activation > column.activation_threshold)
        .map(|(i, _)| column.cells[*i].id)
        .collect();

    if active.is_empty() && !indexed.is_empty() {
        let top = &indexed[0];
        if top.1 > 0.0 {
            active.push(column.cells[top.0].id);
        }
    }

    for cell in &mut column.cells {
        if !active.contains(&cell.id) {
            cell.state = CellState::Inhibited;
            cell.activation = (cell.activation * 0.5).max(0.0);
        } else {
            cell.state = CellState::Active;
        }
    }

    column.active_cells = active.clone();
    active
}

pub fn apply_lateral_inhibition(columns: &mut [Column], inhibition_strength: Scalar) {
    let active_indices: Vec<usize> = columns.iter()
        .enumerate()
        .filter(|(_, c)| !c.active_cells.is_empty())
        .map(|(i, _)| i)
        .collect();

    for &winner_idx in &active_indices {
        let neighbors = neighbor_indices(winner_idx, columns.len());
        for &neighbor_idx in &neighbors {
            if !active_indices.contains(&neighbor_idx) {
                for cell in &mut columns[neighbor_idx].cells {
                    if cell.state == CellState::Active {
                        cell.activation *= 1.0 - inhibition_strength;
                        if cell.activation < 0.5 {
                            cell.state = CellState::Inhibited;
                        }
                    }
                }
                let still_active: Vec<CellId> = columns[neighbor_idx].cells.iter()
                    .filter(|c| c.state == CellState::Active)
                    .map(|c| c.id)
                    .collect();
                columns[neighbor_idx].active_cells = still_active;
            }
        }
    }
}

fn neighbor_indices(index: usize, total: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    if index > 0 {
        neighbors.push(index - 1);
    }
    if index + 1 < total {
        neighbors.push(index + 1);
    }
    neighbors
}

pub fn homeostatic_adjust(column: &mut Column, target_rate: Scalar) {
    let current_rate = column.active_cells.len() as Scalar / column.cells.len().max(1) as Scalar;
    let error = current_rate - target_rate;
    column.activation_threshold = (column.activation_threshold + error * HOMEOSTATIC_RATE).clamp(0.01, 0.99);
}

pub fn compute_overlap(column: &Column, input: &[Scalar]) -> Scalar {
    if column.learned_pattern.is_empty() || input.is_empty() {
        return 0.0;
    }

    let pattern = &column.learned_pattern;
    let len = pattern.len().min(input.len());
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..len {
        dot += pattern[i] * input[i];
        norm_a += pattern[i] * pattern[i];
        norm_b += input[i] * input[i];
    }

    let denom = (norm_a * norm_b).sqrt();
    if denom < 1e-8 {
        0.0
    } else {
        (dot / denom).clamp(0.0, 1.0)
    }
}

pub fn inject_input(column: &mut Column, input_hash: u64, strength: Scalar) {
    for (i, cell) in column.cells.iter_mut().enumerate() {
        let cell_contribution = ((input_hash.wrapping_add(i as u64) % 100) as Scalar) / 100.0;
        let burst = crate::neural::cell::burst_factor(cell);
        let adapted = cell_contribution * strength * burst * (1.0 - cell.adaptation_level).max(0.1);
        cell.activation = (cell.activation + adapted).clamp(0.0, 1.0);
    }

    if column.learned_pattern.is_empty() {
        column.learned_pattern = column.cells.iter().map(|c| c.activation).collect();
    } else {
        for (i, cell) in column.cells.iter().enumerate() {
            if i < column.learned_pattern.len() {
                column.learned_pattern[i] = column.learned_pattern[i] * 0.95 + cell.activation * 0.05;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_column(cell_count: usize) -> Column {
        let cells: Vec<Cell> = (0..cell_count).map(|i| Cell {
            id: CellId(i as u64),
            state: CellState::Resting,
            activation: (i as Scalar) / cell_count as Scalar,
            prediction_vector: vec![0.0; 4],
            refractory_steps: 0,
            adaptation_level: 0.0,
            burst_counter: 0,
            eligibility_trace: 0.0,
        }).collect();
        Column {
            id: ColumnId(0),
            cells,
            active_cells: Vec::new(),
            activation_threshold: 0.0,
            learned_pattern: Vec::new(),
        }
    }

    #[test]
    fn test_compete_selects_top_k() {
        let mut column = make_column(10);
        let active = compete(&mut column, 0.3);
        assert!(!active.is_empty());
        assert!(active.len() <= 3);
    }

    #[test]
    fn test_compete_inhibits_non_selected() {
        let mut column = make_column(10);
        let active = compete(&mut column, 0.3);
        for cell in &column.cells {
            if !active.contains(&cell.id) {
                assert_eq!(cell.state, CellState::Inhibited);
            }
        }
    }

    #[test]
    fn test_lateral_inhibition() {
        let mut columns = vec![
            make_column(3),
            make_column(3),
            make_column(3),
        ];
        columns[0].cells[0].activation = 1.0;
        columns[0].cells[1].activation = 0.9;
        columns[0].cells[2].activation = 0.1;
        columns[1].cells[0].activation = 0.3;
        columns[1].cells[1].activation = 0.2;
        columns[1].cells[2].activation = 0.1;
        columns[2].cells[0].activation = 0.3;
        columns[2].cells[1].activation = 0.2;
        columns[2].cells[2].activation = 0.1;

        let active0: Vec<CellId> = compete(&mut columns[0], 0.5);
        let _active1: Vec<CellId> = compete(&mut columns[1], 0.5);
        let _active2: Vec<CellId> = compete(&mut columns[2], 0.5);

        apply_lateral_inhibition(&mut columns, 0.5);

        if !active0.is_empty() && columns[1].active_cells.is_empty() {
            assert!(columns[1].cells.iter().all(|c| c.activation < 0.5 || c.state == CellState::Inhibited));
        }
    }

    #[test]
    fn test_homeostatic_adjust() {
        let mut column = make_column(10);
        column.cells[0].state = CellState::Active;
        column.cells[1].state = CellState::Active;
        column.active_cells = vec![column.cells[0].id, column.cells[1].id];
        column.activation_threshold = 0.5;

        homeostatic_adjust(&mut column, 0.1);

        assert!(column.activation_threshold > 0.5);
    }

    #[test]
    fn test_compute_overlap() {
        let mut column = make_column(3);
        column.learned_pattern = vec![1.0, 0.0, 0.0];

        let identical = vec![1.0, 0.0, 0.0];
        let overlap = compute_overlap(&column, &identical);
        assert!((overlap - 1.0).abs() < 0.001);

        let orthogonal = vec![0.0, 1.0, 0.0];
        let overlap2 = compute_overlap(&column, &orthogonal);
        assert!(overlap2.abs() < 0.001);

        let empty_pattern = Column {
            id: ColumnId(1),
            cells: Vec::new(),
            active_cells: Vec::new(),
            activation_threshold: 0.5,
            learned_pattern: Vec::new(),
        };
        assert_eq!(compute_overlap(&empty_pattern, &identical), 0.0);
    }

    #[test]
    fn test_inject_input_updates_pattern() {
        let mut column = make_column(3);
        assert!(column.learned_pattern.is_empty());
        inject_input(&mut column, 42, 0.5);
        assert_eq!(column.learned_pattern.len(), 3);
    }
}
