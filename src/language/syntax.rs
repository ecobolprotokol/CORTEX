pub struct SyntaxAnalyzer;

impl SyntaxAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: &[String]) -> Vec<String> {
        tokens.to_vec()
    }
}
