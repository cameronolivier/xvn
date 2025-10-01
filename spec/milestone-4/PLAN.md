# Milestone 4: Version Activation & Auto-Install - Implementation Plan

**Timeline:** Weeks 7-8
**Version:** v0.4.0-v0.5.0
**Status:** Planning

---

## Overview

Milestone 4 brings together all previous work to implement the complete version activation flow with auto-install prompts. This is where xvn becomes fully functional - it can now detect when versions are missing and intelligently prompt users to install them.

**Key Deliverables:**
- Complete activation orchestration logic
- Auto-install prompts with user confirmation (stdin)
- Version mismatch detection and messaging
- Structured error handling with actionable hints
- Comprehensive testing of activation flow

**Prerequisites:**
- Milestone 1: Config and version file detection working
- Milestone 2: Plugin system with nvm/fnm plugins
- Milestone 3: Shell integration and FD:3 protocol

---

## Prerequisites Verification

Before starting Milestone 4, verify these components exist from previous milestones:

**From Milestone 1:**
- [ ] `Config::load()` - Loads and merges config files
- [ ] `VersionFile::find()` - Searches up directory tree for version files
- [ ] `VersionFile::parse()` - Validates and parses version strings (trims whitespace)
- [ ] Version string normalization: `"v18.20.0"` → `"18.20.0"` (leading 'v' stripped)

**From Milestone 2:**
- [ ] `PluginRegistry::new()` - Creates plugin registry from config
- [ ] `PluginRegistry::find_plugin_with_version(version: &str) -> Result<Option<&dyn VersionManagerPlugin>>` - Returns first plugin that has version installed
- [ ] `PluginRegistry::find_available_plugin() -> Result<Option<&dyn VersionManagerPlugin>>` - Returns first available plugin on system
- [ ] `VersionManagerPlugin` trait with methods:
  - `name() -> &str`
  - `is_available() -> Result<bool>`
  - `has_version(version: &str) -> Result<bool>`
  - `activate_command(version: &str) -> Result<String>` - Returns `anyhow::Error` on failure
  - `install_command(version: &str) -> Result<String>` - Returns `anyhow::Error` on failure

**From Milestone 3:**
- [ ] `CommandWriter` struct with `write_command(&mut self, cmd: &str) -> std::io::Result<()>` method
- [ ] `shell/xvn.sh` with `__xvn_chpwd` hook and `XVN_ACTIVE_FILE` idempotency support
- [ ] FD:3 protocol working (commands written to FD:3 are evaluated by shell)

**Verification:**
```bash
# Build project (should succeed)
cargo build

# Run existing tests (should pass)
cargo test --lib

# Verify shell hook exists
ls -l shell/xvn.sh
```

---

## Module Structure

After Milestone 4 implementation, the codebase structure will be:

```
src/
├── main.rs                 # Binary entry point
├── lib.rs                  # Library root (add: pub mod activation;)
├── cli.rs                  # CLI argument parsing
├── config/                 # From Milestone 1
│   ├── mod.rs
│   ├── schema.rs
│   └── loader.rs
├── version_file/           # From Milestone 1
│   ├── mod.rs
│   └── finder.rs
├── plugins/                # From Milestone 2
│   ├── mod.rs
│   ├── trait_def.rs
│   ├── registry.rs
│   ├── nvm.rs
│   ├── fnm.rs
│   └── mock.rs            # For testing
├── shell/                  # From Milestone 3
│   ├── mod.rs
│   └── fd3.rs             # CommandWriter
└── activation/             # NEW in Milestone 4
    ├── mod.rs              # Public exports
    ├── orchestrator.rs     # Main activation logic
    ├── errors.rs           # Error types
    └── user_prompt.rs      # User interaction trait
```

This structure supports both binary usage (`xvn` CLI) and library usage (for testing).

---

## Implementation Tasks

### Task M4.1: Implement `xvn activate` command

**Objective:** Create a complete activation orchestrator that coordinates config loading, plugin matching, and command generation.

**Current State Analysis:**
The current `cli.rs` (lines 71-138) has a basic activation flow but lacks:
- Auto-install logic (currently shows error message only)
- User prompts for confirmation
- Version mismatch detection
- Structured error handling

**Implementation Steps:**

1. **Create activation module structure:**
   ```bash
   mkdir -p src/activation
   touch src/activation/mod.rs
   touch src/activation/orchestrator.rs
   touch src/activation/errors.rs
   touch src/activation/user_prompt.rs
   ```

2. **Define activation module in `src/activation/mod.rs`:**
   ```rust
   mod errors;
   mod orchestrator;
   mod user_prompt;

   pub use errors::{ActivationError, ActivationResult};
   pub use orchestrator::Orchestrator;
   pub use user_prompt::UserPrompt;
   ```

3. **Create `Orchestrator` struct in `src/activation/orchestrator.rs`:**
   ```rust
   use crate::config::Config;
   use crate::plugins::PluginRegistry;
   use crate::version_file::VersionFile;
   use crate::shell::CommandWriter;
   use super::{ActivationError, ActivationResult, UserPrompt};
   use std::path::Path;

   /// Orchestrates the complete version activation flow
   pub struct Orchestrator<'a> {
       config: &'a Config,
       registry: &'a PluginRegistry,
       command_writer: &'a mut CommandWriter,
       user_prompt: Box<dyn UserPrompt>,
   }

   impl<'a> Orchestrator<'a> {
       pub fn new(
           config: &'a Config,
           registry: &'a PluginRegistry,
           command_writer: &'a mut CommandWriter,
       ) -> Self {
           Self {
               config,
               registry,
               command_writer,
               user_prompt: Box::new(StdinUserPrompt::new()),
           }
       }

       /// Set a custom user prompt (for testing)
       pub fn with_user_prompt(mut self, prompt: Box<dyn UserPrompt>) -> Self {
           self.user_prompt = prompt;
           self
       }

       /// Main activation flow
       pub fn activate(&mut self, path: &Path) -> ActivationResult<()> {
           // 1. Find version file
           let version_file = match VersionFile::find(path, &self.config.version_files)? {
               Some(vf) => vf,
               None => {
                   // No version file is not an error - just do nothing
                   return Ok(());
               }
           };

           log::info!("Found version file: {}", version_file.path.display());
           log::info!("Node.js version: {}", version_file.version);

           // 2. Try to find a plugin with this version installed
           match self.registry.find_plugin_with_version(&version_file.version)? {
               Some(plugin) => {
                   // Version is already installed - activate it
                   self.activate_existing_version(plugin, &version_file.version)?;
               }
               None => {
                   // Version not installed - handle auto-install
                   self.handle_missing_version(&version_file.version)?;
               }
           }

           Ok(())
       }

       // Implementation continues in next steps...
   }
   ```

