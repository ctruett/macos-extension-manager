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

        for line in output.lines() {
            // Format: identifier [type] (version) [status]
            // Example: com.example.extension [driver extension] (1.0.0) [activated]
            if let Some(ext) = Self::parse_extension_line(line) {
                debug!("Found system extension: {}", ext.identifier);
                extensions.push(ext);
            }
        }

        extensions
    }

    /// Parse a single line of systemextensionsctl output
    fn parse_extension_line(line: &str) -> Option<SystemExtension> {
        let line = line.trim();
        if line.is_empty() || line.starts_with("system extension") || line.starts_with("There are") {
            return None;
        }

        // Simple parsing - look for identifier pattern (com.domain.name)
        let identifier = if let Some(start) = line.find("com.") {
            line[start..]
                .split_whitespace()
                .next()
                .unwrap_or(line)
                .trim_end_matches(')')
                .to_string()
        } else {
            return None;
        };

        let mut ext = SystemExtension::new(identifier);

        // Parse status
        if line.contains("[activated]") || line.contains("activated") {
            ext.status = ExtensionStatus::Activated;
        } else if line.contains("[deactivated]") || line.contains("deactivated") {
            ext.status = ExtensionStatus::Deactivated;
        } else if line.contains("[pending]") || line.contains("pending") {
            ext.status = ExtensionStatus::Pending;
        } else if line.contains("[failed]") || line.contains("failed") {
            ext.status = ExtensionStatus::Failed;
        }

        // Parse version (parenthesis content)
        if let Some(start) = line.find('(') {
            if let Some(end) = line[start..].find(')') {
                ext.version = line[start + 1..start + end].to_string();
            }
        }

        // Parse extension type
        if line.contains("driver extension") || line.contains("Driver") {
            ext.extension_types.push(ExtensionType::Driver);
        }
        if line.contains("network extension") || line.contains("Network") {
            ext.extension_types.push(ExtensionType::Network);
        }
        if line.contains("endpoint security extension") || line.contains("Endpoint") {
            ext.extension_types.push(ExtensionType::EndpointSecurity);
        }
        if line.contains("app extension") || line.contains("App") {
            ext.extension_types.push(ExtensionType::AppExtension);
        }

        if ext.extension_types.is_empty() {
            ext.extension_types.push(ExtensionType::AppExtension); // Default
        }

        Some(ext)
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