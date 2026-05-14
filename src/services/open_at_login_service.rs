use crate::error::AppResult;
use crate::models::OpenAtLoginItem;
use crate::utils::shell::ShellExecutor;
use std::path::PathBuf;
use tracing::info;

pub struct OpenAtLoginService;

impl OpenAtLoginService {
    pub fn list() -> AppResult<Vec<OpenAtLoginItem>> {
        info!("Listing open-at-login items via System Events");

        let script = "tell application \"System Events\"\n\
            set output to \"\"\n\
            repeat with li in login items\n\
                set lname to name of li\n\
                try\n\
                    set lpath to path of li\n\
                on error\n\
                    set lpath to \"\"\n\
                end try\n\
                set lhidden to hidden of li as string\n\
                set output to output & lname & \"|||\" & lpath & \"|||\" & lhidden & \"\n\"\n\
            end repeat\n\
            return output\n\
            end tell";

        let result = ShellExecutor::execute("osascript", &["-e", script])?;
        let mut items = Vec::new();

        for line in result.stdout.lines() {
            let parts: Vec<&str> = line.splitn(3, "|||").collect();
            if parts.len() == 3 {
                let name = parts[0].trim().to_string();
                if name.is_empty() {
                    continue;
                }
                let path_str = parts[1].trim();
                let path = if path_str.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(path_str))
                };
                let hidden = parts[2].trim() == "true";
                items.push(OpenAtLoginItem { name, path, hidden });
            }
        }

        Ok(items)
    }

    pub fn remove(name: &str) -> AppResult<()> {
        info!("Removing open-at-login item: {}", name);
        let escaped = name.replace('\\', "\\\\").replace('"', "\\\"");
        let script = format!(
            "tell application \"System Events\" to delete login item \"{}\"",
            escaped
        );
        ShellExecutor::execute("osascript", &["-e", &script])?;
        Ok(())
    }
}
