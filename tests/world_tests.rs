use cortex::world::causal::CausalModel;
use cortex::world::entity::{EntityKind, EntityManager};
use cortex::world::simulation::WorldSimulator;
use cortex::world::transition::TransitionModel;

#[test]
fn test_entity_manager_create() {
    let mut manager = EntityManager::new();
    let entity = manager.create("gravity", EntityKind::ConceptualObject);
    assert!(entity.id.raw() > 0);
    assert_eq!(manager.entities.len(), 1);
}

#[test]
fn test_entity_manager_find() {
    let mut manager = EntityManager::new();
    let _ = manager.create("gravity", EntityKind::ConceptualObject);
    let found = manager.find_by_name("gravity");
    assert!(found.is_some());
}

#[test]
fn test_entity_manager_update() {
    let mut manager = EntityManager::new();
    let entity = manager.create("gravity", EntityKind::ConceptualObject);
    manager.update_property(entity.id, "type", "fundamental_force");
    let found = manager.find_by_name("gravity").unwrap();
    assert!(found
        .properties
        .iter()
        .any(|(k, v)| k == "type" && v == "fundamental_force"));
}

#[test]
fn test_transition_model() {
    let model = TransitionModel::new();
    let predicted = model.predict("initial_state", "apply_force");
    assert!(predicted.confidence > 0.0);
}

#[test]
fn test_causal_model() {
    let mut model = CausalModel::new();
    model.add_hypothesis("heat", "expansion");
    model.strengthen("heat", "expansion");
    let hypothesis = model.hypotheses.iter().find(|h| h.cause == "heat").unwrap();
    assert!(hypothesis.strength > 0.1);
}

#[test]
fn test_world_simulator() {
    let simulator = WorldSimulator::new();
    let result = simulator.simulate("initial", &["action1".into(), "action2".into()], None);
    assert!(result.steps.len() > 1);
}
