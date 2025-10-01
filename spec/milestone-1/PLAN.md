# Milestone 1: Core Infrastructure - Implementation Plan

## Overview

This milestone establishes the foundational Rust project with CLI framework, configuration system, and version file detection. By the end, you'll have a working binary that can parse CLI arguments, load configuration from multiple sources with correct precedence, and discover version files by traversing the directory tree.

**Key Goals:**
- Functional CLI with clap (setup, activate, status commands)
- Configuration system with YAML parsing and precedence
- Version file detection with directory traversal
- Robust error handling and logging
- >80% test coverage

## Prerequisites

**Required Tools:**
- Rust 1.70+ (`rustup install stable`)
- Cargo (comes with Rust)
- Git

**Verification:**
```bash
rustc --version   # Should be >= 1.70
cargo --version
git --version
```

## Error Handling Strategy

Throughout this milestone, we use two different Result types:

- **`anyhow::Result<T>`** - Use in application code (main.rs, cli.rs)
  - Convenient for CLI applications
  - Automatic error conversion with `?` operator
  - Rich context with `.context()` method
  - Good for top-level error handling

- **`crate::error::Result<T>`** - Use in library code (lib.rs, modules)
  - Structured, typed errors with XvnError enum
  - Clear API contracts
  - Better error documentation
  - Allows library users to match on specific errors

**Rule of thumb:** Library code returns `crate::error::Result`, application code uses `anyhow::Result` and converts at the boundary.

## Implementation Tasks

### Task M1.1: Set up Rust project structure with Cargo.toml

**Objective:** Initialize the Rust project with proper module structure and dependencies configured.

**Implementation Steps:**

1. **Initialize the project:**
   ```bash
   cd /Users/cam/mo/dev/tools/xvn
   cargo init --name xvn
   ```

2. **Configure `Cargo.toml`:**
   ```toml
   [package]
   name = "xvn"
   version = "0.1.0"
   edition = "2021"
   authors = ["cameronolivier@gmail.com"]
   description = "Extreme Version Switcher for Node.js"
   license = "MIT"

   [[bin]]
   name = "xvn"
   path = "src/main.rs"

   [dependencies]
   clap = { version = "4.5", features = ["derive"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_yaml = "0.9"
   anyhow = "1.0"
   thiserror = "1.0"
   dirs = "5.0"
   env_logger = "0.11"
   log = "0.4"

   [dev-dependencies]
   tempfile = "3.10"
   assert_cmd = "2.0"
   predicates = "3.1"
   ```

3. **Create module structure:**
   ```bash
   mkdir -p src/config src/version_file
   touch src/lib.rs
   touch src/cli.rs
   touch src/error.rs
   touch src/config/mod.rs
   touch src/config/schema.rs
   touch src/config/loader.rs
   touch src/version_file/mod.rs
   touch src/version_file/finder.rs
   ```

4. **Create `src/lib.rs` (library entry point):**
   ```rust
   //! xvn - Extreme Version Switcher for Node.js
   //!
   //! Fast, modular automatic Node.js version switching.

   pub mod cli;
   pub mod config;
   pub mod error;
   pub mod version_file;

   // Re-export key types
   pub use error::XvnError;
   pub use config::Config;
   pub use version_file::VersionFile;
   ```

5. **Create initial `src/main.rs`:**
   ```rust
   use anyhow::Result;

   fn main() -> Result<()> {
       env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
           .init();

       xvn::cli::run()
   }
   ```

6. **Initialize Git (if not already done):**
   ```bash
   git init
   git add .
   git commit -m "feat: initialize Rust project structure"
   ```

7. **Verify the project builds:**
   ```bash
   cargo build
   cargo run -- --help
   ```

**Code Structure:**
```
xvn/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── cli.rs            # CLI framework (empty for now)
│   ├── error.rs          # Error types (empty for now)
│   ├── config/
│   │   ├── mod.rs        # Module exports
│   │   ├── schema.rs     # Config structs
│   │   └── loader.rs     # Config loading
│   └── version_file/
│       ├── mod.rs        # Module exports
│       └── finder.rs     # File discovery
└── tests/                # Integration tests (create later)
```

**Key Considerations:**
- Use `edition = "2021"` for latest Rust features
- Include both `[[bin]]` for the CLI and library code for testability
- Pin dependency versions to avoid breaking changes

**Testing:**
```bash
cargo build            # Should compile without errors
cargo run -- --help    # Should show help (will implement in M1.2)
```

**Dependencies:**
- None (this is the foundation)

**Enables:**
- M1.2 (CLI implementation)
- M1.3 (Config system)
- M1.4 (Version file detection)

---

### Task M1.2: Implement CLI framework with clap

**Objective:** Create a functional CLI with subcommands (setup, activate, status) and global flags using clap's derive API.

**Implementation Steps:**

1. **Create `src/cli.rs` with clap structures:**
   ```rust
   use clap::{Parser, Subcommand};
   use anyhow::Result;
   use std::path::PathBuf;
   use log::{info, debug};

   /// Extreme Version Switcher for Node.js
   #[derive(Parser, Debug)]
   #[command(name = "xvn")]
   #[command(about = "Automatic Node.js version switching", long_about = None)]
   #[command(version)]
   pub struct Cli {
       /// Enable verbose output
       #[arg(short, long, global = true)]
       pub verbose: bool,

       #[command(subcommand)]
       pub command: Option<Commands>,
   }

   #[derive(Subcommand, Debug)]
   pub enum Commands {
       /// Install xvn shell hooks and create default config
       Setup {
           /// Shell to configure (bash, zsh, or auto-detect)
           #[arg(short, long, default_value = "auto")]
           shell: String,

           /// Force reinstallation even if already set up
           #[arg(short, long)]
           force: bool,
       },

       /// Activate Node.js version for current directory
       Activate {
           /// Directory to activate for (defaults to current directory)
           #[arg(default_value = ".")]
           path: PathBuf,
       },

       /// Show current xvn status and configuration
       Status,
   }

   pub fn run() -> Result<()> {
       let cli = Cli::parse();

       // Set log level based on verbose flag
       if cli.verbose {
           log::set_max_level(log::LevelFilter::Debug);
       }

       match cli.command {
           Some(Commands::Setup { shell, force }) => {
               info!("Running setup command (shell: {}, force: {})", shell, force);
               println!("Setup command - not yet implemented");
               println!("  Shell: {}", shell);
               println!("  Force: {}", force);
               Ok(())
           }
           Some(Commands::Activate { path }) => {
               info!("Running activate command for path: {:?}", path);
               println!("Activate command - not yet implemented");
               println!("  Path: {}", path.display());
               Ok(())
           }
           Some(Commands::Status) => {
               info!("Running status command");
               println!("Status command - not yet implemented");
               Ok(())
           }
           None => {
               // No subcommand provided - show help
               Cli::parse_from(&["xvn", "--help"]);
               Ok(())
           }
       }
   }
   ```

