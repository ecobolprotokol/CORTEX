use crate::types::*;
use crate::learning::LearningSignal;

pub fn compute(experience: &Experience) -> LearningSignal {
    let magnitude = experience.error.magnitude;
    let attribution = experience.attribution;
    LearningSignal {
        magnitude,
        attribution,
        timestamp: Timestamp::now(),
    }
}