4. **Implement `activate_existing_version` method:**
   ```rust
   fn activate_existing_version(
       &mut self,
       plugin: &dyn VersionManagerPlugin,
       version: &str,
   ) -> ActivationResult<()> {
       log::info!("Using plugin: {}", plugin.name());

       let cmd = plugin.activate_command(version)
           .map_err(|e| ActivationError::PluginError {
               plugin: plugin.name().to_string(),
               source: e,
           })?;

       log::info!("Activation command: {}", cmd);

       // Write command to FD:3
       self.command_writer.write_command(&cmd)?;

       // Print success message to stdout
       println!("✓ Switched to Node.js {} (via {})", version, plugin.name());

       Ok(())
   }
   ```

5. **Update `src/cli.rs` to use the orchestrator:**
   Replace the existing `Commands::Activate` handler (lines 71-138) with:
   ```rust
   Some(Commands::Activate { path }) => {
       info!("Running activate command for path: {path:?}");

       // Load config
       let config = crate::config::Config::load()
           .context("failed to load configuration")?;

       // Create plugin registry
       let registry = crate::plugins::PluginRegistry::new(&config.plugins);

       // Open FD:3 for writing commands
       let mut fd3 = crate::shell::CommandWriter::new()?;

       // Create orchestrator
       let mut orchestrator = crate::activation::Orchestrator::new(
           &config,
           &registry,
           &mut fd3,
       );

       // Run activation
       match orchestrator.activate(&path) {
           Ok(()) => Ok(()),
           Err(e) => {
               eprintln!("{}", e);
               if let Some(hint) = e.hint() {
                   eprintln!("\n{}", hint);
               }
               std::process::exit(1);
           }
       }
   }
   ```

6. **Add orchestrator module to `src/lib.rs`:**
   ```rust
   pub mod activation;
   ```

**Code Structure:**
- File: `src/activation/mod.rs` - Module exports
- File: `src/activation/orchestrator.rs` - Main orchestration logic
- File: `src/activation/errors.rs` - Error types (see M4.7)
- File: `src/activation/user_prompt.rs` - User prompt abstraction (see M4.2)
- File: `src/cli.rs` - Updated to use orchestrator

**Key Considerations:**
- Use dependency injection for testability (UserPrompt trait)
- Keep orchestrator stateless - pass dependencies through constructor
- Clear separation: orchestrator coordinates, doesn't implement details
- Write user messages to stdout, commands to FD:3, errors to stderr

**Testing:**
- Unit test: orchestrator with mock plugins and mock user prompt
- Integration test: full activation flow with real plugins (if nvm/fnm available)
- Test idempotency: activating same version twice should be fast

**Dependencies:**
- Requires: M4.7 (error types), M4.2 (user prompt)
- Enables: All other M4 tasks

---

### Task M4.2: Implement auto-install logic

**Objective:** Add logic to prompt users to install missing Node.js versions and execute install commands.

**Implementation Steps:**

1. **Create user prompt trait in `src/activation/user_prompt.rs`:**
   ```rust
   use std::io::{self, Write};

   /// Abstraction for user prompts (allows testing without stdin)
   pub trait UserPrompt {
       /// Ask user a yes/no question
       /// Returns: true if user confirms, false if user declines
       fn confirm(&mut self, message: &str) -> io::Result<bool>;
   }

   /// Production implementation that reads from stdin
   pub struct StdinUserPrompt {
       stdin: io::Stdin,
       stdout: io::Stdout,
   }

   impl StdinUserPrompt {
       pub fn new() -> Self {
           Self {
               stdin: io::stdin(),
               stdout: io::stdout(),
           }
       }
   }

   impl UserPrompt for StdinUserPrompt {
       fn confirm(&mut self, message: &str) -> io::Result<bool> {
           // Print prompt
           write!(self.stdout, "{} [Y/n]: ", message)?;
           self.stdout.flush()?;

           // Read response
           let mut response = String::new();
           self.stdin.read_line(&mut response)?;

           // Parse response (default to "yes" if just Enter pressed)
           let response = response.trim().to_lowercase();
           Ok(response.is_empty() || response.starts_with('y'))
       }
   }

   /// Mock implementation for testing
   #[cfg(test)]
   pub struct MockUserPrompt {
       pub responses: Vec<bool>,
       pub prompts_received: Vec<String>,
   }

   #[cfg(test)]
   impl MockUserPrompt {
       pub fn new(responses: Vec<bool>) -> Self {
           Self {
               responses,
               prompts_received: Vec::new(),
           }
       }
   }

   #[cfg(test)]
   impl UserPrompt for MockUserPrompt {
       fn confirm(&mut self, message: &str) -> io::Result<bool> {
           self.prompts_received.push(message.to_string());
           Ok(self.responses.pop().unwrap_or(false))
       }
   }
   ```

2. **Implement `handle_missing_version` in orchestrator:**
   ```rust
   fn handle_missing_version(&mut self, version: &str) -> ActivationResult<()> {
       use crate::config::AutoInstallMode;

       log::info!("Version {} not installed", version);

       // Find first available plugin to use for installation
       let plugin = self.registry.find_available_plugin()?
           .ok_or_else(|| ActivationError::NoPluginsAvailable)?;

       log::info!("Will use plugin {} for installation", plugin.name());

       // Check auto-install mode
       match self.config.auto_install {
           AutoInstallMode::Never => {
               // Show error and exit
               return Err(ActivationError::VersionNotInstalled {
                   version: version.to_string(),
                   hint: format!(
                       "To install this version:\n  {}",
                       plugin.install_command(version)?
                   ),
               });
           }
           AutoInstallMode::Always => {
               // Install without prompting
               self.install_and_activate(plugin, version)?;
           }
           AutoInstallMode::Prompt => {
               // Prompt user for confirmation
               let message = format!(
                   "Node.js {} is not installed. Install it using {}?",
                   version,
                   plugin.name()
               );

               match self.user_prompt.confirm(&message) {
                   Ok(true) => {
                       // User confirmed - install
                       self.install_and_activate(plugin, version)?;
                   }
                   Ok(false) => {
                       // User declined - show mismatch
                       println!("Install declined.");
                       self.show_version_mismatch(version)?;
                       return Ok(());
                   }
                   Err(e) => {
                       return Err(ActivationError::IoError(e));
                   }
               }
           }
       }

       Ok(())
   }
   ```

3. **Implement `install_and_activate` method:**
   ```rust
   fn install_and_activate(
       &mut self,
       plugin: &dyn VersionManagerPlugin,
       version: &str,
   ) -> ActivationResult<()> {
       // Generate install command
       let install_cmd = plugin.install_command(version)
           .map_err(|e| ActivationError::PluginError {
               plugin: plugin.name().to_string(),
               source: e,
           })?;

       // Generate activate command
       let activate_cmd = plugin.activate_command(version)
           .map_err(|e| ActivationError::PluginError {
               plugin: plugin.name().to_string(),
               source: e,
           })?;

       log::info!("Install command: {}", install_cmd);
       log::info!("Activate command: {}", activate_cmd);

       // Write both commands to FD:3 (chained with &&)
       let combined_cmd = format!("{} && {}", install_cmd, activate_cmd);
       self.command_writer.write_command(&combined_cmd)?;

       // Print message to stdout
       println!("Installing Node.js {} using {}...", version, plugin.name());

       Ok(())
   }
   ```