2. **Update `src/main.rs` to use the CLI:**
   ```rust
   use anyhow::Result;

   fn main() -> Result<()> {
       // Initialize logger (RUST_LOG=debug for verbose output)
       env_logger::Builder::from_env(
           env_logger::Env::default().default_filter_or("info")
       ).init();

       xvn::cli::run()
   }
   ```

3. **Verify CLI commands work:**
   ```bash
   cargo run -- --version        # Should show version
   cargo run -- --help           # Should show help
   cargo run -- setup --help     # Should show setup command help
   cargo run -- activate .       # Should show "not yet implemented"
   cargo run -- status           # Should show "not yet implemented"
   cargo run -- --verbose status # Should show debug logs
   ```

4. **Create integration test `tests/cli_test.rs`:**
   ```rust
   use assert_cmd::Command;
   use predicates::prelude::*;

   #[test]
   fn test_version_flag() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("--version")
           .assert()
           .success()
           .stdout(predicate::str::contains("xvn 0.1.0"));
   }

   #[test]
   fn test_help_flag() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("--help")
           .assert()
           .success()
           .stdout(predicate::str::contains("Extreme Version Switcher"));
   }

   #[test]
   fn test_setup_command() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("setup")
           .assert()
           .success()
           .stdout(predicate::str::contains("not yet implemented"));
   }

   #[test]
   fn test_activate_command() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(".")
           .assert()
           .success()
           .stdout(predicate::str::contains("not yet implemented"));
   }

   #[test]
   fn test_status_command() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("status")
           .assert()
           .success()
           .stdout(predicate::str::contains("not yet implemented"));
   }

   #[test]
   fn test_no_command_shows_help() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.assert()
           .success()
           .stdout(predicate::str::contains("Usage:"));
   }
   ```

5. **Run tests:**
   ```bash
   cargo test --test cli_test
   ```

**Code Structure:**
- File: `src/cli.rs`
  - `Cli` struct with clap Parser derive
  - `Commands` enum with Subcommand derive
  - `run()` function to execute commands
- File: `tests/cli_test.rs`
  - Integration tests using assert_cmd

**Key Considerations:**
- Use `#[command(version)]` to auto-generate version from Cargo.toml
- Use `#[arg(global = true)]` for flags that apply to all subcommands
- Return `Result<()>` from all command handlers for error propagation
- Commands are stubs for now - will be implemented in later milestones

**Testing:**
- Run `cargo test` - all CLI tests should pass
- Manually test each command with `cargo run --`
- Verify help text is clear and accurate

**Dependencies:**
- Requires: M1.1 (project structure)

**Enables:**
- M1.3 (can add config loading to status command)
- M1.4 (can add version file detection to activate command)

---

### Task M1.3: Implement configuration system

**Objective:** Create a configuration system that loads settings from multiple sources (defaults → ~/.xvnrc → .xvn.yaml) with proper precedence and validation.

**Implementation Steps:**

1. **Create `src/config/schema.rs` (configuration data structures):**
   ```rust
   use serde::{Deserialize, Serialize};

   /// Main configuration structure
   #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
   #[serde(default)]
   pub struct Config {
       /// Version manager plugins in priority order
       pub plugins: Vec<String>,

       /// Auto-install behavior: "prompt", "always", "never"
       pub auto_install: AutoInstallMode,

       /// Version files to search for (in priority order)
       pub version_files: Vec<String>,
   }

   #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
   #[serde(rename_all = "lowercase")]
   pub enum AutoInstallMode {
       Prompt,
       Always,
       Never,
   }

   impl Default for Config {
       fn default() -> Self {
           Self {
               plugins: vec!["nvm".to_string(), "fnm".to_string()],
               auto_install: AutoInstallMode::Prompt,
               version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
           }
       }
   }

   impl Config {
       /// Validate configuration values
       pub fn validate(&self) -> Result<(), String> {
           if self.plugins.is_empty() {
               return Err("at least one plugin must be configured".to_string());
           }

           if self.version_files.is_empty() {
               return Err("at least one version file must be configured".to_string());
           }

           Ok(())
       }
   }
   ```

2. **Create `src/config/loader.rs` (configuration loading logic):**
   ```rust
   use super::schema::Config;
   use anyhow::{Context, Result};
   use log::{debug, warn};
   use std::fs;
   use std::path::{Path, PathBuf};

   impl Config {
       /// Load configuration from all sources with proper precedence
       /// Precedence: project config > user config > defaults
       pub fn load() -> Result<Self> {
           debug!("Loading configuration");

           let mut config = Self::default();
           debug!("Using default config: {:?}", config);

           // 1. Load user config: ~/.xvnrc
           if let Some(user_config) = Self::load_user_config()? {
               debug!("Merging user config: {:?}", user_config);
               config = config.merge(user_config);
           }

           // 2. Load project config: walk up from cwd to find .xvn.yaml
           if let Some(project_config) = Self::load_project_config()? {
               debug!("Merging project config: {:?}", project_config);
               config = config.merge(project_config);
           }

           // 3. Validate final configuration
           config.validate()
               .context("invalid configuration")?;

           debug!("Final config: {:?}", config);
           Ok(config)
       }

       /// Load user configuration from ~/.xvnrc
       fn load_user_config() -> Result<Option<Self>> {
           let home = dirs::home_dir()
               .ok_or_else(|| anyhow::anyhow!("cannot determine home directory"))?;

           let path = home.join(".xvnrc");

           if !path.exists() {
               debug!("User config not found at {:?}", path);
               return Ok(None);
           }

           debug!("Loading user config from {:?}", path);
           Self::load_from_file(&path).map(Some)
       }

       /// Load project configuration from .xvn.yaml (walk up directory tree)
       fn load_project_config() -> Result<Option<Self>> {
           let start_dir = std::env::current_dir()
               .context("failed to get current directory")?;

           let config_path = Self::find_project_config(&start_dir)?;

           if let Some(path) = config_path {
               debug!("Loading project config from {:?}", path);
               Self::load_from_file(&path).map(Some)
           } else {
               debug!("No project config found");
               Ok(None)
           }
       }

       /// Find .xvn.yaml by walking up directory tree (stop at HOME)
       fn find_project_config(start_dir: &Path) -> Result<Option<PathBuf>> {
           let home = dirs::home_dir().unwrap_or_default();
           let mut dir = start_dir.to_path_buf();

           loop {
               let config_path = dir.join(".xvn.yaml");

               if config_path.exists() && config_path.is_file() {
                   return Ok(Some(config_path));
               }

               // Stop at home directory
               if dir == home {
                   break;
               }

               // Move up one directory
               if !dir.pop() {
                   break;
               }
           }

           Ok(None)
       }

       /// Load configuration from a YAML file
       fn load_from_file(path: &Path) -> Result<Self> {
           let content = fs::read_to_string(path)
               .with_context(|| format!("failed to read config file: {}", path.display()))?;

           let config: Self = serde_yaml::from_str(&content)
               .with_context(|| format!("failed to parse config file: {}", path.display()))?;

           config.validate()
               .with_context(|| format!("invalid config in file: {}", path.display()))?;

           Ok(config)
       }

       /// Merge another config into self (other takes precedence)
       fn merge(mut self, other: Self) -> Self {
           // Only override if other has non-default values
           if !other.plugins.is_empty() {
               self.plugins = other.plugins;
           }

           // auto_install always overrides (even if set to Prompt in override config)
           // This allows project configs to explicitly set "prompt" behavior
           // even when user config has "always" or "never"
           self.auto_install = other.auto_install;

           if !other.version_files.is_empty() {
               self.version_files = other.version_files;
           }

           self
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       use std::env;
       use tempfile::tempdir;

       #[test]
       fn test_default_config() {
           let config = Config::default();
           assert_eq!(config.plugins, vec!["nvm", "fnm"]);
           assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
       }

       #[test]
       fn test_merge_configs() {
           let base = Config::default();
           let override_config = Config {
               plugins: vec!["fnm".to_string()],
               ..Config::default()
           };

           let merged = base.merge(override_config);
           assert_eq!(merged.plugins, vec!["fnm"]);
       }
   }
   ```

