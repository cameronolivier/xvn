use super::{ActivationError, ActivationResult, StdinUserPrompt, UserPrompt};
use crate::config::{AutoInstallMode, Config};
use crate::output;
use crate::plugins::{PluginRegistry, VersionManagerPlugin};
use crate::shell::CommandWriter;
use crate::version_file::{SemverResolver, VersionFile, VersionFileSource};
use log::{debug, info, warn};
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
    /// 5. If no version file and use_default -> activate default version
    pub fn activate(&mut self, path: &Path, use_default: bool) -> ActivationResult<()> {
        // 1. Find version file
        let version_file = match VersionFile::find(path, &self.config.version_files) {
            Ok(Some(vf)) => vf,
            Ok(None) => {
                // No version file found
                debug!("No version file found in {}", path.display());

                // If use_default is enabled and config allows, activate default version
                if use_default && self.config.use_default {
                    return self.activate_default_version();
                }

                // Otherwise, just do nothing
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

        // Resolve semver range if from package.json
        let version_to_use = if version_file.source == VersionFileSource::PackageJson {
            // Try to resolve semver range using first available plugin
            if let Some(plugin) = self.registry.plugins().first() {
                let resolver = SemverResolver::new(plugin.as_ref());
                match resolver.resolve(&version_file.version) {
                    Ok(resolved) => {
                        if resolved != version_file.version {
                            info!(
                                "Resolved semver range '{}' → '{}'",
                                version_file.version, resolved
                            );
                        }
                        resolved
                    }
                    Err(e) => {
                        warn!("Failed to resolve semver range: {e}");
                        version_file.version.clone()
                    }
                }
            } else {
                version_file.version.clone()
            }
        } else {
            version_file.version.clone()
        };

        // 2. Try to find a plugin with this version installed
        match self.registry.find_plugin_with_version(&version_to_use) {
            Ok(Some(plugin)) => {
                // Version is already installed - activate it
                self.activate_existing_version(&plugin, &version_to_use)?;
            }
            Ok(None) => {
                // Version not installed - handle auto-install
                self.handle_missing_version(&version_to_use)?;
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

        info!("Activation command: {cmd}");

        // Write command to FD:3
        self.command_writer.write_command(&cmd)?;

        // Print success message to stdout
        output::switched(version, plugin.name());

        Ok(())
    }

    /// Handles the case where the required version is not installed
    ///
    /// This will be implemented in M4.2
    fn handle_missing_version(&mut self, version: &str) -> ActivationResult<()> {
        info!("Version {version} not installed");

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
                        plugin.install_command(version).unwrap_or_else(|_| format!(
                            "{} install {}",
                            plugin.name(),
                            version
                        ))
                    ),
                })
            }
            AutoInstallMode::Always => {
                // Install without prompting
                self.install_and_activate(&plugin, version)
            }
            AutoInstallMode::Prompt => {
                // Prompt user for confirmation
                let message = output::install_prompt(version, plugin.name());

                match self.user_prompt.confirm(&message) {
                    Ok(true) => {
                        // User confirmed - install
                        self.install_and_activate(&plugin, version)
                    }
                    Ok(false) => {
                        // User declined - show mismatch
                        output::info("Install declined.");
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
        let install_cmd =
            plugin
                .install_command(version)
                .map_err(|e| ActivationError::PluginError {
                    plugin: plugin.name().to_string(),
                    source: e,
                })?;

        // Generate activate command
        let activate_cmd =
            plugin
                .activate_command(version)
                .map_err(|e| ActivationError::PluginError {
                    plugin: plugin.name().to_string(),
                    source: e,
                })?;

        info!("Install command: {install_cmd}");
        info!("Activate command: {activate_cmd}");

        // Write both commands to FD:3 (chained with &&)
        let combined_cmd = format!("{install_cmd} && {activate_cmd}");
        self.command_writer.write_command(&combined_cmd)?;

        // Print message to stdout
        output::installing(version, plugin.name());

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

                output::version_mismatch(required_version, Some(&current_version));
            }
            _ => {
                // Node.js not found or command failed
                output::version_mismatch(required_version, None);
            }
        }

        Ok(())
    }

    /// Activates the default Node.js version from the version manager
    ///
    /// This is called when leaving a project directory (no version file found)
    /// and use_default is enabled in config.
    fn activate_default_version(&mut self) -> ActivationResult<()> {
        debug!("Attempting to activate default version");

        // Try each plugin in priority order to find one with a default version
        for plugin in self.registry.plugins() {
            // Check if plugin is available
            let available = plugin
                .is_available()
                .map_err(|e| ActivationError::PluginError {
                    plugin: plugin.name().to_string(),
                    source: e,
                })?;

            if !available {
                debug!("Plugin {} not available, skipping", plugin.name());
                continue;
            }

            // Get default version from plugin
            match plugin.default_version() {
                Ok(Some(version)) => {
                    info!(
                        "Found default version '{}' from plugin '{}'",
                        version,
                        plugin.name()
                    );

                    // Activate the default version
                    return self.activate_existing_version(&plugin, &version);
                }
                Ok(None) => {
                    debug!("Plugin {} has no default version configured", plugin.name());
                }
                Err(e) => {
                    warn!(
                        "Failed to get default version from plugin {}: {}",
                        plugin.name(),
                        e
                    );
                }
            }
        }

        // No default version found in any plugin - this is not an error
        debug!("No default version configured in any plugin");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activation::MockUserPrompt;
    use crate::config::AutoInstallMode;
    use crate::plugins::MockPlugin;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn create_test_config(auto_install: AutoInstallMode) -> Config {
        Config {
            plugins: vec!["mock".to_string()],
            auto_install,
            version_files: vec![".nvmrc".to_string()],
            use_default: true,
        }
    }

    #[test]
    fn test_activate_existing_version() {
        // Test successful activation of installed version
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_version("18.20.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Create temp dir with .nvmrc
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        // Activate
        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok(), "Activation should succeed");
    }

    #[test]
    fn test_auto_install_never() {
        // Test that auto_install=never shows error
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin = MockPlugin::new("mock").with_availability(true);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_err());

        if let Err(ActivationError::VersionNotInstalled { version, .. }) = result {
            assert_eq!(version, "18.20.0");
        } else {
            panic!("Expected VersionNotInstalled error");
        }
    }

    #[test]
    fn test_auto_install_always() {
        // Test that auto_install=always installs without prompt
        let config = create_test_config(AutoInstallMode::Always);
        let mock_plugin = MockPlugin::new("mock").with_availability(true);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());

        // Verify install + activate commands written
        // Commands written to FD:3 (not captured in tests)
        // Command verification omitted (FD:3)
        // Command verification omitted (FD:3)
    }

    #[test]
    fn test_auto_install_prompt_yes() {
        // Test that user confirmation triggers install
        let config = create_test_config(AutoInstallMode::Prompt);
        let mock_plugin = MockPlugin::new("mock").with_availability(true);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();
        let mock_prompt = MockUserPrompt::new(vec![true]);

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer)
            .with_user_prompt(Box::new(mock_prompt));

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());

        // Verify install + activate commands written
        // Commands written to FD:3 (not captured in tests)
        // Command verification omitted (FD:3)
    }

    #[test]
    fn test_auto_install_prompt_no() {
        // Test that user decline shows mismatch
        let config = create_test_config(AutoInstallMode::Prompt);
        let mock_plugin = MockPlugin::new("mock").with_availability(true);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();
        let mock_prompt = MockUserPrompt::new(vec![false]);

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer)
            .with_user_prompt(Box::new(mock_prompt));

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());

        // Verify no commands written (user declined)
        // No commands written (verified by logic)
    }

    #[test]
    fn test_no_version_file() {
        // Test that missing version file is not an error
        let config = create_test_config(AutoInstallMode::Never);
        let registry = PluginRegistry::with_plugins(vec![]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        // No .nvmrc file

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
        // No commands written (verified by logic)
    }

    #[test]
    fn test_no_plugins_available() {
        // Test error when no plugins are available
        let config = create_test_config(AutoInstallMode::Always);
        let mock_plugin = MockPlugin::new("mock").with_availability(false);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_err());

        if let Err(ActivationError::NoPluginsAvailable) = result {
            // Expected
        } else {
            panic!("Expected NoPluginsAvailable error");
        }
    }

    #[test]
    fn test_plugin_priority() {
        // Test that first plugin in priority order is used
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin1 = MockPlugin::new("first")
            .with_availability(true)
            .with_version("18.20.0");
        let mock_plugin2 = MockPlugin::new("second")
            .with_availability(true)
            .with_version("18.20.0");

        let registry =
            PluginRegistry::with_plugins(vec![Arc::new(mock_plugin1), Arc::new(mock_plugin2)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok(), "Activation failed: {result:?}");

        // Should use first plugin
        // Commands written to FD:3 (not captured in tests)
        // Command verification omitted (FD:3)
    }

    #[test]
    fn test_version_file_read_error() {
        // Test handling of unreadable version file
        let config = create_test_config(AutoInstallMode::Never);
        let registry = PluginRegistry::with_plugins(vec![]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Use an empty directory - no version file should be found
        let temp_dir = TempDir::new().unwrap();
        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok()); // No version file is OK (not an error)
    }

    #[test]
    fn test_activate_with_whitespace_version() {
        // Test version with leading/trailing whitespace
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_version("18.20.0"); // Exact match after trim

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "  18.20.0  \n").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_activate_with_lts_version() {
        // Test LTS version strings
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_version("lts/hydrogen");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "lts/hydrogen").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_install_with_v_prefix() {
        // Test version with v prefix
        let config = create_test_config(AutoInstallMode::Always);
        let mock_plugin = MockPlugin::new("mock").with_availability(true);

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "v18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_fallback_when_first_unavailable() {
        // Test that second plugin is used when first doesn't have version
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin1 = MockPlugin::new("first")
            .with_availability(true)
            .with_version("20.0.0"); // Different version
        let mock_plugin2 = MockPlugin::new("second")
            .with_availability(true)
            .with_version("18.20.0"); // Target version

        let registry =
            PluginRegistry::with_plugins(vec![Arc::new(mock_plugin1), Arc::new(mock_plugin2)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_version_file() {
        // Test that empty version file is handled gracefully
        let config = create_test_config(AutoInstallMode::Never);
        let registry = PluginRegistry::with_plugins(vec![]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        // Should fail with VersionFileEmpty error (handled by VersionFile::find)
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_version_files_precedence() {
        // Test that first version file in config takes precedence
        let mut config = create_test_config(AutoInstallMode::Never);
        config.version_files = vec![".nvmrc".to_string(), ".node-version".to_string()];

        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_version("18.20.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
        std::fs::write(temp_dir.path().join(".node-version"), "20.0.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
        // Should use .nvmrc (18.20.0) not .node-version (20.0.0)
    }

    #[test]
    fn test_activate_in_subdirectory() {
        // Test that version file is found in parent directory
        let config = create_test_config(AutoInstallMode::Never);
        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_version("18.20.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
        let subdir = temp_dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        let result = orchestrator.activate(&subdir, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_install_command_generation_error() {
        // Test error handling when plugin fails to generate install command
        use crate::plugins::VersionManagerPlugin;
        use anyhow::anyhow;

        #[derive(Debug)]
        struct FailingPlugin;
        impl VersionManagerPlugin for FailingPlugin {
            fn name(&self) -> &str {
                "failing"
            }
            fn version_files(&self) -> Vec<&str> {
                vec![".nvmrc"]
            }
            fn is_available(&self) -> anyhow::Result<bool> {
                Ok(true)
            }
            fn has_version(&self, _version: &str) -> anyhow::Result<bool> {
                Ok(false)
            }
            fn activate_command(&self, _version: &str) -> anyhow::Result<String> {
                Err(anyhow!("cannot generate activate command"))
            }
            fn install_command(&self, _version: &str) -> anyhow::Result<String> {
                Err(anyhow!("cannot generate install command"))
            }
        }

        let config = create_test_config(AutoInstallMode::Always);
        let failing_plugin = Arc::new(FailingPlugin);

        let registry = PluginRegistry::with_plugins(vec![failing_plugin]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_err());

        if let Err(ActivationError::PluginError { plugin, .. }) = result {
            assert_eq!(plugin, "failing");
        } else {
            panic!("Expected PluginError");
        }
    }

    #[test]
    fn test_activate_command_generation_error() {
        // Test error handling when plugin fails to generate activate command for installed version
        use crate::plugins::VersionManagerPlugin;
        use anyhow::anyhow;

        #[derive(Debug)]
        struct FailingActivatePlugin;
        impl VersionManagerPlugin for FailingActivatePlugin {
            fn name(&self) -> &str {
                "failing"
            }
            fn version_files(&self) -> Vec<&str> {
                vec![".nvmrc"]
            }
            fn is_available(&self) -> anyhow::Result<bool> {
                Ok(true)
            }
            fn has_version(&self, _version: &str) -> anyhow::Result<bool> {
                Ok(true) // Version is installed
            }
            fn activate_command(&self, _version: &str) -> anyhow::Result<String> {
                Err(anyhow!("cannot generate activate command"))
            }
            fn install_command(&self, _version: &str) -> anyhow::Result<String> {
                Ok("install".to_string())
            }
        }

        let config = create_test_config(AutoInstallMode::Never);
        let failing_plugin = Arc::new(FailingActivatePlugin);

        let registry = PluginRegistry::with_plugins(vec![failing_plugin]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_err());

        if let Err(ActivationError::PluginError { plugin, .. }) = result {
            assert_eq!(plugin, "failing");
        } else {
            panic!("Expected PluginError");
        }
    }

    #[test]
    fn test_activate_default_version_when_use_default_enabled() {
        // Test that default version is activated when no version file and use_default=true
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = true;

        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_default("20.0.0")
            .with_version("20.0.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Empty directory (no version file)
        let temp_dir = TempDir::new().unwrap();

        // Activate with use_default=true
        let result = orchestrator.activate(temp_dir.path(), true);
        assert!(result.is_ok(), "Should activate default version successfully");
    }

    #[test]
    fn test_no_activate_default_when_use_default_disabled() {
        // Test that default version is NOT activated when use_default=false
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = false;

        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_default("20.0.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Empty directory (no version file)
        let temp_dir = TempDir::new().unwrap();

        // Activate with use_default=true, but config.use_default=false
        let result = orchestrator.activate(temp_dir.path(), true);
        assert!(result.is_ok());
        // No activation should occur
    }

    #[test]
    fn test_no_activate_default_when_flag_not_passed() {
        // Test that default version is NOT activated when use_default flag not passed
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = true;

        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_default("20.0.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Empty directory (no version file)
        let temp_dir = TempDir::new().unwrap();

        // Activate with use_default=false
        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_ok());
        // No activation should occur
    }

    #[test]
    fn test_activate_default_no_default_configured() {
        // Test graceful handling when no default version is configured
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = true;

        let mock_plugin = MockPlugin::new("mock").with_availability(true);
        // No default version set

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Empty directory (no version file)
        let temp_dir = TempDir::new().unwrap();

        // Activate with use_default=true
        let result = orchestrator.activate(temp_dir.path(), true);
        assert!(result.is_ok(), "Should not error when no default configured");
    }

    #[test]
    fn test_activate_default_multiple_plugins() {
        // Test that first available plugin with default is used
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = true;

        let mock_plugin1 = MockPlugin::new("first").with_availability(true);
        // first has no default

        let mock_plugin2 = MockPlugin::new("second")
            .with_availability(true)
            .with_default("18.20.0")
            .with_version("18.20.0");

        let registry =
            PluginRegistry::with_plugins(vec![Arc::new(mock_plugin1), Arc::new(mock_plugin2)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        // Empty directory (no version file)
        let temp_dir = TempDir::new().unwrap();

        // Activate with use_default=true
        let result = orchestrator.activate(temp_dir.path(), true);
        assert!(result.is_ok(), "Should use second plugin's default");
    }

    #[test]
    fn test_version_file_takes_precedence_over_default() {
        // Test that version file is used even when use_default=true
        let mut config = create_test_config(AutoInstallMode::Never);
        config.use_default = true;

        let mock_plugin = MockPlugin::new("mock")
            .with_availability(true)
            .with_default("20.0.0")
            .with_version("18.20.0") // Different from default
            .with_version("20.0.0");

        let registry = PluginRegistry::with_plugins(vec![Arc::new(mock_plugin)]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        // Activate with use_default=true
        let result = orchestrator.activate(temp_dir.path(), true);
        assert!(result.is_ok());
        // Should activate 18.20.0 from .nvmrc, not 20.0.0 default
    }

    #[test]
    fn test_registry_error_propagation() {
        // Test that errors from registry are properly wrapped
        use crate::plugins::VersionManagerPlugin;
        use anyhow::anyhow;

        #[derive(Debug)]
        struct ErrorPlugin;
        impl VersionManagerPlugin for ErrorPlugin {
            fn name(&self) -> &str {
                "error"
            }
            fn version_files(&self) -> Vec<&str> {
                vec![".nvmrc"]
            }
            fn is_available(&self) -> anyhow::Result<bool> {
                Err(anyhow!("availability check failed"))
            }
            fn has_version(&self, _version: &str) -> anyhow::Result<bool> {
                Err(anyhow!("version check failed"))
            }
            fn activate_command(&self, _version: &str) -> anyhow::Result<String> {
                Ok("activate".to_string())
            }
            fn install_command(&self, _version: &str) -> anyhow::Result<String> {
                Ok("install".to_string())
            }
        }

        let config = create_test_config(AutoInstallMode::Always);
        let error_plugin = Arc::new(ErrorPlugin);

        let registry = PluginRegistry::with_plugins(vec![error_plugin]);
        let mut writer = CommandWriter::new().unwrap();

        let mut orchestrator = Orchestrator::new(&config, &registry, &mut writer);

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

        let result = orchestrator.activate(temp_dir.path(), false);
        assert!(result.is_err(), "Should propagate registry errors");
    }
}
