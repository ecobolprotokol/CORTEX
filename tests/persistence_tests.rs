use cortex::persistence::format::FormatHandler;
use cortex::persistence::checkpoint::CheckpointManager;

#[test]
fn test_format_handler_serialize() {
    let handler = FormatHandler::new();
    let data = b"test data";
    let serialized = handler.serialize(data).unwrap();
    assert!(!serialized.is_empty());
    assert!(serialized.len() > data.len());
}

#[test]
fn test_format_handler_roundtrip() {
    let handler = FormatHandler::new();
    let data = b"test data for roundtrip";
    let serialized = handler.serialize(data).unwrap();
    let deserialized = handler.deserialize(&serialized).unwrap();
    assert_eq!(data.to_vec(), deserialized);
}

#[test]
fn test_format_handler_checksum() {
    let data = b"test data";
    let checksum1 = FormatHandler::compute_checksum(data);
    let checksum2 = FormatHandler::compute_checksum(data);
    assert_eq!(checksum1, checksum2);
}

#[test]
fn test_checkpoint_manager() {
    let mut manager = CheckpointManager::new(5);
    let checkpoint = manager.create_checkpoint(1024, 100);
    assert!(checkpoint.id.raw() > 0);
    assert_eq!(manager.checkpoints.len(), 1);
}

#[test]
fn test_checkpoint_manager_pruning() {
    let mut manager = CheckpointManager::new(3);
    for i in 0..5 {
        let _ = manager.create_checkpoint(1024 * i, 100 * i);
    }
    assert!(manager.checkpoints.len() <= 3);
}
