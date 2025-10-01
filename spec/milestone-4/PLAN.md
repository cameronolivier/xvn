# Milestone 4: Version Activation & Auto-Install - Implementation Plan

## Overview

This milestone implements the complete activation orchestration with auto-install prompts. The `xvn activate` command is already partially implemented (basic structure exists in `src/cli.rs:71-138`), but it lacks:

1. Auto-install prompt UI with user interaction
2. Combined install + activate command generation
3. Version mismatch detection
4. Proper error handling with actionable messages
5. Idempotency checks

**Current state:** Milestone 3 is complete (shell integration, FD:3 protocol, setup command all working). The activate command currently exits with an error when a version is not installed (lines 108-117 in cli.rs), instead of prompting to install.

**Goal:** Transform the basic activation into a full-featured orchestration system that handles missing versions gracefully.

---

## Prerequisites

**Required from previous milestones:**
- ✅ Config system (`src/config/`) - loads ~/.xvnrc and .xvn.yaml
- ✅ Plugin system (`src/plugins/`) - VersionManagerPlugin trait, nvm/fnm plugins
- ✅ Plugin registry (`src/plugins/registry.rs`) - find_plugin_with_version() method
- ✅ Version file finder (`src/version_file/`) - VersionFile::find() method
- ✅ FD:3 protocol (`src/shell/fd3.rs`) - CommandWriter for writing commands
- ✅ Shell hooks (`shell/xvn.sh`) - triggers activation on directory change

**Dependencies to install:**
None - all required crates (anyhow, clap, serde) are already in Cargo.toml from Milestone 1-3.

---

## Implementation Tasks

### Task 1: Extract activation logic into dedicated module

**Objective:** Refactor the activation code from `src/cli.rs` into a new `src/activation/` module for better organization and testability.

**Implementation Steps:**

1. Create new directory structure:
   ```
   src/activation/
   ├── mod.rs           # Public API and ActivationOrchestrator
   ├── auto_install.rs  # Auto-install prompt logic
   └── mismatch.rs      # Version mismatch detection
   ```

2. Create `src/activation/mod.rs` with the main orchestrator:
   ```rust
   use anyhow::{Context, Result};
   use std::path::Path;
   use std::sync::Arc;
   use crate::config::Config;
   use crate::plugins::{PluginRegistry, VersionManagerPlugin};
   use crate::shell::CommandWriter;
   use crate::version_file::VersionFile;

   mod auto_install;
   mod mismatch;

   pub use auto_install::{AutoInstaller, AutoInstallChoice};
   pub use mismatch::VersionMismatchDetector;

   /// Orchestrates the complete activation flow
   pub struct ActivationOrchestrator {
       config: Config,
       registry: PluginRegistry,
       fd3: CommandWriter,
   }

   impl ActivationOrchestrator {
       /// Create new orchestrator with loaded config
       pub fn new(config: Config, fd3: CommandWriter) -> Self {
           let registry = PluginRegistry::new(&config.plugins);
           Self { config, registry, fd3 }
       }

       /// Execute activation for given path
       pub fn activate(&mut self, path: &Path) -> Result<ActivationResult> {
           // Implementation in next task
           unimplemented!()
       }
   }

   /// Result of activation attempt
   pub enum ActivationResult {
       /// Version was activated successfully
       Activated { plugin: String, version: String },
       /// Version was installed and activated
       Installed { plugin: String, version: String },
       /// User declined installation
       Declined { version: String },
       /// No version file found (not an error)
       NoVersionFile,
   }
   ```

3. Update `src/lib.rs` to expose the new module:
   ```rust
   pub mod activation;
   ```

4. Update `src/cli.rs` to use the new module (replace lines 71-138):
   ```rust
   Some(Commands::Activate { path }) => {
       info!("Running activate command for path: {path:?}");

       let mut fd3 = crate::shell::CommandWriter::new()?;
       let config = crate::config::Config::load()
           .context("failed to load configuration")?;

       let mut orchestrator = crate::activation::ActivationOrchestrator::new(config, fd3);

       match orchestrator.activate(&path) {
           Ok(result) => {
               // Handle result (print messages)
               result.print_message();
               Ok(())
           }
           Err(e) => {
               eprintln!("Error: {e}");
               std::process::exit(1);
           }
       }
   }
   ```

