//! Privilege service for admin authentication

use crate::error::{AppError, AppResult};
use crate::utils::shell::ShellExecutor;
use tracing::info;

/// Service for handling admin privilege operations
pub struct PrivilegeService;

impl PrivilegeService {
    /// Check if we have admin privileges
    pub fn check_privileges() -> AppResult<bool> {
        // Try a simple command that requires admin
        let result = ShellExecutor::execute("osascript", &[
            "-e",
            "do shell script \"echo test\" with administrator privileges"
        ])?;

        Ok(result.success)
    }

    /// Request admin privileges for an operation
    pub fn authorize(operation: &str) -> AppResult<()> {
        info!("Requesting admin privileges for: {}", operation);
        
        // Use AppleScript to prompt for admin
        let script = format!(
            "do shell script \"echo Authorization granted for: {}\" with administrator privileges",
            operation
        );

        let result = ShellExecutor::execute("osascript", &["-e", &script])?;

        if !result.success {
            if result.stderr.contains("User canceled") {
                return Err(AppError::UserCancelled("Admin authorization was cancelled by user".into()));
            }
            return Err(AppError::PermissionDenied(
                format!("Failed to obtain admin privileges: {}", result.stderr)
            ));
        }

        Ok(())
    }

    /// Execute a command with admin privileges
    pub fn execute_admin(command: &str, args: &[&str]) -> AppResult<String> {
        let full_command = format!("{} {}", command, args.join(" "));
        
        let script = format!(
            "do shell script \"{}\" with administrator privileges",
            full_command
        );

        let result = ShellExecutor::execute("osascript", &["-e", &script])?;

        if !result.success {
            if result.stderr.contains("User canceled") {
                return Err(AppError::UserCancelled("Admin authorization was cancelled by user".into()));
            }
            return Err(AppError::PermissionDenied(result.stderr.clone()));
        }

        Ok(result.stdout)
    }
}