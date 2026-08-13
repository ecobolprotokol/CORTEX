//! Persistence roundtrip integration test.

use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

#[test]
fn test_state_persistence() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    // Process some observations to build state
    let _ = runtime.process("Test observation 1");
    let _ = runtime.process("Test observation 2");

    // Shutdown should save state
    runtime.shutdown().unwrap();

    // Boot again should load state
    let config2 = CortexConfig::default();
    let mut runtime2 = CortexRuntime::new(config2).unwrap();
    runtime2.boot().unwrap();

    // Should be able to process after reload
    let response = runtime2.process("Post-reload observation").unwrap();
    assert!(!response.is_empty());
}
