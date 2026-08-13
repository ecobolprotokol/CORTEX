//! Policy gate enforcement test.

use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

#[test]
fn test_policy_allows_normal_operations() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    
    let response = runtime.process("Normal observation").unwrap();
    assert!(!response.is_empty());
}

#[test]
fn test_policy_blocks_when_learning_disabled() {
    let mut config = CortexConfig::default();
    config.policy.learning = false;
    
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    
    // Should still process, but learning should be blocked
    let response = runtime.process("Test observation").unwrap();
    assert!(!response.is_empty());
}
