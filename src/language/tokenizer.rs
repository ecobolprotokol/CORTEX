use crate::error::Result;
use crate::types::*;

pub fn tokenize(text: &str, state: &mut LanguageState) -> Result<Vec<Symbol>> {
    let mut symbols = Vec::new();
    let normalized = normalize(text);
    let segments = segment(&normalized);
    for segment in segments {
        if segment.trim().is_empty() {
            continue;
        }
        let kind = classify_segment(&segment);
        let existing = state.symbols.iter().find(|s| s.text == segment);
        if let Some(mut sym) = existing.cloned() {
            sym.frequency += 1;
            sym.activation = 1.0;
            if let Some(s) = state.symbols.iter_mut().find(|s| s.text == segment) {
                s.frequency += 1;
                s.activation = 1.0;
            }
            symbols.push(sym);
        } else if state.vocabulary_size < 65536 {
            let id = state.next_symbol_id;
            state.next_symbol_id = id.next();
            state.vocabulary_size += 1;
            let symbol = Symbol {
                id,
                text: segment.clone(),
                kind,
                frequency: 1,
                activation: 1.0,
                confidence: 0.5,
            };
            state.symbols.push(symbol.clone());
            symbols.push(symbol);
        }
    }
    Ok(symbols)
}

fn normalize(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }
        result.push(ch);
    }
    result
}

fn segment(text: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut in_word = false;
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
            if !in_word && !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            in_word = true;
            current.push(ch.to_lowercase().next().unwrap_or(ch));
        } else if ch.is_whitespace() {
            if !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            in_word = false;
        } else {
            if !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            segments.push(ch.to_string());
            in_word = false;
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn classify_segment(segment: &str) -> SymbolKind {
    if segment.len() == 1 && !segment.chars().next().map_or(false, |c| c.is_alphanumeric()) {
        SymbolKind::Punctuation
    } else if segment.chars().all(|c| c.is_ascii_digit()) {
        SymbolKind::Number
    } else if segment.starts_with('<') && segment.ends_with('>') {
        SymbolKind::Special
    } else {
        SymbolKind::Word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let mut state = LanguageState {
            symbols: Vec::new(),
            tokens: Vec::new(),
            vocabulary_size: 0,
            next_symbol_id: SymbolId(1),
        };
        let symbols = tokenize("hello world", &mut state).unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].text, "hello");
        assert_eq!(symbols[1].text, "world");
        assert_eq!(state.vocabulary_size, 2);
    }

    #[test]
    fn test_tokenize_punctuation() {
        let mut state = LanguageState {
            symbols: Vec::new(),
            tokens: Vec::new(),
            vocabulary_size: 0,
            next_symbol_id: SymbolId(1),
        };
        let symbols = tokenize("hello, world!", &mut state).unwrap();
        assert!(symbols.iter().any(|s| s.kind == SymbolKind::Punctuation));
    }

    #[test]
    fn test_tokenize_frequency() {
        let mut state = LanguageState {
            symbols: Vec::new(),
            tokens: Vec::new(),
            vocabulary_size: 0,
            next_symbol_id: SymbolId(1),
        };
        tokenize("hello world", &mut state).unwrap();
        tokenize("hello again", &mut state).unwrap();
        let hello = state.symbols.iter().find(|s| s.text == "hello").unwrap();
        assert_eq!(hello.frequency, 2);
    }
}
