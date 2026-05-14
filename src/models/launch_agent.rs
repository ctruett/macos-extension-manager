//! Launch agent model

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Standard paths configuration for a launch agent/daemon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StandardPaths {
    /// Path to run the program in
    pub run_at_load: Option<PathBuf>,

    /// Standard out path
    pub standard_out_path: Option<PathBuf>,

    /// Standard error path
    pub standard_error_path: Option<PathBuf>,

    /// Working directory
    pub working_directory: Option<PathBuf>,
}

/// Represents a LaunchAgent (user-level service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchAgent {
    /// Unique label for this agent
    pub label: String,

    /// Path to the .plist configuration file
    pub plist_path: PathBuf,

    /// Path to the program to execute
    pub program: PathBuf,

    /// Arguments to pass to the program
    pub program_arguments: Vec<String>,

    /// Whether to run at system startup
    pub run_at_load: bool,

    /// Whether to keep the agent running
    pub keep_alive: bool,

    /// Standard paths configuration
    pub standard_paths: StandardPaths,

    /// Whether the agent is currently loaded in launchd
    pub loaded: bool,

    /// Process ID if running
    pub pid: Option<u32>,

    /// Last exit status
    pub last_exit_status: Option<i32>,
}

impl LaunchAgent {
    /// Create a new LaunchAgent
    pub fn new(label: String, program: PathBuf) -> Self {
        Self {
            label,
            plist_path: PathBuf::new(),
            program,
            program_arguments: Vec::new(),
            run_at_load: false,
            keep_alive: false,
            standard_paths: StandardPaths::default(),
            loaded: false,
            pid: None,
            last_exit_status: None,
        }
    }

    /// Get the name from the label (last component after /)
    pub fn name(&self) -> String {
        self.label
            .rsplit('/')
            .next()
            .unwrap_or(&self.label)
            .to_string()
    }

    /// Get the bundle name from program path
    pub fn bundle_name(&self) -> String {
        self.program
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.name())
    }

    /// Check if the plist file exists
    pub fn plist_exists(&self) -> bool {
        self.plist_path.exists()
    }
}

impl std::fmt::Display for LaunchAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}]",
            self.label,
            if self.loaded { "loaded" } else { "unloaded" }
        )
    }
}

/// Represents a LaunchDaemon (system-level service)
pub type LaunchDaemon = LaunchAgent;

impl LaunchDaemon {
    /// Check if daemon requires admin privileges
    pub fn requires_admin(&self) -> bool {
        true
    }
}