**Code Structure:**
- File: `src/activation/user_prompt.rs` - User prompt trait and implementations
- File: `src/activation/orchestrator.rs` - Add methods: `handle_missing_version`, `install_and_activate`

**Key Considerations:**
- Default to "yes" if user just presses Enter (better UX)
- Chain install and activate commands with `&&` to ensure install succeeds first
- Respect `auto_install` config setting (prompt/always/never)
- UserPrompt trait allows testing without real stdin

**Testing:**
- Unit test with MockUserPrompt: confirm = true
- Unit test with MockUserPrompt: confirm = false
- Unit test: AutoInstallMode::Always (no prompt)
- Unit test: AutoInstallMode::Never (shows error)
- Integration test: Full flow with mock stdin

**Dependencies:**
- Requires: M4.1 (orchestrator), M4.7 (errors)
- Enables: M4.3 (version mismatch), M4.5 (tests)

---

### Task M4.3: Implement version mismatch detection

**Objective:** Show users the current vs. required Node.js version when they decline installation.

**Implementation Steps:**

1. **Add `show_version_mismatch` method to orchestrator:**
   ```rust
   fn show_version_mismatch(&self, required_version: &str) -> ActivationResult<()> {
       use std::process::Command;

       // Get current Node.js version
       let output = Command::new("node")
           .arg("--version")
           .output();

       match output {
           Ok(output) if output.status.success() => {
               let current_version = String::from_utf8_lossy(&output.stdout)
                   .trim()
                   .trim_start_matches('v')
                   .to_string();

               println!();
               println!("⚠ Version mismatch:");
               println!("  Required: {}", required_version);
               println!("  Current:  {}", current_version);
               println!();
               println!("This may cause compatibility issues.");
           }
           _ => {
               // Node.js not found or command failed
               println!();
               println!("⚠ Node.js {} is required but not active.", required_version);
               println!("This may cause compatibility issues.");
           }
       }

       Ok(())
   }
   ```

2. **Add helper function for version comparison (future enhancement):**
   ```rust
   // Optional: Add in future milestone for semantic version comparison
   // For now, just show the raw versions
   fn versions_match(current: &str, required: &str) -> bool {
       // Simple exact match for MVP
       current == required
   }
   ```

**Code Structure:**
- File: `src/activation/orchestrator.rs` - Add `show_version_mismatch` method

**Key Considerations:**
- Use `node --version` to get current version (works with all version managers)
- Strip leading "v" from node output (e.g., "v18.20.0" → "18.20.0")
- Handle case where node command fails (not installed or not in PATH)
- Use warning symbol (⚠) to indicate this is not an error
- Future enhancement: semantic version comparison (major.minor.patch)

**Testing:**
- Unit test: Mock `node --version` command
- Test with node available: shows both versions
- Test with node unavailable: shows required version only
- Integration test: Full flow with install declined

**Dependencies:**
- Requires: M4.2 (called when user declines install)
- Enables: M4.5 (tests)

---

### Task M4.4: Verify and test idempotency implementation

**Objective:** Ensure that shell hooks correctly prevent redundant activations for the same version file.

**Current State Analysis:**
The shell hook (`shell/xvn.sh`) already implements idempotency:
- Lines 54-58: Check if `XVN_ACTIVE_FILE` matches current version file
- Lines 96-99: Clear `XVN_ACTIVE_FILE` when leaving directory
- Line 82: Set `XVN_ACTIVE_FILE` after successful activation

**Implementation Steps:**

1. **Review existing shell hook logic:**
   - Verify idempotency check in `__xvn_activate` (line 55-57)
   - Verify clearing logic in `__xvn_chpwd` (line 97-99)
   - Verify setting logic after eval (line 82)

2. **Add comments to clarify idempotency design:**
   Update `shell/xvn.sh` line 54-58:
   ```bash
   # Check if already activated for this file (idempotency)
   # This prevents re-activation when:
   # - User runs 'cd .' in same directory
   # - User cd's into subdirectory of same project
   # - Shell re-runs hook on prompt refresh
   if [[ "${XVN_ACTIVE_FILE:-}" == "$version_file" ]]; then
       __xvn_debug "Already activated for $version_file, skipping"
       return 0
   fi
   ```

3. **Create integration test for idempotency:**
   Create `tests/integration/idempotency_test.rs`:
   ```rust
   use std::fs;
   use std::path::PathBuf;
   use tempfile::TempDir;

   #[test]
   fn test_idempotency_same_directory() {
       let temp_dir = TempDir::new().unwrap();
       let project_dir = temp_dir.path().join("project");
       fs::create_dir(&project_dir).unwrap();

       // Create .nvmrc
       fs::write(project_dir.join(".nvmrc"), "18.20.0\n").unwrap();

       // First activation should work
       let result1 = run_activation(&project_dir);
       assert!(result1.is_ok());

       // Second activation with same file should be skipped
       // (would be tested via shell, checking XVN_ACTIVE_FILE)
       // This is primarily a shell-level test
   }

   #[test]
   fn test_idempotency_subdirectory() {
       let temp_dir = TempDir::new().unwrap();
       let project_dir = temp_dir.path().join("project");
       let sub_dir = project_dir.join("subdir");
       fs::create_dir_all(&sub_dir).unwrap();

       // Create .nvmrc in parent
       fs::write(project_dir.join(".nvmrc"), "18.20.0\n").unwrap();

       // Activation from subdirectory should find parent's .nvmrc
       // But if XVN_ACTIVE_FILE is already set, should skip
       // (would be tested via shell)
   }
   ```

4. **Create shell-level test:**
   Create `tests/shell/test_idempotency.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   # Test idempotency behavior

   # Setup
   source shell/xvn.sh
   export XVN_DEBUG=1

   # Create test directory with .nvmrc
   TEST_DIR=$(mktemp -d)
   echo "18.20.0" > "$TEST_DIR/.nvmrc"

   # First activation
   cd "$TEST_DIR"
   # Should activate

   # Second activation in same directory
   cd "$TEST_DIR"
   # Should skip (idempotent)

   # Check XVN_ACTIVE_FILE is set
   [[ -n "$XVN_ACTIVE_FILE" ]] || exit 1

   # Navigate away
   cd /tmp
   # Should clear XVN_ACTIVE_FILE
   [[ -z "${XVN_ACTIVE_FILE:-}" ]] || exit 1

   # Cleanup
   rm -rf "$TEST_DIR"

   echo "✓ Idempotency tests passed"
   ```

