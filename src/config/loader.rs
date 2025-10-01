use super::schema::Config;
use anyhow::{Context, Result};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

impl Config {
    /// Load configuration from all sources with proper precedence
    /// Precedence: project config > user config > defaults
    pub fn load() -> Result<Self> {
        debug!("Loading configuration");

        let mut config = Self::default();
        debug!("Using default config: {config:?}");

        // 1. Load user config: ~/.xvnrc
        if let Some(user_config) = Self::load_user_config()? {
            debug!("Merging user config: {user_config:?}");
            config = config.merge(user_config);
        }

        // 2. Load project config: walk up from cwd to find .xvn.yaml
        if let Some(project_config) = Self::load_project_config()? {
            debug!("Merging project config: {project_config:?}");
            config = config.merge(project_config);
        }

        // 3. Validate final configuration
        config.validate().context("invalid configuration")?;

        debug!("Final config: {config:?}");
        Ok(config)
    }

    /// Load user configuration from ~/.xvnrc
    fn load_user_config() -> Result<Option<Self>> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("cannot determine home directory"))?;

        let path = home.join(".xvnrc");

        if !path.exists() {
            debug!("User config not found at {path:?}");
            return Ok(None);
        }

        debug!("Loading user config from {path:?}");
        Self::load_from_file(&path).map(Some)
    }

    /// Load project configuration from .xvn.yaml (walk up directory tree)
    fn load_project_config() -> Result<Option<Self>> {
        let start_dir = std::env::current_dir().context("failed to get current directory")?;

        let config_path = Self::find_project_config(&start_dir)?;

        if let Some(path) = config_path {
            debug!("Loading project config from {path:?}");
            Self::load_from_file(&path).map(Some)
        } else {
            debug!("No project config found");
            Ok(None)
        }
    }

    /// Find .xvn.yaml by walking up directory tree (stop at HOME)
    fn find_project_config(start_dir: &Path) -> Result<Option<PathBuf>> {
        let home = dirs::home_dir().unwrap_or_default();
        let mut dir = start_dir.to_path_buf();

        loop {
            let config_path = dir.join(".xvn.yaml");

            if config_path.exists() && config_path.is_file() {
                return Ok(Some(config_path));
            }

            // Stop at home directory
            if dir == home {
                break;
            }

            // Move up one directory
            if !dir.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Load configuration from a YAML file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file: {}", path.display()))?;

        let config: Self = serde_yaml::from_str(&content)
            .with_context(|| format!("failed to parse config file: {}", path.display()))?;

        config
            .validate()
            .with_context(|| format!("invalid config in file: {}", path.display()))?;

        Ok(config)
    }

    /// Merge another config into self (other takes precedence)
    pub(crate) fn merge(mut self, other: Self) -> Self {
        // Only override if other has non-default values
        if !other.plugins.is_empty() {
            self.plugins = other.plugins;
        }

        // auto_install always overrides (even if set to Prompt in override config)
        // This allows project configs to explicitly set "prompt" behavior
        // even when user config has "always" or "never"
        self.auto_install = other.auto_install;

        if !other.version_files.is_empty() {
            self.version_files = other.version_files;
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AutoInstallMode;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.plugins, vec!["nvm", "fnm"]);
        assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
    }

    #[test]
    fn test_merge_configs() {
        let base = Config::default();
        let override_config = Config {
            plugins: vec!["fnm".to_string()],
            ..Config::default()
        };

        let merged = base.merge(override_config);
        assert_eq!(merged.plugins, vec!["fnm"]);
    }

    #[test]
    fn test_merge_auto_install() {
        let base = Config {
            auto_install: AutoInstallMode::Never,
            ..Config::default()
        };
        let override_config = Config {
            auto_install: AutoInstallMode::Always,
            ..Config::default()
        };

        let merged = base.merge(override_config);
        assert_eq!(merged.auto_install, AutoInstallMode::Always);
    }

    #[test]
    fn test_merge_version_files() {
        let base = Config::default();
        let override_config = Config {
            version_files: vec![".node-version".to_string(), ".nvmrc".to_string()],
            ..Config::default()
        };

        let merged = base.merge(override_config);
        assert_eq!(merged.version_files, vec![".node-version", ".nvmrc"]);
    }

    #[test]
    fn test_merge_empty_plugins_preserves_base() {
        let base = Config {
            plugins: vec!["nvm".to_string()],
            ..Config::default()
        };
        let override_config = Config {
            plugins: vec![],
            ..Config::default()
        };

        let merged = base.merge(override_config);
        // Empty plugins shouldn't override
        assert_eq!(merged.plugins, vec!["nvm"]);
    }

    #[test]
    fn test_load_from_file_valid() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("test.yaml");
        fs::write(
            &config_path,
            r#"
plugins:
  - nvm
version_files:
  - .nvmrc
auto_install: always
"#,
        )
        .unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.plugins, vec!["nvm"]);
        assert_eq!(config.auto_install, AutoInstallMode::Always);
    }

    #[test]
    fn test_load_from_file_invalid_yaml() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("test.yaml");
        fs::write(&config_path, "invalid: [unclosed").unwrap();

        let result = Config::load_from_file(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_invalid_config() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("test.yaml");
        fs::write(
            &config_path,
            r#"
plugins: []
version_files: []
"#,
        )
        .unwrap();

        let result = Config::load_from_file(&config_path);
        assert!(result.is_err()); // Should fail validation
    }

    #[test]
    fn test_find_project_config_in_current_dir() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join(".xvn.yaml");
        fs::write(&config_path, "plugins: [nvm]").unwrap();

        let result = Config::find_project_config(temp.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_find_project_config_in_parent() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join(".xvn.yaml");
        fs::write(&config_path, "plugins: [nvm]").unwrap();

        let subdir = temp.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let result = Config::find_project_config(&subdir).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_find_project_config_not_found() {
        let temp = TempDir::new().unwrap();

        let result = Config::find_project_config(temp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_project_config_stops_at_home() {
        // This test verifies the walk-up stops at home directory
        // Hard to test in isolation, but we verify the logic doesn't panic
        let temp = TempDir::new().unwrap();
        let result = Config::find_project_config(temp.path());
        assert!(result.is_ok());
    }
}