3. **Create `src/config/mod.rs` (module exports):**
   ```rust
   mod schema;
   mod loader;

   pub use schema::{Config, AutoInstallMode};
   ```

4. **Update `src/cli.rs` to use config in status command:**
   ```rust
   // In the Status command handler:
   Some(Commands::Status) => {
       info!("Running status command");

       match crate::config::Config::load() {
           Ok(config) => {
               println!("xvn status:");
               println!("  Plugins: {}", config.plugins.join(", "));
               println!("  Auto-install: {:?}", config.auto_install);
               println!("  Version files: {}", config.version_files.join(", "));
           }
           Err(e) => {
               eprintln!("Error loading config: {}", e);
               std::process::exit(1);
           }
       }
       Ok(())
   }
   ```

5. **Create unit tests in `tests/config_test.rs`:**
   ```rust
   use xvn::config::{Config, AutoInstallMode};
   use tempfile::tempdir;
   use std::fs;
   use std::env;

   #[test]
   fn test_load_default_config() {
       let config = Config::default();
       assert!(config.validate().is_ok());
       assert_eq!(config.plugins.len(), 2);
   }

   #[test]
   fn test_load_user_config() {
       let temp_dir = tempdir().unwrap();
       let config_path = temp_dir.path().join(".xvnrc");

       fs::write(&config_path, r#"
   plugins:
     - fnm
     - nvm
   auto_install: always
   version_files:
     - .nvmrc
   "#).unwrap();

       env::set_var("HOME", temp_dir.path());
       let config = Config::load().unwrap();

       assert_eq!(config.plugins, vec!["fnm", "nvm"]);
       assert_eq!(config.auto_install, AutoInstallMode::Always);
   }

   #[test]
   fn test_project_config_precedence() {
       let temp_dir = tempdir().unwrap();

       // Create user config
       let user_config = temp_dir.path().join(".xvnrc");
       fs::write(&user_config, r#"
   plugins:
     - nvm
   "#).unwrap();

       // Create project config
       let project_config = temp_dir.path().join(".xvn.yaml");
       fs::write(&project_config, r#"
   plugins:
     - fnm
   "#).unwrap();

       env::set_var("HOME", temp_dir.path());
       env::set_current_dir(temp_dir.path()).unwrap();

       let config = Config::load().unwrap();

       // Project config should override user config
       assert_eq!(config.plugins, vec!["fnm"]);
   }

   #[test]
   fn test_invalid_config() {
       let temp_dir = tempdir().unwrap();
       let config_path = temp_dir.path().join(".xvnrc");

       fs::write(&config_path, r#"
   plugins: []
   "#).unwrap();

       env::set_var("HOME", temp_dir.path());
       let result = Config::load();

       assert!(result.is_err());
       assert!(result.unwrap_err().to_string().contains("at least one plugin"));
   }
   ```

6. **Test the configuration system:**
   ```bash
   # Create test config
   echo "plugins:\n  - fnm" > ~/.xvnrc

   # Run status command
   cargo run -- status

   # Run tests
   cargo test config
   ```

**Code Structure:**
- File: `src/config/schema.rs` - Config struct, AutoInstallMode enum, validation
- File: `src/config/loader.rs` - Config loading logic, file parsing, merging
- File: `src/config/mod.rs` - Module exports
- File: `tests/config_test.rs` - Unit tests for config system

**Key Considerations:**
- Use `#[serde(default)]` to apply Default trait when fields are missing
- Validate config after loading to catch errors early
- Use `anyhow::Context` to add context to errors
- Stop directory traversal at HOME to avoid scanning entire filesystem
- Project config (.xvn.yaml) overrides user config (~/.xvnrc) overrides defaults

**Testing:**
- Test default config values
- Test loading from file
- Test config precedence (project > user > default)
- Test invalid configs (empty plugins, etc.)
- Test missing config files (should use defaults)

**Dependencies:**
- Requires: M1.1 (project structure), M1.2 (CLI framework)

**Enables:**
- M1.4 (version file detection can use config.version_files)
- Future milestones (plugin system uses config.plugins)

---

### Task M1.4: Implement version file detection

**Objective:** Create a system to discover version files (.nvmrc, .node-version) by walking up the directory tree from the current location, stopping at the HOME directory.

**Implementation Steps:**

1. **Create `src/version_file/finder.rs`:**
   ```rust
   use anyhow::{Context, Result};
   use log::{debug, trace};
   use std::fs;
   use std::path::{Path, PathBuf};

   /// Represents a discovered version file
   #[derive(Debug, Clone, PartialEq)]
   pub struct VersionFile {
       /// Absolute path to the version file
       pub path: PathBuf,

       /// Node.js version string (e.g., "18.20.0", "lts/hydrogen")
       pub version: String,
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
           debug!("Searching for version file in {:?}", start_dir);
           debug!("Looking for: {:?}", filenames);

           let home = dirs::home_dir().unwrap_or_default();
           let mut dir = start_dir.to_path_buf();

           // Ensure we have an absolute path
           if dir.is_relative() {
               dir = std::env::current_dir()?.join(&dir);
           }
           dir = dir.canonicalize()
               .context("failed to canonicalize start directory")?;

           loop {
               trace!("Checking directory: {:?}", dir);

               // Try each filename in priority order
               for filename in filenames {
                   let file_path = dir.join(filename);

                   if file_path.exists() && file_path.is_file() {
                       debug!("Found version file: {:?}", file_path);

                       let version = Self::parse(&file_path)
                           .with_context(|| format!("failed to parse version file: {}", file_path.display()))?;

                       return Ok(Some(Self {
                           path: file_path,
                           version,
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

           anyhow::bail!("version file is empty or contains only comments: {}", path.display())
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       use tempfile::tempdir;
       use std::fs;

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

           let result = VersionFile::find(
               temp_dir.path(),
               &[".nvmrc".to_string()]
           ).unwrap();

           assert!(result.is_some());
           let vf = result.unwrap();
           assert_eq!(vf.version, "18.20.0");
           assert_eq!(vf.path, version_file);
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
           let result = VersionFile::find(
               &subdir,
               &[".nvmrc".to_string()]
           ).unwrap();

           assert!(result.is_some());
           let vf = result.unwrap();
           assert_eq!(vf.version, "18.20.0");
       }

       #[test]
       fn test_find_no_version_file() {
           let temp_dir = tempdir().unwrap();

           let result = VersionFile::find(
               temp_dir.path(),
               &[".nvmrc".to_string()]
           ).unwrap();

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
               &[".nvmrc".to_string(), ".node-version".to_string()]
           ).unwrap();

           assert!(result.is_some());
           assert_eq!(result.unwrap().version, "18.20.0");
       }
   }
   ```