5. **Add performance benchmark:**
   Create benchmark in `benches/activation_benchmark.rs`:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn benchmark_idempotent_activation(c: &mut Criterion) {
       // Benchmark activation when XVN_ACTIVE_FILE matches
       // Target: <5ms (should be near-instant)
       c.bench_function("idempotent_activation", |b| {
           b.iter(|| {
               // Shell-level check should be extremely fast
               let active_file = "/path/to/project/.nvmrc";
               let current_file = "/path/to/project/.nvmrc";
               black_box(active_file == current_file)
           });
       });
   }

   criterion_group!(benches, benchmark_idempotent_activation);
   criterion_main!(benches);
   ```

**Code Structure:**
- File: `shell/xvn.sh` - Add clarifying comments (no logic changes)
- File: `tests/integration/idempotency_test.rs` - Rust-level tests
- File: `tests/shell/test_idempotency.sh` - Shell-level tests
- File: `benches/activation_benchmark.rs` - Performance benchmark

**Key Considerations:**
- Idempotency is implemented at shell level (before calling Rust binary)
- `XVN_ACTIVE_FILE` environment variable is the source of truth
- Shell hook must clear `XVN_ACTIVE_FILE` when leaving project directory
- Performance is critical: idempotency check must be <5ms (shell-only, no binary call)

**Testing:**
- Shell test: Verify XVN_ACTIVE_FILE prevents re-activation
- Shell test: Verify XVN_ACTIVE_FILE cleared when leaving directory
- Shell test: Verify activation in subdirectory uses parent's version file
- Benchmark: Idempotent activation should be <5ms (target: <1ms)

**Dependencies:**
- Requires: Milestone 3 (shell integration already implemented)
- Enables: M4.5 (performance tests)

---

### Task M4.5: Unit tests for activation

**Objective:** Comprehensive unit tests for all activation logic, ensuring >85% code coverage.

**Implementation Steps:**

1. **Create test module in `src/activation/orchestrator.rs`:**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use crate::config::{AutoInstallMode, Config};
       use crate::plugins::MockPlugin;
       use crate::shell::MockCommandWriter;
       use std::sync::{Arc, Mutex};

       fn create_test_config(auto_install: AutoInstallMode) -> Config {
           Config {
               plugins: vec!["mock".to_string()],
               auto_install,
               version_files: vec![".nvmrc".to_string()],
           }
       }

       #[test]
       fn test_activate_existing_version() {
           // Test successful activation of installed version
           let config = create_test_config(AutoInstallMode::Never);
           let mut mock_plugin = MockPlugin::new("mock");
           mock_plugin.set_available(true);
           mock_plugin.set_has_version("18.20.0", true);

           let registry = PluginRegistry::new(&config.plugins);
           registry.register(Box::new(mock_plugin));

           let mut mock_writer = MockCommandWriter::new();
           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           );

           // Create temp dir with .nvmrc
           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           // Activate
           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_ok());

           // Verify command was written
           assert_eq!(mock_writer.commands.len(), 1);
           assert!(mock_writer.commands[0].contains("mock use 18.20.0"));
       }

       #[test]
       fn test_auto_install_never() {
           // Test that auto_install=never shows error
           let config = create_test_config(AutoInstallMode::Never);
           let mut mock_plugin = MockPlugin::new("mock");
           mock_plugin.set_available(true);
           mock_plugin.set_has_version("18.20.0", false);

           let registry = PluginRegistry::new(&config.plugins);
           registry.register(Box::new(mock_plugin));

           let mut mock_writer = MockCommandWriter::new();
           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           );

           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_err());

           if let Err(ActivationError::VersionNotInstalled { version, .. }) = result {
               assert_eq!(version, "18.20.0");
           } else {
               panic!("Expected VersionNotInstalled error");
           }
       }

       #[test]
       fn test_auto_install_always() {
           // Test that auto_install=always installs without prompt
           let config = create_test_config(AutoInstallMode::Always);
           let mut mock_plugin = MockPlugin::new("mock");
           mock_plugin.set_available(true);
           mock_plugin.set_has_version("18.20.0", false);

           let registry = PluginRegistry::new(&config.plugins);
           registry.register(Box::new(mock_plugin));

           let mut mock_writer = MockCommandWriter::new();
           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           );

           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_ok());

           // Verify install + activate commands written
           assert_eq!(mock_writer.commands.len(), 1);
           assert!(mock_writer.commands[0].contains("mock install 18.20.0"));
           assert!(mock_writer.commands[0].contains("mock use 18.20.0"));
       }

       #[test]
       fn test_auto_install_prompt_yes() {
           // Test that user confirmation triggers install
           let config = create_test_config(AutoInstallMode::Prompt);
           let mut mock_plugin = MockPlugin::new("mock");
           mock_plugin.set_available(true);
           mock_plugin.set_has_version("18.20.0", false);

           let registry = PluginRegistry::new(&config.plugins);
           registry.register(Box::new(mock_plugin));

           let mut mock_writer = MockCommandWriter::new();
           let mut mock_prompt = MockUserPrompt::new(vec![true]);

           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           ).with_user_prompt(Box::new(mock_prompt));

           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_ok());

           // Verify install + activate commands written
           assert_eq!(mock_writer.commands.len(), 1);
       }

       #[test]
       fn test_auto_install_prompt_no() {
           // Test that user decline shows mismatch
           let config = create_test_config(AutoInstallMode::Prompt);
           let mut mock_plugin = MockPlugin::new("mock");
           mock_plugin.set_available(true);
           mock_plugin.set_has_version("18.20.0", false);

           let registry = PluginRegistry::new(&config.plugins);
           registry.register(Box::new(mock_plugin));

           let mut mock_writer = MockCommandWriter::new();
           let mut mock_prompt = MockUserPrompt::new(vec![false]);

           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           ).with_user_prompt(Box::new(mock_prompt));

           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_ok());

           // Verify no commands written (user declined)
           assert_eq!(mock_writer.commands.len(), 0);
       }

       #[test]
       fn test_no_version_file() {
           // Test that missing version file is not an error
           let config = create_test_config(AutoInstallMode::Never);
           let registry = PluginRegistry::new(&config.plugins);
           let mut mock_writer = MockCommandWriter::new();

           let mut orchestrator = Orchestrator::new(
               &config,
               &registry,
               &mut mock_writer,
           );

           let temp_dir = tempfile::TempDir::new().unwrap();
           // No .nvmrc file

           let result = orchestrator.activate(temp_dir.path());
           assert!(result.is_ok());
           assert_eq!(mock_writer.commands.len(), 0);
       }
   }
   ```

