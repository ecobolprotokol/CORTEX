use cortex::api::auth::Authenticator;
use cortex::api::handlers::RequestHandler;
use cortex::api::routes::Router;

#[test]
fn test_authenticator_valid() {
    let auth = Authenticator::new("test-secret-key");
    let result = auth.validate("test-secret-key");
    assert!(result.is_ok());
}

#[test]
fn test_authenticator_invalid() {
    let auth = Authenticator::new("test-secret-key");
    let result = auth.validate("wrong-key");
    assert!(result.is_err());
}

#[test]
fn test_request_handler_inference() {
    let mut handler = RequestHandler::new();
    let response = handler.handle_inference("test input");
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

#[test]
fn test_request_handler_observe() {
    let mut handler = RequestHandler::new();
    let response = handler.handle_observe("test observation");
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

#[test]
fn test_request_handler_query() {
    let mut handler = RequestHandler::new();
    let response = handler.handle_query("test query");
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

#[test]
fn test_request_handler_status() {
    let handler = RequestHandler::new();
    let response = handler.handle_status();
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

#[test]
fn test_router() {
    let router = Router::new();
    let route = router.route("GET", "/v1/status").unwrap();
    assert!(!route.is_empty());
}
