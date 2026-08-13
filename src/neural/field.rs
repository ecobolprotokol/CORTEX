use crate::types::*;

pub fn integrate_fields(fields: &[NeuralField], _context: &ContextState) -> Vec<Scalar> {
    fields.iter().map(|f| f.average_activation).collect()
}

pub fn enforce_sparsity(fields: &mut [NeuralField], max_active: usize) {
    for field in fields.iter_mut() {
        let total_cells: usize = field.columns.iter().map(|c| c.cells.len()).sum();
        let current_active: usize = field.columns.iter().map(|c| c.active_cells.len()).sum();
        if current_active > max_active {
            for column in &mut field.columns {
                column.active_cells.truncate(column.active_cells.len() * max_active / current_active.max(1));
            }
        }
    }
}
