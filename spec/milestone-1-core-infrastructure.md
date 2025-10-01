# Milestone 1: Core Infrastructure

**Timeline:** Weeks 1-2
**Status:** Planning
**Version:** v0.1.0

---

## Plan

### Goal

Establish foundational Rust project structure with CLI framework, configuration system, and version file detection capabilities.

### Deliverables

- [ ] Rust project structure with Cargo.toml
- [ ] CLI framework with clap (setup, version, help commands)
- [ ] Configuration system (parse ~/.xvnrc YAML)
- [ ] Version file detection (.nvmrc, .node-version)
- [ ] Directory traversal logic (walk up to HOME)
- [ ] Basic error handling and logging

### Key Decisions

**CLI Framework:** `clap` v4
- Declarative syntax with derive macros
- Generates help automatically
- Well-maintained and widely adopted

**Config Parsing:** `serde_yaml`
- Standard Rust YAML parsing
- Integrates with serde ecosystem
- Good error messages

**File I/O:** `std::fs`
- Sufficient for MVP
- No async complexity needed
- Synchronous operations faster for small files

**Logging:** `env_logger`
- Simple and standard
- Controlled via RUST_LOG environment variable

### Testing

**Unit Tests:**
- Config parsing (valid, invalid, missing files)
- Version file discovery (nested dirs, stop at HOME)
- Integration tests for CLI argument parsing

**Success Criteria:**
- `xvn --version` returns correct version
- Config file parsed correctly with defaults
- Version file found in parent directories
- Tests passing with >80% coverage

---

## Architecture

### Module Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── cli.rs               # Command-line interface
├── config/
│   ├── mod.rs           # Config module exports
│   ├── schema.rs        # Config data structures
│   └── loader.rs        # Config loading logic
└── version_file/
    ├── mod.rs           # Version file module exports
    ├── finder.rs        # Directory traversal
    └── parser.rs        # File content parsing
```

### CLI Module (src/cli.rs)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xvn")]
#[command(about = "Extreme Version Switcher for Node.js")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install xvn shell hooks and create config
    Setup {
        /// Force reinstallation even if already set up
        #[arg(short, long)]
        force: bool,
    },

    /// Activate Node.js version from file
    Activate {
        /// Path to version file
        version_file: PathBuf,
    },

    /// Show current version and configuration
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup { force }) => {
            println!("Setup command (not yet implemented)");
            Ok(())
        }
        Some(Commands::Activate { version_file }) => {
            println!("Activate command (not yet implemented)");
            Ok(())
        }
        Some(Commands::Status) => {
            println!("Status command (not yet implemented)");
            Ok(())
        }
        None => {
            Cli::parse_from(&["xvn", "--help"]);
            Ok(())
        }
    }
}
```

### Configuration Module (src/config/schema.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Version manager plugins in priority order
    pub plugins: Vec<String>,

    /// Auto-install behavior: "prompt", "always", "never"
    pub auto_install: AutoInstallMode,

    /// Version files to search for
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
            plugins: vec!["nvm".into(), "fnm".into()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".into(), ".node-version".into()],
        }
    }
}
```

### Configuration Loader (src/config/loader.rs)

```rust
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use super::schema::Config;

impl Config {
    /// Load configuration from multiple sources
    pub fn load() -> Result<Self> {
        let mut config = Self::default();

        // 1. Load user config: ~/.xvnrc
        if let Some(user_config) = Self::load_user_config()? {
            config = config.merge(user_config);
        }

        // 2. Load project config: walk up from cwd to find .xvn.yaml
        if let Some(project_config) = Self::load_project_config()? {
            config = config.merge(project_config);
        }

        Ok(config)
    }

