use cortex::neural::cell::{Cell, CellState};
use cortex::neural::column::Column;
use cortex::neural::field::Field;
use cortex::neural::plasticity::PlasticityRule;
use cortex::neural::temporal::TemporalBuffer;
use cortex::types::ids::CellId;

#[test]
fn test_cell_activation() {
    let mut cell = Cell::new(CellId::from(1));
    cell.set_activation(0.8);
    cell.activate(0.5);
    assert_eq!(cell.state, CellState::Active);
    assert!(cell.is_active());
}

#[test]
fn test_cell_inhibition() {
    let mut cell = Cell::new(CellId::from(1));
    cell.set_activation(0.8);
    cell.activate(0.5);
    cell.inhibit();
    assert_eq!(cell.state, CellState::Inhibited);
    assert_eq!(cell.activation, 0.0);
}

#[test]
fn test_cell_adapt() {
    let mut cell = Cell::new(CellId::from(1));
    cell.set_activation(0.5);
    cell.start_learning();
    cell.adapt(0.1);
    assert!(cell.weight > 0.0);
}

#[test]
fn test_column_competition() {
    let mut column = Column::new(10);
    column.set_activations(&[0.1, 0.9, 0.3, 0.8, 0.2, 0.7, 0.4, 0.6, 0.5, 0.15]);
    let active = column.compete(0.3);
    assert!(!active.is_empty());
    assert!(active.len() <= 3);
}

#[test]
fn test_field_sparsity() {
    let mut field = Field::new(4, 10);
    for column in &mut field.columns {
        for cell in &mut column.cells {
            cell.set_activation(0.5);
        }
        let _ = column.compete(0.3);
    }
    let total_active: usize = field.columns.iter().map(|c| c.active_cell_count()).sum();
    field.enforce_sparsity(5);
    let total_after: usize = field.columns.iter().map(|c| c.active_cell_count()).sum();
    assert!(total_after <= total_active);
}

#[test]
fn test_plasticity_update() {
    let rule = PlasticityRule::new(0.01, 0.1);
    let update = rule.compute_update(0.5, 0.8, 0.3, 1.0);
    assert!(update.abs() <= 0.1);
}

#[test]
fn test_temporal_buffer() {
    let mut buffer = TemporalBuffer::new(5);
    for i in 0..10 {
        buffer.encode(vec![i as f32, i as f32 + 1.0]);
    }
    let recent = buffer.last_n(3);
    assert_eq!(recent.len(), 3);
}
