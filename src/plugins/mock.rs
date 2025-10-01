use super::VersionManagerPlugin;
use anyhow::Result;
use std::collections::HashSet;

/// Mock plugin for testing
///
/// Allows tests to control availability and installed versions without
/// requiring actual version managers.
#[derive(Debug, Clone)]
pub struct MockPlugin {
    name: String,
    available: bool,
    installed_versions: HashSet<String>,
}

impl MockPlugin {
    /// Create a new mock plugin
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            available: true,
            installed_versions: HashSet::new(),
        }
    }

    /// Set availability
    pub fn with_availability(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// Add an installed version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.installed_versions.insert(version.into());
        self
    }

    /// Add multiple installed versions
    pub fn with_versions(mut self, versions: &[&str]) -> Self {
        for version in versions {
            self.installed_versions.insert(version.to_string());
        }
        self
    }
}

impl VersionManagerPlugin for MockPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version_files(&self) -> Vec<&str> {
        vec![".nvmrc"]
    }

    fn is_available(&self) -> Result<bool> {
        Ok(self.available)
    }

    fn has_version(&self, version: &str) -> Result<bool> {
        Ok(self.installed_versions.contains(version))
    }

    fn activate_command(&self, version: &str) -> Result<String> {
        Ok(format!("{} use {}", self.name, version))
    }

    fn install_command(&self, version: &str) -> Result<String> {
        Ok(format!("{} install {}", self.name, version))
    }
}
