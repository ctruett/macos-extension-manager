//! Background task management item model

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackgroundItem {
    pub identifier: String,
    pub name: String,
    pub developer: String,
    pub type_str: String,
    pub enabled: bool,
    pub allowed: bool,
    pub uid: i64,
    pub plist_path: Option<PathBuf>,
}

impl BackgroundItem {
    pub fn display_name(&self) -> &str {
        if !self.name.is_empty() && self.name != "(null)" {
            &self.name
        } else {
            &self.identifier
        }
    }

    pub fn is_active(&self) -> bool {
        self.enabled && self.allowed
    }

    pub fn status_str(&self) -> &'static str {
        if !self.allowed {
            "disallowed"
        } else if self.enabled {
            "enabled"
        } else {
            "disabled"
        }
    }
}
