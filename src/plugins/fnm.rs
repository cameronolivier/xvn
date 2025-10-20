use crate::plugins::VersionManagerPlugin;
use anyhow::{Context, Result};
use std::process::Command;
use std::sync::{Arc, Mutex};

/// Plugin for Fast Node Manager (fnm)
///
/// fnm is a binary-based Node.js version manager. Unlike nvm, it doesn't require
/// shell sourcing and can be invoked directly as a command.
#[derive(Debug, Clone)]
pub struct FnmPlugin {
    /// Cached availability status (None = not yet checked)
    available: Arc<Mutex<Option<bool>>>,
}

impl FnmPlugin {
    /// Create a new FnmPlugin instance
    pub fn new() -> Self {
        Self {
            available: Arc::new(Mutex::new(None)),
        }
    }

    /// Run an fnm command and capture output
    fn run_fnm_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("fnm")
            .args(args)
            .output()
            .context("Failed to execute fnm command")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("fnm command failed: {stderr}")
        }
    }

    /// Parse fnm list output to check if a version is installed
    ///
    /// fnm list output format:
    /// ```text
    /// * v18.20.0 default
    ///   v20.0.0
    ///   system
    /// ```
    fn parse_fnm_list(&self, output: &str, version: &str) -> bool {
        // Normalize version (with or without 'v' prefix)
        let version_without_v = version.trim_start_matches('v');
        let version_with_v = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{version}")
        };

        for line in output.lines() {
            let line = line.trim();

            // Skip system entry
            if line == "system" {
                continue;
            }

            // Remove markers (* for active, default label, etc.)
            let version_part = line
                .trim_start_matches('*')
                .split_whitespace()
                .next()
                .unwrap_or("");

            if version_part == version_with_v
                || version_part.trim_start_matches('v') == version_without_v
            {
                return true;
            }
        }

        false
    }

    /// Escape a version string for safe shell usage
    fn escape_version(version: &str) -> String {
        shell_escape::escape(version.into()).into_owned()
    }
}

impl Default for FnmPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionManagerPlugin for FnmPlugin {
    fn name(&self) -> &str {
        "fnm"
    }

    fn version_files(&self) -> Vec<&str> {
        // fnm supports both .nvmrc and .node-version
        vec![".nvmrc", ".node-version"]
    }

    fn is_available(&self) -> Result<bool> {
        // Check cache first
        {
            let cache = self
                .available
                .lock()
                .map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
            if let Some(available) = *cache {
                return Ok(available);
            }
        }

        // Try to run fnm --version
        let available = match Command::new("fnm").arg("--version").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };

        // Cache result
        {
            let mut cache = self
                .available
                .lock()
                .map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
            *cache = Some(available);
        }

        Ok(available)
    }

    fn has_version(&self, version: &str) -> Result<bool> {
        if !self.is_available()? {
            return Ok(false);
        }

        match self.run_fnm_command(&["list"]) {
            Ok(output) => {
                let has_it = self.parse_fnm_list(&output, version);
                Ok(has_it)
            }
            Err(_) => Ok(false),
        }
    }

    fn current_version(&self) -> Result<Option<String>> {
        if !self.is_available()? {
            return Ok(None);
        }

        match self.run_fnm_command(&["current"]) {
            Ok(output) => {
                let trimmed = output.trim();
                if trimmed.is_empty() || trimmed == "none" {
                    Ok(None)
                } else {
                    Ok(Some(trimmed.to_string()))
                }
            }
            Err(_) => Ok(None),
        }
    }

    fn activate_command(&self, version: &str) -> Result<String> {
        let escaped = Self::escape_version(version);
        Ok(format!("fnm use {escaped}"))
    }

    fn install_command(&self, version: &str) -> Result<String> {
        let escaped = Self::escape_version(version);
        Ok(format!("fnm install {escaped}"))
    }

    fn resolve_version(&self, version: &str) -> Result<String> {
        // fnm doesn't have built-in alias resolution like nvm
        // Just return the version as-is
        Ok(version.to_string())
    }

    fn default_version(&self) -> Result<Option<String>> {
        // Get fnm's default version by parsing `fnm list` output
        // The default version is marked with "default" label, e.g.:
        // * v18.20.0 default
        //   v20.0.0
        match self.run_fnm_command(&["list"]) {
            Ok(output) => {
                for line in output.lines() {
                    let line = line.trim();

                    // Check if this line has the "default" marker
                    if line.contains("default") {
                        // Extract version from line like "* v18.20.0 default"
                        let version_part = line
                            .trim_start_matches('*')
                            .split_whitespace()
                            .next()
                            .unwrap_or("");

                        if !version_part.is_empty() && version_part != "system" {
                            // Remove 'v' prefix if present
                            let version = version_part.trim_start_matches('v');
                            return Ok(Some(version.to_string()));
                        }
                    }
                }
                // No default found
                Ok(None)
            }
            Err(_) => {
                // If command fails, assume no default is configured
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let plugin = FnmPlugin::new();
        assert_eq!(plugin.name(), "fnm");
    }

    #[test]
    fn test_version_files() {
        let plugin = FnmPlugin::new();
        assert_eq!(plugin.version_files(), vec![".nvmrc", ".node-version"]);
    }

    #[test]
    fn test_activate_command() {
        let plugin = FnmPlugin::new();
        let cmd = plugin.activate_command("18.20.0").unwrap();
        assert_eq!(cmd, "fnm use 18.20.0");
    }

    #[test]
    fn test_install_command() {
        let plugin = FnmPlugin::new();
        let cmd = plugin.install_command("18.20.0").unwrap();
        assert_eq!(cmd, "fnm install 18.20.0");
    }

    #[test]
    fn test_parse_fnm_list() {
        let plugin = FnmPlugin::new();

        let output = "* v18.20.0 default\n  v20.0.0\n  system";

        assert!(plugin.parse_fnm_list(output, "18.20.0"));
        assert!(plugin.parse_fnm_list(output, "v18.20.0"));
        assert!(plugin.parse_fnm_list(output, "20.0.0"));
        assert!(!plugin.parse_fnm_list(output, "16.0.0"));
    }

    #[test]
    fn test_shell_escaping_activate() {
        let plugin = FnmPlugin::new();
        let cmd = plugin.activate_command("18.20.0; rm -rf /").unwrap();
        // Verify that the version is properly quoted/escaped
        assert!(
            cmd.contains("'") || cmd.contains("\\"),
            "Command should escape/quote special characters: {cmd}"
        );
        assert!(cmd.starts_with("fnm use "));
    }

    #[test]
    fn test_shell_escaping_install() {
        let plugin = FnmPlugin::new();
        let cmd = plugin
            .install_command("18.20.0 && cat /etc/passwd")
            .unwrap();
        // Verify that the version is properly quoted/escaped
        assert!(
            cmd.contains("'") || cmd.contains("\\"),
            "Command should escape/quote special characters: {cmd}"
        );
        assert!(cmd.starts_with("fnm install "));
    }

    #[test]
    fn test_availability_caching() {
        let plugin = FnmPlugin::new();

        // First call should populate cache
        let result1 = plugin.is_available();
        // Second call should use cache
        let result2 = plugin.is_available();

        // Both should return the same result
        assert_eq!(result1.is_ok(), result2.is_ok());
        if let (Ok(r1), Ok(r2)) = (result1, result2) {
            assert_eq!(r1, r2);
        }
    }
}
