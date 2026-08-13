use serde::{Deserialize, Serialize};
use crate::types::ids::EntityId;
use crate::types::common::Timestamp;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    Person,
    Object,
    Place,
    Organization,
    ConceptualObject,
    Event,
    System,
    Process,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub kind: EntityKind,
    pub properties: Vec<(String, String)>,
    pub confidence: Scalar,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone)]
pub struct EntityManager {
    pub entities: Vec<Entity>,
    pub next_id: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, name: &str, kind: EntityKind) -> Entity {
        let e = Entity {
            id: EntityId::from(self.next_id),
            name: name.to_string(),
            kind,
            properties: Vec::new(),
            confidence: 0.5,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };
        self.next_id += 1;
        self.entities.push(e.clone());
        e
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Entity> {
        self.entities.iter().find(|e| e.name == name)
    }

    pub fn update_property(&mut self, id: EntityId, key: &str, value: &str) {
        if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
            if let Some(prop) = e.properties.iter_mut().find(|(k, _)| k == key) {
                prop.1 = value.to_string();
            } else {
                e.properties.push((key.to_string(), value.to_string()));
            }
            e.updated_at = Timestamp::now();
        }
    }
}
