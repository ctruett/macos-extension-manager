//! System Extension Manager - A TUI application for managing macOS system extensions
//!
//! This library provides functionality to manage:
//! - Login Items
//! - Launch Agents
//! - Launch Daemons
//! - System Extensions

pub mod error;
pub mod models;
pub mod services;
pub mod state;
pub mod ui;
pub mod utils;

pub use error::{AppError, AppResult};
pub use models::{
    BackgroundItem, ExtensionStatus, ExtensionType, ItemType, LaunchAgent, LaunchDaemon, LoginItem,
    OpenAtLoginItem, SystemExtension,
};
pub use services::{
    BackgroundItemsService, LaunchAgentsService, LaunchDaemonsService, LoginItemsService,
    OpenAtLoginService, PrivilegeService, SystemExtensionsService,
};
pub use state::{AppState, LoadingState};
pub use ui::TuiApp;
pub use utils::{PlistParser, ShellExecutor};