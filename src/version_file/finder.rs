use super::PackageJson;
use anyhow::{Context, Result};
use log::{debug, trace};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a discovered version file
#[derive(Debug, Clone, PartialEq)]
pub struct VersionFile {
    /// Absolute path to the version file
    pub path: PathBuf,

    /// Node.js version string (e.g., "18.20.0", "lts/hydrogen", ">=18.0.0")
    pub version: String,

    /// Source type of version file (for logging/debugging)
    pub source: VersionFileSource,
}

/// Type of version file found
#[derive(Debug, Clone, PartialEq)]
pub enum VersionFileSource {
    /// .nvmrc file
    Nvmrc,
    /// .node-version file
    NodeVersion,
    /// package.json engines.node field
    PackageJson,
    /// .tool-versions file (asdf)
    ToolVersions,
    /// Other/unknown
    Other(String),
}

impl VersionFile {
    /// Find version file by walking up directory tree
    ///
    /// Searches for files matching `filenames` starting from `start_dir`
    /// and walking up to HOME directory. Returns the first match found.
    ///
    /// # Arguments
    /// * `start_dir` - Directory to start search from
    /// * `filenames` - List of filenames to search for (in priority order)
    ///
    /// # Returns
    /// * `Ok(Some(VersionFile))` - Version file found
    /// * `Ok(None)` - No version file found
    /// * `Err(_)` - IO error or parse error
    pub fn find(start_dir: &Path, filenames: &[String]) -> Result<Option<Self>> {
        debug!("Searching for version file in {start_dir:?}");
        debug!("Looking for: {filenames:?}");

        let home = dirs::home_dir().unwrap_or_default();
        let mut dir = start_dir.to_path_buf();

        // Ensure we have an absolute path
        if dir.is_relative() {
            dir = std::env::current_dir()?.join(&dir);
        }
        dir = dir
            .canonicalize()
            .context("failed to canonicalize start directory")?;

        loop {
            trace!("Checking directory: {dir:?}");

            // Try each filename in priority order
            for filename in filenames {
                let file_path = dir.join(filename);

                if file_path.exists() && file_path.is_file() {
                    debug!("Found version file: {file_path:?}");

                    // Special handling for package.json
                    if filename == "package.json" {
                        if let Ok(pkg) = PackageJson::parse(&file_path) {
                            if let Some(node_version) = pkg.node_version() {
                                debug!("Found Node.js version in package.json: {node_version}");
                                return Ok(Some(Self {
                                    path: file_path,
                                    version: node_version.to_string(),
                                    source: VersionFileSource::PackageJson,
                                }));
                            } else {
                                debug!("package.json has no engines.node field, skipping");
                                continue;
                            }
                        } else {
                            debug!("Failed to parse package.json, skipping");
                            continue;
                        }
                    }

                    // Parse regular version files (.nvmrc, .node-version, etc.)
                    let version = Self::parse(&file_path).with_context(|| {
                        format!("failed to parse version file: {}", file_path.display())
                    })?;

                    let source = Self::detect_source(filename);

                    return Ok(Some(Self {
                        path: file_path,
                        version,
                        source,
                    }));
                }
            }

            // Stop at home directory
            if dir == home {
                debug!("Reached HOME directory, stopping search");
                break;
            }

            // Move up one directory
            if !dir.pop() {
                debug!("Reached filesystem root, stopping search");
                break;
            }
        }

        debug!("No version file found");
        Ok(None)
    }

    /// Parse version string from file
    ///
    /// Reads the first non-empty line and trims whitespace.
    /// Supports comments (lines starting with #).
    fn parse(path: &Path) -> Result<String> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read file: {}", path.display()))?;

