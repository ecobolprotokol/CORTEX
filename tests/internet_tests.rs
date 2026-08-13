use cortex::internet::fetch::Fetcher;
use cortex::internet::parse::ContentParser;

#[test]
fn test_fetcher_creation() {
    let fetcher = Fetcher::new(15, 4);
    assert_eq!(fetcher.timeout_seconds, 15);
    assert_eq!(fetcher.max_response_mb, 4);
}

#[test]
fn test_content_parser_text_extraction() {
    let parser = ContentParser::new();
    let html = "<html><body><p>Hello world</p></body></html>";
    let text = parser.extract_text(html);
    assert!(text.contains("Hello world"));
    assert!(!text.contains("<html>"));
}

#[test]
fn test_content_parser_strips_scripts() {
    let parser = ContentParser::new();
    let html = "Hello <script>alert('xss')</script> world";
    let text = parser.extract_text(html);
    assert!(text.contains("Hello"));
    assert!(text.contains("world"));
    assert!(!text.contains("alert"));
}

#[test]
fn test_content_parser_title() {
    let parser = ContentParser::new();
    let html = "<html><head><title>Test Page</title></head><body>content</body></html>";
    let title = parser.extract_title(html);
    assert_eq!(title, Some("Test Page".to_string()));
}

#[test]
fn test_content_parser_summary() {
    let parser = ContentParser::new();
    let html = "<html><head><title>T</title><meta name=\"description\" content=\"Test description\"></head><body>content</body></html>";
    let summary = parser.extract_meta_description(html);
    assert_eq!(summary, Some("Test description".to_string()));
}

#[test]
fn test_content_parser_links() {
    let parser = ContentParser::new();
    let html = r#"<a href="https://example.com">Link</a>"#;
    let links = parser.extract_links(html);
    assert!(!links.is_empty());
    assert!(links.contains(&"https://example.com".to_string()));
}

#[test]
fn test_content_parser_word_count() {
    let parser = ContentParser::new();
    let count = parser.word_count("hello world foo bar");
    assert_eq!(count, 4);
}

#[test]
fn test_content_parser_summarize() {
    let parser = ContentParser::new();
    let summary = parser.summarize("First sentence. Second sentence. Third sentence.", 2);
    assert!(summary.contains("First"));
    assert!(summary.contains("Second"));
}
