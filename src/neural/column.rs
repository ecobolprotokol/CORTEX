use crate::types::ids::CellId;
use crate::types::scalars::Scalar;
use super::cell::Cell;

#[derive(Debug, Clone)]
pub struct Column {
    pub cells: Vec<Cell>,
    pub active_cells: Vec<CellId>,
}

impl Column {
    pub fn new(cell_count: u32) -> Self {
        let cells = (0..cell_count)
            .map(|i| Cell::new(CellId::from(i as u64)))
            .collect();
        Self {
            cells,
            active_cells: Vec::new(),
        }
    }

    pub fn compete(&mut self, sparsity: Scalar) -> Vec<CellId> {
        let max_active = ((self.cells.len() as Scalar * sparsity).ceil() as usize).max(1);

        let mut indexed: Vec<(usize, Scalar)> = self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.activation))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        self.active_cells = indexed[..max_active]
            .iter()
            .map(|(i, _)| self.cells[*i].id)
            .collect();

        for cell in &mut self.cells {
            if !self.active_cells.contains(&cell.id) {
                cell.inhibit();
            }
        }

        self.active_cells.clone()
    }
}
