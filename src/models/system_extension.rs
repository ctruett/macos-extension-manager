//! System extension model

use serde::{Deserialize, Serialize};

/// Status of a system extension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ExtensionStatus {
    /// Extension is activated and running
    Activated,
    /// Extension is installed but not activated
    Deactivated,
    /// Extension activation is pending
    Pending,
    /// Extension activation failed
    Failed,
    /// Extension status is unknown
    #[default]
    Unknown,
}


impl std::fmt::Display for ExtensionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionStatus::Activated => write!(f, "Activated"),
            ExtensionStatus::Deactivated => write!(f, "Deactivated"),
            ExtensionStatus::Pending => write!(f, "Pending"),
            ExtensionStatus::Failed => write!(f, "Failed"),
            ExtensionStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Type of system extension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    /// Driver extension
    Driver,
    /// Network extension
    Network,
    /// Endpoint security extension
    EndpointSecurity,
    /// App extension
    AppExtension,
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionType::Driver => write!(f, "Driver"),
            ExtensionType::Network => write!(f, "Network"),
            ExtensionType::EndpointSecurity => write!(f, "Endpoint Security"),
            ExtensionType::AppExtension => write!(f, "App Extension"),
        }
    }
}

/// Represents a System Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemExtension {
    /// Bundle identifier
    pub identifier: String,

    /// Extension version
    pub version: String,

    /// Extension types
    pub extension_types: Vec<ExtensionType>,

    /// Current status
    pub status: ExtensionStatus,

    /// Path to the extension bundle
    pub path: Option<std::path::PathBuf>,
}

impl SystemExtension {
    /// Create a new SystemExtension
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            version: String::new(),
            extension_types: Vec::new(),
            status: ExtensionStatus::Unknown,
            path: None,
        }
    }

    /// Check if the extension is activated
    pub fn is_activated(&self) -> bool {
        self.status == ExtensionStatus::Activated
    }
}

impl std::fmt::Display for SystemExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{} [{}]",
            self.identifier,
            self.version,
            self.status
        )
    }
}