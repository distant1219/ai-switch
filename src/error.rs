use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiSwitchError {
    #[error("Provider '{0}' not found")]
    ProviderNotFound(String),
    #[error("Target '{0}' not found")]
    TargetNotFound(String),
    #[error("Target type '{0}' is not supported")]
    UnsupportedTargetType(String),
    #[error("Provider '{0}' is already mapped to target '{1}'")]
    AlreadyMapped(String, String),
}
