use super::cell::Cell;
use crate::types::ids::CellId;

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

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn set_activations(&mut self, activations: &[f32]) {
        for (cell, &act) in self.cells.iter_mut().zip(activations.iter()) {
            cell.set_activation(act);
        }
    }

    pub fn compete(&mut self, sparsity: f32) -> Vec<CellId> {
        let max_active = ((self.cells.len() as f32 * sparsity).ceil() as usize).max(1);
        let max_active = max_active.min(self.cells.len());

        let mut indexed: Vec<(usize, f32)> = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.activation))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let winners: Vec<usize> = indexed[..max_active].iter().map(|(i, _)| *i).collect();

        let winner_ids: Vec<CellId> = winners.iter().map(|&i| self.cells[i].id).collect();

        for (i, cell) in self.cells.iter_mut().enumerate() {
            if winners.contains(&i) {
                cell.activate(0.0);
            } else {
                cell.inhibit();
            }
        }

        self.active_cells = winner_ids.clone();
        winner_ids
    }

    pub fn total_activation(&self) -> f32 {
        self.cells.iter().map(|c| c.activation).sum()
    }

    pub fn average_activation(&self) -> f32 {
        if self.cells.is_empty() {
            return 0.0;
        }
        self.total_activation() / self.cells.len() as f32
    }

    pub fn active_cell_count(&self) -> usize {
        self.active_cells.len()
    }

    pub fn reset(&mut self) {
        self.active_cells.clear();
        for cell in &mut self.cells {
            cell.reset();
        }
    }
}
