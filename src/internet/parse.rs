use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPage {
    pub title: Option<String>,
    pub description: Option<String>,
    pub headings: Vec<String>,
    pub paragraphs: Vec<String>,
    pub lists: Vec<ListItem>,
    pub text: String,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub ordered: bool,
    pub content: String,
}

impl Default for ParsedPage {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            headings: Vec::new(),
            paragraphs: Vec::new(),
            lists: Vec::new(),
            text: String::new(),
            meta: HashMap::new(),
        }
    }
}

pub fn decode_entities(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.char_indices();
    while let Some((i, ch)) = chars.next() {
        if ch == '&' {
            if let Some(semi) = text[i..].find(';') {
                let entity = &text[i + 1..i + semi];
                let decoded = match entity {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    "nbsp" => Some(' '),
                    "#39" => Some('\''),
                    "#47" => Some('/'),
                    "#92" => Some('\\'),
                    _ => None,
                };
                if let Some(d) = decoded {
                    result.push(d);
                    for _ in 0..semi {
                        chars.next();
                    }
                    continue;
                }
            }
        }
        result.push(ch);
    }
    result
}

fn extract_tag_name(chars: &[char], start: usize) -> (String, usize) {
    let mut end = start;
    while end < len(chars) && chars[end] != '>' && chars[end] != ' ' && chars[end] != '/' {
        end += 1;
    }
    let name: String = chars[start..end].iter().collect();
    (name, end)
}

fn skip_to_gt(chars: &[char], mut pos: usize) -> usize {
    while pos < len(chars) && chars[pos] != '>' {
        pos += 1;
    }
    if pos < len(chars) {
        pos + 1
    } else {
        pos
    }
}

fn len(chars: &[char]) -> usize {
    chars.len()
}

fn extract_inner_text(chars: &[char], start: usize, closing_tag: &str) -> (String, usize) {
    let mut text = String::new();
    let mut i = start;

    while i < len(chars) {
        if chars[i] == '<' && i + 1 < len(chars) && chars[i + 1] == '/' {
            let tag_start = i + 2;
            let (name, _) = extract_tag_name(chars, tag_start);
            if name.to_lowercase() == closing_tag.to_lowercase() {
                let end = skip_to_gt(chars, i);
                return (text, end);
            }
        }
        if chars[i] == '<' && i + 1 < len(chars) && chars[i + 1] != '/' {
            let tag_start = i + 1;
            let (name, _) = extract_tag_name(chars, tag_start);
            let tag_lower = name.to_lowercase();
            if tag_lower == "script" || tag_lower == "style" || tag_lower == "noscript" {
                i = skip_to_gt(chars, i);
                while i < len(chars) {
                    if chars[i] == '<' && i + 1 < len(chars) && chars[i + 1] == '/' {
                        let inner_start = i + 2;
                        let (inner_name, _) = extract_tag_name(chars, inner_start);
                        if inner_name.to_lowercase() == tag_lower {
                            i = skip_to_gt(chars, i);
                            break;
                        }
                    }
                    i += 1;
                }
                continue;
            }
            i = skip_to_gt(chars, i);
            continue;
        }
        text.push(chars[i]);
        i += 1;
    }
    (text, i)
}

pub fn extract_text(html: &str) -> String {
    let page = parse_html(html);
    page.text
}

pub fn strip_html_tags(html: &str) -> String {
    extract_text(html)
}

