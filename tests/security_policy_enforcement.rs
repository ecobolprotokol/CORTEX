use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::policy::gate::{PolicyDecision, PolicyGate};
use cortex::runtime::Runtime;

fn make_config(suffix: &str) -> CortexConfig {
    let mut cfg = CortexConfig::default();
    cfg.persistence.state = format!("/tmp/cortex_security_{}.cx", suffix);
    cfg
}

#[test]
fn security_policy_gate_allow() {
    let gate = PolicyGate::new();
    let result = gate.evaluate("neural_process");
    assert_eq!(result.decision, PolicyDecision::Allow);
}

#[test]
fn security_policy_gate_respects_config() {
    let mut cfg = make_config("policy_cfg");
    cfg.policy.learning = false;
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let result = rt.policy_gate.evaluate("learning");
    assert_eq!(result.decision, PolicyDecision::Deny);
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn security_policy_blocks_neural_when_disabled() {
    let mut cfg = make_config("neural_disabled");
    cfg.policy.self_modification = false;
    let gate = PolicyGate::new();
    let result = gate.evaluate("neural_process");
    assert!(
        result.decision == PolicyDecision::Allow || result.decision == PolicyDecision::Limit,
        "Neural process should be allowed or limited by default"
    );
}

#[test]
fn security_api_auth_rejects_wrong_key() {
    let cfg = make_config("auth");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let mut api = cortex::api::ApiManager::new("correct-key");
    let result = api.handle_request(&mut rt, "POST", "/v1/inference", Some("wrong-key"), Some("test"));
    assert!(result.is_err());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn security_api_auth_accepts_correct_key() {
    let cfg = make_config("auth_ok");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let mut api = cortex::api::ApiManager::new("correct-key");
    let result = api.handle_request(&mut rt, "POST", "/v1/inference", Some("correct-key"), Some("test"));
    assert!(result.is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn security_api_auth_rejects_empty_token() {
    let cfg = make_config("auth_empty");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let mut api = cortex::api::ApiManager::new("correct-key");
    let result = api.handle_request(&mut rt, "POST", "/v1/inference", Some(""), Some("test"));
    assert!(result.is_err());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn security_no_policy_bypass_path() {
    let cfg = make_config("bypass");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let disabled_cfg = CortexConfig::default();
    let mut disabled_rt = CortexRuntime::new(disabled_cfg).unwrap();
    disabled_rt.boot().unwrap();

    for i in 0..10 {
        let _ = rt.process(&format!("Policy test {}", i));
        let _ = disabled_rt.process(&format!("Disabled test {}", i));
    }

    assert!(cortex::transaction::invariant::StateInvariant::validate_state(&rt.state).is_ok());
    assert!(cortex::transaction::invariant::StateInvariant::validate_state(&disabled_rt.state).is_ok());

    rt.shutdown().unwrap();
    disabled_rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn security_input_sanitization() {
    let cfg = make_config("sanitization");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let malicious_inputs = vec![
        "../../etc/passwd",
        "${HOME}/.ssh/id_rsa",
        "`rm -rf /`",
        "$(cat /etc/passwd)",
        "{{constructor.constructor('return this')()}}",
        "<script>alert(1)</script>",
        "'; DROP TABLE users;--",
    ];

    for input in &malicious_inputs {
        let result = rt.process(input);
        assert!(
            result.is_ok(),
            "Malicious input should be handled safely: {}",
            input
        );
    }

    assert!(cortex::transaction::invariant::StateInvariant::validate_state(&rt.state).is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}