    fn load_user_config() -> Result<Option<Self>> {
        let path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("cannot determine home directory"))?
            .join(".xvnrc");

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(Some(config))
    }

    fn load_project_config() -> Result<Option<Self>> {
        let mut dir = std::env::current_dir()?;
        let home = dirs::home_dir().unwrap_or_default();

        loop {
            let config_path = dir.join(".xvn.yaml");
            if config_path.exists() {
                let content = fs::read_to_string(&config_path)?;
                let config: Self = serde_yaml::from_str(&content)?;
                return Ok(Some(config));
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

    /// Merge other config into self (other takes precedence)
    fn merge(mut self, other: Self) -> Self {
        if !other.plugins.is_empty() {
            self.plugins = other.plugins;
        }
        if other.auto_install != AutoInstallMode::Prompt {
            self.auto_install = other.auto_install;
        }
        if !other.version_files.is_empty() {
            self.version_files = other.version_files;
        }
        self
    }
}
```

### Version File Finder (src/version_file/finder.rs)

```rust
use std::path::{Path, PathBuf};
use anyhow::Result;

pub struct VersionFile {
    pub path: PathBuf,
    pub version: String,
}

impl VersionFile {
    /// Find version file by walking up directory tree
    pub fn find(start_dir: &Path, filenames: &[String]) -> Result<Option<Self>> {
        let mut dir = start_dir.to_path_buf();
        let home = dirs::home_dir().unwrap_or_default();

        loop {
            // Try each filename in priority order
            for filename in filenames {
                let file_path = dir.join(filename);
                if file_path.exists() && file_path.is_file() {
                    let version = Self::parse(&file_path)?;
                    return Ok(Some(Self {
                        path: file_path,
                        version,
                    }));
                }
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

    /// Parse version from file (read first line, trim whitespace)
    fn parse(path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(path)?;
        
        let version = content
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        if version.is_empty() {
            anyhow::bail!("version file is empty: {}", path.display());
        }

        Ok(version)
    }
}
```

### Error Handling (src/error.rs)

```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum XvnError {
    #[error("no version file found")]
    NoVersionFile,

    #[error("version file is empty: {path}")]
    VersionFileEmpty { path: PathBuf },

    #[error("failed to read version file: {path}")]
    VersionFileUnreadable {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("configuration error: {message}")]
    ConfigError { message: String },

    #[error("no version manager available")]
    NoPluginAvailable,
}
```

### Dependencies (Cargo.toml)

```toml
[package]
name = "xvn"
version = "0.1.0"
edition = "2021"
authors = ["cameronolivier@gmail.com"]
description = "Extreme Version Switcher for Node.js"
license = "MIT"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
anyhow = "1"
thiserror = "1"
dirs = "5"
env_logger = "0.11"
log = "0.4"

[dev-dependencies]
tempfile = "3"
assert_cmd = "2"
predicates = "3"
```

### Example Test (tests/config_test.rs)

```rust
use xvn::config::Config;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_config_default_values() {
    let config = Config::default();
    assert_eq!(config.plugins, vec!["nvm", "fnm"]);
    assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
}

#[test]
fn test_config_load_user_config() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join(".xvnrc");
    
    fs::write(&config_path, r#"
plugins:
  - fnm
  - nvm
auto_install: always
"#).unwrap();

    std::env::set_var("HOME", temp_dir.path());
    let config = Config::load().unwrap();
    
    assert_eq!(config.plugins, vec!["fnm", "nvm"]);
}

#[test]
fn test_version_file_parsing() {
    let temp_dir = tempdir().unwrap();
    let version_file = temp_dir.path().join(".nvmrc");
    
    fs::write(&version_file, "  18.20.0  \n").unwrap();
    
    let result = VersionFile::find(temp_dir.path(), &[".nvmrc".into()]).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().version, "18.20.0");
}
```

---

## Implementation Tasks

### Week 1

1. **Day 1-2: Project Setup**
   - Initialize Rust project with `cargo init`
   - Set up Git repository
   - Configure Cargo.toml dependencies
   - Create module structure

2. **Day 3-4: CLI Framework**
   - Implement CLI with clap
   - Add setup, activate, status commands
   - Implement --version and --help flags
   - Write CLI integration tests

3. **Day 5: Configuration System**
   - Implement Config struct with serde
   - Add YAML parsing logic
   - Implement config precedence (defaults → user → project)
   - Write config unit tests

### Week 2

1. **Day 1-2: Version File Detection**
   - Implement directory traversal logic
   - Add version file parsing
   - Handle edge cases (empty files, permissions, whitespace)
   - Write version file tests

2. **Day 3: Error Handling**
   - Define XvnError enum with thiserror
   - Implement error context and messages
   - Add error handling to all modules

3. **Day 4: Integration & Testing**
   - Write integration tests
   - Achieve >80% code coverage
   - Fix any failing tests

4. **Day 5: Documentation**
   - Add inline documentation (rustdoc)
   - Update README with build instructions
   - Document module responsibilities

---

## Success Metrics

- ✅ `cargo build --release` completes without errors
- ✅ `cargo test` passes all tests
- ✅ `cargo tarpaulin` shows >80% coverage
- ✅ CLI commands parse correctly
- ✅ Config loaded from multiple sources with correct precedence
- ✅ Version files discovered in parent directories
- ✅ No clippy warnings (`cargo clippy`)