2. **Create mock command writer for testing:**
   Add to `src/shell/fd3.rs` or create `src/shell/mock.rs`:
   ```rust
   #[cfg(test)]
   pub struct MockCommandWriter {
       pub commands: Vec<String>,
   }

   #[cfg(test)]
   impl MockCommandWriter {
       pub fn new() -> Self {
           Self {
               commands: Vec::new(),
           }
       }
   }

   #[cfg(test)]
   impl CommandWriter for MockCommandWriter {
       fn write_command(&mut self, cmd: &str) -> std::io::Result<()> {
           self.commands.push(cmd.to_string());
           Ok(())
       }
   }
   ```

   Note: This assumes `CommandWriter` is a trait. If it's a struct from Milestone 3, you may need to adjust this to work with the existing implementation.

3. **Add performance benchmark:**
   Create `benches/activation_benchmark.rs`:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   use std::time::Duration;

   fn benchmark_full_activation(c: &mut Criterion) {
       // Benchmark full activation flow
       // Target: <100ms (P95)
       c.bench_function("full_activation", |b| {
           // Setup test environment
           let temp_dir = tempfile::TempDir::new().unwrap();
           std::fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();

           b.iter(|| {
               // Run activation
               // (In real benchmark, use actual xvn binary)
               black_box(temp_dir.path())
           });
       });
   }

   criterion_group! {
       name = benches;
       config = Criterion::default()
           .sample_size(100)
           .measurement_time(Duration::from_secs(10));
       targets = benchmark_full_activation
   }
   criterion_main!(benches);
   ```

4. **Add Cargo.toml dependencies for testing:**
   ```toml
   [dev-dependencies]
   tempfile = "3"
   criterion = "0.5"

   [[bench]]
   name = "activation_benchmark"
   harness = false
   ```

**Code Structure:**
- File: `src/activation/orchestrator.rs` - Add `#[cfg(test)] mod tests`
- File: `src/shell/fd3.rs` - Add `MockCommandWriter`
- File: `benches/activation_benchmark.rs` - Performance benchmarks

**Key Considerations:**
- Use `tempfile` crate for temporary directories in tests
- Mock all external dependencies (plugins, stdin, file system where possible)
- Test all code paths: success, errors, user confirmations
- Use property-based testing for version string parsing (future enhancement)
- Performance target: <100ms (P95) for full activation

**Testing:**
- All test cases must pass
- Run: `cargo test --all-features`
- Run: `cargo bench` to verify performance
- Coverage: `cargo tarpaulin` should show >85%

**Dependencies:**
- Requires: M4.1, M4.2, M4.3 (all activation logic)
- Enables: M4.8 (code quality checks)

---

### Task M4.6: Integration tests

**Objective:** End-to-end tests that verify the complete activation flow with real shell integration.

**Implementation Steps:**

1. **Create integration test directory structure:**
   ```bash
   mkdir -p tests/integration
   touch tests/integration/activation_test.rs
   touch tests/integration/auto_install_test.rs
   ```

2. **Create `tests/integration/activation_test.rs`:**
   ```rust
   use assert_cmd::Command;
   use predicates::prelude::*;
   use std::fs;
   use tempfile::TempDir;

   #[test]
   fn test_activate_with_version_file() {
       let temp_dir = TempDir::new().unwrap();
       fs::write(temp_dir.path().join(".nvmrc"), "18.20.0\n").unwrap();

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(temp_dir.path())
           .assert()
           .success();

       // Note: Actual activation requires plugin to have version installed
       // This test verifies command doesn't crash
   }

   #[test]
   fn test_activate_no_version_file() {
       let temp_dir = TempDir::new().unwrap();
       // No .nvmrc file

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(temp_dir.path())
           .assert()
           .success();

       // Should succeed silently (no version file is not an error)
   }

   #[test]
   fn test_activate_nested_directories() {
       let temp_dir = TempDir::new().unwrap();
       let project_dir = temp_dir.path().join("project");
       let sub_dir = project_dir.join("src").join("components");
       fs::create_dir_all(&sub_dir).unwrap();

       // Create .nvmrc in project root
       fs::write(project_dir.join(".nvmrc"), "18.20.0\n").unwrap();

       // Activate from subdirectory
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(&sub_dir)
           .assert()
           .success();

       // Should find parent's .nvmrc
   }

   #[test]
   fn test_activate_invalid_version_file() {
       let temp_dir = TempDir::new().unwrap();
       fs::write(temp_dir.path().join(".nvmrc"), "").unwrap();

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(temp_dir.path())
           .assert()
           .failure()
           .stderr(predicate::str::contains("empty"));
   }
   ```

3. **Create `tests/integration/auto_install_test.rs`:**
   ```rust
   use assert_cmd::Command;
   use predicates::prelude::*;
   use std::fs;
   use tempfile::TempDir;

   // Note: These tests require mocking stdin, which is complex
   // For MVP, we'll rely on unit tests with MockUserPrompt
   // Shell-level tests will verify the full flow manually

   #[test]
   fn test_version_not_installed_never_mode() {
       let temp_dir = TempDir::new().unwrap();

       // Create config with auto_install=never
       let config_dir = temp_dir.path().join(".xvn");
       fs::create_dir(&config_dir).unwrap();
       fs::write(
           config_dir.join("config.yaml"),
           "auto_install: never\n"
       ).unwrap();

       // Create .nvmrc with unlikely version
       fs::write(temp_dir.path().join(".nvmrc"), "99.99.99\n").unwrap();

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(temp_dir.path())
           .env("XVN_CONFIG_DIR", config_dir)
           .assert()
           .failure()
           .stderr(predicate::str::contains("not installed"));
   }
   ```

4. **Add shell-level integration test:**
   Create `tests/shell/test_activation.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   # Test full activation flow with shell integration

   echo "=== Shell Integration Test ==="

   # Build xvn binary
   cargo build --release
   export PATH="$PWD/target/release:$PATH"

   # Source shell hook
   source shell/xvn.sh

   # Create test project
   TEST_DIR=$(mktemp -d)
   echo "18.20.0" > "$TEST_DIR/.nvmrc"

   echo "Test directory: $TEST_DIR"

   # Test 1: Navigate to directory with .nvmrc
   echo "Test 1: Navigate to project directory"
   cd "$TEST_DIR"
   # Should trigger activation

   # Test 2: Navigate to subdirectory
   echo "Test 2: Navigate to subdirectory"
   mkdir -p "$TEST_DIR/src"
   cd "$TEST_DIR/src"
   # Should be idempotent (skip activation)

   # Test 3: Navigate away
   echo "Test 3: Navigate away"
   cd /tmp
   # Should clear XVN_ACTIVE_FILE
   [[ -z "${XVN_ACTIVE_FILE:-}" ]] || {
       echo "ERROR: XVN_ACTIVE_FILE not cleared"
       exit 1
   }

   # Cleanup
   rm -rf "$TEST_DIR"

   echo "✓ All shell integration tests passed"
   ```

