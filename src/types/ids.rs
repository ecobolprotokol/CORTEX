use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name(u64);

        impl $name {
            pub const NULL: Self = Self(0);

            pub fn new(value: u64) -> Self {
                Self(value)
            }

            pub fn next() -> Self {
                Self(COUNTER.fetch_add(1, Ordering::Relaxed))
            }

            pub fn raw(self) -> u64 {
                self.0
            }

            pub fn is_null(&self) -> bool {
                self.0 == 0
            }
        }

        impl From<u64> for $name {
            fn from(v: u64) -> Self {
                Self(v)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_id!(CellId);
define_id!(ColumnId);
define_id!(FieldId);

define_id!(EpisodeId);
define_id!(KnowledgeId);
define_id!(ProcedureId);
define_id!(AssociationId);
define_id!(MemoryId);

define_id!(SymbolId);
define_id!(TokenId);
define_id!(ConceptId);

define_id!(EntityId);
define_id!(RelationId);
define_id!(EventId);
define_id!(TransitionId);

define_id!(HypothesisId);
define_id!(EvidenceId);

define_id!(PlanId);
define_id!(GoalId);
define_id!(ActionId);

define_id!(ClaimId);

define_id!(SourceId);
define_id!(ProvenanceId);

define_id!(CheckpointId);
define_id!(SessionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalId {
    Cell(CellId),
    Column(ColumnId),
    Episode(EpisodeId),
    Concept(ConceptId),
    Entity(EntityId),
    Procedure(ProcedureId),
    Association(AssociationId),
    Hypothesis(HypothesisId),
    Symbol(SymbolId),
}
