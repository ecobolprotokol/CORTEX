use std::fmt;

#[derive(Debug, Clone)]
pub enum CortexError {
    InputError(String),
    EncodingError(String),
    LanguageError(String),
    MemoryError(String),
    WorldModelError(String),
    ReasoningError(String),
    PlanningError(String),
    VerificationError(String),
    LearningError(String),
    PersistenceError(String),
    PolicyError(String),
    ResourceError(String),
    NetworkError(String),
    RuntimeError(String),
    ConfigError(String),
    StateError(String),
    SerializationError(String),
    SubsystemDisabled(String),
}

impl fmt::Display for CortexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(msg) => write!(f, "InputError: {}", msg),
            Self::EncodingError(msg) => write!(f, "EncodingError: {}", msg),
            Self::LanguageError(msg) => write!(f, "LanguageError: {}", msg),
            Self::MemoryError(msg) => write!(f, "MemoryError: {}", msg),
            Self::WorldModelError(msg) => write!(f, "WorldModelError: {}", msg),
            Self::ReasoningError(msg) => write!(f, "ReasoningError: {}", msg),
            Self::PlanningError(msg) => write!(f, "PlanningError: {}", msg),
            Self::VerificationError(msg) => write!(f, "VerificationError: {}", msg),
            Self::LearningError(msg) => write!(f, "LearningError: {}", msg),
            Self::PersistenceError(msg) => write!(f, "PersistenceError: {}", msg),
            Self::PolicyError(msg) => write!(f, "PolicyError: {}", msg),
            Self::ResourceError(msg) => write!(f, "ResourceError: {}", msg),
            Self::NetworkError(msg) => write!(f, "NetworkError: {}", msg),
            Self::RuntimeError(msg) => write!(f, "RuntimeError: {}", msg),
            Self::ConfigError(msg) => write!(f, "ConfigError: {}", msg),
            Self::StateError(msg) => write!(f, "StateError: {}", msg),
            Self::SerializationError(msg) => write!(f, "SerializationError: {}", msg),
            Self::SubsystemDisabled(msg) => write!(f, "SubsystemDisabled: {}", msg),
        }
    }
}

impl std::error::Error for CortexError {}

impl CortexError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::ResourceError(_) | Self::SubsystemDisabled(_)
        )
    }
}
