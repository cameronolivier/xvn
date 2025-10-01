use super::{ActivationError, ActivationResult, StdinUserPrompt, UserPrompt};
use crate::config::{AutoInstallMode, Config};
use crate::plugins::{PluginRegistry, VersionManagerPlugin};
use crate::shell::CommandWriter;
use crate::version_file::VersionFile;
use log::{debug, info};
use std::path::Path;
use std::sync::Arc;

/// Orchestrates the complete version activation flow
pub struct Orchestrator<'a> {
    config: &'a Config,
    registry: &'a PluginRegistry,
    command_writer: &'a mut CommandWriter,
    user_prompt: Box<dyn UserPrompt>,
}

impl<'a> Orchestrator<'a> {
    /// Creates a new orchestrator with default stdin prompt
    pub fn new(
        config: &'a Config,
        registry: &'a PluginRegistry,
        command_writer: &'a mut CommandWriter,
    ) -> Self {
        Self {
            config,
            registry,
            command_writer,
            user_prompt: Box::new(StdinUserPrompt::new()),
        }
    }

    /// Sets a custom user prompt (for testing)
    #[cfg(test)]
    pub fn with_user_prompt(mut self, prompt: Box<dyn UserPrompt>) -> Self {
        self.user_prompt = prompt;
        self
    }

    /// Main activation flow
    ///
    /// 1. Find version file
    /// 2. Try to find a plugin with this version installed
    /// 3. If found -> activate
    /// 4. If not found -> handle auto-install
    pub fn activate(&mut self, path: &Path) -> ActivationResult<()> {
        // 1. Find version file
        let version_file = match VersionFile::find(path, &self.config.version_files) {
            Ok(Some(vf)) => vf,
            Ok(None) => {
                // No version file is not an error - just do nothing
                debug!("No version file found in {}", path.display());
                return Ok(());
            }
            Err(e) => {
                return Err(ActivationError::PluginError {
                    plugin: "version_file".to_string(),
                    source: e,
                });
            }
        };

        info!("Found version file: {}", version_file.path.display());
        info!("Node.js version: {}", version_file.version);

        // 2. Try to find a plugin with this version installed
        match self
            .registry
            .find_plugin_with_version(&version_file.version)
        {
            Ok(Some(plugin)) => {
                // Version is already installed - activate it
                self.activate_existing_version(&plugin, &version_file.version)?;
            }
            Ok(None) => {
                // Version not installed - handle auto-install
                self.handle_missing_version(&version_file.version)?;
            }
            Err(e) => {
                return Err(ActivationError::PluginError {
                    plugin: "unknown".to_string(),
                    source: e,
                });
            }
        }

        Ok(())
    }

    /// Activates an already-installed version
    fn activate_existing_version(
        &mut self,
        plugin: &Arc<dyn VersionManagerPlugin>,
        version: &str,
    ) -> ActivationResult<()> {
        info!("Using plugin: {}", plugin.name());

        let cmd = plugin
            .activate_command(version)
            .map_err(|e| ActivationError::PluginError {
                plugin: plugin.name().to_string(),
                source: e,
            })?;

        info!("Activation command: {}", cmd);

        // Write command to FD:3
        self.command_writer.write_command(&cmd)?;

        // Print success message to stdout
        println!("✓ Switched to Node.js {} (via {})", version, plugin.name());

        Ok(())
    }

    /// Handles the case where the required version is not installed
    ///
    /// This will be implemented in M4.2
    fn handle_missing_version(&mut self, version: &str) -> ActivationResult<()> {
        info!("Version {} not installed", version);

        // Find first available plugin to use for installation
        let plugin = self
            .registry
            .find_available_plugin()
            .map_err(|e| ActivationError::PluginError {
                plugin: "unknown".to_string(),
                source: e,
            })?
            .ok_or(ActivationError::NoPluginsAvailable)?;

        info!("Will use plugin {} for installation", plugin.name());

        // Check auto-install mode
        match self.config.auto_install {
            AutoInstallMode::Never => {
                // Show error and exit
                Err(ActivationError::VersionNotInstalled {
                    version: version.to_string(),
                    hint: format!(
                        "To install this version:\n  {}",
                        plugin
                            .install_command(version)
                            .unwrap_or_else(|_| format!("{} install {}", plugin.name(), version))
                    ),
                })
            }
            AutoInstallMode::Always => {
                // Install without prompting
                self.install_and_activate(&plugin, version)
            }
            AutoInstallMode::Prompt => {
                // Prompt user for confirmation
                let message = format!(
                    "Node.js {} is not installed. Install it using {}?",
                    version,
                    plugin.name()
                );

                match self.user_prompt.confirm(&message) {
                    Ok(true) => {
                        // User confirmed - install
                        self.install_and_activate(&plugin, version)
                    }
                    Ok(false) => {
                        // User declined - show mismatch
                        println!("Install declined.");
                        self.show_version_mismatch(version)?;
                        Ok(())
                    }
                    Err(e) => Err(ActivationError::IoError(e)),
                }
            }
        }
    }

    /// Installs and activates a Node.js version
    ///
    /// This will be fully implemented in M4.2
    fn install_and_activate(
        &mut self,
        plugin: &Arc<dyn VersionManagerPlugin>,
        version: &str,
    ) -> ActivationResult<()> {
        // Generate install command
        let install_cmd = plugin
            .install_command(version)
            .map_err(|e| ActivationError::PluginError {
                plugin: plugin.name().to_string(),
                source: e,
            })?;

        // Generate activate command
        let activate_cmd = plugin
            .activate_command(version)
            .map_err(|e| ActivationError::PluginError {
                plugin: plugin.name().to_string(),
                source: e,
            })?;

        info!("Install command: {}", install_cmd);
        info!("Activate command: {}", activate_cmd);

        // Write both commands to FD:3 (chained with &&)
        let combined_cmd = format!("{} && {}", install_cmd, activate_cmd);
        self.command_writer.write_command(&combined_cmd)?;

        // Print message to stdout
        println!("Installing Node.js {} using {}...", version, plugin.name());

        Ok(())
    }

    /// Shows version mismatch message
    ///
    /// This will be fully implemented in M4.3
    fn show_version_mismatch(&self, required_version: &str) -> ActivationResult<()> {
        use std::process::Command;

        // Get current Node.js version
        let output = Command::new("node").arg("--version").output();

        match output {
            Ok(output) if output.status.success() => {
                let current_version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .trim_start_matches('v')
                    .to_string();

                println!();
                println!("⚠ Version mismatch:");
                println!("  Required: {}", required_version);
                println!("  Current:  {}", current_version);
                println!();
                println!("This may cause compatibility issues.");
            }
            _ => {
                // Node.js not found or command failed
                println!();
                println!("⚠ Node.js {} is required but not active.", required_version);
                println!("This may cause compatibility issues.");
            }
        }

        Ok(())
    }
}
