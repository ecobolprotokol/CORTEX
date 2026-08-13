pub struct LanguageModel;

impl LanguageModel {
    pub fn new() -> Self {
        Self
    }

    pub fn predict_next(&self, context: &[String]) -> Vec<(String, f32)> {
        let _ = context;
        Vec::new()
    }
}
