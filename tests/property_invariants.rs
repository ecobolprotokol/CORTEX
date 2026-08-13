use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::memory::retrieval::RetrievalEngine;
use cortex::persistence::format::FormatHandler;
use cortex::runtime::Runtime;
use cortex::transaction::invariant::StateInvariant;
use cortex::types::state::{CortexState, ARCHITECTURE_VERSION, SCHEMA_VERSION};

fn make_config() -> CortexConfig {
    let mut cfg = CortexConfig::default();
    cfg.persistence.state = format!("/tmp/cortex_property_{}.cx", std::process::id());
    cfg.persistence.checkpoint_interval = 5;
    cfg.learning.consolidation_interval = 10;
    cfg
}

#[test]
fn property_state_always_valid_after_boot() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    assert!(StateInvariant::validate_state(&rt.state).is_ok());
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_state_always_valid_after_process() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    for i in 0..20 {
        let input = format!("Test observation number {}", i);
        let _ = rt.process(&input);
        assert!(
            StateInvariant::validate_state(&rt.state).is_ok(),
            "State invalid after process #{}",
            i
        );
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_confidence_always_in_bounds() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    for i in 0..50 {
        let input = format!("Observation {}", i);
        let _ = rt.process(&input);
        assert!(
            rt.state.verification.confidence_threshold >= 0.0
                && rt.state.verification.confidence_threshold <= 1.0,
            "Confidence threshold out of bounds: {}",
            rt.state.verification.confidence_threshold
        );
        assert!(
            rt.state.self_model.prediction_accuracy >= 0.0
                && rt.state.self_model.prediction_accuracy <= 1.0,
            "Prediction accuracy out of bounds"
        );
        assert!(
            rt.state.self_model.uncertainty_level >= 0.0
                && rt.state.self_model.uncertainty_level <= 1.0,
            "Uncertainty level out of bounds"
        );
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_learning_rate_always_bounded() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    for i in 0..30 {
        let _ = rt.process(&format!("Learning test {}", i));
        assert!(
            rt.state.learning.learning_rate >= 0.0 && rt.state.learning.learning_rate <= 1.0,
            "Learning rate out of bounds"
        );
        assert!(
            rt.state.learning.plasticity_rate >= 0.0 && rt.state.learning.plasticity_rate <= 1.0,
            "Plasticity rate out of bounds"
        );
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_episode_count_never_decreases() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let mut last_count = 0u64;
    for i in 0..20 {
        let _ = rt.process(&format!("Count test {}", i));
        assert!(
            rt.state.metadata.episode_count >= last_count,
            "Episode count decreased from {} to {}",
            last_count,
            rt.state.metadata.episode_count
        );
        last_count = rt.state.metadata.episode_count;
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_vocabulary_only_grows() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let mut last_size = 0u32;
    for i in 0..20 {
        let _ = rt.process(&format!("Vocab test unique_word_{}", i));
        let size = rt.language_vocabulary.size();
        assert!(
            size >= last_size,
            "Vocabulary decreased from {} to {}",
            last_size,
            size
        );
        last_size = size;
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn property_serialization_roundtrip_preserves_state() {
    let state = CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();
    let handler = FormatHandler::new();
    let compressed = handler.serialize(&bincode_data).unwrap();
    let decompressed = handler.deserialize(&compressed).unwrap();
    let restored: CortexState = bincode::deserialize(&decompressed).unwrap();
    assert_eq!(
        state.metadata.architecture_version,
        restored.metadata.architecture_version
    );
    assert_eq!(
        state.metadata.schema_version, restored.metadata.schema_version
    );
}

#[test]
fn property_retrieval_scores_are_bounded() {
    use cortex::types::common::ContextState;
    use cortex::memory::episodic::Episode;
    use cortex::types::common::Timestamp;
    use cortex::types::ids::EpisodeId;
    use cortex::types::observation::Observation;

    let episodes: Vec<Episode> = (0..10)
        .map(|i| Episode {
            id: EpisodeId::from(i),
            observation: Observation::user_provided(&format!("Episode {} about cats and dogs", i)),
            timestamp: Timestamp::now(),
            importance: 0.5,
            confidence: 0.7,
            consolidated: false,
            retrieval_count: 0,
        })
        .collect();
    let context = ContextState::default();
    let ranked = RetrievalEngine::rank_episodes("cats", &episodes, &context, &[]);
    for (_, score) in &ranked {
        assert!(
            *score >= 0.0 && *score <= 1.0,
            "Retrieval score out of bounds: {}",
            score
        );
    }
}

#[test]
fn property_mutation_log_records_all_operations() {
    let cfg = make_config();
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();
    let _ = rt.process("Mutation log test");
    let log = &rt.mutation_log;
    assert!(!log.records.is_empty(), "Mutation log should not be empty");
    for record in &log.records {
        assert!(
            record.pre_version <= record.post_version || !record.success,
            "pre_version should be <= post_version for successful mutations"
        );
    }
    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}