5. **Add test runner script:**
   Create `tests/run_all_tests.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   echo "=== Running all xvn tests ==="

   # Unit tests
   echo "Running unit tests..."
   cargo test --lib

   # Integration tests
   echo "Running integration tests..."
   cargo test --test '*'

   # Shell tests
   echo "Running shell tests..."
   bash tests/shell/test_activation.sh
   bash tests/shell/test_idempotency.sh

   # Benchmarks (smoke test only)
   echo "Running benchmarks (smoke test)..."
   cargo bench --no-run

   echo "✓ All tests passed!"
   ```

6. **Add test dependencies to `Cargo.toml`:**
   ```toml
   [dev-dependencies]
   assert_cmd = "2"
   predicates = "3"
   tempfile = "3"
   ```

**Code Structure:**
- File: `tests/integration/activation_test.rs` - Activation flow tests
- File: `tests/integration/auto_install_test.rs` - Auto-install tests
- File: `tests/shell/test_activation.sh` - Shell integration tests
- File: `tests/shell/test_idempotency.sh` - Idempotency tests (from M4.4)
- File: `tests/run_all_tests.sh` - Test runner script

**Key Considerations:**
- Integration tests should be realistic but not require actual nvm/fnm
- Use mock plugins for tests that need plugin interactions
- Shell tests are critical - they verify the FD:3 protocol works
- Keep tests fast: use small version files, avoid network calls

**Testing:**
- Run: `cargo test --test '*'` for integration tests
- Run: `bash tests/run_all_tests.sh` for all tests
- CI should run all tests on every commit

**Dependencies:**
- Requires: M4.1-M4.5 (all activation logic)
- Enables: M4.8 (code quality checks)

---

### Task M4.7: Implement structured error handling

**Objective:** Create a comprehensive error type hierarchy with actionable hints for users.

**Implementation Steps:**

1. **Create `src/activation/errors.rs`:**
   ```rust
   use thiserror::Error;
   use std::io;

   /// Result type for activation operations
   pub type ActivationResult<T> = Result<T, ActivationError>;

   /// Errors that can occur during version activation
   #[derive(Debug, Error)]
   pub enum ActivationError {
       /// No version file found (not an error, just informational)
       #[error("no version file found")]
       NoVersionFile,

       /// Version file exists but is invalid
       #[error("invalid version file: {path}")]
       InvalidVersionFile {
           path: String,
           #[source]
           source: io::Error,
       },

       /// Version file is empty
       #[error("version file is empty: {path}")]
       EmptyVersionFile { path: String },

       /// Required version is not installed
       #[error("Node.js version {version} is not installed")]
       VersionNotInstalled {
           version: String,
           hint: String,
       },

       /// No plugins are available on the system
       #[error("no version manager plugins available")]
       NoPluginsAvailable,

       /// Plugin-specific error
       #[error("plugin error ({plugin})")]
       PluginError {
           plugin: String,
           #[source]
           source: anyhow::Error,
       },

       /// Configuration error
       #[error("configuration error")]
       ConfigError(#[from] crate::config::ConfigError),

       /// I/O error
       #[error("I/O error")]
       IoError(#[from] io::Error),
   }

   impl ActivationError {
       /// Get an actionable hint for this error
       pub fn hint(&self) -> Option<String> {
           match self {
               Self::NoPluginsAvailable => Some(
                   "Install a Node.js version manager:\n\
                    • nvm: https://github.com/nvm-sh/nvm\n\
                    • fnm: https://github.com/Schniz/fnm\n\
                    • n: https://github.com/tj/n"
                       .to_string(),
               ),
               Self::VersionNotInstalled { hint, .. } => Some(hint.clone()),
               Self::EmptyVersionFile { path } => Some(format!(
                   "The version file '{}' is empty.\n\
                    Add a Node.js version (e.g., '18.20.0') to the file.",
                   path
               )),
               Self::InvalidVersionFile { path, .. } => Some(format!(
                   "The version file '{}' could not be read.\n\
                    Check file permissions and format.",
                   path
               )),
               Self::ConfigError(_) => Some(
                   "Run 'xvn setup' to create a default configuration, or check ~/.xvnrc syntax."
                       .to_string(),
               ),
               _ => None,
           }
       }
   }
   ```

2. **Update config error type in `src/config/loader.rs`:**
   ```rust
   use thiserror::Error;

   #[derive(Debug, Error)]
   pub enum ConfigError {
       #[error("failed to load configuration file: {path}")]
       LoadError {
           path: String,
           #[source]
           source: std::io::Error,
       },

       #[error("failed to parse configuration file: {path}")]
       ParseError {
           path: String,
           #[source]
           source: serde_yaml::Error,
       },

       #[error("invalid configuration: {0}")]
       ValidationError(String),
   }
   ```

3. **Update error display in `src/cli.rs`:**
   Replace error handling in `Commands::Activate` (see M4.1 step 5):
   ```rust
   match orchestrator.activate(&path) {
       Ok(()) => Ok(()),
       Err(e) => {
           // Print main error message
           eprintln!("Error: {}", e);

           // Print hint if available
           if let Some(hint) = e.hint() {
               eprintln!();
               eprintln!("Hint: {}", hint);
           }

           // Exit with error code
           std::process::exit(1);
       }
   }
   ```

4. **Update version file errors in `src/version_file/mod.rs`:**
   ```rust
   use crate::activation::ActivationError;

   pub fn parse(path: &Path) -> Result<Self, ActivationError> {
       let content = fs::read_to_string(path)
           .map_err(|e| ActivationError::InvalidVersionFile {
               path: path.display().to_string(),
               source: e,
           })?;

       let version = content.trim();

       if version.is_empty() {
           return Err(ActivationError::EmptyVersionFile {
               path: path.display().to_string(),
           });
       }

       Ok(Self {
           path: path.to_path_buf(),
           version: version.to_string(),
       })
   }
   ```

