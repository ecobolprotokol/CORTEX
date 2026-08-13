use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::persistence::format::FormatHandler;
use cortex::runtime::Runtime;
use cortex::transaction::invariant::StateInvariant;

fn make_config(suffix: &str) -> CortexConfig {
    let mut cfg = CortexConfig::default();
    cfg.persistence.state = format!("/tmp/cortex_fault_{}.cx", suffix);
    cfg.persistence.checkpoint_interval = 3;
    cfg.learning.consolidation_interval = 5;
    cfg
}

#[test]
fn fault_corrupted_state_file_falls_back_to_default() {
    let path = "/tmp/cortex_fault_corrupt.cx";
    std::fs::write(path, b"this is not valid state data").ok();
    let mut cfg = make_config("corrupt");
    cfg.persistence.state = path.to_string();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    assert_eq!(rt.state.metadata.episode_count, 0);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(path);
}

#[test]
fn fault_truncated_state_file_falls_back() {
    let path = "/tmp/cortex_fault_truncated.cx";
    std::fs::write(path, b"short").ok();
    let mut cfg = make_config("truncated");
    cfg.persistence.state = path.to_string();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    assert_eq!(rt.state.metadata.episode_count, 0);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(path);
}

#[test]
fn fault_empty_state_file_falls_back() {
    let path = "/tmp/cortex_fault_empty.cx";
    std::fs::write(path, b"").ok();
    let mut cfg = make_config("empty");
    cfg.persistence.state = path.to_string();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    assert_eq!(rt.state.metadata.episode_count, 0);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(path);
}

#[test]
fn fault_invalid_magic_bytes_falls_back() {
    let handler = FormatHandler::new();
    let state = cortex::types::state::CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();
    let mut corrupted = handler.serialize(&bincode_data).unwrap();
    if corrupted.len() > 4 {
        corrupted[0] = 0xFF;
        corrupted[1] = 0xFF;
        corrupted[2] = 0xFF;
        corrupted[3] = 0xFF;
    }
    let path = "/tmp/cortex_fault_magic.cx";
    std::fs::write(path, &corrupted).ok();
    let mut cfg = make_config("magic");
    cfg.persistence.state = path.to_string();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    assert_eq!(rt.state.metadata.episode_count, 0);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(path);
}

#[test]
fn fault_checksum_mismatch_rejected() {
    let handler = FormatHandler::new();
    let state = cortex::types::state::CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();
    let mut corrupted = handler.serialize(&bincode_data).unwrap();
    if corrupted.len() > 40 {
        corrupted[8] ^= 0xFF;
    }
    let result = handler.deserialize(&corrupted);
    assert!(result.is_err(), "Corrupted checksum should be rejected");
}

#[test]
fn fault_nonexistent_directory_for_state() {
    let mut cfg = make_config("nodir");
    cfg.persistence.state = "/nonexistent/dir/state.cx".to_string();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let result = rt.save_state();
    assert!(result.is_err(), "Save to nonexistent dir should fail");
    rt.shutdown().unwrap();
}

#[test]
fn fault_empty_input_handled_gracefully() {
    let cfg = make_config("empty_input");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let result = rt.process("");
    assert!(result.is_ok(), "Empty input should be handled gracefully");
    assert!(!result.unwrap().is_empty());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn fault_very_long_input_handled() {
    let cfg = make_config("long_input");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let long_input = "word ".repeat(10000);
    let result = rt.process(&long_input);
    assert!(result.is_ok(), "Long input should be handled");
    assert!(StateInvariant::validate_state(&rt.state).is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn fault_special_characters_in_input() {
    let cfg = make_config("special_chars");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let inputs = vec![
        "Hello\x00World",
        "Test\nNewline\tTab",
        "Unicode: \u{00e9}\u{00f1}\u{00fc}",
        "Quotes: \"double\" and 'single'",
        "Backslash: \\path\\to\\file",
        "",
    ];
    for input in &inputs {
        let result = rt.process(input);
        assert!(
            result.is_ok(),
            "Special chars should be handled: {:?}",
            input
        );
    }
    assert!(StateInvariant::validate_state(&rt.state).is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn fault_rapid_process_cycles_stable() {
    let cfg = make_config("rapid");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    for i in 0..100 {
        let _ = rt.process(&format!("Rapid cycle {}", i));
    }
    assert!(StateInvariant::validate_state(&rt.state).is_ok());
    assert!(rt.state.metadata.episode_count >= 100);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn fault_policy_gate_blocks_when_disabled() {
    let mut cfg = make_config("policy");
    cfg.learning.enabled = false;
    cfg.reasoning.enabled = false;
    cfg.planning.enabled = false;
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let result = rt.process("Test with disabled subsystems");
    assert!(result.is_ok());
    assert!(StateInvariant::validate_state(&rt.state).is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn fault_state_save_load_roundtrip() {
    let cfg = make_config("roundtrip");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let _ = rt.process("First observation");
    let _ = rt.process("Second observation");
    let episodes_before = rt.state.metadata.episode_count;
    let state_path = rt.config.persistence.state.clone();
    rt.save_state().unwrap();
    rt.shutdown().unwrap();

    let mut cfg2 = make_config("roundtrip2");
    cfg2.persistence.state = state_path.clone();
    let mut rt2 = CortexRuntime::new(cfg2).unwrap();
    rt2.boot().unwrap();
    assert!(rt2.state.metadata.episode_count >= episodes_before);
    rt2.shutdown().unwrap();
    let _ = std::fs::remove_file(&state_path);
}