2. **Create `src/version_file/mod.rs`:**
   ```rust
   mod finder;

   pub use finder::VersionFile;
   ```

3. **Update `src/cli.rs` to use version file detection in activate command:**
   ```rust
   // In the Activate command handler:
   Some(Commands::Activate { path }) => {
       info!("Running activate command for path: {:?}", path);

       // Load config to get version file names
       let config = crate::config::Config::load()
           .context("failed to load configuration")?;

       // Find version file
       match crate::version_file::VersionFile::find(&path, &config.version_files) {
           Ok(Some(version_file)) => {
               println!("Found version file: {}", version_file.path.display());
               println!("Node.js version: {}", version_file.version);
               println!("\nActivation not yet implemented (requires plugin system)");
           }
           Ok(None) => {
               println!("No version file found in {} or parent directories", path.display());
               std::process::exit(1);
           }
           Err(e) => {
               eprintln!("Error: {}", e);
               std::process::exit(1);
           }
       }

       Ok(())
   }
   ```

4. **Create integration test `tests/version_file_test.rs`:**
   ```rust
   use xvn::version_file::VersionFile;
   use tempfile::tempdir;
   use std::fs;

   #[test]
   fn test_find_version_file_basic() {
       let temp_dir = tempdir().unwrap();
       let version_file = temp_dir.path().join(".nvmrc");
       fs::write(&version_file, "18.20.0").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert!(result.is_some());
       assert_eq!(result.unwrap().version, "18.20.0");
   }

   #[test]
   fn test_find_walks_up_tree() {
       let temp_dir = tempdir().unwrap();

       // Create nested structure: temp/project/.nvmrc, temp/project/src/
       let project_dir = temp_dir.path().join("project");
       let src_dir = project_dir.join("src");
       fs::create_dir_all(&src_dir).unwrap();

       let version_file = project_dir.join(".nvmrc");
       fs::write(&version_file, "20.0.0").unwrap();

       // Search from src_dir, should find parent's .nvmrc
       let result = VersionFile::find(
           &src_dir,
           &[".nvmrc".to_string()]
       ).unwrap();

       assert!(result.is_some());
       let vf = result.unwrap();
       assert_eq!(vf.version, "20.0.0");
       assert_eq!(vf.path, version_file);
   }

   #[test]
   fn test_multiple_version_file_types() {
       let temp_dir = tempdir().unwrap();

       // Only create .node-version
       fs::write(temp_dir.path().join(".node-version"), "18.20.0").unwrap();

       // Should find .node-version when .nvmrc doesn't exist
       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string(), ".node-version".to_string()]
       ).unwrap();

       assert!(result.is_some());
       assert_eq!(result.unwrap().version, "18.20.0");
   }
   ```

5. **Test the version file detection:**
   ```bash
   # Create test project structure
   mkdir -p /tmp/test-xvn/src
   echo "18.20.0" > /tmp/test-xvn/.nvmrc

   # Run activate from subdirectory
   cd /tmp/test-xvn/src
   cargo run -- activate .

   # Run tests
   cargo test version_file
   ```

**Code Structure:**
- File: `src/version_file/finder.rs` - VersionFile struct and find/parse logic
- File: `src/version_file/mod.rs` - Module exports
- File: `tests/version_file_test.rs` - Integration tests