5. **Add tests for error messages:**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_error_hints() {
           let err = ActivationError::NoPluginsAvailable;
           let hint = err.hint().unwrap();
           assert!(hint.contains("nvm"));
           assert!(hint.contains("fnm"));
       }

       #[test]
       fn test_error_display() {
           let err = ActivationError::VersionNotInstalled {
               version: "18.20.0".to_string(),
               hint: "Run: nvm install 18.20.0".to_string(),
           };

           let msg = format!("{}", err);
           assert!(msg.contains("18.20.0"));
           assert!(msg.contains("not installed"));
       }
   }
   ```

**Code Structure:**
- File: `src/activation/errors.rs` - Error type definitions
- File: `src/config/loader.rs` - Config error types
- File: `src/cli.rs` - Error display logic
- File: `src/version_file/mod.rs` - Update to use new errors

**Key Considerations:**
- Use `thiserror` crate for ergonomic error handling
- Every error should have clear message and actionable hint
- Distinguish between user errors (bad config) and system errors (I/O)
- Include context in errors (file paths, version strings)
- Don't use `panic!` or `unwrap()` in production code

**Testing:**
- Unit test: All error variants
- Unit test: Error hints are present and helpful
- Integration test: Verify error messages shown to user
- Test: Error messages don't contain debug info (internal paths, etc.)

**Dependencies:**
- Requires: None (can be done early)
- Enables: M4.1, M4.2, M4.3 (all use these errors)

---

### Task M4.8: Code quality checks

**Objective:** Ensure code meets quality standards with linting, formatting, and coverage checks.

**Implementation Steps:**

1. **Run clippy and fix warnings:**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

   Fix all warnings, paying special attention to:
   - Unused variables or imports
   - Unnecessary clones or allocations
   - Potential logic errors
   - Inefficient code patterns

2. **Run formatter:**
   ```bash
   cargo fmt --all
   ```

3. **Verify all tests pass:**
   ```bash
   cargo test --all-features
   ```

4. **Check test coverage:**
   ```bash
   cargo install cargo-tarpaulin  # If not already installed
   cargo tarpaulin --out Lcov --output-dir coverage
   ```

   Target: >85% line coverage for activation module

5. **Create quality check script:**
   Create `scripts/quality_check.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   echo "=== xvn Quality Checks ==="

   # Format check
   echo "Checking code formatting..."
   cargo fmt --all -- --check || {
       echo "ERROR: Code is not formatted. Run 'cargo fmt'"
       exit 1
   }

   # Clippy
   echo "Running clippy..."
   cargo clippy --all-targets --all-features -- -D warnings || {
       echo "ERROR: Clippy warnings found"
       exit 1
   }

   # Tests
   echo "Running tests..."
   cargo test --all-features || {
       echo "ERROR: Tests failed"
       exit 1
   }

   # Coverage
   echo "Checking test coverage..."
   cargo tarpaulin --out Stdout | tee coverage.txt
   COVERAGE=$(grep "Coverage" coverage.txt | awk '{print $2}' | sed 's/%//')

   if (( $(echo "$COVERAGE < 85" | bc -l) )); then
       echo "ERROR: Coverage $COVERAGE% is below 85% threshold"
       exit 1
   fi

   echo "✓ All quality checks passed!"
   echo "  Coverage: $COVERAGE%"
   ```

6. **Add quality check to CI:**
   Update `.github/workflows/ci.yml` (if exists, or create):
   ```yaml
   name: CI

   on: [push, pull_request]

   jobs:
     quality:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: actions-rs/toolchain@v1
           with:
             toolchain: stable
             components: clippy, rustfmt

         - name: Format check
           run: cargo fmt --all -- --check

         - name: Clippy
           run: cargo clippy --all-targets --all-features -- -D warnings

         - name: Tests
           run: cargo test --all-features

         - name: Coverage
           run: |
             cargo install cargo-tarpaulin
             cargo tarpaulin --out Lcov --output-dir coverage

         - name: Upload coverage
           uses: codecov/codecov-action@v3
           with:
             files: ./coverage/lcov.info
   ```

7. **Fix any issues found:**
   - Address all clippy warnings
   - Add tests to reach >85% coverage
   - Fix any failing tests

**Code Structure:**
- File: `scripts/quality_check.sh` - Quality check script
- File: `.github/workflows/ci.yml` - CI configuration (if using GitHub)

**Key Considerations:**
- Run quality checks before every commit
- Address warnings immediately - don't accumulate tech debt
- Coverage target is 85% for new code, not entire codebase
- Use `#[allow(clippy::...)]` sparingly and only with justification

**Testing:**
- Run: `bash scripts/quality_check.sh`
- Verify all checks pass
- CI should run these checks on every push

**Dependencies:**
- Requires: All previous M4 tasks (tests must exist)
- Enables: Milestone 5 (testing & polish)

---

## Integration Points

**How tasks work together:**

1. **M4.7 (errors)** → **M4.1 (orchestrator)**: Error types are used throughout orchestrator
2. **M4.1 (orchestrator)** → **M4.2 (auto-install)**: Orchestrator delegates to auto-install logic
3. **M4.2 (auto-install)** → **M4.3 (mismatch)**: Auto-install calls mismatch detection when user declines
4. **M4.1-M4.3** → **M4.5 (unit tests)**: Tests verify all activation logic
5. **M4.5 (unit tests)** → **M4.6 (integration)**: Integration tests build on unit tests
6. **M4.1-M4.6** → **M4.8 (quality)**: Quality checks verify everything works

**Data flow:**
```
User runs: cd ~/project
    ↓
Shell hook: __xvn_chpwd
    ↓
Find version file: .nvmrc
    ↓
Execute: xvn activate ~/project
    ↓
Orchestrator.activate()
    ├─ Load config
    ├─ Find version file
    ├─ Load plugins
    ├─ Check if version installed
    │   ├─ Yes → activate_existing_version()
    │   └─ No → handle_missing_version()
    │       ├─ Check auto_install mode
    │       ├─ Prompt user (if mode=prompt)
    │       ├─ Install + activate (if confirmed)
    │       └─ Show mismatch (if declined)
    └─ Write commands to FD:3
    ↓
Shell evaluates FD:3 commands
    ↓
Node.js version activated
```

---

## Testing Strategy

**Test Pyramid:**
1. **Unit tests (fast, many):** Test individual functions
2. **Integration tests (medium):** Test component interactions
3. **Shell tests (slow, few):** Test end-to-end with real shell

**Coverage targets:**
- Unit tests: >85% line coverage
- Integration tests: All major flows (activate, install, decline)
- Shell tests: Real-world scenarios (cd, subdirectory, idempotency)

**Performance targets:**
- Activation (version installed): <100ms (P95)
- Activation (idempotent): <5ms (P95)
- User prompt: <500ms response time acceptable

**Test execution:**
```bash
# All tests
cargo test --all-features

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Shell tests
bash tests/run_all_tests.sh

# Benchmarks
cargo bench

# Coverage
cargo tarpaulin --out Lcov
```

---

## Success Criteria

Milestone 4 is complete when:

✅ **Functional:**
- [ ] `xvn activate` works with installed versions
- [ ] Auto-install prompts appear for missing versions
- [ ] User can confirm or decline installation
- [ ] Version mismatch shown when user declines
- [ ] All error cases handled gracefully

✅ **Quality:**
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Code coverage >85% for activation module
- [ ] No clippy warnings
- [ ] Code formatted (`cargo fmt`)

✅ **Performance:**
- [ ] Activation <100ms (P95) for installed versions
- [ ] Idempotent activation <5ms (P95)

✅ **Documentation:**
- [ ] All public APIs documented
- [ ] Error messages are clear and actionable
- [ ] Test coverage documented

