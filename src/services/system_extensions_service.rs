//! System extensions service

use crate::error::{AppError, AppResult};
use crate::models::{ExtensionStatus, ExtensionType, SystemExtension};
use crate::utils::shell::ShellExecutor;
use tracing::{debug, info};

/// Service for managing system extensions
pub struct SystemExtensionsService;

impl SystemExtensionsService {
    /// List all system extensions
    pub fn list() -> AppResult<Vec<SystemExtension>> {
        info!("Listing system extensions");

        let result = ShellExecutor::execute("systemextensionsctl", &["list"])?;

        if !result.success {
            // systemextensionsctl may require elevated privileges
            debug!("systemextensionsctl list failed: {}", result.stderr);
            return Ok(Vec::new());
        }

        let extensions = Self::parse_extensions_output(&result.stdout);
        Ok(extensions)
    }

    /// Parse the output of systemextensionsctl list
    fn parse_extensions_output(output: &str) -> Vec<SystemExtension> {
        let mut extensions = Vec::new();

        // Skip header lines and category headers
        let mut in_extension = false;

        for line in output.lines() {
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            
            // Skip category headers like "5 extension(s)" and "--- com.apple.system_extension.xxx"
            if line.starts_with("---") || line.starts_with("There are") || line.starts_with("enabled") || line.starts_with("*") {
                if line.starts_with("enabled") || line.starts_with("*") {
                    // This is a data line
                    if let Some(ext) = Self::parse_extension_line(line) {
                        debug!("Found system extension: {}", ext.identifier);
                        extensions.push(ext);
                    }
                }
                continue;
            }
            
            // Skip lines with instructions
            if line.starts_with("Go to 'System Settings") {
                continue;
            }

            // Check if it's a data line (starts with * or is tab-separated)
            if line.starts_with('*') || line.contains('\t') {
                if let Some(ext) = Self::parse_extension_line(line) {
                    debug!("Found system extension: {}", ext.identifier);
                    extensions.push(ext);
                }
            }
        }

        extensions
    }

    /// Parse a single line of systemextensionsctl output
    /// Format: *	*	teamID	bundleID (version)	name	[state]
    fn parse_extension_line(line: &str) -> Option<SystemExtension> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        // Split by tabs
        let parts: Vec<&str> = line.split('\t').collect();
        
        // Need at least: enabled, active, teamID, bundleID (version), name, [state]
        if parts.len() < 6 {
            return None;
        }

        // Extract bundle ID and version from "bundleID (version)" format
        let bundle_version = parts.get(3)?; // e.g., "org.pqrs.Karabiner-DriverKit-VirtualHIDDevice (1.8.0/1.8.0)"
        let (bundle_id, version) = Self::parse_bundle_version(bundle_version)?;
        
        // Extract name from parts[4]
        let name = parts.get(4)?.trim().to_string();
        
        // Extract state from parts[5] e.g., "[activated enabled]"
        let state = parts.get(5)?;
        let (status, _active_state) = Self::parse_state(state)?;

        // Determine extension type from bundle ID or name
        let ext_type = Self::infer_extension_type(&bundle_id, &name);
        
        let mut ext = SystemExtension::new(bundle_id);
        ext.version = version;
        ext.status = status;
        ext.extension_types = vec![ext_type];
        ext.name = Some(name);

        Some(ext)
    }

    /// Parse "bundleID (version)" into parts
    fn parse_bundle_version(s: &str) -> Option<(String, String)> {
        // Format: "org.pqrs.Karabiner (1.0.0/1.0.0)" or "org.pqrs.Karabiner (1.0.0)"
        let (bundle_id, version_part) = s.rsplit_once('(')?;
        let version = version_part.trim_end_matches(')').to_string();
        Some((bundle_id.trim().to_string(), version))
    }

    /// Parse state like "[activated enabled]" or "[deactivated waiting for user]"
    fn parse_state(state: &str) -> Option<(ExtensionStatus, String)> {
        let state = state.trim();
        let state = state.trim_start_matches('[').trim_end_matches(']');
        
        let parts: Vec<&str> = state.split_whitespace().collect();
        
        let status = match parts.first() {
            Some(&"activated") => ExtensionStatus::Activated,
            Some(&"deactivated") => ExtensionStatus::Deactivated,
            Some(&"pending") => ExtensionStatus::Pending,
            Some(&"failed") => ExtensionStatus::Failed,
            _ => ExtensionStatus::Unknown,
        };
        
        let detail = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
        
        Some((status, detail))
    }

    /// Infer extension type from bundle ID or name
    fn infer_extension_type(bundle_id: &str, _name: &str) -> ExtensionType {
        if bundle_id.contains("networkextension") || _name.to_lowercase().contains("network") {
            ExtensionType::Network
        } else if bundle_id.contains("driver") || _name.to_lowercase().contains("driver") {
            ExtensionType::Driver
        } else if bundle_id.contains("endpoint") || _name.to_lowercase().contains("endpoint") {
            ExtensionType::EndpointSecurity
        } else {
            ExtensionType::AppExtension
        }
    }

    /// Activate a system extension (requires admin)
    pub fn activate(identifier: &str) -> AppResult<()> {
        info!("Activating system extension (admin required): {}", identifier);
        
        let result = ShellExecutor::execute_admin("systemextensionsctl", &["enable", identifier])?;
        
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(
                result.stderr.clone()
            ));
        }
        
        Ok(())
    }

    /// Deactivate a system extension (requires admin)
    pub fn deactivate(identifier: &str) -> AppResult<()> {
        info!("Deactivating system extension (admin required): {}", identifier);
        
        let result = ShellExecutor::execute_admin("systemextensionsctl", &["disable", identifier])?;
        
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(
                result.stderr.clone()
            ));
        }
        
        Ok(())
    }

    /// Install a system extension from a code signature (requires admin)
    pub fn install(request_path: &str) -> AppResult<()> {
        info!("Installing system extension (admin required) from: {}", request_path);
        
        // Use systemextensionsctl to install
        let result = ShellExecutor::execute_admin("systemextensionsctl", &["install", request_path])?;
        
        if !result.success {
            return Err(AppError::ExtensionActivationFailed(
                result.stderr.clone()
            ));
        }
        
        Ok(())
    }
}