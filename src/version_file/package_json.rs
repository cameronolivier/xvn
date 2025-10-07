use anyhow::{Context, Result};
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a package.json file
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PackageJson {
    #[serde(skip)]
    pub path: PathBuf,

    #[serde(default)]
    pub engines: Option<EnginesField>,

    // Other common fields (optional, for future use)
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub version: Option<String>,
}

/// Represents the "engines" field in package.json
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EnginesField {
    #[serde(default)]
    pub node: Option<String>,

    #[serde(default)]
    pub npm: Option<String>,
}

impl PackageJson {
    /// Parse package.json file
    ///
    /// # Arguments
    /// * `path` - Path to package.json file
    ///
    /// # Returns
    /// * `Ok(PackageJson)` - Successfully parsed
    /// * `Err(_)` - File not found or invalid JSON
    pub fn parse(path: &Path) -> Result<Self> {
        debug!("Parsing package.json: {path:?}");

        // Read file
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read package.json: {}", path.display()))?;

        // Parse JSON
        let mut pkg: PackageJson = serde_json::from_str(&content)
            .with_context(|| format!("invalid JSON in package.json: {}", path.display()))?;

        // Store the path
        pkg.path = path.to_path_buf();

        trace!("Parsed package.json: engines={:?}", pkg.engines);

        Ok(pkg)
    }

    /// Extract Node.js version requirement from engines.node
    ///
    /// # Returns
    /// * `Some(&str)` - Version requirement exists (e.g., ">=18.0.0", "18.20.0")
    /// * `None` - No engines.node field
    pub fn node_version(&self) -> Option<&str> {
        self.engines.as_ref().and_then(|e| e.node.as_deref())
    }

    /// Check if package.json has Node.js version requirement
    pub fn has_node_version(&self) -> bool {
        self.node_version().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_valid_package_json_with_engines() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "name": "test-app",
            "version": "1.0.0",
            "engines": {
                "node": ">=18.0.0",
                "npm": ">=9.0.0"
            }
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.name, Some("test-app".to_string()));
        assert_eq!(pkg.node_version(), Some(">=18.0.0"));
        assert!(pkg.has_node_version());
    }

    #[test]
    fn test_parse_package_json_without_engines() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "name": "test-app",
            "version": "1.0.0"
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.name, Some("test-app".to_string()));
        assert_eq!(pkg.node_version(), None);
        assert!(!pkg.has_node_version());
    }

    #[test]
    fn test_parse_package_json_with_engines_no_node() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "name": "test-app",
            "engines": {
                "npm": ">=9.0.0"
            }
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.node_version(), None);
        assert!(!pkg.has_node_version());
    }

    #[test]
    fn test_parse_package_json_with_exact_version() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "engines": {
                "node": "18.20.0"
            }
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.node_version(), Some("18.20.0"));
    }

    #[test]
    fn test_parse_package_json_with_caret_range() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "engines": {
                "node": "^20.0.0"
            }
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.node_version(), Some("^20.0.0"));
    }

    #[test]
    fn test_parse_package_json_with_tilde_range() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "engines": {
                "node": "~18.20.0"
            }
        }"#,
        )
        .unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.node_version(), Some("~18.20.0"));
    }

    #[test]
    fn test_parse_invalid_json() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(&pkg_path, "{ invalid json }").unwrap();

        let result = PackageJson::parse(&pkg_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid JSON"));
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("nonexistent.json");

        let result = PackageJson::parse(&pkg_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_minimal_package_json() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(&pkg_path, "{}").unwrap();

        let pkg = PackageJson::parse(&pkg_path).unwrap();
        assert_eq!(pkg.name, None);
        assert_eq!(pkg.version, None);
        assert_eq!(pkg.node_version(), None);
    }
}
