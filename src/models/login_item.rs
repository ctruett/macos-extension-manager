//! Login item model

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a login item (application that starts at login)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginItem {
    /// Unique identifier for the login item
    pub id: String,

    /// Display name of the application
    pub name: String,

    /// Path to the application bundle
    pub path: PathBuf,

    /// Whether the login item is currently enabled
    pub enabled: bool,

    /// Whether the item should be hidden at launch
    pub hidden: bool,

    /// Path to the .plist file if stored as LaunchAgent
    pub plist_path: Option<PathBuf>,
}

impl LoginItem {
    /// Create a new LoginItem
    pub fn new(id: String, name: String, path: PathBuf) -> Self {
        Self {
            id,
            name,
            path,
            enabled: true,
            hidden: false,
            plist_path: None,
        }
    }

    /// Get the file name as the display name
    pub fn file_name(&self) -> String {
        self.path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.name)
            .to_string()
    }
}

impl std::fmt::Display for LoginItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, if self.enabled { "enabled" } else { "disabled" })
    }
}