✅ **Integration:**
- [ ] Shell integration works (bash/zsh)
- [ ] FD:3 protocol functions correctly
- [ ] Idempotency prevents redundant activations

---

## Implementation Order

**Week 7 (Core Activation):**

**Day 1-2: M4.7 - Error handling (foundation)**
- Create `src/activation/errors.rs` with all error types
- Implement `ActivationError` enum with `hint()` method
- Add unit tests for error messages
- Update `src/config/loader.rs` with ConfigError
- Integrate errors with existing code (version_file module)

**Day 3-5: M4.1 - Orchestrator (core logic)**
- Create activation module structure
- Implement `Orchestrator` struct with basic flow
- Implement `activate_existing_version` method
- Update `src/cli.rs` to use orchestrator
- Add basic unit tests with mocks

**Day 6-8: M4.2 - Auto-install (user interaction)**
- Create `UserPrompt` trait and `StdinUserPrompt` implementation
- Implement `handle_missing_version` method
- Implement `install_and_activate` method
- Add `MockUserPrompt` for testing
- Test all auto-install modes (prompt/always/never)

**Day 9-10: M4.3 - Version mismatch (UX polish)**
- Implement `show_version_mismatch` method
- Test with node available and unavailable
- Polish error messages and user output

**Week 8 (Testing & Quality):**

**Day 11-13: M4.5 - Unit tests (comprehensive)**
- Add comprehensive test suite to orchestrator
- Create `MockCommandWriter` implementation
- Test all code paths and error scenarios
- Add performance benchmarks
- Verify >85% code coverage

**Day 14-15: M4.6 - Integration tests (end-to-end)**
- Create integration test directory
- Write activation flow tests
- Write auto-install tests
- Create shell integration tests
- Create test runner script

**Day 16: M4.4 - Idempotency verification (shell-level)**
- Review shell hook idempotency logic
- Add clarifying comments to xvn.sh
- Create shell-level idempotency tests
- Add performance benchmark for idempotency

**Day 17-18: M4.8 - Code quality checks (final polish)**
- Run and fix all clippy warnings
- Run cargo fmt
- Verify all tests pass
- Check coverage >85%
- Create quality check script
- Final review and cleanup

---

## Dependencies on Previous Milestones

- **Milestone 1:** Config loading, version file detection
- **Milestone 2:** Plugin system, plugin registry
- **Milestone 3:** Shell integration, FD:3 protocol, idempotency

All three previous milestones must be complete before starting M4.

---

## Troubleshooting Common Implementation Issues

### Issue: Tests fail with "trait method not found" for MockCommandWriter
**Cause:** `MockCommandWriter` doesn't properly implement the `CommandWriter` trait
**Fix:** Ensure you have `impl CommandWriter for MockCommandWriter { ... }` block, not just regular methods
**Code:**
```rust
impl CommandWriter for MockCommandWriter {
    fn write_command(&mut self, cmd: &str) -> std::io::Result<()> {
        self.commands.push(cmd.to_string());
        Ok(())
    }
}
```

### Issue: Compilation error "cannot find type `MockPlugin`"
**Cause:** MockPlugin from Milestone 2 might not be exported in tests
**Fix:** Check that `src/plugins/mod.rs` exports MockPlugin in test configuration:
```rust
#[cfg(test)]
pub use mock::MockPlugin;
```

### Issue: Clippy warning about needless returns
**Cause:** Explicit `return` statements in orchestrator methods (Rust convention is implicit return)
**Fix:** Remove `return` keyword from last expression in function:
```rust
// Before
fn foo() -> Result<()> {
    do_something()?;
    return Ok(());  // Clippy warning
}

// After
fn foo() -> Result<()> {
    do_something()?;
    Ok(())  // Implicit return
}
```

### Issue: Integration tests hang waiting for stdin
**Cause:** Using `StdinUserPrompt` in tests that don't provide stdin input
**Fix:** Always use `MockUserPrompt` in tests:
```rust
let mut orchestrator = Orchestrator::new(&config, &registry, &mut fd3)
    .with_user_prompt(Box::new(MockUserPrompt::new(vec![true])));
```

### Issue: Shell tests fail with "XVN_ACTIVE_FILE not cleared"
**Cause:** Shell hook `__xvn_chpwd` not detecting directory change or clear logic missing
**Fix:** Verify Milestone 3's `shell/xvn.sh` implements clearing logic:
```bash
# In __xvn_chpwd function
if version_file=$(__xvn_find_file "$PWD"); then
    __xvn_activate "$version_file"
else
    # Clear active file when no version file found
    if [[ -n "${XVN_ACTIVE_FILE:-}" ]]; then
        unset XVN_ACTIVE_FILE
    fi
fi
```

### Issue: Error "field `user_prompt` has private access"
**Cause:** Trying to set `user_prompt` directly instead of using builder method
**Fix:** Use the `with_user_prompt()` builder method:
```rust
// Wrong
orchestrator.user_prompt = Box::new(MockUserPrompt::new(vec![true]));

// Correct
let orchestrator = orchestrator.with_user_prompt(Box::new(MockUserPrompt::new(vec![true])));
```

### Issue: Version mismatch shows "v18.20.0" instead of "18.20.0"
**Cause:** Not stripping leading 'v' from node --version output
**Fix:** Use `.trim_start_matches('v')`:
```rust
let current_version = String::from_utf8_lossy(&output.stdout)
    .trim()
    .trim_start_matches('v')  // Strip leading 'v'
    .to_string();
```

### Issue: Activation fails with "NoPluginsAvailable" but plugins are installed
**Cause:** Plugin registry might not be checking `is_available()` correctly
**Fix:** Verify Milestone 2's plugin implementations return `Ok(true)` from `is_available()`
**Debug:** Add logging:
```rust
for plugin in &self.plugins {
    log::debug!("Plugin {}: available = {:?}", plugin.name(), plugin.is_available());
}
```

### Issue: Commands not executed by shell (no Node.js version switch)
**Cause:** FD:3 protocol not working correctly
**Fix:** Verify shell hook is using correct redirection:
```bash
# In shell/xvn.sh __xvn_activate function
commands=$(xvn activate "$(dirname "$version_file")" 3>&1 1>&2 2>&3)
```
**Test:** Add debug output:
```bash
if [[ -n "$commands" ]]; then
    echo "[DEBUG] Commands to eval: $commands" >&2
    eval "$commands"
fi
```

---

## Next Steps

After completing Milestone 4:
- **Milestone 5:** Comprehensive testing, benchmarks, documentation
- **Milestone 6:** Release preparation, CI/CD, npm packaging

Milestone 4 marks the completion of core functionality - xvn is now fully functional for version switching with auto-install support!

---

**END OF MILESTONE 4 IMPLEMENTATION PLAN**
