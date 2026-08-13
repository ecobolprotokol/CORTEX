use crate::types::*;

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

    let active: Vec<CellId> = indexed.iter()
        .take(max_active)
        .filter(|(_, activation)| *activation > 0.0)
        .map(|(i, _)| column.cells[*i].id)
        .collect();

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

pub fn inject_input(column: &mut Column, input_hash: u64, strength: Scalar) {
    for (i, cell) in column.cells.iter_mut().enumerate() {
        let cell_contribution = ((input_hash.wrapping_add(i as u64) % 100) as Scalar) / 100.0;
        cell.activation = (cell.activation + cell_contribution * strength).clamp(0.0, 1.0);
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
        }).collect();
        Column {
            id: ColumnId(0),
            cells,
            active_cells: Vec::new(),
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
}
