use crate::config::Config;
use crate::plugins::PluginRegistry;
use anyhow::Result;
use log::{debug, info, warn};
use semver::{Version, VersionReq};

/// Smart engines resolver for package.json engines.node requirements
///
/// Implements intelligent version selection logic:
/// 1. Try user's default version first
/// 2. If default satisfies requirement, use it
/// 3. If default < requirement, find latest LTS that satisfies
/// 4. Fallback to highest matching version
pub struct EnginesResolver<'a> {
    registry: &'a PluginRegistry,
    config: &'a Config,
}

impl<'a> EnginesResolver<'a> {
    /// Create new engines resolver
    pub fn new(registry: &'a PluginRegistry, config: &'a Config) -> Self {
        Self { registry, config }
    }

    /// Resolve engines requirement using smart logic
    ///
    /// # Arguments
    /// * `requirement` - Semver requirement from engines.node (e.g., ">=14.0.0")
    ///
    /// # Returns
    /// * `Ok(String)` - Resolved version to use
    /// * `Err(_)` - Failed to resolve
    pub fn resolve_smart(&self, requirement: &str) -> Result<String> {
        info!("Smart engines resolution for requirement: {requirement}");

        // 1. Get user's default version
        if let Some(default_version) = self.get_default_version()? {
            debug!("User default version: {default_version}");

            // 2. Check if default version satisfies requirement
            if self.version_satisfies(&default_version, requirement) {
                info!("Default version {default_version} satisfies {requirement}, using it");
                return Ok(default_version);
            } else {
                debug!("Default version {default_version} does not satisfy {requirement}");
            }
        } else {
            debug!("No default version available");
        }

        // 3. Find best match (prefer LTS)
        self.find_best_match(requirement)
    }

    /// Get user's default Node.js version
    ///
    /// Returns the version that would be used when no version file is present
    fn get_default_version(&self) -> Result<Option<String>> {
        // Try to get from config first
        if let Some(default) = &self.config.default_version {
            if !default.is_empty() {
                debug!("Default version from config: {default}");
                return Ok(Some(default.clone()));
            }
        }

        // Try to detect current version from first available plugin
        if let Some(plugin) = self.registry.plugins().first() {
            match plugin.current_version() {
                Ok(Some(current)) => {
                    debug!("Current version from plugin: {current}");
                    return Ok(Some(current));
                }
                Ok(None) => debug!("No current version from plugin"),
                Err(e) => warn!("Failed to get current version: {e}"),
            }
        }

        Ok(None)
    }

    /// Check if a version satisfies a semver requirement
    fn version_satisfies(&self, version: &str, requirement: &str) -> bool {
        // Parse version (handle v prefix)
        let version_str = version.trim_start_matches('v');
        let parsed_version = match Version::parse(version_str) {
            Ok(v) => v,
            Err(e) => {
                debug!("Failed to parse version '{version}': {e}");
                return false;
            }
        };

        // Parse requirement
        let version_req = match VersionReq::parse(requirement) {
            Ok(req) => req,
            Err(e) => {
                debug!("Failed to parse requirement '{requirement}': {e}");
                return false;
            }
        };

        // Check if version satisfies requirement
        let satisfies = version_req.matches(&parsed_version);
        debug!("Version {version} satisfies {requirement}: {satisfies}");
        satisfies
    }

    /// Find best matching version, preferring LTS
    fn find_best_match(&self, requirement: &str) -> Result<String> {
        debug!("Finding best match for requirement: {requirement}");

        // Get all installed versions from all plugins
        let mut all_versions = Vec::new();
        for plugin in self.registry.plugins() {
            match plugin.list_versions() {
                Ok(versions) => {
                    for version in versions {
                        all_versions.push((version.clone(), plugin.name()));
                    }
                }
                Err(e) => {
                    warn!("Failed to list versions from {}: {}", plugin.name(), e);
                }
            }
        }

        if all_versions.is_empty() {
            warn!("No installed versions found, returning original requirement");
            return Ok(requirement.to_string());
        }

        // Parse requirement
        let version_req = match VersionReq::parse(requirement) {
            Ok(req) => req,
            Err(_) => {
                debug!("Not a valid semver requirement, returning as-is: {requirement}");
                return Ok(requirement.to_string());
            }
        };

        // Filter versions that satisfy requirement
        let mut matching_versions = Vec::new();
        for (version, plugin_name) in all_versions {
            let version_str = version.trim_start_matches('v');
            if let Ok(parsed_version) = Version::parse(version_str) {
                if version_req.matches(&parsed_version) {
                    matching_versions.push((parsed_version, version, plugin_name));
                }
            }
        }

        if matching_versions.is_empty() {
            warn!("No installed versions satisfy requirement: {requirement}");
            return Ok(requirement.to_string());
        }

        // Sort by preference: LTS (even major) first, then by version (descending)
        matching_versions.sort_by(|a, b| {
            let a_is_lts = self.is_lts_version(&a.0);
            let b_is_lts = self.is_lts_version(&b.0);

            // Prefer LTS versions
            match (a_is_lts, b_is_lts) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => {
                    // Both LTS or both not LTS, sort by version (descending)
                    b.0.cmp(&a.0)
                }
            }
        });

