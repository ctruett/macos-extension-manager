//! Background task management service (no elevated privileges required)

use crate::error::{AppError, AppResult};
use crate::models::BackgroundItem;
use crate::utils::plist_parser::PlistParser;
use crate::utils::shell::ShellExecutor;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::info;

pub struct BackgroundItemsService;

impl BackgroundItemsService {
    pub fn list() -> AppResult<Vec<BackgroundItem>> {
        info!("Listing background items via launchctl");

        // Current user UID
        let uid_result = ShellExecutor::execute("id", &["-u"])?;
        let uid_str = uid_result.stdout.trim().to_string();
        let uid: i64 = uid_str.parse().unwrap_or(0);

        // Get all user-domain service states — no auth required
        let domain = format!("gui/{}", uid_str);
        let result = ShellExecutor::execute("launchctl", &["print-disabled", &domain])?;
        if !result.success {
            return Ok(Vec::new());
        }

        // Build set of labels already covered by the Launch Agents section
        // so we don't duplicate them here
        let agent_labels = Self::known_agent_labels();

        // Build bundle-id → display-name map from installed apps
        let name_map = Self::build_name_map();

        let mut items = Vec::new();

        for line in result.stdout.lines() {
            let line = line.trim();
            let (identifier, enabled) = match Self::parse_line(line) {
                Some(pair) => pair,
                None => continue,
            };

            // Skip items already shown in Launch Agents
            if agent_labels.contains(&identifier) {
                continue;
            }
            // Skip Apple-internal system services
            if identifier.starts_with("com.apple.") {
                continue;
            }

            let name = name_map
                .get(&identifier)
                .cloned()
                .unwrap_or_else(|| identifier.clone());

            items.push(BackgroundItem {
                identifier: identifier.clone(),
                name,
                developer: String::new(),
                type_str: "app".to_string(),
                enabled,
                allowed: true,
                uid,
                plist_path: None,
            });
        }

        Ok(items)
    }

    /// Parse a single line from `launchctl print-disabled` output.
    /// Expected format: `"com.example.service" => enabled`
    fn parse_line(line: &str) -> Option<(String, bool)> {
        let (id_part, state_part) = line.split_once(" => ")?;
        let id = id_part.trim().trim_matches('"').to_string();
        let state = state_part.trim();
        if id.is_empty() || (state != "enabled" && state != "disabled") {
            return None;
        }
        Some((id, state == "enabled"))
    }

    /// Collect plist file stems from known LaunchAgents directories
    fn known_agent_labels() -> HashSet<String> {
        let mut labels = HashSet::new();
        let home = std::env::var("HOME").unwrap_or_default();
        let dirs = [
            format!("{}/Library/LaunchAgents", home),
            "/Library/LaunchAgents".to_string(),
        ];
        for dir in &dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "plist").unwrap_or(false) {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            labels.insert(stem.to_string());
                        }
                    }
                }
            }
        }
        labels
    }

    /// Scan /Applications/ and ~/Applications/ to build bundle-id → display-name map
    fn build_name_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        let home = std::env::var("HOME").unwrap_or_default();
        let dirs: Vec<PathBuf> = vec![
            PathBuf::from("/Applications"),
            PathBuf::from(format!("{}/Applications", home)),
        ];

        for dir in &dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let app_path = entry.path();
                    if app_path.extension().map(|e| e == "app").unwrap_or(false) {
                        let info_plist = app_path.join("Contents/Info.plist");
                        if let Ok(dict) = PlistParser::read(&info_plist) {
                            let bundle_id = PlistParser::get_string(&dict, "CFBundleIdentifier");
                            let name = PlistParser::get_string(&dict, "CFBundleDisplayName")
                                .or_else(|| PlistParser::get_string(&dict, "CFBundleName"));
                            if let (Some(id), Some(nm)) = (bundle_id, name) {
                                map.insert(id, nm);
                            }
                        }
                    }
                }
            }
        }

        map
    }

    pub fn enable(item: &BackgroundItem) -> AppResult<()> {
        info!("Enabling background item: {}", item.identifier);
        let domain = format!("gui/{}", item.uid);
        let target = format!("{}/{}", domain, item.identifier);
        let result = ShellExecutor::execute("launchctl", &["enable", &target])?;
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(result.stderr.clone()));
        }
        Ok(())
    }

    pub fn disable(item: &BackgroundItem) -> AppResult<()> {
        info!("Disabling background item: {}", item.identifier);
        let domain = format!("gui/{}", item.uid);
        let target = format!("{}/{}", domain, item.identifier);
        let result = ShellExecutor::execute("launchctl", &["disable", &target])?;
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(result.stderr.clone()));
        }
        Ok(())
    }

    /// Disable and stop a background item (no plist to delete; marks it permanently disabled)
    pub fn delete(item: &BackgroundItem) -> AppResult<()> {
        info!("Deleting background item: {}", item.identifier);
        let domain = format!("gui/{}", item.uid);
        let target = format!("{}/{}", domain, item.identifier);
        // Stop if running
        let _ = ShellExecutor::execute("launchctl", &["bootout", &target]);
        // Permanently disable
        let result = ShellExecutor::execute("launchctl", &["disable", &target])?;
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(result.stderr.clone()));
        }
        Ok(())
    }
}
