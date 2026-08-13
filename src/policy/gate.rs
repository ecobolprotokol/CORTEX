use crate::policy::{PolicyDecision, PolicyEngine, ProposedOperation};

pub fn gate_decide(engine: &dyn PolicyEngine, operation: &ProposedOperation) -> PolicyDecision {
    engine.evaluate(operation)
}
