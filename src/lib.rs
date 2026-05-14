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
pub use models::{ItemType, LaunchAgent, LaunchDaemon, LoginItem, SystemExtension};
pub use services::{
    LaunchAgentsService, LaunchDaemonsService, LoginItemsService, PrivilegeService,
    SystemExtensionsService,
};
pub use state::{AppState, LoadingState};
pub use ui::TuiApp;
pub use utils::{PlistParser, ShellExecutor};