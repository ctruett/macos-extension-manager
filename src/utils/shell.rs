//! Shell command executor utility

use crate::error::{AppError, AppResult};
use std::process::{Command, Output};
use tracing::{debug, info};

/// Result of a shell command execution
#[derive(Debug)]
pub struct CommandResult {
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Exit code
    pub exit_code: i32,
    /// Whether the command succeeded
    pub success: bool,
}

impl CommandResult {
    /// Create a new CommandResult from a process Output
    fn from_output(output: Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Self {
            stdout,
            stderr,
            exit_code,
            success,
        }
    }
}

/// Shell executor for running system commands
pub struct ShellExecutor;

impl ShellExecutor {
    /// Execute a shell command and return the result
    pub fn execute(command: &str, args: &[&str]) -> AppResult<CommandResult> {
        debug!("Executing command: {} {:?}", command, args);

        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| AppError::ShellCommandFailed(format!("Failed to execute {}: {}", command, e)))?;

        let result = CommandResult::from_output(output);
        debug!("Command exit code: {}", result.exit_code);

        if !result.stdout.is_empty() {
            debug!("stdout: {}", result.stdout.trim());
        }
        if !result.stderr.is_empty() {
            debug!("stderr: {}", result.stderr.trim());
        }

        Ok(result)
    }

    /// Execute a shell command and check if it succeeded
    pub fn run(command: &str, args: &[&str]) -> AppResult<()> {
        let result = Self::execute(command, args)?;

        if !result.success {
            let error_msg = if result.stderr.is_empty() {
                format!("Command {} failed with exit code {}", command, result.exit_code)
            } else {
                result.stderr.clone()
            };
            return Err(AppError::ShellCommandFailed(error_msg));
        }

        Ok(())
    }

    /// Execute a command with admin privileges via osascript
    pub fn execute_admin(command: &str, args: &[&str]) -> AppResult<CommandResult> {
        let full_command = format!(
            "do shell script \"{} {}\" with administrator privileges",
            command,
            args.join(" ")
        );

        info!("Executing admin command via osascript");

        let output = Command::new("osascript")
            .args(["-e", &full_command])
            .output()
            .map_err(|e| AppError::ShellCommandFailed(format!("Failed to run osascript: {}", e)))?;

        let result = CommandResult::from_output(output);

        if !result.success {
            if result.stderr.contains("User canceled") {
                return Err(AppError::UserCancelled("Admin authorization was cancelled".into()));
            }
            return Err(AppError::PermissionDenied(result.stderr.clone()));
        }

        Ok(result)
    }

    /// Execute a launchctl command
    pub fn launchctl(args: &[&str]) -> AppResult<CommandResult> {
        Self::execute("launchctl", args)
    }

    /// Execute a launchctl command with admin privileges
    pub fn launchctl_admin(action: &str, plist_path: &str) -> AppResult<CommandResult> {
        let _full_cmd = format!("launchctl {} {}", action, plist_path);
        Self::execute_admin("launchctl", &[action, plist_path])
    }
}

/// Parse launchctl list output
pub fn parse_launchctl_list(output: &str) -> Vec<(String, i32, Option<u32>)> {
    let mut items = Vec::new();

    for line in output.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let label = parts[0].trim_matches('"').to_string();
            let status = parts[1].parse::<i32>().unwrap_or(-1);
            let pid = if parts[2] == "-" {
                None
            } else {
                parts[2].parse::<u32>().ok()
            };
            items.push((label, status, pid));
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_launchctl_list() {
        let output = "Label                          Status          PID\n\"com.apple.Safari\"             0               1234\n\"com.apple.mail\"              -               -\n";
        let items = parse_launchctl_list(output);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].0, "com.apple.Safari");
        assert_eq!(items[0].2, Some(1234));
        assert_eq!(items[1].2, None);
    }
}