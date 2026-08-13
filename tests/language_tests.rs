use cortex::language::tokenizer::Tokenizer;
use cortex::language::vocabulary::Vocabulary;

#[test]
fn test_tokenizer_basic() {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("Hello, world!").unwrap();
    assert!(!tokens.is_empty());
    assert!(tokens.contains(&"hello".to_string()));
    assert!(tokens.contains(&"world".to_string()));
}

#[test]
fn test_tokenizer_normalization() {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("  HELLO   WORLD  ").unwrap();
    assert!(tokens.contains(&"hello".to_string()));
    assert!(tokens.contains(&"world".to_string()));
}

#[test]
fn test_tokenizer_punctuation() {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("hello,world!").unwrap();
    assert!(tokens.contains(&"hello".to_string()));
    assert!(tokens.contains(&"world".to_string()));
}

#[test]
fn test_vocabulary_lookup() {
    let mut vocab = Vocabulary::new(1000);
    let id1 = vocab.lookup_or_create("hello");
    let id2 = vocab.lookup_or_create("world");
    assert_ne!(id1.raw(), id2.raw());
    assert_eq!(vocab.size(), 2);
}

#[test]
fn test_vocabulary_frequency() {
    let mut vocab = Vocabulary::new(1000);
    let id = vocab.lookup_or_create("hello");
    let _ = vocab.lookup_or_create("hello");
    let _ = vocab.lookup_or_create("hello");
    assert_eq!(vocab.frequency(id), 3);
}

#[test]
fn test_vocabulary_capacity() {
    let mut vocab = Vocabulary::new(5);
    for i in 0..10 {
        let _ = vocab.lookup_or_create(&format!("word{}", i));
    }
    assert!(vocab.size() <= 5);
}

#[test]
fn test_vocabulary_confidence() {
    let mut vocab = Vocabulary::new(1000);
    let id = vocab.lookup_or_create("hello");
    for _ in 0..10 {
        let _ = vocab.lookup_or_create("hello");
    }
    let confidence = vocab.confidence(id);
    assert!(confidence > 0.0);
    assert!(confidence <= 1.0);
}