        let best_version = &matching_versions[0];
        info!(
            "Selected best match: {} (from {}) for requirement: {}",
            best_version.1, best_version.2, requirement
        );

        Ok(best_version.1.clone())
    }

    /// Check if a version is likely an LTS version
    ///
    /// LTS versions are typically even major versions (16, 18, 20, 22, etc.)
    fn is_lts_version(&self, version: &Version) -> bool {
        // Even major versions are typically LTS
        version.major % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::plugins::mock::MockPlugin;
    use crate::plugins::VersionManagerPlugin;
    use std::sync::Arc;

    fn create_test_registry(plugins: Vec<MockPlugin>) -> PluginRegistry {
        let arc_plugins: Vec<Arc<dyn VersionManagerPlugin>> = plugins
            .into_iter()
            .map(|p| Arc::new(p) as Arc<dyn VersionManagerPlugin>)
            .collect();
        PluginRegistry::with_plugins(arc_plugins)
    }

    fn create_test_config(default_version: Option<&str>) -> Config {
        Config {
            default_version: default_version.map(|s| s.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_engines_prefers_default_when_satisfies() {
        let mock_plugin = MockPlugin {
            available_versions: vec!["18.20.0", "20.11.0", "22.0.0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            default_version: Some("20.11.0".to_string()),
            ..Default::default()
        };

        let registry = create_test_registry(vec![mock_plugin]);
        let config = create_test_config(Some("20.11.0"));
        let resolver = EnginesResolver::new(&registry, &config);

        let result = resolver.resolve_smart(">=18.0.0").unwrap();
        assert_eq!(result, "20.11.0"); // Should use default
    }

    #[test]
    fn test_engines_uses_lts_when_default_insufficient() {
        let mock_plugin = MockPlugin {
            available_versions: vec!["18.20.0", "20.11.0", "22.0.0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            default_version: Some("18.20.0".to_string()),
            ..Default::default()
        };

        let registry = create_test_registry(vec![mock_plugin]);
        let config = create_test_config(Some("18.20.0"));
        let resolver = EnginesResolver::new(&registry, &config);

        let result = resolver.resolve_smart(">=20.0.0").unwrap();
        assert_eq!(result, "22.0.0"); // Should use highest LTS >= 20.0.0
    }

    #[test]
    fn test_engines_fallback_to_highest_when_no_lts() {
        let mock_plugin = MockPlugin {
            available_versions: vec!["19.0.0", "21.0.0", "23.0.0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            default_version: Some("19.0.0".to_string()),
            ..Default::default()
        };

        let registry = create_test_registry(vec![mock_plugin]);
        let config = create_test_config(Some("19.0.0"));
        let resolver = EnginesResolver::new(&registry, &config);

        let result = resolver.resolve_smart(">=20.0.0").unwrap();
        assert_eq!(result, "23.0.0"); // Should use highest matching (no LTS available)
    }

    #[test]
    fn test_engines_no_default_version() {
        let mock_plugin = MockPlugin {
            available_versions: vec!["18.20.0", "20.11.0", "22.0.0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            default_version: None,
            ..Default::default()
        };

        let registry = create_test_registry(vec![mock_plugin]);
        let config = create_test_config(None);
        let resolver = EnginesResolver::new(&registry, &config);

        let result = resolver.resolve_smart(">=18.0.0").unwrap();
        assert_eq!(result, "22.0.0"); // Should use latest LTS
    }

    #[test]
    fn test_engines_no_matching_versions() {
        let mock_plugin = MockPlugin {
            available_versions: vec!["16.20.0", "18.20.0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            default_version: Some("18.20.0".to_string()),
            ..Default::default()
        };

        let registry = create_test_registry(vec![mock_plugin]);
        let config = create_test_config(Some("18.20.0"));
        let resolver = EnginesResolver::new(&registry, &config);

        let result = resolver.resolve_smart(">=20.0.0").unwrap();
        assert_eq!(result, ">=20.0.0"); // Should return original requirement
    }

    #[test]
    fn test_is_lts_version() {
        let registry = create_test_registry(vec![]);
        let config = create_test_config(None);
        let resolver = EnginesResolver::new(&registry, &config);

        assert!(resolver.is_lts_version(&Version::new(16, 0, 0)));
        assert!(resolver.is_lts_version(&Version::new(18, 0, 0)));
        assert!(resolver.is_lts_version(&Version::new(20, 0, 0)));
        assert!(resolver.is_lts_version(&Version::new(22, 0, 0)));

        assert!(!resolver.is_lts_version(&Version::new(17, 0, 0)));
        assert!(!resolver.is_lts_version(&Version::new(19, 0, 0)));
        assert!(!resolver.is_lts_version(&Version::new(21, 0, 0)));
    }
}
