pub fn extract_text(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag = true;
            }
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag && !in_script => {
                text.push(ch);
            }
            _ => {}
        }
    }
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn strip_html_tags(html: &str) -> String {
    extract_text(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text() {
        let html = "<p>Hello <b>world</b></p>";
        let text = extract_text(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("world"));
        assert!(!text.contains("<p>"));
    }

    #[test]
    fn test_strip_html() {
        let html = "<div>Test content</div>";
        let text = strip_html_tags(html);
        assert!(text.contains("Test content"));
    }
}