**Code Structure:**
- `src/activation/mod.rs` - 150-200 lines (main orchestration logic)
- `src/activation/auto_install.rs` - 80-100 lines (prompt UI, stdin handling)
- `src/activation/mismatch.rs` - 40-60 lines (version detection, formatting)

**Key Considerations:**
- Keep existing behavior intact during refactor
- All logic moves from cli.rs to activation module
- ActivationOrchestrator owns CommandWriter (fd3) for writing commands
- Use builder pattern if configuration gets complex

**Testing:**
- No new tests yet - just moving code
- Existing integration tests should still pass
- Verify `cargo build` succeeds

**Dependencies:**
- None (this is a refactoring task)

---

### Task 2: Implement core activation orchestration

**Objective:** Implement the `ActivationOrchestrator::activate()` method with the complete activation flow.

**Implementation Steps:**

1. Implement the activation flow in `src/activation/mod.rs`:
   ```rust
   impl ActivationOrchestrator {
       pub fn activate(&mut self, path: &Path) -> Result<ActivationResult> {
           // Step 1: Find version file
           let version_file = match VersionFile::find(path, &self.config.version_files)? {
               Some(vf) => vf,
               None => return Ok(ActivationResult::NoVersionFile),
           };

           log::info!("Found version file: {}", version_file.path.display());
           log::info!("Required version: {}", version_file.version);

           // Step 2: Find plugin with this version installed
           match self.registry.find_plugin_with_version(&version_file.version)? {
               Some(plugin) => {
                   // Version is installed - activate it
                   self.activate_installed_version(plugin, &version_file.version)
               }
               None => {
                   // Version not installed - handle auto-install
                   self.handle_missing_version(&version_file.version)
               }
           }
       }

       fn activate_installed_version(
           &mut self,
           plugin: Arc<dyn VersionManagerPlugin>,
           version: &str,
       ) -> Result<ActivationResult> {
           log::info!("Using plugin: {}", plugin.name());

           let cmd = plugin.activate_command(version)?;
           log::info!("Activation command: {}", cmd);

           self.fd3.write_command(&cmd)?;

           Ok(ActivationResult::Activated {
               plugin: plugin.name().to_string(),
               version: version.to_string(),
           })
       }

       fn handle_missing_version(&mut self, version: &str) -> Result<ActivationResult> {
           log::info!("Version {} not installed", version);

           // Find first available plugin
           let plugin = self.registry.find_available_plugin()?
               .context("no version manager plugins available")?;

           // Check auto_install mode
           match self.config.auto_install {
               AutoInstallMode::Never => {
                   // Show error and exit
                   self.print_install_error(plugin.as_ref(), version)?;
                   anyhow::bail!("version not installed");
               }
               AutoInstallMode::Always => {
                   // Install without prompting
                   self.install_and_activate(plugin, version)
               }
               AutoInstallMode::Prompt => {
                   // Prompt user
                   let auto_installer = AutoInstaller::new(plugin, version);
                   match auto_installer.prompt()? {
                       AutoInstallChoice::Yes => {
                           self.install_and_activate(plugin, version)
                       }
                       AutoInstallChoice::No => {
                           // Show mismatch message
                           VersionMismatchDetector::print_mismatch(version)?;
                           Ok(ActivationResult::Declined {
                               version: version.to_string(),
                           })
                       }
                   }
               }
           }
       }

       fn install_and_activate(
           &mut self,
           plugin: Arc<dyn VersionManagerPlugin>,
           version: &str,
       ) -> Result<ActivationResult> {
           let install_cmd = plugin.install_command(version)?;
           let activate_cmd = plugin.activate_command(version)?;

           // Combine commands with &&
           let combined = format!("{} && {}", install_cmd, activate_cmd);
           log::info!("Install + activate: {}", combined);

           self.fd3.write_command(&combined)?;

           Ok(ActivationResult::Installed {
               plugin: plugin.name().to_string(),
               version: version.to_string(),
           })
       }

       fn print_install_error(
           &self,
           plugin: &dyn VersionManagerPlugin,
           version: &str,
       ) -> Result<()> {
           let install_cmd = plugin.install_command(version)?;
           eprintln!("Version {} is not installed.", version);
           eprintln!("To install: {}", install_cmd);
           Ok(())
       }
   }
   ```

