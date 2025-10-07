use crate::config::{Config, AutoInstallMode};
use crate::setup::shell_detection::Shell;
use anyhow::Result;

/// Validate a configuration
pub fn validate_config(config: &Config) -> Result<()> {
    validate_plugins(&config.plugins)?;
    validate_version_files(&config.version_files)?;
    validate_auto_install(&config.auto_install)?;
    Ok(())
}

/// Validate plugin list
fn validate_plugins(plugins: &[String]) -> Result<()> {
    // It's OK to have no plugins - user might install them later
    // But warn if empty (done in prompt)

    // Validate known plugin names
    let known_plugins = ["nvm", "fnm", "n", "asdf", "volta"];
    for plugin in plugins {
        if !known_plugins.contains(&plugin.as_str()) {
            log::warn!("Unknown plugin: {plugin}");
        }
    }

    Ok(())
}

/// Validate version files list
fn validate_version_files(files: &[String]) -> Result<()> {
    if files.is_empty() {
        anyhow::bail!("At least one version file must be specified");
    }

    // Validate file names
    for file in files {
        if !file.starts_with('.') {
            log::warn!("Version file should start with '.': {file}");
        }
    }

    Ok(())
}

/// Validate auto-install mode
fn validate_auto_install(mode: &AutoInstallMode) -> Result<()> {
    // All modes are valid
    // Just log for visibility
    match mode {
        AutoInstallMode::Always => {
            log::info!("Auto-install mode: always (automatic installation)");
        }
        AutoInstallMode::Prompt => {
            log::info!("Auto-install mode: prompt (ask before installing)");
        }
        AutoInstallMode::Never => {
            log::info!("Auto-install mode: never (show error only)");
        }
    }
    Ok(())
}

/// Validate shell is supported
pub fn validate_shell(shell: &Shell) -> Result<()> {
    match shell {
        Shell::Bash | Shell::Zsh => Ok(()),
        // Add more shells in future
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_version_files_empty() {
        let result = validate_version_files(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_version_files_valid() {
        let files = vec![".nvmrc".to_string(), ".node-version".to_string()];
        assert!(validate_version_files(&files).is_ok());
    }

    #[test]
    fn test_validate_plugins() {
        let plugins = vec!["nvm".to_string(), "fnm".to_string()];
        assert!(validate_plugins(&plugins).is_ok());
    }

    #[test]
    fn test_validate_config() {
        let config = Config {
            plugins: vec!["nvm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string()],
        };
        assert!(validate_config(&config).is_ok());
    }
}
