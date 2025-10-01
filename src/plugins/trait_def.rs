use anyhow::Result;
use std::fmt::Debug;

/// Trait that all version manager plugins must implement
///
/// This trait defines the interface for interacting with Node.js version managers
/// like nvm, fnm, n, asdf, volta, etc.
pub trait VersionManagerPlugin: Debug + Send + Sync {
    /// Returns the name of this plugin (e.g., "nvm", "fnm")
    fn name(&self) -> &str;

    /// Returns the list of version file names this plugin supports
    ///
    /// This is informational only - the actual version files searched for are
    /// determined by the user's config (`config.version_files`). This method
    /// indicates which files the plugin is *capable* of supporting.
    ///
    /// Example: nvm returns [".nvmrc"], fnm might return [".nvmrc", ".node-version"]
    ///
    /// **Note**: In Milestone 2, this method is not actively used by the activation
    /// logic. It's provided for future features (e.g., `xvn doctor` to suggest
    /// plugins based on version files found).
    fn version_files(&self) -> Vec<&str>;

    /// Checks if this version manager is available on the system
    ///
    /// This is called to determine if the plugin can be used. Should be fast
    /// and cache results when possible.
    ///
    /// # Returns
    /// - `Ok(true)` if the version manager is installed and functional
    /// - `Ok(false)` if the version manager is not available
    /// - `Err(_)` only for unexpected errors (not for "not installed")
    fn is_available(&self) -> Result<bool>;

    /// Checks if a specific Node.js version is installed by this version manager
    ///
    /// # Arguments
    /// * `version` - The version string (e.g., "18.20.0", "lts/hydrogen")
    ///
    /// # Returns
    /// - `Ok(true)` if the version is installed
    /// - `Ok(false)` if the version is not installed
    /// - `Err(_)` if unable to determine (e.g., version manager not available)
    fn has_version(&self, version: &str) -> Result<bool>;

    /// Generates the shell command to activate a specific version
    ///
    /// The returned command will be executed in the user's shell to switch
    /// Node.js versions.
    ///
    /// # Arguments
    /// * `version` - The version to activate
    ///
    /// # Returns
    /// The shell command as a string (e.g., "nvm use 18.20.0")
    ///
    /// # Security
    /// Must properly escape version strings to prevent command injection
    fn activate_command(&self, version: &str) -> Result<String>;

    /// Generates the shell command to install a specific version
    ///
    /// The returned command will be presented to the user (and potentially
    /// executed if auto_install is enabled).
    ///
    /// # Arguments
    /// * `version` - The version to install
    ///
    /// # Returns
    /// The shell command as a string (e.g., "nvm install 18.20.0")
    ///
    /// # Security
    /// Must properly escape version strings to prevent command injection
    fn install_command(&self, version: &str) -> Result<String>;

    /// Resolves a version string to a concrete version
    ///
    /// For example, resolves "lts/hydrogen" to "18.20.0" or "latest" to "21.0.0".
    ///
    /// Default implementation returns the version unchanged.
    ///
    /// # Arguments
    /// * `version` - The version string to resolve
    ///
    /// # Returns
    /// The resolved version string
    fn resolve_version(&self, version: &str) -> Result<String> {
        // Default implementation: return version as-is
        Ok(version.to_string())
    }
}
