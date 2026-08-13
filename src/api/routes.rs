pub struct Router;

impl Router {
    pub fn new() -> Self { Self }

    pub fn route(&self, method: &str, path: &str) -> String {
        format!("{} {}", method, path)
    }
}
