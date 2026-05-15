//! Item type enum representing different system extension categories

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ItemType {
    #[default]
    LoginItem,
    OpenAtLogin,
    LaunchAgent,
    LaunchDaemon,
    SystemExtension,
    BackgroundItem,
}


impl ItemType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemType::LoginItem => "Login Items",
            ItemType::OpenAtLogin => "Open at Login",
            ItemType::LaunchAgent => "Launch Agents",
            ItemType::LaunchDaemon => "Launch Daemons",
            ItemType::SystemExtension => "System Extensions",
            ItemType::BackgroundItem => "Background Items",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ItemType::LoginItem => "→",
            ItemType::OpenAtLogin => "⇪",
            ItemType::LaunchAgent => "▶",
            ItemType::LaunchDaemon => "⚙",
            ItemType::SystemExtension => "◉",
            ItemType::BackgroundItem => "◈",
        }
    }

    /// Return a distinct RGB color for this type.
    /// Colors are chosen with similar saturation/lightness so they feel cohesive.
    pub fn rgb_color(&self) -> (u8, u8, u8) {
        match self {
            ItemType::LoginItem => (230, 170, 60),     // amber/gold
            ItemType::OpenAtLogin => (230, 130, 70),   // orange
            ItemType::LaunchAgent => (70, 210, 100),    // green
            ItemType::LaunchDaemon => (70, 150, 230),   // blue
            ItemType::SystemExtension => (180, 70, 230), // purple
            ItemType::BackgroundItem => (70, 220, 200),  // teal/cyan
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}