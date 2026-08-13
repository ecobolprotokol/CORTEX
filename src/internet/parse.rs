pub struct ContentParser;

impl ContentParser {
    pub fn new() -> Self { Self }

    pub fn extract_text(&self, html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }
        result.trim().to_string()
    }
}
