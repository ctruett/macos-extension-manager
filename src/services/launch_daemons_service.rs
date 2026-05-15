//! Launch daemons service

use crate::error::AppResult;
use crate::models::LaunchDaemon;
use crate::utils::plist_parser::PlistParser;
use crate::utils::shell::ShellExecutor;
use std::path::{Path, PathBuf};
use tracing::info;

/// Service for managing launch daemons (requires admin privileges)
pub struct LaunchDaemonsService;

impl LaunchDaemonsService {
    /// Path to system launch daemons directory
    const DAEMONS_PATH: &'static str = "/Library/LaunchDaemons";

    /// List all launch daemons
    pub fn list() -> AppResult<Vec<LaunchDaemon>> {
        info!("Listing launch daemons (requires admin for system daemons)");

        // Get currently loaded daemons
        let result = ShellExecutor::launchctl(&["list"])?;
        let loaded_daemons = Self::parse_loaded_daemons(&result.stdout);

        let mut daemons = Vec::new();
        let daemons_path = PathBuf::from(Self::DAEMONS_PATH);

        if daemons_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&daemons_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "plist").unwrap_or(false) {
                        if let Ok(daemon) = Self::parse_daemon(&path, &loaded_daemons) {
                            daemons.push(daemon);
                        }
                    }
                }
            }
        }

        Ok(daemons)
    }

    /// Parse a daemon from a plist
    fn parse_daemon(
        path: &Path,
        loaded_daemons: &std::collections::HashMap<String, (i32, Option<u32>)>,
    ) -> AppResult<LaunchDaemon> {
        let dict = PlistParser::read(path)?;

        let label = PlistParser::get_string(&dict, "Label")
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string()
            });

        let program = PlistParser::get_path(&dict, "Program")
            .or_else(|| {
                PlistParser::get_array(&dict, "ProgramArguments")
                    .and_then(|arr| arr.into_iter().next())
                    .and_then(|v| v.as_string().map(|s| s.to_string()))
                    .map(PathBuf::from)
            })
            .unwrap_or_else(|| PathBuf::from("/usr/bin/false"));

        let program_arguments: Vec<String> = PlistParser::get_array(&dict, "ProgramArguments")
            .map(|arr| {
                arr.into_iter()
                    .filter_map(|v| v.as_string().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let run_at_load = PlistParser::get_bool(&dict, "RunAtLoad");
        let keep_alive = PlistParser::get_bool(&dict, "KeepAlive");

        let (pid, last_exit_status) = loaded_daemons
            .get(&label)
            .map(|(status, pid)| (*pid, Some(*status)))
            .unwrap_or((None, None));

        let loaded = pid.is_some() || last_exit_status.is_some();

        Ok(LaunchDaemon {
            label,
            plist_path: path.to_path_buf(),
            program,
            program_arguments,
            run_at_load,
            keep_alive,
            standard_paths: Default::default(),
            loaded,
            pid,
            last_exit_status,
        })
    }

    /// Parse loaded daemons from launchctl list output
    fn parse_loaded_daemons(output: &str) -> std::collections::HashMap<String, (i32, Option<u32>)> {
        let mut daemons = std::collections::HashMap::new();

        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let label = parts[2].trim_matches('"').to_string();
                // Only include system daemons (com.apple.* or those in /Library/LaunchDaemons)
                if label.contains('.') && !label.starts_with("com.apple.") {
                    continue;
                }
                let status = parts[0].parse::<i32>().unwrap_or(-1);
                let pid = if parts[1] == "-" {
                    None
                } else {
                    parts[1].parse::<u32>().ok()
                };
                daemons.insert(label, (status, pid));
            }
        }

        daemons
    }

    /// Load a launch daemon (requires admin)
    pub fn load(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Loading launch daemon (admin required): {} ({})", label, plist_path);
        ShellExecutor::launchctl_admin("load", plist_path)?;
        Ok(())
    }

    /// Unload a launch daemon (requires admin)
    pub fn unload(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Unloading launch daemon (admin required): {} ({})", label, plist_path);
        ShellExecutor::launchctl_admin("unload", plist_path)?;
        Ok(())
    }

    /// Create a new launch daemon (requires admin)
    pub fn create(
        label: &str,
        program: &str,
        run_at_load: bool,
        keep_alive: bool,
    ) -> AppResult<LaunchDaemon> {
        info!("Creating launch daemon (admin required): {}", label);

        let plist_path = PathBuf::from(Self::DAEMONS_PATH).join(format!("{}.plist", label));

        let mut dict = PlistParser::create_launch_agent_plist(label, program, run_at_load);
        
        if keep_alive {
            dict.insert("KeepAlive".to_string(), plist::Value::Boolean(true));
        }

        PlistParser::write(&plist_path, &dict)?;

        let daemon = LaunchDaemon {
            label: label.to_string(),
            plist_path: plist_path.clone(),
            program: PathBuf::from(program),
            program_arguments: vec![program.to_string()],
            run_at_load,
            keep_alive,
            standard_paths: Default::default(),
            loaded: false,
            pid: None,
            last_exit_status: None,
        };

        // Load the daemon (will prompt for admin)
        Self::load(label, &plist_path.to_string_lossy())?;

        Ok(daemon)
    }

    /// Delete a launch daemon (requires admin)
    pub fn delete(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Deleting launch daemon (admin required): {} ({})", label, plist_path);
        
        // Unload first (will prompt for admin)
        let _ = Self::unload(label, plist_path);
        
        // Delete the plist
        std::fs::remove_file(plist_path)?;
        
        Ok(())
    }
}