2. Implement `ActivationResult::print_message()`:
   ```rust
   impl ActivationResult {
       pub fn print_message(&self) {
           match self {
               ActivationResult::Activated { plugin, version } => {
                   println!("Activated Node.js {} using {}", version, plugin);
               }
               ActivationResult::Installed { plugin, version } => {
                   println!("Installed and activated Node.js {} using {}", version, plugin);
               }
               ActivationResult::Declined { version } => {
                   // Message already printed by VersionMismatchDetector
               }
               ActivationResult::NoVersionFile => {
                   // Silent - this is expected behavior
               }
           }
       }
   }
   ```

**Code Structure:**
- `src/activation/mod.rs` - Add ~100 lines for orchestration logic
- Clear separation: find version → check installed → handle missing → write commands

**Key Considerations:**
- **Error vs. Warning:** NoVersionFile is Ok, not Err (it's expected)
- **Command combination:** Use `&&` to chain install + activate (shell will stop if install fails)
- **Plugin selection:** Always use first available plugin for install (respects config priority)
- **Logging:** Use log::info for internal state, println! for user messages
- **FD:3 writes:** All shell commands go to fd3, user messages go to stdout/stderr

**Testing:**
- Unit test: activate_installed_version returns correct result
- Unit test: handle_missing_version with mode=never returns error
- Unit test: install_and_activate generates correct combined command
- Mock plugin and CommandWriter for testing

**Dependencies:**
- Requires Task 1 complete (module structure)
- Blocks Task 3 (auto-install UI needs this orchestrator)

---

### Task 3: Implement auto-install prompt UI

**Objective:** Create interactive prompt that asks user for confirmation before installing missing versions.

**Implementation Steps:**

1. Create `src/activation/auto_install.rs`:
   ```rust
   use anyhow::{Context, Result};
   use std::io::{self, BufRead, Write};
   use crate::plugins::VersionManagerPlugin;

   /// User's choice in response to auto-install prompt
   pub enum AutoInstallChoice {
       Yes,
       No,
   }

   /// Handles auto-install prompts and user input
   pub struct AutoInstaller<'a> {
       plugin: &'a dyn VersionManagerPlugin,
       version: &'a str,
   }

   impl<'a> AutoInstaller<'a> {
       pub fn new(plugin: &'a dyn VersionManagerPlugin, version: &'a str) -> Self {
           Self { plugin, version }
       }

       /// Prompt user for install confirmation
       pub fn prompt(&self) -> Result<AutoInstallChoice> {
           self.prompt_with_stdin(io::stdin().lock())
       }

       /// Prompt with custom stdin (for testing)
       pub fn prompt_with_stdin<R: BufRead>(&self, mut stdin: R) -> Result<AutoInstallChoice> {
           // Print prompt to stderr (so it doesn't mix with fd:3 output)
           eprint!(
               "Node.js {} is not installed. Install using {}? [Y/n] ",
               self.version,
               self.plugin.name()
           );
           io::stderr().flush()?;

           // Read from stdin
           let mut response = String::new();
           stdin.read_line(&mut response)
               .context("failed to read user input")?;

           // Parse response
           let trimmed = response.trim().to_lowercase();
           match trimmed.as_str() {
               "" | "y" | "yes" => Ok(AutoInstallChoice::Yes),
               "n" | "no" => Ok(AutoInstallChoice::No),
               _ => {
                   // Invalid input - default to No for safety
                   eprintln!("Invalid response '{}', treating as 'no'", trimmed);
                   Ok(AutoInstallChoice::No)
               }
           }
       }
   }
   ```

2. Update `src/activation/mod.rs` to use AutoInstaller:
   ```rust
   // In handle_missing_version():
   AutoInstallMode::Prompt => {
       let auto_installer = AutoInstaller::new(plugin.as_ref(), version);
       match auto_installer.prompt()? {
           AutoInstallChoice::Yes => self.install_and_activate(plugin, version),
           AutoInstallChoice::No => {
               VersionMismatchDetector::print_mismatch(version)?;
               Ok(ActivationResult::Declined { version: version.to_string() })
           }
       }
   }
   ```

**Code Structure:**
- `src/activation/auto_install.rs` - 80-100 lines
- Simple, focused module: prompt → read stdin → parse response

**Key Considerations:**
- **Prompt to stderr:** Keeps fd:3 clean for shell commands
- **Default to No:** If invalid input, safer to not install
- **Flush stderr:** Ensures prompt appears before waiting for input
- **Trim input:** Handle whitespace, case-insensitive matching
- **Blocking I/O:** This is fine - we need user input before proceeding

**Testing:**
- Unit test with mock stdin:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      use std::io::Cursor;

      #[test]
      fn test_parse_yes_responses() {
          // Test "y", "Y", "yes", "YES", "" (empty = yes)
      }

      #[test]
      fn test_parse_no_responses() {
          // Test "n", "N", "no", "NO"
      }

      #[test]
      fn test_invalid_input_defaults_to_no() {
          // Test "maybe", "quit", "123", etc.
      }
  }
  ```
- Integration test: Mock stdin with Cursor, verify prompt behavior
- Manual test: `xvn activate` in directory with .nvmrc for uninstalled version

**Dependencies:**
- Requires Task 2 complete (orchestrator calls this)
- Blocks Task 5 (tests need this implemented)

---

### Task 4: Implement version mismatch detection

**Objective:** Detect and display current vs. required Node.js version when user declines installation.

**Implementation Steps:**

1. Create `src/activation/mismatch.rs`:
   ```rust
   use anyhow::Result;
   use std::process::Command;

   /// Detects and reports version mismatches
   pub struct VersionMismatchDetector;

   impl VersionMismatchDetector {
       /// Get current Node.js version from `node --version`
       pub fn get_current_version() -> Result<Option<String>> {
           let output = Command::new("node")
               .arg("--version")
               .output();

           match output {
               Ok(out) if out.status.success() => {
                   let version = String::from_utf8_lossy(&out.stdout)
                       .trim()
                       .trim_start_matches('v')  // Remove leading 'v'
                       .to_string();
                   Ok(Some(version))
               }
               Ok(_) => Ok(None),  // Command failed (non-zero exit)
               Err(_) => Ok(None), // node not found or not in PATH
           }
       }

       /// Print version mismatch message
       pub fn print_mismatch(required: &str) -> Result<()> {
           match Self::get_current_version()? {
               Some(current) => {
                   eprintln!("Version mismatch:");
                   eprintln!("  Required: {}", required);
                   eprintln!("  Current:  {}", current);
               }
               None => {
                   eprintln!("Version mismatch:");
                   eprintln!("  Required: {}", required);
                   eprintln!("  Current:  (no Node.js in PATH)");
               }
           }
           Ok(())
       }
   }
   ```

2. Call from orchestrator when user declines install:
   ```rust
   // Already implemented in Task 2:
   AutoInstallChoice::No => {
       VersionMismatchDetector::print_mismatch(version)?;
       Ok(ActivationResult::Declined { version: version.to_string() })
   }
   ```

**Code Structure:**
- `src/activation/mismatch.rs` - 40-60 lines
- Simple utility: run `node --version`, format output

**Key Considerations:**
- **Graceful degradation:** If `node` not found, show "(no Node.js in PATH)"
- **Version normalization:** Strip leading 'v' from `node --version` output (v18.20.0 → 18.20.0)
- **No panics:** Command::new().output() returns Result, handle errors gracefully
- **Not a blocker:** If detection fails, still show required version

**Testing:**
- Unit test: `get_current_version()` with mock Command (use trait for testability)
- Unit test: `print_mismatch()` formats correctly
- Integration test: Verify message appears when declining install
- Mock `node --version` output in tests

**Dependencies:**
- Requires Task 2 complete (orchestrator calls this)
- Can be implemented in parallel with Task 3

---

### Task 5: Verify and test idempotency implementation

**Objective:** Verify that the idempotency check already implemented in shell/xvn.sh works correctly and add tests.

**Current State:** Idempotency is already implemented in shell/xvn.sh:
- Lines 54-57: Check if XVN_ACTIVE_FILE matches current version file, skip if same
- Lines 81: Update XVN_ACTIVE_FILE after successful activation
- Lines 96-99: Clear XVN_ACTIVE_FILE when leaving versioned directory

This task focuses on testing the existing implementation rather than implementing new functionality.

**Implementation Steps:**

1. Review the existing implementation in `shell/xvn.sh`:
   - Line 54-57: Idempotency check (skip if same file)
   - Line 81: Update XVN_ACTIVE_FILE after successful activation
   - Line 96-99: Clear XVN_ACTIVE_FILE when leaving directory

2. No code changes needed - implementation is complete!

3. Add integration tests to verify behavior (see Testing section below)

**Code Structure:**
- No new code needed - shell/xvn.sh already implements this
- XVN_ACTIVE_FILE environment variable already tracks active version file

**Key Considerations:**
- **Already implemented:** Lines 54-57, 81, 96-99 in xvn.sh handle all idempotency logic
- **Path comparison:** Uses full path for XVN_ACTIVE_FILE (from version file finder)
- **Clear on exit:** XVN_ACTIVE_FILE cleared when leaving versioned directories
- **Exit code check:** XVN_ACTIVE_FILE only updated if activation succeeded
- **Focus on testing:** This task is about verifying existing code works correctly

**Testing:**
- Shell integration test:
  ```bash
  cd /tmp/project-with-nvmrc
  # First activation occurs
  cd subdir
  # No activation (same .nvmrc applies)
  cd ..
  # No activation (same .nvmrc)
  cd ~
  # XVN_ACTIVE_FILE cleared
  cd /tmp/project-with-nvmrc
  # Activation occurs again
  ```
- Test with `tests/shell_integration.rs` (add new test case)

**Dependencies:**
- Milestone 3 complete (idempotency already implemented there)
- Can add tests independently of other M4 tasks

---

### Task 6: Improve error messages with actionable guidance

**Objective:** Provide clear, helpful error messages for all failure scenarios.

**Implementation Steps:**

1. Create `src/activation/errors.rs` with structured error types:
   ```rust
   use thiserror::Error;

   #[derive(Error, Debug)]
   pub enum ActivationError {
       #[error("No version manager plugins are available")]
       NoPluginsAvailable {
           hint: String,
       },

       #[error("Failed to parse version file: {path}")]
       InvalidVersionFile {
           path: String,
           reason: String,
       },

       #[error("Configuration error: {message}")]
       ConfigError {
           message: String,
       },
   }

   impl ActivationError {
       /// Get actionable hint for this error
       pub fn hint(&self) -> String {
           match self {
               Self::NoPluginsAvailable { hint } => hint.clone(),
               Self::InvalidVersionFile { reason, .. } => {
                   format!("Check the version file format. {}", reason)
               }
               Self::ConfigError { .. } => {
                   "Run 'xvn status' to check your configuration.".to_string()
               }
           }
       }

       pub fn no_plugins_available() -> Self {
           Self::NoPluginsAvailable {
               hint: "Install a version manager like nvm or fnm:\n  \
                      nvm: https://github.com/nvm-sh/nvm\n  \
                      fnm: https://github.com/Schniz/fnm".to_string(),
           }
       }
   }
   ```

2. Update orchestrator to use structured errors:
   ```rust
   // In handle_missing_version():
   let plugin = self.registry.find_available_plugin()?
       .ok_or_else(|| ActivationError::no_plugins_available())?;
   ```

3. Update CLI error handling:
   ```rust
   // In src/cli.rs:
   match orchestrator.activate(&path) {
       Ok(result) => {
           result.print_message();
           Ok(())
       }
       Err(e) => {
           if let Some(activation_err) = e.downcast_ref::<ActivationError>() {
               eprintln!("Error: {}", activation_err);
               eprintln!("\n{}", activation_err.hint());
           } else {
               eprintln!("Error: {e}");
           }
           std::process::exit(1);
       }
   }
   ```

**Code Structure:**
- `src/activation/errors.rs` - 60-80 lines
- Structured errors with hints for each error type
- Use thiserror for clean error definitions

**Key Considerations:**
- **Actionable hints:** Every error should tell user what to do next
- **Not too verbose:** Keep hints concise (1-2 lines when possible)
- **Links to docs:** Include URLs for installing version managers
- **Consistent format:** "Error: <description>\n\n<hint>"

**Testing:**
- Unit test: Each error type formats correctly
- Integration test: Trigger each error condition, verify output
- Manual test: Force errors (remove version managers, corrupt config, etc.)

**Dependencies:**
- Requires Tasks 1-4 complete (orchestrator needs to exist)
- Can be implemented in parallel with Task 5

---

### Task 7: Unit tests for activation module

**Objective:** Comprehensive unit tests for all activation logic.

**Implementation Steps:**

1. Add MockCommandWriter to `src/shell/fd3.rs`:
   ```rust
   #[cfg(test)]
   pub struct MockCommandWriter {
       commands: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
   }

   #[cfg(test)]
   impl MockCommandWriter {
       pub fn new() -> Self {
           Self {
               commands: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
           }
       }

       pub fn get_commands(&self) -> Vec<String> {
           self.commands.lock().unwrap().clone()
       }
   }

   #[cfg(test)]
   impl CommandWriter {
       pub fn mock() -> MockCommandWriter {
           MockCommandWriter::new()
       }
   }
   ```

2. Create `tests/activation_tests.rs`:
   ```rust
   use xvn::activation::{ActivationOrchestrator, ActivationResult};
   use xvn::config::{Config, AutoInstallMode};
   use xvn::plugins::MockPlugin;
   use xvn::shell::MockCommandWriter;
   use std::path::PathBuf;

   #[test]
   fn test_activate_installed_version() {
       // Setup mock plugin that has version "18.20.0"
       // Setup mock CommandWriter to capture fd:3 output
       // Run activate()
       // Assert: ActivationResult::Activated
       // Assert: fd:3 contains "nvm use 18.20.0"
   }

   #[test]
   fn test_activate_missing_version_mode_never() {
       // Setup mock plugin without version
       // Config: auto_install = never
       // Run activate()
       // Assert: Returns Err
       // Assert: Error message includes install command
   }

   #[test]
   fn test_activate_missing_version_mode_always() {
       // Setup mock plugin without version
       // Config: auto_install = always
       // Run activate()
       // Assert: ActivationResult::Installed
       // Assert: fd:3 contains "nvm install 18.20.0 && nvm use 18.20.0"
   }

   #[test]
   fn test_activate_no_version_file() {
       // Setup path with no .nvmrc
       // Run activate()
       // Assert: ActivationResult::NoVersionFile
       // Assert: No fd:3 output
       // Assert: No error
   }

   #[test]
   fn test_plugin_priority_ordering() {
       // Setup config: plugins = ["nvm", "fnm"]
       // Setup: nvm doesn't have version, fnm does
       // Run activate()
       // Assert: Uses fnm (second in priority)
   }
   ```

3. Extend mock plugin for richer testing (in src/plugins/mock.rs):
   ```rust
   impl MockPlugin {
       pub fn with_installed_versions(versions: Vec<&str>) -> Self {
           // Configure mock to return true for has_version() for these versions
           Self {
               name: "mock".to_string(),
               installed_versions: versions.iter().map(|s| s.to_string()).collect(),
           }
       }

       pub fn get_commands_written(&self) -> Vec<String> {
           // Return list of commands generated
           vec![]
       }
   }
   ```

4. Tests for auto-install prompt are already in src/activation/auto_install.rs (see Task 3)
   ```rust
   // In src/activation/auto_install.rs:
   #[cfg(test)]
   mod tests {
       use super::*;
       use std::io::Cursor;

       #[test]
       fn test_prompt_yes() {
           let input = Cursor::new(b"y\n");
           let plugin = crate::plugins::MockPlugin::new("nvm");
           let installer = AutoInstaller::new(&plugin, "18.20.0");
           let choice = installer.prompt_with_stdin(input).unwrap();
           assert!(matches!(choice, AutoInstallChoice::Yes));
       }

       #[test]
       fn test_prompt_empty_defaults_yes() {
           let input = Cursor::new(b"\n");
           let plugin = crate::plugins::MockPlugin::new("nvm");
           let installer = AutoInstaller::new(&plugin, "18.20.0");
           let choice = installer.prompt_with_stdin(input).unwrap();
           assert!(matches!(choice, AutoInstallChoice::Yes));
       }

       #[test]
       fn test_prompt_no() {
           let input = Cursor::new(b"n\n");
           let plugin = crate::plugins::MockPlugin::new("nvm");
           let installer = AutoInstaller::new(&plugin, "18.20.0");
           let choice = installer.prompt_with_stdin(input).unwrap();
           assert!(matches!(choice, AutoInstallChoice::No));
       }

       #[test]
       fn test_prompt_invalid_defaults_no() {
           let input = Cursor::new(b"maybe\n");
           let plugin = crate::plugins::MockPlugin::new("nvm");
           let installer = AutoInstaller::new(&plugin, "18.20.0");
           let choice = installer.prompt_with_stdin(input).unwrap();
           assert!(matches!(choice, AutoInstallChoice::No));
       }
   }
   ```

**Code Structure:**
- `tests/activation_tests.rs` - 200-250 lines (5-7 test functions)
- `src/activation/auto_install.rs` - Add 80-100 lines of tests
- `src/plugins/mock.rs` - Extend with 40-50 lines for richer mocking

**Key Considerations:**
- **Mock stdin:** Use `std::io::Cursor` for testing user input
- **Mock plugins:** Extend MockPlugin to configure installed versions
- **Mock fd:3:** Create MockCommandWriter to verify commands written
- **Test all paths:** Installed, missing (always/never/prompt), no file, errors
- **Deterministic tests:** No actual shell execution, no actual stdin

**Testing:**
- Run with `cargo test`
- Target: >85% line coverage for `src/activation/`
- All tests should be fast (<50ms each)

**Dependencies:**
- Requires Tasks 1-6 complete (all activation code must exist)

---

### Task 8: Integration tests for activation flow

**Objective:** End-to-end tests with real filesystem and mock plugins.

**Implementation Steps:**

1. Create `tests/integration/activation.rs`:
   ```rust
   use std::fs;
   use tempfile::TempDir;
   use xvn::activation::ActivationOrchestrator;
   use xvn::config::{Config, AutoInstallMode};

   #[test]
   fn test_end_to_end_activation() {
       // Create temp directory with .nvmrc
       let temp = TempDir::new().unwrap();
       let nvmrc = temp.path().join(".nvmrc");
       fs::write(&nvmrc, "18.20.0\n").unwrap();

       // Create config with mock plugin
       let config = Config {
           plugins: vec!["mock".to_string()],
           auto_install: AutoInstallMode::Never,
           version_files: vec![".nvmrc".to_string()],
       };

       // Run activation
       let fd3 = MockCommandWriter::new();
       let mut orchestrator = ActivationOrchestrator::new(config, fd3.clone());
       let result = orchestrator.activate(temp.path()).unwrap();

       // Assert
       match result {
           ActivationResult::Activated { version, .. } => {
               assert_eq!(version, "18.20.0");
           }
           _ => panic!("Expected Activated"),
       }

       // Verify command written to fd:3
       let commands = fd3.get_commands();
       assert_eq!(commands.len(), 1);
       assert!(commands[0].contains("18.20.0"));
   }

   #[test]
   fn test_nested_version_files() {
       // Create: /tmp/project/.nvmrc (18.0.0)
       //         /tmp/project/subdir/.nvmrc (20.0.0)
       // cd /tmp/project → activates 18.0.0
       // cd /tmp/project/subdir → activates 20.0.0
       // cd /tmp/project → activates 18.0.0 again
   }

   #[test]
   fn test_auto_install_always_mode() {
       // Config: auto_install = always
       // Mock plugin without version
       // Result: ActivationResult::Installed
       // fd:3: Contains "install && use"
   }
   ```

2. Add integration test for version file priority:
   ```rust
   #[test]
   fn test_version_file_priority() {
       // Create temp dir with both .nvmrc and .node-version
       let temp = TempDir::new().unwrap();
       fs::write(temp.path().join(".nvmrc"), "18.20.0").unwrap();
       fs::write(temp.path().join(".node-version"), "20.0.0").unwrap();

       // Config: version_files = [".nvmrc", ".node-version"]
       // Assert: Uses .nvmrc (first in priority)
   }
   ```

**Code Structure:**
- `tests/integration/activation.rs` - 150-200 lines
- Use tempfile crate for temp directories
- Use real version file finder, mock plugins

**Key Considerations:**
- **Temp directories:** Use `tempfile::TempDir` for isolated tests
- **Real file I/O:** Tests actual version file discovery and parsing
- **Mock plugins:** Don't shell out to real nvm/fnm (too slow, unreliable)
- **Cleanup:** tempfile::TempDir auto-cleans on drop
- **Test fixtures:** Helper functions to create test directories

**Testing:**
- Run with `cargo test --test integration`
- Tests should be slower than unit tests but still fast (<500ms total)

**Dependencies:**
- Requires Tasks 1-6 complete
- Add `tempfile` to dev-dependencies in Cargo.toml

---

## Integration Points

### With Config Module
- `Config::load()` provides auto_install mode and plugin priority
- Config precedence: project .xvn.yaml overrides user ~/.xvnrc

### With Plugin System
- `PluginRegistry::find_plugin_with_version()` checks all plugins in priority order
- `PluginRegistry::find_available_plugin()` returns first available plugin for install

### With Shell Integration
- Shell hook calls `xvn activate <path>` when version file changes
- Shell hook tracks `XVN_ACTIVE_FILE` to prevent redundant activations
- FD:3 protocol: Rust writes commands, shell evaluates them

### With Version File Finder
- `VersionFile::find()` walks up directory tree to find version file
- Returns absolute path to version file for idempotency tracking

---

## Testing Strategy

### Unit Tests (Fast, Isolated)
- Test each activation path independently
- Mock all external dependencies (plugins, stdin, fd:3)
- Target: >85% coverage

### Integration Tests (Medium Speed, Multi-Component)
- Test with real filesystem, mock plugins
- Test version file discovery + activation orchestration
- Test multiple version files, nested directories

### Manual Tests (Slow, End-to-End)
- Test with real nvm/fnm on developer machine
- Test prompts with actual user input
- Test in bash and zsh
- Test edge cases: corrupt version files, missing version managers

### Performance Tests
- Benchmark activation time (should be <100ms without install)
- Test with deep directory hierarchies (20+ levels)
- Test with many plugins (10+ in config)

---

## Success Criteria

✅ **All automated tests pass:**
- Unit tests: >85% coverage
- Integration tests: All scenarios covered
- No test flakiness

✅ **Functional requirements met:**
- Activates installed versions correctly
- Prompts for missing versions (mode=prompt)
- Auto-installs without prompt (mode=always)
- Shows error and exits (mode=never)
- Respects user choice (Y/n)
- Shows version mismatch when declining
- Handles all error cases gracefully
- No redundant activations (idempotency)

✅ **Code quality:**
- No clippy warnings
- Code is well-documented
- Error messages are actionable
- Module is testable and well-organized

✅ **Performance:**
- Activation <100ms (P95) without install
- No noticeable delay when cd'ing between directories

✅ **User experience:**
- Clear, concise messages
- Helpful error messages with install instructions
- Prompts are obvious and easy to understand
- Silent when no version file exists

---

## Example Usage Scenarios

### Scenario 1: First-time activation (prompt mode)
```bash
$ cd ~/project
Node.js 18.20.0 is not installed. Install using nvm? [Y/n] y
Installed and activated Node.js 18.20.0 using nvm

