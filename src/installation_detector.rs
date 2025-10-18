//! Installation conflict detection
//!
//! Detects multiple anvs installations (npm, Homebrew, Cargo) and warns users
//! to prevent conflicts.

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum InstallMethod {
    Npm,
    Homebrew,
    Cargo,
}

impl InstallMethod {
    pub fn description(&self) -> &str {
        match self {
            Self::Npm => "npm global package (@olvrcc/anvs)",
            Self::Homebrew => "Homebrew (brew install anvs)",
            Self::Cargo => "Cargo (cargo install anvs)",
        }
    }

    pub fn uninstall_command(&self) -> &str {
        match self {
            Self::Npm => "npm uninstall -g @olvrcc/anvs",
            Self::Homebrew => "brew uninstall anvs",
            Self::Cargo => "cargo uninstall anvs",
        }
    }
}

pub struct InstallationDetector;

impl InstallationDetector {
    /// Detect all anvs installations in PATH
    pub fn detect_all() -> Vec<(InstallMethod, PathBuf)> {
        let mut installations = vec![];

        // Get all anvs binaries in PATH
        if let Ok(paths) = which::which_all("anvs") {
            for path in paths {
                let path_str = path.to_string_lossy();

                // Skip our own symlink installation
                if path_str.contains("/.anvs/bin/anvs") || path_str.contains("/.anvs/current/") {
                    continue;
                }

                // Detect installation method
                if path_str.contains("node_modules") {
                    installations.push((InstallMethod::Npm, path));
                } else if path_str.contains("/Cellar/anvs") || path_str.contains("/homebrew") {
                    installations.push((InstallMethod::Homebrew, path));
                } else if path_str.contains("/.cargo/bin") {
                    installations.push((InstallMethod::Cargo, path));
                }
            }
        }

        installations
    }

    /// Check if conflicts exist (more than one installation)
    pub fn has_conflicts() -> bool {
        Self::detect_all().len() > 1
    }

    /// Get the conflict warning file path
    fn conflict_file_path() -> anyhow::Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        Ok(home.join(".anvs").join("conflict_warning"))
    }

    /// Mark that conflicts exist (create warning file)
    pub fn mark_conflict() {
        if Self::has_conflicts() {
            if let Ok(path) = Self::conflict_file_path() {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).ok();
                }
                std::fs::write(path, "").ok();
            }
        }
    }

    /// Clear conflict warning (delete warning file)
    pub fn clear_conflict() {
        if let Ok(path) = Self::conflict_file_path() {
            std::fs::remove_file(path).ok();
        }
    }

    /// Check if conflict warning should be shown
    pub fn should_warn() -> bool {
        if let Ok(path) = Self::conflict_file_path() {
            path.exists()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_method_descriptions() {
        assert_eq!(
            InstallMethod::Npm.description(),
            "npm global package (@olvrcc/anvs)"
        );
        assert_eq!(
            InstallMethod::Homebrew.description(),
            "Homebrew (brew install anvs)"
        );
        assert_eq!(
            InstallMethod::Cargo.description(),
            "Cargo (cargo install anvs)"
        );
    }

    #[test]
    fn test_uninstall_commands() {
        assert_eq!(
            InstallMethod::Npm.uninstall_command(),
            "npm uninstall -g @olvrcc/anvs"
        );
        assert_eq!(
            InstallMethod::Homebrew.uninstall_command(),
            "brew uninstall anvs"
        );
        assert_eq!(
            InstallMethod::Cargo.uninstall_command(),
            "cargo uninstall anvs"
        );
    }
}
