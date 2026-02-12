use thiserror::Error;

/// Agent errors
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Shutdown failed: {0}")]
    ShutdownFailed(String),

    #[error("Communication failed: {0}")]
    CommunicationFailed(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}