$ node --version
v18.20.0
```

### Scenario 2: Already installed
```bash
$ cd ~/project
Activated Node.js 18.20.0 using nvm

$ node --version
v18.20.0
```

### Scenario 3: Declining installation
```bash
$ cd ~/project
Node.js 18.20.0 is not installed. Install using nvm? [Y/n] n
Version mismatch:
  Required: 18.20.0
  Current:  20.0.0
```

### Scenario 4: Auto-install mode (always)
```bash
$ cd ~/project
Installed and activated Node.js 18.20.0 using nvm

$ node --version
v18.20.0
```

### Scenario 5: Idempotency
```bash
$ cd ~/project
Activated Node.js 18.20.0 using nvm

$ cd subdir
# No output - same version file applies

$ cd ..
# No output - still same version file

$ cd ~/other-project
Activated Node.js 20.0.0 using fnm
```

---

## Notes

- **Backward compatibility:** No compatibility with avn required (fresh implementation)
- **Shell support:** Works with bash/zsh (from Milestone 3)
- **Error handling:** Never crash the shell, always exit cleanly
- **Security:** Version strings are escaped in shell commands (via plugin methods)
- **Extensibility:** Activation orchestrator can be extended for future features (daemon mode, custom resolvers)

---

**END OF PLAN.md**
