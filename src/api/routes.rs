pub fn route<'a>(method: &'a str, path: &'a str) -> Option<&'a str> {
    match (method, path) {
        ("POST", "/v1/inference") => Some("inference"),
        ("POST", "/v1/observe") => Some("observe"),
        ("POST", "/v1/experience") => Some("experience"),
        ("POST", "/v1/learn") => Some("learn"),
        ("POST", "/v1/query") => Some("query"),
        ("GET", "/v1/status") => Some("status"),
        ("POST", "/v1/checkpoint") => Some("checkpoint"),
        ("GET", "/v1/health") => Some("health"),
        _ => None,
    }
}
