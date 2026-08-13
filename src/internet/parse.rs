pub struct ContentParser {
    pub max_content_length: usize,
    pub strip_scripts: bool,
    pub strip_styles: bool,
}

impl ContentParser {
    pub fn new() -> Self {
        Self {
            max_content_length: 1_000_000,
            strip_scripts: true,
            strip_styles: true,
        }
    }

    pub fn extract_text(&self, html: &str) -> String {
        let content = if html.len() > self.max_content_length {
            &html[..self.max_content_length]
        } else {
            html
        };

        let mut result = String::new();
        let mut in_script = false;
        let mut in_style = false;
        let mut depth = 0u32;

        let bytes = content.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'<' {
                let tag_start = i + 1;
                let mut tag_end = tag_start;
                while tag_end < bytes.len() && bytes[tag_end] != b'>' {
                    tag_end += 1;
                }

                if tag_end < bytes.len() {
                    let tag = String::from_utf8_lossy(&bytes[tag_start..tag_end]).to_lowercase();

                    if self.strip_scripts && tag.starts_with("script") {
                        in_script = true;
                        depth += 1;
                    } else if self.strip_styles && tag.starts_with("style") {
                        in_style = true;
                        depth += 1;
                    } else if tag.starts_with('/') {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            in_script = false;
                            in_style = false;
                        }
                    }

                    if !in_script && !in_style {
                        if tag.starts_with("p")
                            || tag.starts_with("br")
                            || tag.starts_with("div")
                            || tag.starts_with("h1")
                            || tag.starts_with("h2")
                            || tag.starts_with("h3")
                            || tag.starts_with("h4")
                            || tag.starts_with("h5")
                            || tag.starts_with("h6")
                            || tag.starts_with("li")
                            || tag.starts_with("tr")
                        {
                            if !result.ends_with('\n') && !result.is_empty() {
                                result.push('\n');
                            }
                        }
                    }

                    i = tag_end + 1;
                } else {
                    i += 1;
                }
            } else if !in_script && !in_style {
                result.push(bytes[i] as char);
                i += 1;
            } else {
                i += 1;
            }
        }

        let cleaned = result
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        cleaned.trim().to_string()
    }

    pub fn extract_meta_description(&self, html: &str) -> Option<String> {
        let lower = html.to_lowercase();
        if let Some(start) = lower.find("meta") {
            if let Some(desc_start) = lower[start..].find("description") {
                let rest = &html[start + desc_start..];
                if let Some(content_start) = rest.find("content=\"") {
                    let content_start = content_start + 9;
                    if let Some(content_end) = rest[content_start..].find('"') {
                        return Some(rest[content_start..content_start + content_end].to_string());
                    }
                }
            }
        }
        None
    }

    pub fn extract_title(&self, html: &str) -> Option<String> {
        let lower = html.to_lowercase();
        if let Some(start) = lower.find("<title>") {
            let content_start = start + 7;
            if let Some(end) = lower[content_start..].find("</title>") {
                return Some(html[content_start..content_start + end].trim().to_string());
            }
        }
        None
    }

    pub fn extract_links(&self, html: &str) -> Vec<String> {
        let mut links = Vec::new();
        let lower = html.to_lowercase();
        let mut pos = 0;

        while let Some(href_pos) = lower[pos..].find("href=\"") {
            let start = pos + href_pos + 6;
            if let Some(end) = lower[start..].find('"') {
                let link = html[start..start + end].to_string();
                if link.starts_with("http") {
                    links.push(link);
                }
                pos = start + end + 1;
            } else {
                break;
            }
        }

        links
    }

    pub fn word_count(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }

    pub fn summarize(&self, text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text
            .split(|c| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        sentences
            .into_iter()
            .take(max_sentences)
            .collect::<Vec<&str>>()
            .join(". ")
    }
}

impl Default for ContentParser {
    fn default() -> Self {
        Self::new()
    }
}
