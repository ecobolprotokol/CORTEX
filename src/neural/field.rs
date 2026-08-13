use crate::types::scalars::Scalar;
use super::column::Column;

#[derive(Debug, Clone)]
pub struct Field {
    pub columns: Vec<Column>,
    pub average_activation: Scalar,
    pub coherence: Scalar,
}

impl Field {
    pub fn new(column_count: u32, cells_per_column: u32) -> Self {
        let columns = (0..column_count)
            .map(|_| Column::new(cells_per_column))
            .collect();
        Self {
            columns,
            average_activation: 0.0,
            coherence: 0.0,
        }
    }

    pub fn enforce_sparsity(&mut self, max_active: usize) {
        let total_active: usize = self.columns.iter()
            .map(|c| c.active_cells.len())
            .sum();

        if total_active > max_active {
            for column in &mut self.columns {
                if column.active_cells.len() > 1 {
                    let _ = column.active_cells.pop();
                }
            }
        }

        let total: Scalar = self.columns.iter()
            .flat_map(|c| c.cells.iter())
            .map(|c| c.activation)
            .sum();
        let count = self.columns.iter()
            .map(|c| c.cells.len())
            .sum::<usize>() as Scalar;
        self.average_activation = if count > 0.0 { total / count } else { 0.0 };
    }
}
