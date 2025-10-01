use super::{FnmPlugin, NvmPlugin, VersionManagerPlugin};
use anyhow::Result;
use log::{debug, info};
use std::sync::Arc;

/// Registry for managing version manager plugins
///
/// Responsibilities:
/// - Load built-in plugins (nvm, fnm)
/// - Maintain priority ordering based on config
/// - Find the first available plugin for a given version
/// - Cache plugin instances
#[derive(Debug)]
pub struct PluginRegistry {
    /// Ordered list of plugins (priority order from config)
    plugins: Vec<Arc<dyn VersionManagerPlugin>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    ///
    /// # Arguments
    /// * `plugin_names` - List of plugin names in priority order (from config)
    pub fn new(plugin_names: &[String]) -> Self {
        info!("Initializing plugin registry with: {:?}", plugin_names);

        let mut plugins: Vec<Arc<dyn VersionManagerPlugin>> = Vec::new();

        for name in plugin_names {
            match name.as_str() {
                "nvm" => {
                    debug!("Loading nvm plugin");
                    plugins.push(Arc::new(NvmPlugin::new()));
                }
                "fnm" => {
                    debug!("Loading fnm plugin");
                    plugins.push(Arc::new(FnmPlugin::new()));
                }
                _ => {
                    log::warn!("Unknown plugin '{}' in config (ignoring)", name);
                }
            }
        }

        if plugins.is_empty() {
            log::warn!("No valid plugins loaded! Version switching will not work.");
        }

        Self { plugins }
    }

    /// Get all registered plugins
    pub fn plugins(&self) -> &[Arc<dyn VersionManagerPlugin>] {
        &self.plugins
    }

    /// Find the first available plugin
    ///
    /// Returns the first plugin in priority order that reports is_available() = true.
    ///
    /// # Returns
    /// - `Ok(Some(plugin))` - First available plugin
    /// - `Ok(None)` - No plugins are available
    /// - `Err(_)` - Error checking availability
    pub fn find_available_plugin(&self) -> Result<Option<Arc<dyn VersionManagerPlugin>>> {
        debug!("Searching for available plugin...");

        for plugin in &self.plugins {
            match plugin.is_available() {
                Ok(true) => {
                    info!("Found available plugin: {}", plugin.name());
                    return Ok(Some(Arc::clone(plugin)));
                }
                Ok(false) => {
                    debug!("Plugin {} not available", plugin.name());
                }
                Err(e) => {
                    log::warn!("Error checking availability for {}: {}", plugin.name(), e);
                }
            }
        }

        debug!("No available plugins found");
        Ok(None)
    }

    /// Find a plugin that has the specified version installed
    ///
    /// Returns the first plugin (in priority order) that:
    /// 1. Is available on the system
    /// 2. Has the specified version installed
    ///
    /// # Arguments
    /// * `version` - The Node.js version to look for
    ///
    /// # Returns
    /// - `Ok(Some(plugin))` - First plugin with this version
    /// - `Ok(None)` - No plugin has this version installed
    /// - `Err(_)` - Error checking plugins
    pub fn find_plugin_with_version(
        &self,
        version: &str,
    ) -> Result<Option<Arc<dyn VersionManagerPlugin>>> {
        debug!("Searching for plugin with version {}...", version);

        for plugin in &self.plugins {
            // Skip if plugin not available
            if !plugin.is_available().unwrap_or(false) {
                continue;
            }

            match plugin.has_version(version) {
                Ok(true) => {
                    info!("Found plugin {} with version {}", plugin.name(), version);
                    return Ok(Some(Arc::clone(plugin)));
                }
                Ok(false) => {
                    debug!("Plugin {} does not have version {}", plugin.name(), version);
                }
                Err(e) => {
                    log::warn!(
                        "Error checking version {} on {}: {}",
                        version,
                        plugin.name(),
                        e
                    );
                }
            }
        }

        debug!("No plugin has version {}", version);
        Ok(None)
    }

    /// Get list of all available plugins
    ///
    /// Returns plugins that report is_available() = true.
    pub fn available_plugins(&self) -> Vec<Arc<dyn VersionManagerPlugin>> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.is_available().unwrap_or(false))
            .map(Arc::clone)
            .collect()
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<Arc<dyn VersionManagerPlugin>> {
        self.plugins
            .iter()
            .find(|plugin| plugin.name() == name)
            .map(Arc::clone)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        // Default to nvm, fnm priority order
        Self::new(&["nvm".to_string(), "fnm".to_string()])
    }
}

#[cfg(test)]
impl PluginRegistry {
    /// Create a registry with custom plugins (for testing)
    pub fn with_plugins(plugins: Vec<Arc<dyn VersionManagerPlugin>>) -> Self {
        Self { plugins }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loads_plugins() {
        let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);
        assert_eq!(registry.plugins().len(), 2);
    }

    #[test]
    fn test_registry_ignores_unknown_plugins() {
        let registry = PluginRegistry::new(&[
            "nvm".to_string(),
            "unknown".to_string(),
            "fnm".to_string(),
        ]);
        // Should only load nvm and fnm, ignore unknown
        assert_eq!(registry.plugins().len(), 2);
    }

    #[test]
    fn test_registry_respects_priority_order() {
        let registry = PluginRegistry::new(&["fnm".to_string(), "nvm".to_string()]);
        let plugins = registry.plugins();

        assert_eq!(plugins[0].name(), "fnm");
        assert_eq!(plugins[1].name(), "nvm");
    }

    #[test]
    fn test_get_plugin_by_name() {
        let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);

        let nvm = registry.get_plugin("nvm");
        assert!(nvm.is_some());
        assert_eq!(nvm.unwrap().name(), "nvm");

        let unknown = registry.get_plugin("unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_default_registry() {
        let registry = PluginRegistry::default();
        assert_eq!(registry.plugins().len(), 2);
        assert_eq!(registry.plugins()[0].name(), "nvm");
        assert_eq!(registry.plugins()[1].name(), "fnm");
    }
}
