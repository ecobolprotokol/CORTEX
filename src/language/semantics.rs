pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn extract(&self, tokens: &[String]) -> Vec<String> {
        tokens.to_vec()
    }
}
