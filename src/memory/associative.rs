use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::ids::AssociationId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssociationKind {
    Semantic,
    Temporal,
    Contextual,
    Causal,
    Episodic,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub id: AssociationId,
    pub source: u64,
    pub target: u64,
    pub kind: AssociationKind,
    pub strength: Scalar,
    pub created_at: u64,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct AssociativeMemory {
    pub associations: Vec<Association>,
    pub forward_index: HashMap<u64, Vec<AssociationId>>,
    pub backward_index: HashMap<u64, Vec<AssociationId>>,
    pub next_id: u64,
}

impl Default for AssociativeMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl AssociativeMemory {
    pub fn new() -> Self {
        Self {
            associations: Vec::new(),
            forward_index: HashMap::new(),
            backward_index: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(
        &mut self,
        source: u64,
        target: u64,
        kind: AssociationKind,
    ) -> Association {
        let a = Association {
            id: AssociationId::from(self.next_id),
            source,
            target,
            kind,
            strength: 0.5,
            created_at: crate::types::common::Timestamp::now().as_millis(),
            access_count: 0,
        };
        self.next_id += 1;

        self.forward_index
            .entry(source)
            .or_default()
            .push(a.id);
        self.backward_index
            .entry(target)
            .or_default()
            .push(a.id);

        self.associations.push(a.clone());
        a
    }

    pub fn get_associations(&self, entity_id: u64) -> Vec<&Association> {
        self.associations
            .iter()
            .filter(|a| a.source == entity_id || a.target == entity_id)
            .collect()
    }

    pub fn get_associations_mut(&mut self, entity_id: u64) -> Vec<&mut Association> {
        self.associations
            .iter_mut()
            .filter(|a| a.source == entity_id || a.target == entity_id)
            .collect()
    }

    pub fn strengthen(&mut self, id: AssociationId, delta: Scalar) {
        if let Some(a) = self.associations.iter_mut().find(|a| a.id == id) {
            a.strength = (a.strength + delta).clamp(0.0, 1.0);
            a.access_count += 1;
        }
    }

    pub fn weaken(&mut self, id: AssociationId, delta: Scalar) {
        if let Some(a) = self.associations.iter_mut().find(|a| a.id == id) {
            a.strength = (a.strength - delta).clamp(0.0, 1.0);
        }
    }

    pub fn get(&self, id: AssociationId) -> Option<&Association> {
        self.associations.iter().find(|a| a.id == id)
    }

    pub fn get_mut(&mut self, id: AssociationId) -> Option<&mut Association> {
        self.associations.iter_mut().find(|a| a.id == id)
    }

    pub fn between(&self, source: u64, target: u64) -> Vec<&Association> {
        self.associations
            .iter()
            .filter(|a| a.source == source && a.target == target)
            .collect()
    }

    pub fn strongest_for(&self, entity_id: u64) -> Option<&Association> {
        self.get_associations(entity_id)
            .into_iter()
            .max_by(|a, b| {
                a.strength
                    .partial_cmp(&b.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub fn remove(&mut self, id: AssociationId) -> bool {
        if let Some(pos) = self.associations.iter().position(|a| a.id == id) {
            let a = self.associations.remove(pos);
            if let Some(ids) = self.forward_index.get_mut(&a.source) {
                ids.retain(|x| *x != id);
            }
            if let Some(ids) = self.backward_index.get_mut(&a.target) {
                ids.retain(|x| *x != id);
            }
            true
        } else {
            false
        }
    }

    pub fn decay_all(&mut self, decay_rate: Scalar) {
        for a in &mut self.associations {
            a.strength = (a.strength * (1.0 - decay_rate)).max(0.0);
        }
    }

    pub fn prune_weak(&mut self, threshold: Scalar) {
        let weak_ids: Vec<AssociationId> = self
            .associations
            .iter()
            .filter(|a| a.strength < threshold)
            .map(|a| a.id)
            .collect();
        for id in weak_ids {
            self.remove(id);
        }
    }

    pub fn usage_bytes(&self) -> usize {
        self.associations.len() * std::mem::size_of::<Association>()
    }
}
