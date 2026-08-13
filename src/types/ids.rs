macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub u64);

        impl $name {
            pub const NULL: Self = Self(0);
            pub fn new(value: u64) -> Self { Self(value) }
            pub fn is_null(&self) -> bool { self.0 == 0 }
            pub fn next(&self) -> Self { Self(self.0 + 1) }
        }

        impl Default for $name {
            fn default() -> Self { Self::NULL }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