pub fn parse_html(html: &str) -> ParsedPage {
    let mut page = ParsedPage::default();
    let chars: Vec<char> = html.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut text_parts: Vec<String> = Vec::new();
    let mut skip_depth: u32 = 0;
    let mut current_skip_tag: Option<String> = None;

    while i < len {
        if chars[i] == '<' {
            let tag_start = i + 1;
            if tag_start >= len {
                break;
            }
            let is_closing = chars[tag_start] == '/';
            let actual_start = if is_closing { tag_start + 1 } else { tag_start };
            let (tag_name, tag_end) = extract_tag_name(&chars, actual_start);
            let tag_lower = tag_name.to_lowercase();

            let mut attrs: HashMap<String, String> = HashMap::new();
            let mut attr_start = tag_end;
            while attr_start < len && chars[attr_start] != '>' {
                while attr_start < len && (chars[attr_start] == ' ' || chars[attr_start] == '\t') {
                    attr_start += 1;
                }
                if attr_start >= len || chars[attr_start] == '>' || chars[attr_start] == '/' {
                    break;
                }
                let name_start = attr_start;
                while attr_start < len && chars[attr_start] != '=' && chars[attr_start] != ' '
                    && chars[attr_start] != '>'
                {
                    attr_start += 1;
                }
                let attr_name: String = chars[name_start..attr_start].iter().collect();
                if attr_start < len && chars[attr_start] == '=' {
                    attr_start += 1;
                    if attr_start < len && (chars[attr_start] == '"' || chars[attr_start] == '\'') {
                        let quote = chars[attr_start];
                        attr_start += 1;
                        let val_start = attr_start;
                        while attr_start < len && chars[attr_start] != quote {
                            attr_start += 1;
                        }
                        let attr_val: String = chars[val_start..attr_start].iter().collect();
                        attrs.insert(attr_name, decode_entities(&attr_val));
                        if attr_start < len {
                            attr_start += 1;
                        }
                    } else {
                        let val_start = attr_start;
                        while attr_start < len && chars[attr_start] != ' '
                            && chars[attr_start] != '>'
                        {
                            attr_start += 1;
                        }
                        let attr_val: String = chars[val_start..attr_start].iter().collect();
                        attrs.insert(attr_name, decode_entities(&attr_val));
                    }
                } else {
                    attrs.insert(attr_name, String::new());
                }
            }

            i = skip_to_gt(&chars, i);

            if tag_lower == "script" || tag_lower == "style" || tag_lower == "noscript" {
                if is_closing && skip_depth > 0 {
                    skip_depth -= 1;
                    if skip_depth == 0 {
                        current_skip_tag = None;
                    }
                } else if !is_closing && skip_depth == 0 {
                    skip_depth = 1;
                    current_skip_tag = Some(tag_lower.clone());
                }
                continue;
            }

            if skip_depth > 0 {
                if is_closing && current_skip_tag.as_deref() == Some(&tag_lower) {
                    skip_depth = 0;
                    current_skip_tag = None;
                }
                continue;
            }

            match tag_lower.as_str() {
                "title" => {
                    let (title_text, new_i) = extract_inner_text(&chars, i, "title");
                    i = new_i;
                    page.title = Some(decode_entities(&title_text.trim().to_string()));
                }
                "meta" => {
                    let name = attrs.get("name").or_else(|| attrs.get("property")).cloned();
                    let content = attrs.get("content").cloned();
                    if let (Some(name_val), Some(content_val)) = (name, content) {
                        let name_lower = name_val.to_lowercase();
                        if name_lower == "description" {
                            page.description = Some(content_val.clone());
                        }
                        page.meta.insert(name_val, content_val);
                    }
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    let (heading_text, new_i) = extract_inner_text(&chars, i, &tag_lower);
                    i = new_i;
                    let decoded = decode_entities(&heading_text.trim().to_string());
                    if !decoded.is_empty() {
                        page.headings.push(decoded.clone());
                        text_parts.push(decoded);
                    }
                }
                "p" => {
                    let (para_text, new_i) = extract_inner_text(&chars, i, "p");
                    i = new_i;
                    let decoded = decode_entities(&para_text.trim().to_string());
                    if !decoded.is_empty() {
                        page.paragraphs.push(decoded.clone());
                        text_parts.push(decoded);
                    }
                }
                "li" => {
                    let (li_text, new_i) = extract_inner_text(&chars, i, "li");
                    i = new_i;
                    let decoded = decode_entities(&li_text.trim().to_string());
                    if !decoded.is_empty() {
                        page.lists.push(ListItem {
                            ordered: false,
                            content: decoded.clone(),
                        });
                        text_parts.push(decoded);
                    }
                }
                "br" => {
                    text_parts.push("\n".to_string());
                }
                "ul" | "ol" => {}
                _ => {}
            }
        } else {
            if skip_depth == 0 {
                let mut text = String::new();
                while i < len && chars[i] != '<' {
                    text.push(chars[i]);
                    i += 1;
                }
                let decoded = decode_entities(&text);
                if !decoded.trim().is_empty() {
                    text_parts.push(decoded);
                }
            } else {
                i += 1;
            }
        }
    }

    page.text = text_parts.join(" ");
    page.text = page
        .text
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    page
}

