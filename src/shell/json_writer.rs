use super::CommandOutput;
use anyhow::Result;
use std::io::{self, Write};

/// Writer for shell commands via JSON protocol (PowerShell)
///
/// This struct handles the JSON command protocol for Windows PowerShell,
/// which is an alternative to the Unix FD:3 protocol.
///
/// # Protocol
/// - anvs writes JSON-formatted commands to stdout wrapped in markers
/// - PowerShell script parses the JSON and executes via Invoke-Expression
/// - Format: `__ANVS_COMMANDS_START__{json}__ANVS_COMMANDS_END__`
///
/// # Safety
/// PowerShell commands must be properly escaped to prevent command injection.
pub struct JsonCommandWriter {
    commands: Vec<String>,
}

impl JsonCommandWriter {
    /// Creates a new JsonCommandWriter
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Add environment variable export command (PowerShell syntax)
    ///
    /// Generates: `$env:KEY = "value"`
    ///
    /// # Arguments
    /// * `key` - Environment variable name
    /// * `value` - Environment variable value (will be escaped)
    pub fn export_env(&mut self, key: &str, value: &str) {
        // Escape PowerShell special characters
        let escaped_value = Self::escape_powershell(value);
        self.commands
            .push(format!(r#"$env:{key} = "{escaped_value}""#));
    }

    /// Add PATH prepend command (PowerShell syntax)
    ///
    /// Generates: `$env:PATH = "path;" + $env:PATH`
    ///
    /// # Arguments
    /// * `path` - Path to prepend (will be escaped)
    pub fn prepend_path(&mut self, path: &str) {
        let escaped_path = Self::escape_powershell(path);
        self.commands
            .push(format!(r#"$env:PATH = "{escaped_path};" + $env:PATH"#));
    }

    /// Add a raw PowerShell command
    ///
    /// # Arguments
    /// * `command` - PowerShell command (caller must ensure it's safe)
    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    /// Escape PowerShell special characters
    ///
    /// Escapes: backtick, dollar sign, quotes
    fn escape_powershell(value: &str) -> String {
        value
            .replace('`', "``") // Backtick escape
            .replace('$', "`$") // Dollar sign
            .replace('"', "`\"") // Quote
    }

    /// Output JSON to stdout with markers
    ///
    /// Writes commands in format:
    /// ```text
    /// __ANVS_COMMANDS_START__
    /// {"commands":["cmd1","cmd2"]}
    /// __ANVS_COMMANDS_END__
    /// ```
    pub fn write(self) -> Result<()> {
        if self.commands.is_empty() {
            return Ok(());
        }

        let output = CommandOutput {
            commands: self.commands,
        };

        let json = serde_json::to_string(&output)?;

        println!("__ANVS_COMMANDS_START__");
        println!("{json}");
        println!("__ANVS_COMMANDS_END__");

        io::stdout().flush()?;

        Ok(())
    }
}

impl Default for JsonCommandWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_export_env() {
        let mut writer = JsonCommandWriter::new();
        writer.export_env("NODE_VERSION", "18.0.0");

        assert_eq!(writer.commands[0], r#"$env:NODE_VERSION = "18.0.0""#);
    }

    #[test]
    fn test_json_prepend_path() {
        let mut writer = JsonCommandWriter::new();
        writer.prepend_path(r"C:\nvm\v18.0.0");

        assert_eq!(
            writer.commands[0],
            r#"$env:PATH = "C:\nvm\v18.0.0;" + $env:PATH"#
        );
    }

    #[test]
    fn test_json_escaping() {
        let mut writer = JsonCommandWriter::new();
        writer.export_env("TEST", r#"value with "quotes" and $vars"#);

        // Should escape quotes and dollar signs
        assert!(writer.commands[0].contains(r#"`""#));
        assert!(writer.commands[0].contains(r#"`$"#));
    }

    #[test]
    fn test_json_escaping_backtick() {
        let mut writer = JsonCommandWriter::new();
        writer.export_env("TEST", "value with `backticks`");

        // Should escape backticks
        assert!(writer.commands[0].contains("``"));
    }

    #[test]
    fn test_multiple_commands() {
        let mut writer = JsonCommandWriter::new();
        writer.export_env("NODE_VERSION", "18.0.0");
        writer.prepend_path(r"C:\nvm\v18.0.0");

        assert_eq!(writer.commands.len(), 2);
    }

    #[test]
    fn test_empty_writer() {
        let writer = JsonCommandWriter::new();
        let result = writer.write();
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_raw_command() {
        let mut writer = JsonCommandWriter::new();
        writer.add_command("Write-Host 'Hello'".to_string());

        assert_eq!(writer.commands[0], "Write-Host 'Hello'");
    }

    #[test]
    fn test_json_serialization() {
        let output = CommandOutput {
            commands: vec!["cmd1".to_string(), "cmd2".to_string()],
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("cmd1"));
        assert!(json.contains("cmd2"));
        assert!(json.contains("commands"));
    }
}
