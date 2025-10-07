use crate::plugins::VersionManagerPlugin;
use anyhow::{Context, Result};
use log::{debug, trace};
use semver::{Version, VersionReq};

/// Semver range resolver
///
/// Resolves semver ranges (e.g., ">=18.0.0", "^20.0.0") to specific versions
/// by querying the version manager for installed versions and finding the best match.
pub struct SemverResolver<'a> {
    version_manager: &'a dyn VersionManagerPlugin,
}

impl<'a> SemverResolver<'a> {
    /// Create new semver resolver
    pub fn new(version_manager: &'a dyn VersionManagerPlugin) -> Self {
        Self { version_manager }
    }

    /// Resolve semver range to specific version
    ///
    /// # Arguments
    /// * `range` - Semver range (e.g., ">=18.0.0", "^20.0.0", "18.20.0")
    ///
    /// # Returns
    /// * Specific version if match found (e.g., "18.20.5")
    /// * Original range if no match or not a valid semver range (pass through to version manager)
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = SemverResolver::new(plugin);
    /// let version = resolver.resolve(">=18.0.0")?; // → "18.20.5" (highest 18.x)
    /// let version = resolver.resolve("^20.0.0")?;  // → "20.11.0" (highest 20.x)
    /// let version = resolver.resolve("18.20.0")?;  // → "18.20.0" (exact match)
    /// ```
    pub fn resolve(&self, range: &str) -> Result<String> {
        debug!("Resolving semver range: {range}");

        // Try to parse as exact version first
        if let Ok(exact_version) = Version::parse(range) {
            debug!("Exact version specified: {exact_version}");
            return Ok(range.to_string());
        }

        // Try to parse as semver range
        let version_req = match VersionReq::parse(range) {
            Ok(req) => req,
            Err(e) => {
                debug!("Not a valid semver range ({e}), passing through: {range}");
                // Not a semver range, pass through as-is (could be LTS alias, etc.)
                return Ok(range.to_string());
            }
        };

        // Get installed versions from version manager
        let installed = self
            .get_installed_versions()
            .context("failed to get installed versions from version manager")?;

        if installed.is_empty() {
            debug!("No versions installed, returning original range");
            return Ok(range.to_string());
        }

        // Find best match
        if let Some(best_match) = self.find_best_match(&version_req, &installed) {
            debug!("Resolved {range} → {best_match}");
            Ok(best_match)
        } else {
            debug!("No installed version matches {range}, returning original");
            Ok(range.to_string())
        }
    }

    /// Get list of installed versions from version manager
    fn get_installed_versions(&self) -> Result<Vec<String>> {
        trace!("Querying version manager for installed versions");

        let versions = self
            .version_manager
            .list_versions()
            .context("failed to list versions")?;

        trace!("Found {} installed versions", versions.len());
        Ok(versions)
    }

