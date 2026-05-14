//! Plist parser utility

use crate::error::{AppError, AppResult};
use std::path::Path;
use tracing::debug;
use std::io::Cursor;

/// Type alias for a plist dictionary
pub type Dictionary = std::collections::HashMap<String, plist::Value>;

/// Parser for property list files
pub struct PlistParser;

impl PlistParser {
    /// Read a plist file and return as a dictionary
    pub fn read(path: &Path) -> AppResult<Dictionary> {
        debug!("Reading plist from: {:?}", path);

        if !path.exists() {
            return Err(AppError::FileNotFound(path.display().to_string()));
        }

        let data = std::fs::read(path)?;
        
        // Try parsing as dictionary directly
        match plist::from_bytes::<plist::Dictionary>(&data) {
            Ok(dict) => Ok(dict.into_iter().collect()),
            Err(_) => {
                // Try parsing as Value and converting
                if let Ok(value) = plist::Value::from_reader_xml(Cursor::new(&data)) {
                    return Self::value_to_dict(&value, path);
                }
                Err(AppError::InvalidPlist(format!("Failed to parse {}", path.display())))
            }
        }
    }
    
    /// Convert a plist Value to a dictionary
    fn value_to_dict(value: &plist::Value, path: &Path) -> AppResult<Dictionary> {
        match value {
            plist::Value::Dictionary(dict) => Ok(dict.clone().into_iter().collect()),
            _ => Err(AppError::InvalidPlist(format!(
                "Expected dictionary in {}, got different type",
                path.display()
            ))),
        }
    }

    /// Write a dictionary to a plist file
    pub fn write(path: &Path, dict: &Dictionary) -> AppResult<()> {
        debug!("Writing plist to: {:?}", path);

        let xml = Self::dict_to_xml(dict);
        std::fs::write(path, xml)?;
        Ok(())
    }
    
    /// Convert dictionary to XML plist string
    fn dict_to_xml(dict: &Dictionary) -> String {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str("\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">");
        xml.push_str("\n<plist version=\"1.0\">");
        xml.push_str("\n<dict>");
        
        for (key, value) in dict {
            xml.push_str(&format!("\n    <key>{}</key>", Self::escape_xml(key)));
            xml.push_str(&Self::value_to_xml(value));
        }
        
        xml.push_str("\n</dict>\n</plist>");
        xml
    }
    
    /// Convert a plist Value to XML
    fn value_to_xml(value: &plist::Value) -> String {
        match value {
            plist::Value::String(s) => format!("\n    <string>{}</string>", Self::escape_xml(s)),
            plist::Value::Boolean(b) => {
                if *b {
                    "\n    <true/>".to_string()
                } else {
                    "\n    <false/>".to_string()
                }
            }
            plist::Value::Integer(i) => format!("\n    <integer>{}</integer>", i),
            plist::Value::Real(r) => format!("\n    <real>{}</real>", r),
            plist::Value::Array(arr) => {
                let mut xml = "\n    <array>".to_string();
                for item in arr {
                    xml.push_str(&Self::value_to_xml(item).replace("\n    ", "\n        "));
                }
                xml.push_str("\n    </array>");
                xml
            }
            plist::Value::Dictionary(dict) => {
                let mut xml = "\n    <dict>".to_string();
                for (key, val) in dict {
                    xml.push_str(&format!("\n        <key>{}</key>", Self::escape_xml(key)));
                    xml.push_str(&Self::value_to_xml(val).replace("\n    ", "\n        "));
                }
                xml.push_str("\n    </dict>");
                xml
            }
            plist::Value::Date(date) => {
                use std::fmt::Write as FmtWrite;
                let mut s = String::new();
                let _ = write!(s, "\n    <date>{:?}</date>", date);
                s
            }
            plist::Value::Data(data) => {
                use base64::{Engine as _, engine::general_purpose::STANDARD};
                format!("\n    <data>{}</data>", STANDARD.encode(data))
            }
            plist::Value::Uid(uid) => format!("\n    <integer>{}</integer>", uid.get()),
            _ => "\n    <string></string>".to_string(),
        }
    }
    
    /// Escape special XML characters
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// Get a string value from a dictionary
    pub fn get_string(dict: &Dictionary, key: &str) -> Option<String> {
        dict.get(key).and_then(|v| v.as_string().map(|s| s.to_string()))
    }

    /// Get a boolean value from a dictionary
    pub fn get_bool(dict: &Dictionary, key: &str) -> bool {
        dict.get(key).and_then(|v| v.as_boolean()).unwrap_or(false)
    }

    /// Get an integer value from a dictionary
    pub fn get_integer(dict: &Dictionary, key: &str) -> Option<i64> {
        dict.get(key).and_then(|v| v.as_signed_integer())
    }

    /// Get an array value from a dictionary
    pub fn get_array(dict: &Dictionary, key: &str) -> Option<Vec<plist::Value>> {
        dict.get(key).and_then(|v| v.as_array().map(|a| a.to_vec()))
    }

    /// Get a path value from a dictionary (converts to PathBuf)
    pub fn get_path(dict: &Dictionary, key: &str) -> Option<std::path::PathBuf> {
        Self::get_string(dict, key).map(std::path::PathBuf::from)
    }

    /// Create a minimal launch agent plist
    pub fn create_launch_agent_plist(
        label: &str,
        program: &str,
        run_at_load: bool,
    ) -> Dictionary {
        let mut dict = Dictionary::new();
        dict.insert("Label".to_string(), plist::Value::String(label.to_string()));
        dict.insert(
            "ProgramArguments".to_string(),
            plist::Value::Array(vec![plist::Value::String(program.to_string())])
        );
        dict.insert("RunAtLoad".to_string(), plist::Value::Boolean(run_at_load));
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_plist() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(
            br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.test.agent</string>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
        )
        .unwrap();

        let dict = PlistParser::read(file.path()).unwrap();
        assert_eq!(PlistParser::get_string(&dict, "Label"), Some("com.test.agent".to_string()));
        assert!(PlistParser::get_bool(&dict, "RunAtLoad"));
    }
}