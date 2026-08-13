use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub properties: HashMap<String, String>,
    pub confidence: Scalar,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone)]
pub struct EntityManager {
    pub entities: Vec<Entity>,
    pub name_index: HashMap<String, Vec<EntityId>>,
    pub next_id: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            name_index: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, name: &str, kind: EntityKind) -> Entity {
        let id = EntityId::from(self.next_id);
        self.next_id += 1;

        let e = Entity {
            id,
            name: name.to_string(),
            kind,
            properties: HashMap::new(),
            confidence: 0.5,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };

        self.name_index
            .entry(name.to_string())
            .or_default()
            .push(id);
        self.entities.push(e.clone());
        e
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Entity> {
        self.name_index
            .get(name)
            .and_then(|ids| ids.first())
            .and_then(|id| self.entities.iter().find(|e| e.id == *id))
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn find_by_kind(&self, kind: EntityKind) -> Vec<&Entity> {
        self.entities.iter().filter(|e| e.kind == kind).collect()
    }

    pub fn update_property(&mut self, id: EntityId, key: &str, value: &str) -> bool {
        if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
            e.properties.insert(key.to_string(), value.to_string());
            e.updated_at = Timestamp::now();
            true
        } else {
            false
        }
    }

    pub fn get_property(&self, id: EntityId, key: &str) -> Option<&str> {
        self.find_by_id(id)
            .and_then(|e| e.properties.get(key))
            .map(|s| s.as_str())
    }

    pub fn remove(&mut self, id: EntityId) -> bool {
        let pos = self.entities.iter().position(|e| e.id == id);
        if let Some(pos) = pos {
            let removed = self.entities.remove(pos);
            if let Some(ids) = self.name_index.get_mut(&removed.name) {
                ids.retain(|i| *i != id);
                if ids.is_empty() {
                    self.name_index.remove(&removed.name);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }

    pub fn entities_with_property(&self, key: &str) -> Vec<&Entity> {
        self.entities
            .iter()
            .filter(|e| e.properties.contains_key(key))
            .collect()
    }

    pub fn merge_properties(&mut self, id: EntityId, props: HashMap<String, String>) {
        if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
            for (k, v) in props {
                e.properties.insert(k, v);
            }
            e.updated_at = Timestamp::now();
        }
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
