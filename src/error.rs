use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    SerializationError(String),
    StateError(String),
    SubsystemDisabled(String),
}

impl fmt::Display for CortexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CortexError::InputError(msg) => write!(f, "InputError: {}", msg),
            CortexError::EncodingError(msg) => write!(f, "EncodingError: {}", msg),
            CortexError::LanguageError(msg) => write!(f, "LanguageError: {}", msg),
            CortexError::MemoryError(msg) => write!(f, "MemoryError: {}", msg),
            CortexError::WorldModelError(msg) => write!(f, "WorldModelError: {}", msg),
            CortexError::ReasoningError(msg) => write!(f, "ReasoningError: {}", msg),
            CortexError::PlanningError(msg) => write!(f, "PlanningError: {}", msg),
            CortexError::VerificationError(msg) => write!(f, "VerificationError: {}", msg),
            CortexError::LearningError(msg) => write!(f, "LearningError: {}", msg),
            CortexError::PersistenceError(msg) => write!(f, "PersistenceError: {}", msg),
            CortexError::PolicyError(msg) => write!(f, "PolicyError: {}", msg),
            CortexError::ResourceError(msg) => write!(f, "ResourceError: {}", msg),
            CortexError::NetworkError(msg) => write!(f, "NetworkError: {}", msg),
            CortexError::RuntimeError(msg) => write!(f, "RuntimeError: {}", msg),
            CortexError::ConfigError(msg) => write!(f, "ConfigError: {}", msg),
            CortexError::SerializationError(msg) => write!(f, "SerializationError: {}", msg),
            CortexError::StateError(msg) => write!(f, "StateError: {}", msg),
            CortexError::SubsystemDisabled(msg) => write!(f, "SubsystemDisabled: {}", msg),
        }
    }
}

impl std::error::Error for CortexError {}

impl CortexError {
    pub fn kind(&self) -> &str {
        match self {
            CortexError::InputError(_) => "InputError",
            CortexError::EncodingError(_) => "EncodingError",
            CortexError::LanguageError(_) => "LanguageError",
            CortexError::MemoryError(_) => "MemoryError",
            CortexError::WorldModelError(_) => "WorldModelError",
            CortexError::ReasoningError(_) => "ReasoningError",
            CortexError::PlanningError(_) => "PlanningError",
            CortexError::VerificationError(_) => "VerificationError",
            CortexError::LearningError(_) => "LearningError",
            CortexError::PersistenceError(_) => "PersistenceError",
            CortexError::PolicyError(_) => "PolicyError",
            CortexError::ResourceError(_) => "ResourceError",
            CortexError::NetworkError(_) => "NetworkError",
            CortexError::RuntimeError(_) => "RuntimeError",
            CortexError::ConfigError(_) => "ConfigError",
            CortexError::SerializationError(_) => "SerializationError",
            CortexError::StateError(_) => "StateError",
            CortexError::SubsystemDisabled(_) => "SubsystemDisabled",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CortexError::NetworkError(_)
                | CortexError::ResourceError(_)
                | CortexError::InputError(_)
        )
    }

    pub fn http_status(&self) -> u16 {
        match self {
            CortexError::InputError(_) => 400,
            CortexError::EncodingError(_) => 400,
            CortexError::PolicyError(_) => 403,
            CortexError::SubsystemDisabled(_) => 501,
            CortexError::ResourceError(_) => 503,
            CortexError::NetworkError(_) => 502,
            _ => 500,
        }
    }

    pub fn error_code(&self) -> &str {
        match self {
            CortexError::InputError(_) => "CORTEX_ERR_001",
            CortexError::EncodingError(_) => "CORTEX_ERR_003",
            CortexError::LanguageError(_) => "CORTEX_ERR_008",
            CortexError::MemoryError(_) => "CORTEX_ERR_009",
            CortexError::WorldModelError(_) => "CORTEX_ERR_010",
            CortexError::ReasoningError(_) => "CORTEX_ERR_011",
            CortexError::PlanningError(_) => "CORTEX_ERR_012",
            CortexError::VerificationError(_) => "CORTEX_ERR_013",
            CortexError::LearningError(_) => "CORTEX_ERR_014",
            CortexError::PersistenceError(_) => "CORTEX_ERR_015",
            CortexError::PolicyError(_) => "CORTEX_ERR_016",
            CortexError::ResourceError(_) => "CORTEX_ERR_017",
            CortexError::NetworkError(_) => "CORTEX_ERR_018",
            CortexError::RuntimeError(_) => "CORTEX_ERR_019",
            CortexError::ConfigError(_) => "CORTEX_ERR_020",
            CortexError::SerializationError(_) => "CORTEX_ERR_024",
            CortexError::StateError(_) => "CORTEX_ERR_023",
            CortexError::SubsystemDisabled(_) => "CORTEX_ERR_025",
        }
    }
}

pub type Result<T> = std::result::Result<T, CortexError>;
