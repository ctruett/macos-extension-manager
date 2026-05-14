//! Item type enum representing different system extension categories

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ItemType {
    #[default]
    LoginItem,
    LaunchAgent,
    LaunchDaemon,
    SystemExtension,
    BackgroundItem,
}


impl ItemType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemType::LoginItem => "Login Items",
            ItemType::LaunchAgent => "Launch Agents",
            ItemType::LaunchDaemon => "Launch Daemons",
            ItemType::SystemExtension => "System Extensions",
            ItemType::BackgroundItem => "Background Items",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ItemType::LoginItem => "→",
            ItemType::LaunchAgent => "▶",
            ItemType::LaunchDaemon => "⚙",
            ItemType::SystemExtension => "◉",
            ItemType::BackgroundItem => "◈",
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}