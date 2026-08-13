use serde::{Deserialize, Serialize};
use crate::types::ids::AssociationId;
use crate::types::scalars::Scalar;
use std::collections::HashMap;

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
}

#[derive(Debug, Clone)]
pub struct AssociativeMemory {
    pub associations: Vec<Association>,
    pub forward_index: HashMap<u64, Vec<AssociationId>>,
    pub backward_index: HashMap<u64, Vec<AssociationId>>,
    pub next_id: u64,
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

    pub fn create(&mut self, source: u64, target: u64, kind: AssociationKind) -> Association {
        let a = Association {
            id: AssociationId::from(self.next_id),
            source,
            target,
            kind,
            strength: 0.5,
        };
        self.next_id += 1;

        self.forward_index.entry(source).or_default().push(a.id);
        self.backward_index.entry(target).or_default().push(a.id);

        self.associations.push(a.clone());
        a
    }

    pub fn get_associations(&self, entity_id: u64) -> Vec<&Association> {
        self.associations.iter()
            .filter(|a| a.source == entity_id || a.target == entity_id)
            .collect()
    }
}