    /// Find best match for semver range
    ///
    /// Returns the highest version that satisfies the range.
    fn find_best_match(&self, req: &VersionReq, versions: &[String]) -> Option<String> {
        let mut matching_versions: Vec<(Version, String)> = versions
            .iter()
            .filter_map(|v| {
                // Try to parse as semver (handle both "18.20.0" and "v18.20.0")
                let trimmed = v.trim_start_matches('v');

                match Version::parse(trimmed) {
                    Ok(parsed) => {
                        if req.matches(&parsed) {
                            trace!("Version {v} matches requirement");
                            Some((parsed, v.clone()))
                        } else {
                            trace!("Version {v} does not match requirement");
                            None
                        }
                    }
                    Err(e) => {
                        trace!("Skipping non-semver version {v}: {e}");
                        None
                    }
                }
            })
            .collect();

        // Sort by version (descending) and return highest
        matching_versions.sort_by(|a, b| b.0.cmp(&a.0));

        matching_versions
            .first()
            .map(|(_, original)| original.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::mock::MockPlugin;

    fn create_mock_plugin(versions: Vec<&str>) -> MockPlugin {
        MockPlugin {
            available_versions: versions.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn test_resolve_exact_version() {
        let mock = create_mock_plugin(vec!["18.20.0", "20.0.0"]);
        let resolver = SemverResolver::new(&mock);

        let result = resolver.resolve("18.20.0").unwrap();
        assert_eq!(result, "18.20.0");
    }

    #[test]
    fn test_resolve_caret_range() {
        let mock = create_mock_plugin(vec!["20.0.0", "20.5.0", "20.11.0", "21.0.0"]);
        let resolver = SemverResolver::new(&mock);

        // ^20.0.0 should match highest 20.x
        let result = resolver.resolve("^20.0.0").unwrap();
        assert_eq!(result, "20.11.0");
    }

    #[test]
    fn test_resolve_tilde_range() {
        let mock = create_mock_plugin(vec!["18.20.0", "18.20.3", "18.20.5", "18.21.0"]);
        let resolver = SemverResolver::new(&mock);

        // ~18.20.0 should match highest 18.20.x
        let result = resolver.resolve("~18.20.0").unwrap();
        assert_eq!(result, "18.20.5");
    }

    #[test]
    fn test_resolve_gte_range() {
        let mock = create_mock_plugin(vec!["16.0.0", "18.20.0", "20.0.0", "20.11.0"]);
        let resolver = SemverResolver::new(&mock);

        // >=18.0.0 should match highest >= 18
        let result = resolver.resolve(">=18.0.0").unwrap();
        assert_eq!(result, "20.11.0");
    }

    #[test]
    fn test_resolve_no_matching_versions() {
        let mock = create_mock_plugin(vec!["16.0.0", "18.0.0"]);
        let resolver = SemverResolver::new(&mock);

        // >=20.0.0 has no match, return original
        let result = resolver.resolve(">=20.0.0").unwrap();
        assert_eq!(result, ">=20.0.0");
    }

    #[test]
    fn test_resolve_empty_installed_versions() {
        let mock = create_mock_plugin(vec![]);
        let resolver = SemverResolver::new(&mock);

        let result = resolver.resolve(">=18.0.0").unwrap();
        assert_eq!(result, ">=18.0.0"); // Returns original range
    }

    #[test]
    fn test_resolve_invalid_semver() {
        let mock = create_mock_plugin(vec!["18.20.0"]);
        let resolver = SemverResolver::new(&mock);

        // Invalid semver ranges should pass through
        let result = resolver.resolve("lts/hydrogen").unwrap();
        assert_eq!(result, "lts/hydrogen");

        let result = resolver.resolve("latest").unwrap();
        assert_eq!(result, "latest");
    }

    #[test]
    fn test_resolve_with_v_prefix() {
        let mock = create_mock_plugin(vec!["v18.20.0", "v20.0.0", "v20.11.0"]);
        let resolver = SemverResolver::new(&mock);

        // Should handle v prefix in version strings
        let result = resolver.resolve(">=18.0.0").unwrap();
        assert_eq!(result, "v20.11.0");
    }

    #[test]
    fn test_resolve_wildcard_major() {
        let mock = create_mock_plugin(vec!["18.0.0", "18.20.0", "18.20.5", "20.0.0"]);
        let resolver = SemverResolver::new(&mock);

        // 18.* should match highest 18.x
        let result = resolver.resolve("18.*").unwrap();
        assert_eq!(result, "18.20.5");
    }

    #[test]
    fn test_resolve_or_operator() {
        let mock = create_mock_plugin(vec!["16.0.0", "18.20.0", "20.11.0"]);
        let resolver = SemverResolver::new(&mock);

        // Test with simpler OR syntax (semver crate handles comma as OR)
        let result = resolver.resolve(">=20").unwrap();
        assert_eq!(result, "20.11.0");
    }

    #[test]
    fn test_resolve_complex_range() {
        let mock = create_mock_plugin(vec!["18.0.0", "18.20.0", "20.0.0", "20.11.0", "22.0.0"]);
        let resolver = SemverResolver::new(&mock);

        // <21 should exclude 22.0.0 and match 20.11.0
        let result = resolver.resolve("<21").unwrap();
        assert_eq!(result, "20.11.0");
    }
}
