//! Login items service

use crate::error::AppResult;
use crate::models::LoginItem;
use crate::utils::shell::ShellExecutor;
use crate::utils::plist_parser::PlistParser;
use std::path::PathBuf;
use tracing::{debug, info};

/// Service for managing login items
pub struct LoginItemsService;

impl LoginItemsService {
    /// List all login items
    pub fn list() -> AppResult<Vec<LoginItem>> {
        info!("Listing login items");
        let mut items = Vec::new();

        // First, get login items from launchctl
        let result = ShellExecutor::launchctl(&["list"])?;
        let loaded_labels: std::collections::HashSet<String> = result
            .stdout
            .lines()
            .skip(1) // Skip header
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    Some(parts[2].trim_matches('"').to_string())
                } else {
                    None
                }
            })
            .collect();

        // Look for login item plists in ~/Library/LaunchAgents
        let home = std::env::var("HOME").unwrap_or_default();
        let launch_agents_dir = PathBuf::from(&home).join("Library").join("LaunchAgents");

        if launch_agents_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&launch_agents_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.extension().map(|e| e == "plist").unwrap_or(false) {
                        if let Ok(item) = Self::parse_login_item(&path, &loaded_labels) {
                            debug!("Found login item: {}", item.name);
                            items.push(item);
                        }
                    }
                }
            }
        }

        // Also check /Library/LaunchAgents for system-wide
        let system_agents_dir = PathBuf::from("/Library/LaunchAgents");
        if system_agents_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&system_agents_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.extension().map(|e| e == "plist").unwrap_or(false) {
                        if let Ok(item) = Self::parse_login_item(&path, &loaded_labels) {
                            debug!("Found system login item: {}", item.name);
                            items.push(item);
                        }
                    }
                }
            }
        }

        Ok(items)
    }

    /// Parse a login item from a plist file
    fn parse_login_item(
        path: &PathBuf,
        loaded_labels: &std::collections::HashSet<String>,
    ) -> AppResult<LoginItem> {
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

        let name = program
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&label)
            .to_string();

        let loaded = loaded_labels.contains(&label);

        Ok(LoginItem {
            id: label.clone(),
            name,
            path: program,
            enabled: loaded,
            hidden: false,
            plist_path: Some(path.clone()),
        })
    }

    /// Enable a login item
    pub fn enable(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Enabling login item: {}", label);
        ShellExecutor::launchctl(&["load", plist_path])?;
        Ok(())
    }

    /// Disable a login item
    pub fn disable(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Disabling login item: {}", label);
        ShellExecutor::launchctl(&["unload", plist_path])?;
        Ok(())
    }

    /// Add a new login item
    pub fn add(program_path: &str, run_at_load: bool) -> AppResult<LoginItem> {
        info!("Adding login item: {}", program_path);

        let program = PathBuf::from(program_path);
        let name = program.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown");
        let label = format!("com.loginitem.{}", name.to_lowercase().replace(' ', ""));

        // Create plist content
        let home = std::env::var("HOME").unwrap_or_default();
        let plist_dir = PathBuf::from(&home)
            .join("Library")
            .join("LaunchAgents");
        
        std::fs::create_dir_all(&plist_dir)?;

        let plist_path = plist_dir.join(format!("{}.plist", label));

        let dict = PlistParser::create_launch_agent_plist(&label, program_path, run_at_load);
        PlistParser::write(&plist_path, &dict)?;

        let item = LoginItem {
            id: label.clone(),
            name: name.to_string(),
            path: program,
            enabled: false,
            hidden: false,
            plist_path: Some(plist_path.clone()),
        };

        // Load the new login item
        ShellExecutor::launchctl(&["load", &plist_path.to_string_lossy()])?;

        Ok(item)
    }

    /// Remove a login item
    pub fn remove(label: &str, plist_path: &str) -> AppResult<()> {
        info!("Removing login item: {}", label);
        
        // First unload
        let _ = ShellExecutor::launchctl(&["unload", plist_path]);
        
        // Then delete the plist
        std::fs::remove_file(plist_path)?;
        
        Ok(())
    }
}