//! Error types for System Extension Manager

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid plist: {0}")]
    InvalidPlist(String),

    #[error("Shell command failed: {0}")]
    ShellCommandFailed(String),

    #[error("Extension activation failed: {0}")]
    ExtensionActivationFailed(String),

    #[error("LaunchCtl failed: {0}")]
    LaunchCtlFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] plist::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("User cancelled: {0}")]
    UserCancelled(String),
}

pub type AppResult<T> = Result<T, AppError>;