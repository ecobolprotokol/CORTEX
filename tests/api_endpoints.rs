use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

#[test]
fn test_api_auth_rejects_no_key() {
    let result = cortex::api::auth::authenticate(None, Some("expected_key"));
    assert!(!result);
}

#[test]
fn test_api_auth_accepts_correct_key() {
    let result = cortex::api::auth::authenticate(Some("my_key"), Some("my_key"));
    assert!(result);
}

#[test]
fn test_api_auth_rejects_wrong_key() {
    let result = cortex::api::auth::authenticate(Some("wrong_key"), Some("expected_key"));
    assert!(!result);
}

#[test]
fn test_api_auth_both_none() {
    let result = cortex::api::auth::authenticate(None, None);
    assert!(result);
}

#[test]
fn test_api_routes_health() {
    let route = cortex::api::routes::route("GET", "/v1/health");
    assert!(route.is_some());
}

#[test]
fn test_api_routes_inference() {
    let route = cortex::api::routes::route("POST", "/v1/inference");
    assert!(route.is_some());
}

#[test]
fn test_api_routes_observe() {
    let route = cortex::api::routes::route("POST", "/v1/observe");
    assert!(route.is_some());
}

#[test]
fn test_api_routes_status() {
    let route = cortex::api::routes::route("GET", "/v1/status");
    assert!(route.is_some());
}

#[test]
fn test_api_routes_unknown() {
    let route = cortex::api::routes::route("GET", "/v1/unknown");
    assert!(route.is_none());
}
