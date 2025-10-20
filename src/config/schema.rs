use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Config {
    /// Version manager plugins in priority order
    pub plugins: Vec<String>,

    /// Auto-install behavior: "prompt", "always", "never"
    pub auto_install: AutoInstallMode,

    /// Version files to search for (in priority order)
    pub version_files: Vec<String>,

    /// Whether to switch to default version when leaving a project directory
    /// Default: true (automatically switch to default version)
    pub use_default: bool,

    /// Default Node.js version to use when no version file is present
    pub default_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AutoInstallMode {
    Prompt,
    Always,
    Never,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugins: vec!["nvm".to_string(), "fnm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
            use_default: true,
            default_version: None,
        }
    }
}

impl Config {
    /// Validate configuration values
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.plugins.is_empty() {
            anyhow::bail!("at least one plugin must be configured");
        }

        if self.version_files.is_empty() {
            anyhow::bail!("at least one version file must be configured");
        }

        Ok(())
    }
}