pub fn parse_and_extract(html: &str) -> (String, ParsedPage) {
    let page = parse_html(html);
    let text = page.text.clone();
    (text, page)
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

    #[test]
    fn test_skip_script_tags() {
        let html = r#"<p>Visible</p><script>var x = "hidden";</script><p>Also visible</p>"#;
        let text = extract_text(html);
        assert!(text.contains("Visible"));
        assert!(text.contains("Also visible"));
        assert!(!text.contains("hidden"));
    }

    #[test]
    fn test_skip_style_tags() {
        let html = "<style>.red { color: red; }</style><p>Content</p>";
        let text = extract_text(html);
        assert!(text.contains("Content"));
        assert!(!text.contains("color"));
    }

    #[test]
    fn test_skip_noscript_tags() {
        let html = "<noscript>JS disabled message</noscript><p>Content</p>";
        let text = extract_text(html);
        assert!(text.contains("Content"));
        assert!(!text.contains("JS disabled"));
    }

    #[test]
    fn test_decode_entities() {
        assert_eq!(decode_entities("&amp;"), "&");
        assert_eq!(decode_entities("&lt;"), "<");
        assert_eq!(decode_entities("&gt;"), ">");
        assert_eq!(decode_entities("&quot;"), "\"");
        assert_eq!(decode_entities("&apos;"), "'");
        assert_eq!(decode_entities("&nbsp;"), " ");
        assert_eq!(decode_entities("&#39;"), "'");
        assert_eq!(decode_entities("a &amp; b"), "a & b");
    }

    #[test]
    fn test_decode_entities_in_text() {
        let html = "<p>5 &gt; 3 &amp; 2 &lt; 4</p>";
        let text = extract_text(html);
        assert!(text.contains("5 > 3 & 2 < 4"));
    }

    #[test]
    fn test_parse_html_title() {
        let html = "<html><head><title>My Page</title></head><body></body></html>";
        let page = parse_html(html);
        assert_eq!(page.title.as_deref(), Some("My Page"));
    }

    #[test]
    fn test_parse_html_meta_description() {
        let html = r#"<html><head><meta name="description" content="A test page"></head><body></body></html>"#;
        let page = parse_html(html);
        assert_eq!(page.description.as_deref(), Some("A test page"));
    }

    #[test]
    fn test_parse_html_meta_og() {
        let html = r#"<html><head><meta property="og:title" content="OG Title"></head><body></body></html>"#;
        let page = parse_html(html);
        assert_eq!(page.meta.get("og:title").map(|s| s.as_str()), Some("OG Title"));
    }

    #[test]
    fn test_parse_html_headings() {
        let html = "<html><body><h1>First</h1><h2>Second</h2></body></html>";
        let page = parse_html(html);
        assert_eq!(page.headings, vec!["First", "Second"]);
    }

    #[test]
    fn test_parse_html_paragraphs() {
        let html = "<html><body><p>Para one</p><p>Para two</p></body></html>";
        let page = parse_html(html);
        assert_eq!(page.paragraphs, vec!["Para one", "Para two"]);
    }

    #[test]
    fn test_parse_html_lists() {
        let html = "<html><body><ul><li>Item 1</li><li>Item 2</li></ul></body></html>";
        let page = parse_html(html);
        assert_eq!(page.lists.len(), 2);
        assert_eq!(page.lists[0].content, "Item 1");
        assert_eq!(page.lists[1].content, "Item 2");
    }

    #[test]
    fn test_parse_html_nested_tags() {
        let html = "<div><p><b>Bold</b> and <i>italic</i></p></div>";
        let text = extract_text(html);
        assert!(text.contains("Bold"));
        assert!(text.contains("italic"));
        assert!(!text.contains("<b>"));
        assert!(!text.contains("<i>"));
    }

    #[test]
    fn test_parse_and_extract() {
        let html = "<p>Combined</p>";
        let (text, page) = parse_and_extract(html);
        assert!(text.contains("Combined"));
        assert!(page.paragraphs.contains(&"Combined".to_string()));
    }

    #[test]
    fn test_empty_html() {
        let page = parse_html("");
        assert!(page.text.is_empty());
        assert!(page.headings.is_empty());
        assert!(page.paragraphs.is_empty());
    }

    #[test]
    fn test_meta_entities_in_content() {
        let html = r#"<head><meta name="description" content="A &amp; B"></head>"#;
        let page = parse_html(html);
        assert_eq!(page.description.as_deref(), Some("A & B"));
    }

    #[test]
    fn test_skip_nested_script() {
        let html = "<div><script>var x = 1;</script></div><p>After</p>";
        let text = extract_text(html);
        assert!(text.contains("After"));
        assert!(!text.contains("var x"));
    }

    #[test]
    fn test_entity_in_heading() {
        let html = "<h1>AT&amp;T Company</h1>";
        let page = parse_html(html);
        assert_eq!(page.headings, vec!["AT&T Company"]);
    }

    #[test]
    fn test_entity_in_list_item() {
        let html = "<ul><li>Tom &amp; Jerry</li></ul>";
        let page = parse_html(html);
        assert_eq!(page.lists[0].content, "Tom & Jerry");
    }

    #[test]
    fn test_deeply_nested_tags() {
        let html = "<p>Text <b>with <i>nested</i> tags</b> here</p>";
        let text = extract_text(html);
        assert!(text.contains("Text"));
        assert!(text.contains("nested"));
        assert!(text.contains("here"));
    }

    #[test]
    fn test_script_with_nested_content() {
        let html = r#"<p>Before</p><script type="text/javascript">if (a < b) { x = "</div>"; }</script><p>After</p>"#;
        let text = extract_text(html);
        assert!(text.contains("Before"));
        assert!(text.contains("After"));
        assert!(!text.contains("javascript"));
    }
}
