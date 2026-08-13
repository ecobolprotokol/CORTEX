pub struct Decoder;

impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, concepts: &[String]) -> String {
        concepts.join(" ")
    }
}