        // Find first non-empty, non-comment line
        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            return Ok(trimmed.to_string());
        }

        anyhow::bail!(
            "version file is empty or contains only comments: {}",
            path.display()
        )
    }

    /// Detect source type from filename
    fn detect_source(filename: &str) -> VersionFileSource {
        match filename {
            ".nvmrc" => VersionFileSource::Nvmrc,
            ".node-version" => VersionFileSource::NodeVersion,
            "package.json" => VersionFileSource::PackageJson,
            ".tool-versions" => VersionFileSource::ToolVersions,
            other => VersionFileSource::Other(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_simple_version() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".nvmrc");
        fs::write(&file_path, "18.20.0").unwrap();

        let version = VersionFile::parse(&file_path).unwrap();
        assert_eq!(version, "18.20.0");
    }

    #[test]
    fn test_parse_version_with_whitespace() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".nvmrc");
        fs::write(&file_path, "  18.20.0  \n\n").unwrap();

        let version = VersionFile::parse(&file_path).unwrap();
        assert_eq!(version, "18.20.0");
    }

    #[test]
    fn test_parse_version_with_comments() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".nvmrc");
        fs::write(&file_path, "# This is a comment\n18.20.0").unwrap();

        let version = VersionFile::parse(&file_path).unwrap();
        assert_eq!(version, "18.20.0");
    }

    #[test]
    fn test_parse_lts_version() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".nvmrc");
        fs::write(&file_path, "lts/hydrogen").unwrap();

        let version = VersionFile::parse(&file_path).unwrap();
        assert_eq!(version, "lts/hydrogen");
    }

    #[test]
    fn test_parse_empty_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".nvmrc");
        fs::write(&file_path, "").unwrap();

        let result = VersionFile::parse(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_find_version_file_in_current_dir() {
        let temp_dir = tempdir().unwrap();
        let version_file = temp_dir.path().join(".nvmrc");
        fs::write(&version_file, "18.20.0").unwrap();

        let result = VersionFile::find(temp_dir.path(), &[".nvmrc".to_string()]).unwrap();

        assert!(result.is_some());
        let vf = result.unwrap();
        assert_eq!(vf.version, "18.20.0");
        // Compare canonicalized paths to handle symlinks (e.g., /var vs /private/var on macOS)
        assert_eq!(vf.path, version_file.canonicalize().unwrap());
    }

    #[test]
    fn test_find_version_file_in_parent_dir() {
        let temp_dir = tempdir().unwrap();

        // Create version file in parent
        let version_file = temp_dir.path().join(".nvmrc");
        fs::write(&version_file, "18.20.0").unwrap();

        // Create subdirectory
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        // Search from subdirectory
        let result = VersionFile::find(&subdir, &[".nvmrc".to_string()]).unwrap();

        assert!(result.is_some());
        let vf = result.unwrap();
        assert_eq!(vf.version, "18.20.0");
    }

    #[test]
    fn test_find_no_version_file() {
        let temp_dir = tempdir().unwrap();

        let result = VersionFile::find(temp_dir.path(), &[".nvmrc".to_string()]).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_find_respects_priority_order() {
        let temp_dir = tempdir().unwrap();

        // Create both version files
        fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
        fs::write(temp_dir.path().join(".node-version"), "20.0.0").unwrap();

        // .nvmrc should be found first
        let result = VersionFile::find(
            temp_dir.path(),
            &[".nvmrc".to_string(), ".node-version".to_string()],
        )
        .unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "18.20.0");
    }

    #[test]
    fn test_find_package_json_with_engines() {
        let temp_dir = tempdir().unwrap();
        let pkg_path = temp_dir.path().join("package.json");

        fs::write(
            &pkg_path,
            r#"{
            "name": "test-app",
            "engines": {
                "node": ">=18.0.0"
            }
        }"#,
        )
        .unwrap();

        let result = VersionFile::find(temp_dir.path(), &["package.json".to_string()]).unwrap();

        assert!(result.is_some());
        let vf = result.unwrap();
        assert_eq!(vf.version, ">=18.0.0");
        assert_eq!(vf.source, VersionFileSource::PackageJson);
    }

    #[test]
    fn test_find_package_json_without_engines() {
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

        let result = VersionFile::find(temp_dir.path(), &["package.json".to_string()]).unwrap();

        // Should return None since no engines.node field
        assert!(result.is_none());
    }

    #[test]
    fn test_priority_nvmrc_over_package_json() {
        let temp_dir = tempdir().unwrap();

        fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{
            "engines": { "node": ">=20.0.0" }
        }"#,
        )
        .unwrap();

        // .nvmrc should take precedence
        let result = VersionFile::find(
            temp_dir.path(),
            &[".nvmrc".to_string(), "package.json".to_string()],
        )
        .unwrap();

        assert!(result.is_some());
        let vf = result.unwrap();
        assert_eq!(vf.version, "18.20.0");
        assert_eq!(vf.source, VersionFileSource::Nvmrc);
    }

    #[test]
    fn test_package_json_priority_over_nvmrc() {
        let temp_dir = tempdir().unwrap();

        fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{
            "engines": { "node": ">=20.0.0" }
        }"#,
        )
        .unwrap();

        // package.json first in priority list
        let result = VersionFile::find(
            temp_dir.path(),
            &["package.json".to_string(), ".nvmrc".to_string()],
        )
        .unwrap();

        assert!(result.is_some());
        let vf = result.unwrap();
        assert_eq!(vf.version, ">=20.0.0");
        assert_eq!(vf.source, VersionFileSource::PackageJson);
    }
}