**Key Considerations:**
- Use `canonicalize()` to convert relative paths to absolute paths
- Stop at HOME directory to avoid scanning entire filesystem
- Support comments in version files (lines starting with #)
- Trim whitespace from version strings
- Check file existence AND that it's a file (not a directory)
- Return `Option<VersionFile>` (None if not found, not an error)

**Testing:**
- Test parsing various version formats (semver, lts/name, etc.)
- Test parsing with whitespace and comments
- Test finding file in current directory
- Test walking up directory tree
- Test stopping at HOME directory
- Test priority order when multiple version file types exist

**Dependencies:**
- Requires: M1.1 (project structure), M1.3 (config for version_files list)

**Enables:**
- M2.4 (plugin system can use VersionFile to determine what to activate)

---

### Task M1.5: Set up error handling

**Objective:** Create a comprehensive error handling system using thiserror for structured errors and anyhow for context propagation.

**Implementation Steps:**

1. **Create `src/error.rs`:**
   ```rust
   use thiserror::Error;
   use std::path::PathBuf;

   /// Main error type for xvn
   #[derive(Debug, Error)]
   pub enum XvnError {
       /// No version file found in directory tree
       #[error("no version file found in {path} or parent directories")]
       NoVersionFile { path: PathBuf },

       /// Version file exists but is empty or invalid
       #[error("version file is empty or invalid: {path}")]
       VersionFileEmpty { path: PathBuf },

       /// Failed to read version file
       #[error("failed to read version file: {path}")]
       VersionFileUnreadable {
           path: PathBuf,
           #[source]
           source: std::io::Error,
       },

       /// Configuration file is invalid
       #[error("configuration error: {message}")]
       ConfigError { message: String },

       /// No version manager plugin is available
       #[error("no version manager plugin available (tried: {plugins})")]
       NoPluginAvailable { plugins: String },

       /// Plugin execution failed
       #[error("plugin '{plugin}' failed: {message}")]
       PluginError { plugin: String, message: String },

       /// IO error (generic wrapper)
       #[error("IO error: {0}")]
       Io(#[from] std::io::Error),

       /// YAML parsing error
       #[error("YAML parsing error: {0}")]
       Yaml(#[from] serde_yaml::Error),
   }

   /// Result type alias for xvn operations
   pub type Result<T> = std::result::Result<T, XvnError>;

   /// Helper trait for adding context to errors
   pub trait ErrorContext<T> {
       fn context(self, message: impl Into<String>) -> anyhow::Result<T>;
   }

   impl<T> ErrorContext<T> for Result<T> {
       fn context(self, message: impl Into<String>) -> anyhow::Result<T> {
           self.map_err(|e| anyhow::anyhow!("{}: {}", message.into(), e))
       }
   }
   ```

2. **Update `src/lib.rs` to export error types:**
   ```rust
   pub mod cli;
   pub mod config;
   pub mod error;
   pub mod version_file;

   // Re-export commonly used types
   pub use error::{XvnError, Result};
   pub use config::Config;
   pub use version_file::VersionFile;
   ```

3. **Update existing modules to use structured errors:**

   **In `src/version_file/finder.rs`:**
   ```rust
   // Change return type from anyhow::Result to crate::error::Result
   use crate::error::{XvnError, Result};

   // Update parse method to return structured errors:
   fn parse(path: &Path) -> Result<String> {
       let content = fs::read_to_string(path)
           .map_err(|e| XvnError::VersionFileUnreadable {
               path: path.to_path_buf(),
               source: e,
           })?;

       for line in content.lines() {
           let trimmed = line.trim();
           if trimmed.is_empty() || trimmed.starts_with('#') {
               continue;
           }
           return Ok(trimmed.to_string());
       }

       Err(XvnError::VersionFileEmpty {
           path: path.to_path_buf(),
       })
   }
   ```

   **In `src/config/loader.rs`:**
   ```rust
   // Add error conversion for validation errors
   impl Config {
       pub fn validate(&self) -> Result<(), XvnError> {
           if self.plugins.is_empty() {
               return Err(XvnError::ConfigError {
                   message: "at least one plugin must be configured".to_string(),
               });
           }

           if self.version_files.is_empty() {
               return Err(XvnError::ConfigError {
                   message: "at least one version file must be configured".to_string(),
               });
           }

           Ok(())
       }
   }
   ```

4. **Create error handling tests in `tests/error_test.rs`:**
   ```rust
   use xvn::error::XvnError;
   use std::path::PathBuf;

   #[test]
   fn test_no_version_file_error() {
       let error = XvnError::NoVersionFile {
           path: PathBuf::from("/tmp/test"),
       };

       let message = error.to_string();
       assert!(message.contains("no version file found"));
       assert!(message.contains("/tmp/test"));
   }

   #[test]
   fn test_config_error() {
       let error = XvnError::ConfigError {
           message: "invalid plugin name".to_string(),
       };

       let message = error.to_string();
       assert!(message.contains("configuration error"));
       assert!(message.contains("invalid plugin name"));
   }

   #[test]
   fn test_plugin_error() {
       let error = XvnError::PluginError {
           plugin: "nvm".to_string(),
           message: "not found".to_string(),
       };

       let message = error.to_string();
       assert!(message.contains("plugin 'nvm' failed"));
       assert!(message.contains("not found"));
   }
   ```

5. **Update CLI error handling in `src/cli.rs`:**
   ```rust
   // Update command handlers to provide user-friendly error messages
   pub fn run() -> Result<()> {
       let cli = Cli::parse();

       if cli.verbose {
           log::set_max_level(log::LevelFilter::Debug);
       }

       let result = match cli.command {
           Some(Commands::Setup { shell, force }) => {
               handle_setup(shell, force)
           }
           Some(Commands::Activate { path }) => {
               handle_activate(path)
           }
           Some(Commands::Status) => {
               handle_status()
           }
           None => {
               Cli::parse_from(&["xvn", "--help"]);
               Ok(())
           }
       };

       // Convert errors to user-friendly messages
       if let Err(e) = result {
           eprintln!("Error: {}", e);

           // Show error chain if verbose
           if cli.verbose {
               let mut source = e.source();
               while let Some(err) = source {
                   eprintln!("  Caused by: {}", err);
                   source = err.source();
               }
           } else {
               eprintln!("\nRun with --verbose for more details");
           }

           std::process::exit(1);
       }

       Ok(())
   }
   ```

6. **Test error handling:**
   ```bash
   # Test missing version file
   cargo run -- activate /tmp/no-version-file

   # Test invalid config
   echo "plugins: []" > ~/.xvnrc
   cargo run -- status

   # Test verbose error output
   cargo run -- --verbose activate /tmp/no-version-file

   # Run tests
   cargo test error
   ```

**Code Structure:**
- File: `src/error.rs` - XvnError enum, Result type alias, ErrorContext trait
- File: `tests/error_test.rs` - Error formatting tests
- Updated: All modules to use structured errors

**Key Considerations:**
- Use `thiserror::Error` for library errors (structured, typed)
- Use `anyhow::Result` in CLI code (convenient error propagation)
- Include context in errors (file paths, plugin names, etc.)
- Use `#[from]` for automatic error conversion
- Provide user-friendly error messages
- Show error chains with `--verbose` flag

**Testing:**
- Test error message formatting
- Test error context propagation
- Test user-facing error output
- Verify error types are correctly propagated

**Dependencies:**
- Requires: M1.1 (project structure)

**Enables:**
- All other tasks (provides robust error handling for the entire project)

---

### Task M1.6: Set up logging

**Objective:** Configure structured logging using env_logger to provide visibility into xvn's operations for debugging and troubleshooting.

**Implementation Steps:**

1. **Update `src/main.rs` with logging initialization:**
   ```rust
   use anyhow::Result;
   use log::{info, debug};

   fn main() -> Result<()> {
       // Initialize logger
       // RUST_LOG=debug xvn command    - Show debug logs
       // RUST_LOG=trace xvn command    - Show trace logs
       // RUST_LOG=info xvn command     - Show info logs (default)
       env_logger::Builder::from_env(
           env_logger::Env::default()
               .default_filter_or("info")
       )
       .format_timestamp(None)  // Disable timestamps for CLI output
       .format_module_path(false)  // Disable module paths
       .init();

       debug!("xvn version {}", env!("CARGO_PKG_VERSION"));
       debug!("Current directory: {:?}", std::env::current_dir()?);

       xvn::cli::run()
   }
   ```

2. **Add logging to configuration loading (`src/config/loader.rs`):**
   ```rust
   use log::{debug, info, trace, warn};

   impl Config {
       pub fn load() -> Result<Self> {
           info!("Loading configuration");

           let mut config = Self::default();
           debug!("Default config: {:?}", config);

           if let Some(user_config) = Self::load_user_config()? {
               info!("Loaded user config from ~/.xvnrc");
               debug!("User config: {:?}", user_config);
               config = config.merge(user_config);
           } else {
               debug!("No user config found at ~/.xvnrc");
           }

           if let Some(project_config) = Self::load_project_config()? {
               info!("Loaded project config from .xvn.yaml");
               debug!("Project config: {:?}", project_config);
               config = config.merge(project_config);
           } else {
               debug!("No project config found");
           }

           config.validate()
               .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))?;

           info!("Final configuration loaded");
           debug!("Config: {:?}", config);

           Ok(config)
       }

       fn load_from_file(path: &Path) -> Result<Self> {
           trace!("Reading config file: {:?}", path);

           let content = fs::read_to_string(path)
               .with_context(|| format!("failed to read config: {}", path.display()))?;

           trace!("Parsing YAML content ({} bytes)", content.len());

           let config: Self = serde_yaml::from_str(&content)
               .with_context(|| format!("failed to parse config: {}", path.display()))?;

           config.validate()
               .with_context(|| format!("invalid config: {}", path.display()))?;

           Ok(config)
       }
   }
   ```

3. **Add logging to version file detection (`src/version_file/finder.rs`):**
   ```rust
   use log::{debug, trace, warn};

   impl VersionFile {
       pub fn find(start_dir: &Path, filenames: &[String]) -> Result<Option<Self>> {
           info!("Searching for version file starting from {:?}", start_dir);
           debug!("Version file names: {:?}", filenames);

           let home = dirs::home_dir().unwrap_or_default();
           let mut dir = start_dir.to_path_buf();

           if dir.is_relative() {
               dir = std::env::current_dir()?.join(&dir);
           }
           dir = dir.canonicalize()
               .context("failed to canonicalize start directory")?;

           trace!("Canonicalized start directory: {:?}", dir);
           trace!("Home directory: {:?}", home);

           loop {
               trace!("Checking directory: {:?}", dir);

               for filename in filenames {
                   let file_path = dir.join(filename);
                   trace!("Checking for file: {:?}", file_path);

                   if file_path.exists() && file_path.is_file() {
                       info!("Found version file: {:?}", file_path);

                       let version = Self::parse(&file_path)
                           .with_context(|| format!("failed to parse: {}", file_path.display()))?;

                       debug!("Parsed version: {}", version);

                       return Ok(Some(Self {
                           path: file_path,
                           version,
                       }));
                   }
               }

               if dir == home {
                   debug!("Reached HOME directory, stopping search");
                   break;
               }

               if !dir.pop() {
                   debug!("Reached filesystem root, stopping search");
                   break;
               }
           }

           info!("No version file found");
           Ok(None)
       }

       fn parse(path: &Path) -> Result<String> {
           trace!("Parsing version file: {:?}", path);

           let content = fs::read_to_string(path)
               .map_err(|e| XvnError::VersionFileUnreadable {
                   path: path.to_path_buf(),
                   source: e,
               })?;

           trace!("File content ({} bytes)", content.len());

           for (i, line) in content.lines().enumerate() {
               let trimmed = line.trim();
               trace!("Line {}: {:?}", i, trimmed);

               if trimmed.is_empty() || trimmed.starts_with('#') {
                   trace!("Skipping empty or comment line");
                   continue;
               }

               debug!("Found version on line {}: {}", i, trimmed);
               return Ok(trimmed.to_string());
           }

           warn!("Version file is empty or contains only comments: {:?}", path);
           Err(XvnError::VersionFileEmpty {
               path: path.to_path_buf(),
           })
       }
   }
   ```

4. **Add logging to CLI commands (`src/cli.rs`):**
   ```rust
   use log::{debug, info, warn};

   pub fn run() -> Result<()> {
       let cli = Cli::parse();

       // Update log level based on verbose flag
       if cli.verbose {
           log::set_max_level(log::LevelFilter::Debug);
           debug!("Verbose mode enabled");
       }

       debug!("Parsed CLI args: {:?}", cli);

       match cli.command {
           Some(Commands::Setup { shell, force }) => {
               info!("Running setup command");
               debug!("Shell: {}, Force: {}", shell, force);
               // ... rest of handler
           }
           Some(Commands::Activate { path }) => {
               info!("Running activate command");
               debug!("Path: {:?}", path);
               // ... rest of handler
           }
           Some(Commands::Status) => {
               info!("Running status command");
               // ... rest of handler
           }
           None => {
               debug!("No subcommand provided, showing help");
               Cli::parse_from(&["xvn", "--help"]);
               Ok(())
           }
       }
   }
   ```

5. **Create logging test helper in `tests/common/mod.rs`:**
   ```rust
   use std::sync::Once;

   static INIT: Once = Once::new();

   /// Initialize logging for tests (call once per test)
   pub fn init_test_logging() {
       INIT.call_once(|| {
           env_logger::Builder::from_env(
               env_logger::Env::default()
                   .default_filter_or("debug")
           )
           .is_test(true)
           .try_init()
           .ok();
       });
   }
   ```

6. **Test logging output:**
   ```bash
   # Default (info level)
   cargo run -- status

   # Debug level
   RUST_LOG=debug cargo run -- status

   # Trace level (very verbose)
   RUST_LOG=trace cargo run -- activate .

   # Module-specific logging
   RUST_LOG=xvn::config=debug cargo run -- status

   # Verbose flag
   cargo run -- --verbose status
   ```

**Code Structure:**
- File: `src/main.rs` - Logger initialization
- File: `tests/common/mod.rs` - Test logging helper
- Updated: All modules with appropriate log statements

**Key Considerations:**
- Use log levels appropriately:
  - `error!()` - Critical failures
  - `warn!()` - Warnings that don't stop execution
  - `info!()` - High-level operation info (default)
  - `debug!()` - Detailed debugging info (--verbose)
  - `trace!()` - Very detailed tracing (RUST_LOG=trace)
- Disable timestamps and module paths for cleaner CLI output
- Support both RUST_LOG env var and --verbose flag
- Include context in log messages (paths, versions, etc.)
- Log both inputs and outputs of key operations

**Testing:**
- Run commands with different log levels
- Verify log messages are clear and helpful
- Check that sensitive info is not logged
- Test that --verbose flag works correctly

**Dependencies:**
- Requires: M1.1 (project structure)

**Enables:**
- Debugging and troubleshooting throughout development
- Production logging for issue diagnosis

---

### Task M1.7: Unit tests for core infrastructure

**Objective:** Achieve >80% code coverage with comprehensive unit and integration tests for all core infrastructure components.

**Implementation Steps:**

1. **Set up test infrastructure in `Cargo.toml`:**
   ```toml
   [dev-dependencies]
   tempfile = "3.10"
   assert_cmd = "2.0"
   predicates = "3.1"
   serial_test = "3.0"  # For tests that need to run serially

   [[test]]
   name = "integration"
   path = "tests/integration.rs"
   ```

2. **Create test helper module for shared test utilities:**

   Create `tests/common/mod.rs`:
   ```rust
   use std::sync::Once;

   static INIT: Once = Once::new();

   /// Initialize logging for tests (call once per test that needs logging)
   pub fn init_test_logging() {
       INIT.call_once(|| {
           env_logger::Builder::from_env(
               env_logger::Env::default()
                   .default_filter_or("debug")
           )
           .is_test(true)
           .try_init()
           .ok();
       });
   }
   ```

   **Note:** Use `#[serial]` attribute from `serial_test` crate for tests that modify global state like environment variables (HOME, CWD). This ensures they don't run in parallel and interfere with each other.

3. **Create comprehensive config tests in `tests/config_test.rs`:**
   ```rust
   use xvn::config::{Config, AutoInstallMode};
   use tempfile::tempdir;
   use std::fs;
   use std::env;
   use serial_test::serial;

   #[test]
   fn test_default_config() {
       let config = Config::default();
       assert_eq!(config.plugins, vec!["nvm", "fnm"]);
       assert_eq!(config.auto_install, AutoInstallMode::Prompt);
       assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
       assert!(config.validate().is_ok());
   }

   #[test]
   #[serial]
   fn test_load_user_config() {
       let temp_dir = tempdir().unwrap();
       let config_path = temp_dir.path().join(".xvnrc");

       fs::write(&config_path, r#"
   plugins:
     - fnm
     - nvm
   auto_install: always
   version_files:
     - .nvmrc
   "#).unwrap();

       env::set_var("HOME", temp_dir.path());
       let config = Config::load().unwrap();

       assert_eq!(config.plugins, vec!["fnm", "nvm"]);
       assert_eq!(config.auto_install, AutoInstallMode::Always);
       assert_eq!(config.version_files, vec![".nvmrc"]);
   }

   #[test]
   #[serial]
   fn test_project_config_overrides_user_config() {
       let temp_dir = tempdir().unwrap();

       // User config
       let user_config = temp_dir.path().join(".xvnrc");
       fs::write(&user_config, r#"
   plugins:
     - nvm
   auto_install: never
   "#).unwrap();

       // Project config
       let project_dir = temp_dir.path().join("project");
       fs::create_dir(&project_dir).unwrap();
       let project_config = project_dir.join(".xvn.yaml");
       fs::write(&project_config, r#"
   plugins:
     - fnm
   auto_install: always
   "#).unwrap();

       env::set_var("HOME", temp_dir.path());
       env::set_current_dir(&project_dir).unwrap();

       let config = Config::load().unwrap();

       // Project config should win
       assert_eq!(config.plugins, vec!["fnm"]);
       assert_eq!(config.auto_install, AutoInstallMode::Always);
   }

   #[test]
   fn test_invalid_config_empty_plugins() {
       let temp_dir = tempdir().unwrap();
       let config_path = temp_dir.path().join(".xvnrc");

       fs::write(&config_path, "plugins: []").unwrap();

       env::set_var("HOME", temp_dir.path());
       let result = Config::load();

       assert!(result.is_err());
       assert!(result.unwrap_err().to_string().contains("at least one plugin"));
   }

   #[test]
   fn test_invalid_yaml() {
       let temp_dir = tempdir().unwrap();
       let config_path = temp_dir.path().join(".xvnrc");

       fs::write(&config_path, "invalid: yaml: content:").unwrap();

       env::set_var("HOME", temp_dir.path());
       let result = Config::load();

       assert!(result.is_err());
   }

   #[test]
   fn test_merge_preserves_defaults_when_not_overridden() {
       let base = Config::default();
       let partial = Config {
           plugins: vec!["fnm".to_string()],
           ..Default::default()
       };

       let merged = base.merge(partial);

       assert_eq!(merged.plugins, vec!["fnm"]);
       // Version files should be preserved from base
       assert_eq!(merged.version_files, vec![".nvmrc", ".node-version"]);
   }
   ```

3. **Create comprehensive version file tests in `tests/version_file_test.rs`:**
   ```rust
   use xvn::version_file::VersionFile;
   use tempfile::tempdir;
   use std::fs;

   #[test]
   fn test_find_in_current_directory() {
       let temp_dir = tempdir().unwrap();
       let version_file = temp_dir.path().join(".nvmrc");
       fs::write(&version_file, "18.20.0").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert!(result.is_some());
       let vf = result.unwrap();
       assert_eq!(vf.version, "18.20.0");
       assert_eq!(vf.path, version_file);
   }

   #[test]
   fn test_find_in_parent_directory() {
       let temp_dir = tempdir().unwrap();

       // Create nested structure
       let project_dir = temp_dir.path().join("project");
       let src_dir = project_dir.join("src");
       let nested_dir = src_dir.join("components");
       fs::create_dir_all(&nested_dir).unwrap();

       let version_file = project_dir.join(".nvmrc");
       fs::write(&version_file, "20.0.0").unwrap();

       // Search from deeply nested directory
       let result = VersionFile::find(
           &nested_dir,
           &[".nvmrc".to_string()]
       ).unwrap();

       assert!(result.is_some());
       assert_eq!(result.unwrap().version, "20.0.0");
   }

   #[test]
   fn test_find_returns_none_when_not_found() {
       let temp_dir = tempdir().unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert!(result.is_none());
   }

   #[test]
   fn test_priority_order() {
       let temp_dir = tempdir().unwrap();

       // Create both files
       fs::write(temp_dir.path().join(".nvmrc"), "18.20.0").unwrap();
       fs::write(temp_dir.path().join(".node-version"), "20.0.0").unwrap();

       // .nvmrc should be found first
       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string(), ".node-version".to_string()]
       ).unwrap();

       assert_eq!(result.unwrap().version, "18.20.0");

       // Swap priority
       let result = VersionFile::find(
           temp_dir.path(),
           &[".node-version".to_string(), ".nvmrc".to_string()]
       ).unwrap();

       assert_eq!(result.unwrap().version, "20.0.0");
   }

   #[test]
   fn test_parse_with_whitespace() {
       let temp_dir = tempdir().unwrap();
       let file_path = temp_dir.path().join(".nvmrc");
       fs::write(&file_path, "  18.20.0  \n\n  ").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert_eq!(result.unwrap().version, "18.20.0");
   }

   #[test]
   fn test_parse_with_comments() {
       let temp_dir = tempdir().unwrap();
       let file_path = temp_dir.path().join(".nvmrc");
       fs::write(&file_path, "# Node.js version\n18.20.0\n# End").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert_eq!(result.unwrap().version, "18.20.0");
   }

   #[test]
   fn test_parse_lts_version() {
       let temp_dir = tempdir().unwrap();
       let file_path = temp_dir.path().join(".nvmrc");
       fs::write(&file_path, "lts/hydrogen").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       ).unwrap();

       assert_eq!(result.unwrap().version, "lts/hydrogen");
   }

   #[test]
   fn test_parse_empty_file_returns_error() {
       let temp_dir = tempdir().unwrap();
       let file_path = temp_dir.path().join(".nvmrc");
       fs::write(&file_path, "").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       );

       assert!(result.is_err());
   }

   #[test]
   fn test_parse_only_comments_returns_error() {
       let temp_dir = tempdir().unwrap();
       let file_path = temp_dir.path().join(".nvmrc");
       fs::write(&file_path, "# Comment 1\n# Comment 2").unwrap();

       let result = VersionFile::find(
           temp_dir.path(),
           &[".nvmrc".to_string()]
       );

       assert!(result.is_err());
   }
   ```

4. **Create CLI integration tests in `tests/integration.rs`:**
   ```rust
   use assert_cmd::Command;
   use predicates::prelude::*;
   use tempfile::tempdir;
   use std::fs;

   #[test]
   fn test_version_command() {
       Command::cargo_bin("xvn")
           .unwrap()
           .arg("--version")
           .assert()
           .success()
           .stdout(predicate::str::contains("xvn 0.1.0"));
   }

   #[test]
   fn test_help_command() {
       Command::cargo_bin("xvn")
           .unwrap()
           .arg("--help")
           .assert()
           .success()
           .stdout(predicate::str::contains("Extreme Version Switcher"))
           .stdout(predicate::str::contains("setup"))
           .stdout(predicate::str::contains("activate"))
           .stdout(predicate::str::contains("status"));
   }

   #[test]
   fn test_activate_with_version_file() {
       let temp_dir = tempdir().unwrap();
       let version_file = temp_dir.path().join(".nvmrc");
       fs::write(&version_file, "18.20.0").unwrap();

       Command::cargo_bin("xvn")
           .unwrap()
           .arg("activate")
           .arg(temp_dir.path())
           .assert()
           .success()
           .stdout(predicate::str::contains("18.20.0"));
   }

   #[test]
   fn test_activate_without_version_file() {
       let temp_dir = tempdir().unwrap();

       Command::cargo_bin("xvn")
           .unwrap()
           .arg("activate")
           .arg(temp_dir.path())
           .assert()
           .failure()
           .stdout(predicate::str::contains("No version file found"));
   }

   #[test]
   fn test_status_shows_config() {
       Command::cargo_bin("xvn")
           .unwrap()
           .arg("status")
           .assert()
           .success()
           .stdout(predicate::str::contains("Plugins:"))
           .stdout(predicate::str::contains("nvm"));
   }

   #[test]
   fn test_verbose_flag() {
       Command::cargo_bin("xvn")
           .unwrap()
           .arg("--verbose")
           .arg("status")
           .assert()
           .success();
   }
   ```

5. **Create error handling tests in `tests/error_test.rs`:**
   ```rust
   use xvn::error::XvnError;
   use std::path::PathBuf;

   #[test]
   fn test_error_display() {
       let error = XvnError::NoVersionFile {
           path: PathBuf::from("/tmp/test"),
       };

       let message = error.to_string();
       assert!(message.contains("no version file found"));
       assert!(message.contains("/tmp/test"));
   }

   #[test]
   fn test_config_error_display() {
       let error = XvnError::ConfigError {
           message: "test error".to_string(),
       };

       assert_eq!(error.to_string(), "configuration error: test error");
   }

   #[test]
   fn test_plugin_error_display() {
       let error = XvnError::PluginError {
           plugin: "nvm".to_string(),
           message: "not found".to_string(),
       };

       let message = error.to_string();
       assert!(message.contains("plugin 'nvm' failed"));
       assert!(message.contains("not found"));
   }
   ```

6. **Generate coverage report:**
   ```bash
   # Install tarpaulin
   cargo install cargo-tarpaulin

   # Generate coverage
   cargo tarpaulin --out Lcov --output-dir coverage

   # View coverage report
   # Open coverage/lcov.info in your IDE or coverage viewer
   ```

7. **Create test runner script `scripts/test.sh`:**
   ```bash
   #!/bin/bash
   set -e

   echo "Running unit tests..."
   cargo test --lib

   echo "Running integration tests..."
   cargo test --test '*'

   echo "Running clippy..."
   cargo clippy -- -D warnings

   echo "Checking formatting..."
   cargo fmt -- --check

   echo "Generating coverage report..."
   cargo tarpaulin --out Lcov --output-dir coverage

   echo "All tests passed!"
   ```

**Code Structure:**
- File: `tests/config_test.rs` - Config system tests
- File: `tests/version_file_test.rs` - Version file detection tests
- File: `tests/integration.rs` - End-to-end CLI tests
- File: `tests/error_test.rs` - Error handling tests
- File: `scripts/test.sh` - Test runner script

**Key Considerations:**
- Use `serial_test` for tests that modify global state (env vars)
- Use `tempfile` for creating test directories and files
- Use `assert_cmd` for testing CLI behavior
- Test both success and failure cases
- Test edge cases (empty files, missing files, permissions, etc.)
- Aim for >80% code coverage
- Run tests with `cargo test --all-features`

**Testing:**
```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test config_test

# Run with output
cargo test -- --nocapture

# Run coverage
cargo tarpaulin --out Html --output-dir coverage

# Check coverage threshold
cargo tarpaulin --fail-under 80
```

**Dependencies:**
- Requires: M1.1, M1.2, M1.3, M1.4, M1.5, M1.6 (all previous tasks)

**Enables:**
- Confidence in code correctness
- Regression prevention
- Documentation through tests

---

## Integration Points

**Configuration + Version File Detection:**
- Config provides `version_files` list to VersionFile::find()
- Status command displays loaded configuration

**CLI + Configuration + Version File:**
- Activate command loads config and searches for version files
- Status command displays current configuration
- Verbose flag increases logging throughout the stack

**Error Handling:**
- All modules return structured errors via XvnError
- CLI converts errors to user-friendly messages
- Verbose flag shows error chains

**Logging:**
- All operations log their progress
- RUST_LOG env var controls log level
- Verbose flag enables debug logging

## Testing Strategy

**Unit Tests:**
- Test each module in isolation
- Mock file system operations with tempfile
- Test error conditions and edge cases
- Target >80% line coverage

**Integration Tests:**
- Test CLI commands end-to-end
- Test config loading from real files
- Test version file discovery in real directory trees
- Use assert_cmd for CLI testing

**Manual Testing:**
- Create test projects with version files
- Test walking up directory trees
- Test config precedence with multiple files
- Test error messages are clear

## Success Criteria

✅ `cargo build --release` completes without errors
✅ `cargo test` passes all tests (unit + integration)
✅ `cargo tarpaulin` shows >80% coverage
✅ `cargo clippy` passes with no warnings
✅ `cargo fmt --check` passes
✅ `xvn --version` returns correct version
✅ `xvn --help` shows clear usage information
✅ `xvn status` displays configuration correctly
✅ `xvn activate <path>` finds version files by walking up tree
✅ Config loaded from multiple sources with correct precedence
✅ Error messages are user-friendly
✅ Logging provides visibility into operations

## Next Steps

After completing Milestone 1, you'll be ready to move on to Milestone 2 (Plugin System), which will:
- Define the VersionManagerPlugin trait
- Implement nvm plugin
- Implement fnm plugin
- Add plugin loading and management
