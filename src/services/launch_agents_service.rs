//! Launch agents service

use crate::error::AppResult;
use crate::models::LaunchAgent;
use crate::utils::plist_parser::PlistParser;
use crate::utils::shell::ShellExecutor;
use std::path::PathBuf;
use tracing::{debug, info};

/// Service for managing launch agents
pub struct LaunchAgentsService;

impl LaunchAgentsService {
    /// Path to user launch agents directory
    const AGENTS_PATH: &'static str = "~/Library/LaunchAgents";

    /// Path to system launch agents directory
    const SYSTEM_AGENTS_PATH: &'static str = "/Library/LaunchAgents";

    /// List all launch agents
    pub fn list() -> AppResult<Vec<LaunchAgent>> {
        info!("Listing launch agents");
        let mut agents = Vec::new();

        // Get currently loaded agents from launchctl
        let result = ShellExecutor::launchctl(&["list"])?;
        let loaded_agents = Self::parse_loaded_agents(&result.stdout);

        // Scan user agents directory
        let home = std::env::var("HOME").unwrap_or_default();
        let user_path = PathBuf::from(&home).join("Library").join("LaunchAgents");
        
        if user_path.exists() {
            Self::scan_agents_dir(&user_path, &loaded_agents, &mut agents)?;
        }

        // Scan system agents directory (if readable)
        let system_path = PathBuf::from(Self::SYSTEM_AGENTS_PATH);
        if system_path.exists() {
            if let Ok(()) = std::fs::read_dir(&system_path).map(|_| ()) {
                Self::scan_agents_dir(&system_path, &loaded_agents, &mut agents)?;
            } else {
                debug!("Cannot read system agents directory (permission denied)");
            }
        }

        Ok(agents)
    }

    /// Scan a directory for launch agents
    fn scan_agents_dir(
        dir: &PathBuf,
        loaded_agents: &std::collections::HashMap<String, (i32, Option<u32>)>,
        agents: &mut Vec<LaunchAgent>,
    ) -> AppResult<()> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.extension().map(|e| e == "plist").unwrap_or(false) {
                    if let Ok(agent) = Self::parse_agent(&path, loaded_agents) {
                        debug!("Found agent: {}", agent.label);
                        agents.push(agent);
                    }
                }
            }
        }
        Ok(())
    }

    /// Parse a launch agent from a plist file
    fn parse_agent(
        path: &PathBuf,
        loaded_agents: &std::collections::HashMap<String, (i32, Option<u32>)>,
    ) -> AppResult<LaunchAgent> {
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

        let (pid, last_exit_status) = loaded_agents
            .get(&label)
            .map(|(status, pid)| (*pid, Some(*status)))
            .unwrap_or((None, None));

        let loaded = pid.is_some() || last_exit_status.is_some();

        Ok(LaunchAgent {
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

    /// Parse the output of launchctl list
    fn parse_loaded_agents(output: &str) -> std::collections::HashMap<String, (i32, Option<u32>)> {
        let mut agents = std::collections::HashMap::new();

        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let label = parts[2].trim_matches('"').to_string();
                let status = parts[0].parse::<i32>().unwrap_or(-1);
                let pid = if parts[1] == "-" {
                    None
                } else {
                    parts[1].parse::<u32>().ok()
                };
                agents.insert(label, (status, pid));
            }
        }

        agents
    }

    /// Load a launch agent
    pub fn load(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Loading launch agent: {} ({})", label, plist_path);
        ShellExecutor::launchctl(&["load", plist_path])?;
        Ok(())
    }

    /// Unload a launch agent
    pub fn unload(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Unloading launch agent: {} ({})", label, plist_path);
        ShellExecutor::launchctl(&["unload", plist_path])?;
        Ok(())
    }

    /// Create a new launch agent
    pub fn create(
        label: &str,
        program: &str,
        run_at_load: bool,
        keep_alive: bool,
    ) -> AppResult<LaunchAgent> {
        info!("Creating launch agent: {}", label);

        let home = std::env::var("HOME").unwrap_or_default();
        let plist_dir = PathBuf::from(&home).join("Library").join("LaunchAgents");
        std::fs::create_dir_all(&plist_dir)?;

        let plist_path = plist_dir.join(format!("{}.plist", label));

        let mut dict = PlistParser::create_launch_agent_plist(label, program, run_at_load);
        
        if keep_alive {
            dict.insert(
                "KeepAlive".to_string(),
                plist::Value::Boolean(true),
            );
        }

        PlistParser::write(&plist_path, &dict)?;

        let agent = LaunchAgent {
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

        // Load the agent
        Self::load(label, &plist_path.to_string_lossy())?;

        Ok(agent)
    }

    /// Delete a launch agent
    pub fn delete(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Deleting launch agent: {} ({})", label, plist_path);
        
        // Unload first
        let _ = Self::unload(label, plist_path);
        
        // Delete the plist
        std::fs::remove_file(plist_path)?;
        
        Ok(())
    }
}