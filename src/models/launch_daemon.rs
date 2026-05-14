//! Launch daemon model

use crate::models::launch_agent::LaunchAgent;

/// LaunchDaemon is a system-level service (alias for LaunchAgent with admin requirements)
pub type LaunchDaemon = LaunchAgent;

impl LaunchDaemon {
    /// Default path for system-wide daemons
    pub const SYSTEM_DAEMONS_PATH: &'static str = "/Library/LaunchDaemons";

    /// Default path for user-specific daemons
    pub const USER_DAEMONS_PATH: &'static str = "~/Library/LaunchAgents";
}