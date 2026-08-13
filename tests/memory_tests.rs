use cortex::memory::episodic::EpisodicMemory;
use cortex::memory::semantic::SemanticMemory;
use cortex::memory::working::WorkingMemory;
use cortex::memory::associative::AssociativeMemory;
use cortex::memory::procedural::ProceduralMemory;
use cortex::memory::retrieval::RetrievalEngine;
use cortex::types::observation::Observation;

#[test]
fn test_episodic_memory_store() {
    let mut memory = EpisodicMemory::new(100);
    let obs = Observation::user_provided("Test observation");
    let episode = memory.store(obs);
    assert!(episode.id.raw() > 0);
    assert_eq!(memory.episodes.len(), 1);
}

#[test]
fn test_episodic_memory_eviction() {
    let mut memory = EpisodicMemory::new(5);
    for i in 0..10 {
        let obs = Observation::user_provided(&format!("Observation {}", i));
        memory.store(obs);
    }
    assert!(memory.episodes.len() <= 5);
}

#[test]
fn test_episodic_memory_recent() {
    let mut memory = EpisodicMemory::new(100);
    for i in 0..10 {
        let obs = Observation::user_provided(&format!("Observation {}", i));
        memory.store(obs);
    }
    let recent = memory.recent(3);
    assert_eq!(recent.len(), 3);
}

#[test]
fn test_semantic_memory_store() {
    let mut memory = SemanticMemory::new(100);
    let knowledge = memory.store("gravity", vec![("type".into(), "fundamental_force".into())]);
    assert!(knowledge.id.raw() > 0);
    assert_eq!(memory.knowledge.len(), 1);
}

#[test]
fn test_semantic_memory_find() {
    let mut memory = SemanticMemory::new(100);
    let _ = memory.store("gravity", vec![("type".into(), "fundamental_force".into())]);
    let results = memory.find_by_concept("gravity");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_working_memory() {
    let mut memory = WorkingMemory::new(100);
    memory.set_input("Test input".into());
    assert!(memory.input.is_some());
    memory.clear();
    assert!(memory.input.is_none());
}

#[test]
fn test_associative_memory() {
    let mut memory = AssociativeMemory::new();
    let assoc = memory.create(
        1,
        2,
        cortex::memory::associative::AssociationKind::Semantic,
    );
    assert!(assoc.id.raw() > 0);
    assert_eq!(memory.associations.len(), 1);
}

#[test]
fn test_procedural_memory() {
    let mut memory = ProceduralMemory::new();
    let proc = memory.store("test procedure", "when needed", vec!["step 1".into(), "step 2".into()], "success");
    assert!(proc.id.raw() > 0);
    assert_eq!(memory.procedures.len(), 1);
}

#[test]
fn test_retrieval_scoring() {
    let score = RetrievalEngine::score_relevance("gravity force", "gravity is a fundamental force");
    assert!(score > 0.0);
    assert!(score <= 1.0);
}
