# Milestone 7: Interactive Setup Wizard - Implementation Plan

## Overview

This milestone transforms the basic `xvn setup` command into an interactive wizard that guides users through initial configuration. The wizard makes configuration discoverable, validates user choices, and provides educational context at each step.

**Timeline:** 3 weeks
**Version:** v0.8.0
**Complexity:** Medium-High (new interactive UI, state management)

## Prerequisites

- Milestone 3 completed (shell integration exists)
- Current `setup` command working
- Understanding of current `Config` structure
- Familiarity with terminal UI patterns

## Implementation Tasks

---

### Task 1.1: Add inquire dependency

**Objective:** Add the `inquire` crate for interactive terminal prompts with a rich UI.

**Implementation Steps:**

1. Add to `Cargo.toml`:
```toml
[dependencies]
inquire = "0.7"
```

2. Test basic functionality by creating a simple test file `tests/inquire_test.rs`:
```rust
use inquire::{Confirm, Select, MultiSelect};

#[test]
fn test_inquire_basics() {
    // This test documents how inquire works
    // Remove after confirming it works locally

    // Confirm prompt
    // let confirmed = Confirm::new("Continue?")
    //     .with_default(true)
    //     .prompt()
    //     .unwrap();

    // Select prompt
    // let option = Select::new("Choose:", vec!["A", "B", "C"])
    //     .prompt()
    //     .unwrap();

    // MultiSelect prompt
    // let selections = MultiSelect::new("Pick:", vec!["X", "Y", "Z"])
    //     .prompt()
    //     .unwrap();
}
```

3. Run `cargo build` to verify dependency resolves
4. Test manual interaction: `cargo run --example` (create example if needed)

**Key Considerations:**
- Pin version `0.7` for stability
- `inquire` requires TTY - handle gracefully when not available
- Works on Unix-like systems (Linux, macOS)

**Testing:**
- Cargo build succeeds
- Basic prompts work in terminal
- No prompt appears when stdin is not TTY

**Dependencies:**
- None (foundation task)

---

### Task 1.2: Create init module structure

**Objective:** Set up the module structure for init functionality.

**Implementation Steps:**

1. Create directory structure:
```bash
mkdir src/init
touch src/init/mod.rs
touch src/init/wizard.rs
touch src/init/prompts.rs
touch src/init/detection.rs
touch src/init/validation.rs
```

2. Create `src/init/mod.rs`:
```rust
//! Interactive setup wizard for xvn
//!
//! This module provides an interactive configuration wizard that guides
//! users through initial setup with auto-detection, validation, and
//! educational prompts.

pub mod wizard;
pub mod prompts;
pub mod detection;
pub mod validation;

pub use wizard::{run_interactive_wizard, run_quick_setup, run_non_interactive};
use crate::config::Config;
use anyhow::Result;

/// Main entry point for the init command
pub fn init(quick: bool, non_interactive: bool, force: bool) -> Result<()> {
    if non_interactive {
        run_non_interactive(force)
    } else if quick {
        run_quick_setup(force)
    } else {
        run_interactive_wizard(force)
    }
}
```

3. Add module to `src/lib.rs`:
```rust
pub mod init;
```

4. Add placeholder functions in other files:
```rust
// src/init/wizard.rs
use anyhow::Result;

pub fn run_interactive_wizard(force: bool) -> Result<()> {
    todo!("Implement wizard")
}

pub fn run_quick_setup(force: bool) -> Result<()> {
    todo!("Implement quick setup")
}

pub fn run_non_interactive(force: bool) -> Result<()> {
    todo!("Implement non-interactive")
}

// src/init/prompts.rs
// Empty for now

// src/init/detection.rs
// Empty for now

// src/init/validation.rs
// Empty for now
```

**Code Structure:**
- `src/init/mod.rs` - Public API and entry point
- `src/init/wizard.rs` - Wizard orchestration and state
- `src/init/prompts.rs` - Individual prompt functions
- `src/init/detection.rs` - Auto-detection logic
- `src/init/validation.rs` - Configuration validation

**Key Considerations:**
- Clear module boundaries and responsibilities
- Public API is minimal (just `init()` function)
- Internal functions can be made public with `pub(crate)` for testing

**Testing:**
- Module compiles without errors
- `xvn::init::init()` is accessible
- Placeholder functions exist

**Dependencies:**
- Requires: Task 1.1 (inquire dependency)

---

### Task 1.3: Add Init command to CLI

**Objective:** Add the `init` command to the CLI and wire it up to the init module.

**Implementation Steps:**

1. Update `src/cli.rs` to add Init command:
```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    // ... existing commands ...

    /// Initialize xvn with interactive configuration wizard
    ///
    /// This command guides you through initial setup with auto-detection
    /// and configuration of shell integration, version managers, and preferences.
    ///
    /// For quick setup with defaults: xvn init --quick
    /// For automation/CI: xvn init --non-interactive
    Init {
        /// Skip wizard and use sensible defaults
        #[arg(short, long)]
        quick: bool,

        /// Force overwrite existing configuration
        #[arg(short, long)]
        force: bool,

        /// Shell to configure (bash, zsh, or auto-detect)
        #[arg(short, long)]
        shell: Option<String>,

        /// Non-interactive mode for automation
        #[arg(long)]
        non_interactive: bool,
    },

    /// Set up shell integration (alias for 'init' for compatibility)
    ///
    /// This is an alias for 'xvn init' for backward compatibility.
    /// Use 'xvn init' for the full interactive wizard.
    #[clap(hide = true)] // Hide from main help but still works
    Setup {
        #[arg(short, long, default_value = "auto")]
        shell: String,

        #[arg(short, long)]
        force: bool,
    },
}
```

2. Update command handler in `src/cli.rs`:
```rust
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // ... existing setup ...

    match cli.command {
        Some(Commands::Init { quick, force, shell, non_interactive }) => {
            info!("Running init command (quick: {quick}, force: {force}, non_interactive: {non_interactive})");

            // TODO: Handle shell parameter when provided
            crate::init::init(quick, non_interactive, force)
        }

        Some(Commands::Setup { shell, force }) => {
            // Redirect to init in quick mode
            info!("Running setup command (redirecting to init)");
            crate::init::init(true, false, force)
        }

        // ... other commands ...
    }
}
```

3. Update help documentation in main struct:
```rust
#[command(long_about = r#"
xvn automatically switches your Node.js version when you cd into a directory
with a .nvmrc or .node-version file.

After installation, run 'xvn init' to configure your shell with an interactive
wizard, or 'xvn init --quick' for automatic setup with sensible defaults.

Examples:
  xvn init               Interactive setup wizard (recommended)
  xvn init --quick       Quick setup with defaults
  xvn activate           Manually activate for current directory
  xvn status             Show configuration and test activation

For more information, visit: https://github.com/cameronolivier/xvn
"#)]
```

**Key Considerations:**
- `setup` command still works but is hidden (backward compatibility)
- `--shell` parameter stored for future use but not yet implemented
- Clear help text explains difference between init and quick mode

**Testing:**
```bash
xvn init --help
xvn setup --help
xvn init --quick
xvn init --non-interactive
```

**Dependencies:**
- Requires: Task 1.2 (init module exists)
- Enables: All wizard tasks

---

### Task 2.1: Implement shell detection

**Objective:** Auto-detect the user's shell and provide a selection mechanism.

**Implementation Steps:**

1. Create shell detection in `src/init/detection.rs`:
```rust
use crate::setup::shell_detection::Shell;
use anyhow::{Result, Context};
use std::env;

/// Detect the user's current shell from environment
pub fn detect_shell() -> Result<Shell> {
    // Check $SHELL environment variable
    let shell_path = env::var("SHELL")
        .context("SHELL environment variable not set")?;

    // Parse shell from path (e.g., /bin/zsh -> zsh)
    let shell_name = shell_path
        .rsplit('/')
        .next()
        .context("Invalid SHELL path")?;

    match shell_name {
        "bash" => Ok(Shell::Bash),
        "zsh" => Ok(Shell::Zsh),
        other => {
            log::warn!("Unknown shell: {}, defaulting to bash", other);
            Ok(Shell::Bash)
        }
    }
}

/// Get the profile path for a given shell
pub fn get_profile_path(shell: &Shell) -> Result<std::path::PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;

    let profile_name = match shell {
        Shell::Bash => ".bashrc",
        Shell::Zsh => ".zshrc",
    };

    Ok(home.join(profile_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_shell() {
        // This will use the actual environment
        // In CI, SHELL is typically /bin/bash
        let shell = detect_shell();
        assert!(shell.is_ok());
    }

    #[test]
    fn test_get_profile_path() {
        let bash_profile = get_profile_path(&Shell::Bash).unwrap();
        assert!(bash_profile.to_str().unwrap().ends_with(".bashrc"));

        let zsh_profile = get_profile_path(&Shell::Zsh).unwrap();
        assert!(zsh_profile.to_str().unwrap().ends_with(".zshrc"));
    }
}
```

**Key Considerations:**
- Handle missing `$SHELL` gracefully
- Default to bash if unknown shell detected
- Reuse existing `Shell` enum from `setup::shell_detection`

**Testing:**
- Test with `SHELL=/bin/bash`
- Test with `SHELL=/usr/local/bin/zsh`
- Test with missing `SHELL` (should error gracefully)

**Dependencies:**
- Requires: Existing `setup::shell_detection::Shell`

---

### Task 2.2: Implement version manager detection

**Objective:** Auto-detect installed version managers (nvm, fnm, n).

**Implementation Steps:**

1. Add detection functions to `src/init/detection.rs`:
```rust
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DetectedManager {
    pub name: String,
    pub path: Option<PathBuf>,
}

/// Detect all installed version managers
pub fn detect_version_managers() -> Vec<DetectedManager> {
    let mut managers = Vec::new();

    if let Some(nvm) = check_nvm() {
        managers.push(nvm);
    }
    if let Some(fnm) = check_fnm() {
        managers.push(fnm);
    }
    if let Some(n) = check_n() {
        managers.push(n);
    }

    managers
}

/// Check if nvm is installed
fn check_nvm() -> Option<DetectedManager> {
    // Check $NVM_DIR first
    if let Ok(nvm_dir) = env::var("NVM_DIR") {
        let nvm_sh = PathBuf::from(&nvm_dir).join("nvm.sh");
        if nvm_sh.exists() {
            return Some(DetectedManager {
                name: "nvm".to_string(),
                path: Some(PathBuf::from(nvm_dir)),
            });
        }
    }

    // Check ~/.nvm
    if let Some(home) = dirs::home_dir() {
        let nvm_dir = home.join(".nvm");
        let nvm_sh = nvm_dir.join("nvm.sh");
        if nvm_sh.exists() {
            return Some(DetectedManager {
                name: "nvm".to_string(),
                path: Some(nvm_dir),
            });
        }
    }

    None
}

/// Check if fnm is installed
fn check_fnm() -> Option<DetectedManager> {
    use std::process::Command;

    // Try `which fnm`
    if let Ok(output) = Command::new("which").arg("fnm").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            return Some(DetectedManager {
                name: "fnm".to_string(),
                path: Some(PathBuf::from(path)),
            });
        }
    }

    // Check ~/.fnm
    if let Some(home) = dirs::home_dir() {
        let fnm_dir = home.join(".fnm");
        if fnm_dir.exists() {
            return Some(DetectedManager {
                name: "fnm".to_string(),
                path: Some(fnm_dir),
            });
        }
    }

    None
}

/// Check if n is installed
fn check_n() -> Option<DetectedManager> {
    use std::process::Command;

    // Try `which n`
    if let Ok(output) = Command::new("which").arg("n").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            return Some(DetectedManager {
                name: "n".to_string(),
                path: Some(PathBuf::from(path)),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_managers() {
        // This will detect actual managers on the system
        let managers = detect_version_managers();

        // Should find at least one in dev environment
        // Don't assert on CI as it may not have any installed
        println!("Detected managers: {:?}", managers);
    }
}
```

**Key Considerations:**
- Check multiple locations for each manager
- Use `which` command as fallback
- Don't error if no managers found (just return empty vec)
- Return path info for display to user

**Testing:**
- Mock filesystem for unit tests
- Test with nvm installed
- Test with fnm installed
- Test with no managers installed

**Dependencies:**
- None (uses standard library)

---

### Task 2.3: Implement TTY detection

**Objective:** Detect if running in an interactive terminal.

**Implementation Steps:**

1. Add to `src/init/detection.rs`:
```rust
use std::io::IsTerminal;

/// Check if we're running in an interactive terminal
pub fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}

/// Check if we should run in interactive mode
/// Considers both TTY and --non-interactive flag
pub fn should_run_interactive(non_interactive_flag: bool) -> bool {
    !non_interactive_flag && is_interactive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_run_interactive() {
        // When flag is true, should never be interactive
        assert!(!should_run_interactive(true));

        // When flag is false, depends on TTY
        // (Can't reliably test this in CI)
    }
}
```

**Key Considerations:**
- `std::io::IsTerminal` is stable in Rust 1.70+
- Always respect `--non-interactive` flag
- Used to choose between wizard and automatic setup

**Testing:**
- Test with `--non-interactive` flag
- Test in CI (non-TTY)
- Test in terminal (TTY)

**Dependencies:**
- Rust 1.70+ (check minimum version in Cargo.toml)

---

### Task 3.1: Implement shell selection prompt

**Objective:** Prompt user to confirm auto-detected shell or select manually.

**Implementation Steps:**

1. Create `src/init/prompts.rs`:
```rust
use crate::setup::shell_detection::Shell;
use crate::init::detection::{detect_shell, get_profile_path};
use crate::output;
use anyhow::Result;
use inquire::{Confirm, Select};

/// Prompt user to select shell
pub fn prompt_shell() -> Result<Shell> {
    output::info("Step 1/5: Shell Detection");
    println!();

    // Try to detect shell
    let detected = detect_shell()?;
    let profile_path = get_profile_path(&detected)?;

    println!("  Detected shell: {}", detected.name());
    println!("  Profile: {}", profile_path.display());
    println!();

    // Ask for confirmation
    let use_detected = Confirm::new(&format!("Use {}?", detected.name()))
        .with_default(true)
        .with_help_message("Press Enter to confirm, or 'n' to select manually")
        .prompt()?;

    if use_detected {
        output::success(&format!("Using {}", detected.name()));
        return Ok(detected);
    }

    // Manual selection
    let shells = vec![Shell::Bash, Shell::Zsh];
    let shell_names: Vec<&str> = shells.iter().map(|s| s.name()).collect();

    let selected = Select::new("Select your shell:", shell_names)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()?;

    let shell = shells.into_iter()
        .find(|s| s.name() == selected)
        .expect("Selected shell should exist");

    output::success(&format!("Using {}", shell.name()));
    Ok(shell)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Can't easily unit test interactive prompts
    // Test manually with: cargo run -- init
}
```

**Key Considerations:**
- Show profile path so user knows what will be modified
- Make default action clear (just press Enter)
- Provide help text for navigation

**Testing:**
- Manual test: Run wizard and confirm detection
- Manual test: Run wizard and select manually
- Test that both shells can be selected

**Dependencies:**
- Requires: Task 2.1 (shell detection)
- Requires: Task 1.1 (inquire)

---

### Task 3.2: Implement plugin selection prompt

**Objective:** Show detected version managers and let user select/prioritize.

**Implementation Steps:**

1. Add to `src/init/prompts.rs`:
```rust
use crate::init::detection::{detect_version_managers, DetectedManager};
use inquire::MultiSelect;

/// Prompt user to select version managers
pub fn prompt_plugins() -> Result<Vec<String>> {
    output::info("Step 2/5: Version Managers");
    println!();

    // Detect installed managers
    let detected = detect_version_managers();

    if detected.is_empty() {
        output::warning("No version managers detected!");
        println!();
        println!("  xvn requires a version manager to be installed:");
        println!("  • nvm: https://github.com/nvm-sh/nvm");
        println!("  • fnm: https://github.com/Schniz/fnm");
        println!("  • n: https://github.com/tj/n");
        println!();

        let proceed = Confirm::new("Continue setup anyway?")
            .with_default(false)
            .with_help_message("You can install a version manager later")
            .prompt()?;

        if !proceed {
            anyhow::bail!("Setup cancelled - please install a version manager first");
        }

        // Return empty list but allow setup to continue
        return Ok(Vec::new());
    }

    // Display detected managers
    println!("  Detected version managers:");
    for manager in &detected {
        let path_str = manager.path.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "unknown path".to_string());
        println!("  ✓ {} ({})", manager.name, path_str);
    }
    println!();

    // Create options for MultiSelect
    let options: Vec<String> = detected.iter()
        .map(|m| format!("{} ({})",
            m.name,
            m.path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "system".to_string())
        ))
        .collect();

    // Pre-select all detected
    let defaults: Vec<bool> = vec![true; options.len()];

    let selected = MultiSelect::new("Select version managers to use:", options.clone())
        .with_default(&defaults)
        .with_help_message("Space to toggle, Enter to confirm")
        .prompt()?;

    // Extract manager names from selections
    let mut plugins: Vec<String> = selected.iter()
        .filter_map(|sel| {
            // Parse "nvm (path)" -> "nvm"
            sel.split_whitespace().next().map(|s| s.to_string())
        })
        .collect();

    if plugins.is_empty() {
        output::warning("No version managers selected");
        plugins = detected.iter()
            .map(|m| m.name.clone())
            .collect();
        output::info(&format!("Defaulting to: {}", plugins.join(", ")));
    } else {
        output::success(&format!("Selected: {}", plugins.join(", ")));
    }

    // TODO: Prompt for priority order if multiple selected
    // For now, keep detection order

    Ok(plugins)
}
```

**Key Considerations:**
- Handle case where no managers detected
- Show installation links for common managers
- Pre-select all detected managers
- Extract manager name from formatted display string

**Testing:**
- Test with nvm installed
- Test with multiple managers
- Test with no managers
- Test deselecting all (should warn)

**Dependencies:**
- Requires: Task 2.2 (manager detection)

---

### Task 3.3: Implement auto-install mode prompt

**Objective:** Let user choose auto-install behavior.

**Implementation Steps:**

1. Add to `src/init/prompts.rs`:
```rust
use crate::config::AutoInstallMode;
use inquire::Select;

/// Prompt user for auto-install preference
pub fn prompt_auto_install() -> Result<AutoInstallMode> {
    output::info("Step 3/5: Auto-Install Behavior");
    println!();

    println!("  When a required Node.js version isn't installed:");
    println!();

    let options = vec![
        "Prompt - Ask before installing (recommended)",
        "Always - Install automatically without asking",
        "Never - Show error, don't install",
    ];

    let selected = Select::new("Choose auto-install mode:", options)
        .with_help_message("Use arrow keys and Enter to select")
        .prompt()?;

    let mode = match selected {
        s if s.starts_with("Prompt") => AutoInstallMode::Prompt,
        s if s.starts_with("Always") => AutoInstallMode::Always,
        s if s.starts_with("Never") => AutoInstallMode::Never,
        _ => AutoInstallMode::Prompt, // Default fallback
    };

    let mode_str = match mode {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };

    output::success(&format!("Auto-install: {}", mode_str));
    Ok(mode)
}
```

**Key Considerations:**
- Default to "Prompt" (safest option)
- Clear descriptions of each mode
- Explain implications of "Always" (auto-installs can be slow)

**Testing:**
- Test selecting each option
- Verify correct AutoInstallMode returned

**Dependencies:**
- Requires: Existing `config::AutoInstallMode` enum

---

### Task 3.4: Implement version files prompt

**Objective:** Let user select which version files to check.

**Implementation Steps:**

1. Add to `src/init/prompts.rs`:
```rust
use inquire::MultiSelect;

/// Prompt user for version file preferences
pub fn prompt_version_files() -> Result<Vec<String>> {
    output::info("Step 4/5: Version Files");
    println!();

    println!("  Which files should xvn check for version information?");
    println!();

    let options = vec![
        ".nvmrc (standard Node.js convention)",
        ".node-version (alternative format)",
        ".tool-versions (asdf compatibility)",
    ];

    // Default to .nvmrc and .node-version
    let defaults = vec![true, true, false];

    let selected = MultiSelect::new("Select version files:", options)
        .with_default(&defaults)
        .with_help_message("Space to toggle, Enter to confirm")
        .prompt()?;

    // Extract filenames
    let files: Vec<String> = selected.iter()
        .map(|s| {
            // Parse ".nvmrc (description)" -> ".nvmrc"
            s.split_whitespace()
                .next()
                .unwrap_or(".nvmrc")
                .to_string()
        })
        .collect();

    if files.is_empty() {
        output::warning("No version files selected, using defaults");
        return Ok(vec![".nvmrc".to_string(), ".node-version".to_string()]);
    }

    output::success(&format!("Version files: {}", files.join(", ")));
    Ok(files)
}
```

**Key Considerations:**
- Provide helpful descriptions
- Default to most common files
- Don't allow empty selection (need at least one)

**Testing:**
- Test selecting all options
- Test selecting none (should default)
- Test default selection

**Dependencies:**
- None

---

### Task 3.5: Implement configuration review prompt

**Objective:** Show summary and get final confirmation.

**Implementation Steps:**

1. Add to `src/init/prompts.rs`:
```rust
use crate::setup::shell_detection::Shell;
use crate::config::{Config, AutoInstallMode};
use std::path::PathBuf;

pub struct ConfigSummary {
    pub shell: Shell,
    pub profile_path: PathBuf,
    pub plugins: Vec<String>,
    pub auto_install: AutoInstallMode,
    pub version_files: Vec<String>,
    pub config_path: PathBuf,
}

/// Prompt user to review and confirm configuration
pub fn prompt_confirm_config(summary: &ConfigSummary) -> Result<bool> {
    output::info("Step 5/5: Review Configuration");
    println!();

    println!("  {:<16} {}", "Shell:", summary.shell.name());
    println!("  {:<16} {}", "Profile:", summary.profile_path.display());
    println!("  {:<16} {}", "Plugins:", summary.plugins.join(", "));

    let auto_install_str = match summary.auto_install {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };
    println!("  {:<16} {}", "Auto-install:", auto_install_str);
    println!("  {:<16} {}", "Version files:", summary.version_files.join(", "));
    println!("  {:<16} {}", "Config file:", summary.config_path.display());
    println!();

    let confirmed = Confirm::new("Looks good?")
        .with_default(true)
        .with_help_message("Press Enter to confirm, or 'n' to cancel")
        .prompt()?;

    Ok(confirmed)
}
```

**Key Considerations:**
- Format output nicely with aligned columns
- Show all configuration choices
- Make acceptance the default (Enter to confirm)

**Testing:**
- Test with various configurations
- Verify formatting looks good
- Test both yes and no responses

**Dependencies:**
- Requires: All previous prompt functions
- Requires: Task 4.1 (ConfigSummary struct)

---

### Task 4.1: Implement wizard state management

**Objective:** Create a struct to hold wizard state between steps.

**Implementation Steps:**

1. Create `src/init/wizard.rs`:
```rust
use crate::config::{Config, AutoInstallMode};
use crate::setup::shell_detection::Shell;
use anyhow::Result;

/// Wizard state - collects configuration through steps
#[derive(Debug, Clone)]
pub struct WizardState {
    pub shell: Option<Shell>,
    pub plugins: Vec<String>,
    pub auto_install: AutoInstallMode,
    pub version_files: Vec<String>,
}

impl WizardState {
    /// Create new wizard state with defaults
    pub fn new() -> Self {
        Self {
            shell: None,
            plugins: Vec::new(),
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
        }
    }

    /// Convert wizard state to Config
    pub fn to_config(&self) -> Result<Config> {
        Ok(Config {
            plugins: self.plugins.clone(),
            auto_install: self.auto_install,
            version_files: self.version_files.clone(),
        })
    }

    /// Get shell or error
    pub fn get_shell(&self) -> Result<Shell> {
        self.shell
            .ok_or_else(|| anyhow::anyhow!("Shell not set"))
    }
}

impl Default for WizardState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_defaults() {
        let state = WizardState::new();
        assert_eq!(state.plugins.len(), 0);
        assert!(matches!(state.auto_install, AutoInstallMode::Prompt));
        assert_eq!(state.version_files.len(), 2);
    }

    #[test]
    fn test_to_config() {
        let mut state = WizardState::new();
        state.shell = Some(Shell::Zsh);
        state.plugins = vec!["nvm".to_string()];

        let config = state.to_config().unwrap();
        assert_eq!(config.plugins, vec!["nvm"]);
    }
}
```

**Key Considerations:**
- Use `Option<Shell>` since it's selected in step 1
- Provide sensible defaults for all fields
- Make conversion to `Config` explicit

**Testing:**
- Test default state
- Test state with all fields set
- Test conversion to Config

**Dependencies:**
- Requires: Existing `Config` struct

---

### Task 4.2: Implement interactive wizard flow

**Objective:** Orchestrate the 5-step wizard using prompt functions.

**Implementation Steps:**

1. Implement wizard flow in `src/init/wizard.rs`:
```rust
use crate::init::prompts::*;
use crate::init::detection::get_profile_path;
use crate::output;
use dirs::home_dir;

/// Run the interactive wizard
pub fn run_interactive_wizard(force: bool) -> Result<()> {
    // Print header
    println!();
    output::print_header();
    println!("Welcome! Let's set up xvn for your environment.");
    println!();

    // Initialize state
    let mut state = WizardState::new();

    // Step 1: Shell selection
    let shell = prompt_shell()?;
    state.shell = Some(shell);
    println!();

    // Step 2: Plugin selection
    let plugins = prompt_plugins()?;
    state.plugins = plugins;
    println!();

    // Step 3: Auto-install mode
    let auto_install = prompt_auto_install()?;
    state.auto_install = auto_install;
    println!();

    // Step 4: Version files
    let version_files = prompt_version_files()?;
    state.version_files = version_files;
    println!();

    // Step 5: Review and confirm
    let profile_path = get_profile_path(&state.get_shell()?)?;
    let config_path = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".xvnrc");

    let summary = ConfigSummary {
        shell: state.get_shell()?,
        profile_path,
        plugins: state.plugins.clone(),
        auto_install: state.auto_install,
        version_files: state.version_files.clone(),
        config_path: config_path.clone(),
    };

    let confirmed = prompt_confirm_config(&summary)?;
    if !confirmed {
        output::warning("Setup cancelled");
        return Ok(());
    }
    println!();

    // Generate and write config
    let config = state.to_config()?;
    write_config(&config, &config_path, force)?;
    output::success("Configuration saved!");

    // Install shell integration
    let installer = crate::setup::SetupInstaller::new()?;
    installer.install()?;
    output::success("Shell integration installed!");

    // Print success message
    print_success_message(&summary)?;

    Ok(())
}

// Helper function to write config (implemented in Task 5.3)
fn write_config(config: &Config, path: &std::path::Path, force: bool) -> Result<()> {
    todo!("Implement in Task 5.3")
}

// Helper function to print success (implemented in Task 6.2)
fn print_success_message(summary: &ConfigSummary) -> Result<()> {
    todo!("Implement in Task 6.2")
}
```

**Key Considerations:**
- Clear step-by-step progression
- Blank lines between steps for readability
- Handle user cancellation gracefully
- Validate state before proceeding

**Testing:**
- Manual test: Complete full wizard
- Manual test: Cancel at step 5
- Verify all steps are called in order

**Dependencies:**
- Requires: All prompt functions (Tasks 3.1-3.5)
- Requires: Task 4.1 (WizardState)

---

### Task 4.3: Implement quick mode

**Objective:** Auto-detect everything and skip prompts.

**Implementation Steps:**

1. Add to `src/init/wizard.rs`:
```rust
use crate::init::detection::{detect_shell, detect_version_managers};

/// Run quick setup with auto-detection and defaults
pub fn run_quick_setup(force: bool) -> Result<()> {
    println!();
    output::print_header();
    output::info("Running quick setup with defaults...");
    println!();

    // Auto-detect shell
    let shell = detect_shell()?;
    output::info(&format!("Detected shell: {}", shell.name()));

    // Auto-detect version managers
    let detected = detect_version_managers();
    let plugins: Vec<String> = detected.iter()
        .map(|m| m.name.clone())
        .collect();

    if plugins.is_empty() {
        output::warning("No version managers detected");
        output::info("You'll need to install nvm, fnm, or n manually");
    } else {
        output::info(&format!("Detected managers: {}", plugins.join(", ")));
    }

    // Use defaults
    let auto_install = AutoInstallMode::Prompt;
    let version_files = vec![".nvmrc".to_string(), ".node-version".to_string()];

    output::info("Using defaults:");
    output::info("  • Auto-install: prompt");
    output::info("  • Version files: .nvmrc, .node-version");
    println!();

    // Create state
    let state = WizardState {
        shell: Some(shell),
        plugins,
        auto_install,
        version_files,
    };

    // Generate config
    let config = state.to_config()?;
    let config_path = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".xvnrc");

    write_config(&config, &config_path, force)?;
    output::success("Configuration saved!");

    // Install shell integration
    let installer = crate::setup::SetupInstaller::new()?;
    installer.install()?;
    output::success("Shell integration installed!");

    // Print summary
    let profile_path = get_profile_path(&shell)?;
    let summary = ConfigSummary {
        shell,
        profile_path,
        plugins: state.plugins,
        auto_install: state.auto_install,
        version_files: state.version_files,
        config_path,
    };

    print_success_message(&summary)?;

    Ok(())
}
```

**Key Considerations:**
- No prompts at all
- Log all decisions
- Use sensible defaults
- Complete quickly (<5 seconds)

**Testing:**
- Run `xvn init --quick`
- Verify completes without prompts
- Verify generated config is valid

**Dependencies:**
- Requires: Task 2.1, 2.2 (detection)

---

### Task 4.4: Implement non-interactive mode

**Objective:** Run without TTY for CI/automation.

**Implementation Steps:**

1. Add to `src/init/wizard.rs`:
```rust
/// Run non-interactive setup for CI/automation
pub fn run_non_interactive(force: bool) -> Result<()> {
    // Non-interactive is the same as quick mode
    // but with explicit logging for CI

    eprintln!("xvn: Running in non-interactive mode");

    run_quick_setup(force)
}
```

**Key Considerations:**
- Essentially same as quick mode
- Add logging to stderr for CI visibility
- No prompts ever

**Testing:**
- Run in CI environment
- Run with `xvn init --non-interactive`

**Dependencies:**
- Requires: Task 4.3 (quick setup)

---

### Task 5.1: Implement configuration validation

**Objective:** Validate configuration before saving.

**Implementation Steps:**

1. Create `src/init/validation.rs`:
```rust
use crate::config::{Config, AutoInstallMode};
use crate::setup::shell_detection::Shell;
use anyhow::{Result, Context};

/// Validate a configuration
pub fn validate_config(config: &Config) -> Result<()> {
    validate_plugins(&config.plugins)?;
    validate_version_files(&config.version_files)?;
    validate_auto_install(&config.auto_install)?;
    Ok(())
}

/// Validate plugin list
fn validate_plugins(plugins: &[String]) -> Result<()> {
    // It's OK to have no plugins - user might install them later
    // But warn if empty (done in prompt)

    // Validate known plugin names
    let known_plugins = ["nvm", "fnm", "n", "asdf", "volta"];
    for plugin in plugins {
        if !known_plugins.contains(&plugin.as_str()) {
            log::warn!("Unknown plugin: {}", plugin);
        }
    }

    Ok(())
}

/// Validate version files list
fn validate_version_files(files: &[String]) -> Result<()> {
    if files.is_empty() {
        anyhow::bail!("At least one version file must be specified");
    }

    // Validate file names
    for file in files {
        if !file.starts_with('.') {
            log::warn!("Version file should start with '.': {}", file);
        }
    }

    Ok(())
}

/// Validate auto-install mode
fn validate_auto_install(mode: &AutoInstallMode) -> Result<()> {
    // All modes are valid
    // Just log for visibility
    match mode {
        AutoInstallMode::Always => {
            log::info!("Auto-install mode: always (automatic installation)");
        }
        AutoInstallMode::Prompt => {
            log::info!("Auto-install mode: prompt (ask before installing)");
        }
        AutoInstallMode::Never => {
            log::info!("Auto-install mode: never (show error only)");
        }
    }
    Ok(())
}

/// Validate shell is supported
pub fn validate_shell(shell: &Shell) -> Result<()> {
    match shell {
        Shell::Bash | Shell::Zsh => Ok(()),
        // Add more shells in future
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_version_files_empty() {
        let result = validate_version_files(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_version_files_valid() {
        let files = vec![".nvmrc".to_string(), ".node-version".to_string()];
        assert!(validate_version_files(&files).is_ok());
    }

    #[test]
    fn test_validate_plugins() {
        let plugins = vec!["nvm".to_string(), "fnm".to_string()];
        assert!(validate_plugins(&plugins).is_ok());
    }
}
```

**Key Considerations:**
- Don't be too strict (allow unknown plugins)
- Version files can't be empty
- Log warnings for suspicious values

**Testing:**
- Test with valid config
- Test with empty version_files (should error)
- Test with unknown plugins (should warn)

**Dependencies:**
- None

---

### Task 5.2: Implement config file generation

**Objective:** Generate YAML config with helpful comments.

**Implementation Steps:**

1. Add to `src/init/wizard.rs`:
```rust
use chrono::Local;

/// Generate config file content with comments
fn generate_config(config: &Config) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    let auto_install_str = match config.auto_install {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };

    format!(
        r#"# xvn configuration file
# Generated by: xvn init
# Last modified: {}
#
# To modify this configuration, run: xvn init

# Version manager priority order
# Available: nvm, fnm, n, asdf, volta
plugins:
{}

# Auto-install behavior when version not found
# Options: prompt (ask each time), always (install automatically), never (error)
auto_install: {}

# Version files to search for (in priority order)
version_files:
{}
"#,
        timestamp,
        config.plugins.iter()
            .map(|p| format!("  - {}", p))
            .collect::<Vec<_>>()
            .join("\n"),
        auto_install_str,
        config.version_files.iter()
            .map(|f| format!("  - {}", f))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config() {
        let config = Config {
            plugins: vec!["nvm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string()],
        };

        let yaml = generate_config(&config);
        assert!(yaml.contains("plugins:"));
        assert!(yaml.contains("- nvm"));
        assert!(yaml.contains("auto_install: prompt"));
    }
}
```

**Key Considerations:**
- Include timestamp for tracking
- Add helpful comments explaining each option
- Show available options in comments
- Use proper YAML formatting

**Testing:**
- Test with various configs
- Verify YAML is valid (can parse back)
- Verify comments are present

**Dependencies:**
- May need `chrono` crate for timestamp (optional)

---

### Task 5.3: Implement config file writing

**Objective:** Write config to ~/.xvnrc with proper handling.

**Implementation Steps:**

1. Implement in `src/init/wizard.rs`:
```rust
use std::fs;
use std::path::Path;
use inquire::Confirm;

/// Write configuration to file
fn write_config(config: &Config, path: &Path, force: bool) -> Result<()> {
    // Check if config exists
    if path.exists() && !force {
        let overwrite = Confirm::new("Configuration file already exists. Overwrite?")
            .with_default(false)
            .with_help_message("Use --force to skip this prompt")
            .prompt()?;

        if !overwrite {
            anyhow::bail!("Config write cancelled - existing file preserved");
        }
    }

    // Validate config
    crate::init::validation::validate_config(config)?;

    // Generate YAML content
    let content = generate_config(config);

    // Write to file
    fs::write(path, content)
        .context("Failed to write configuration file")?;

    log::info!("Config written to: {}", path.display());
    Ok(())
}
```

**Key Considerations:**
- Check if file exists first
- Respect --force flag
- Validate before writing
- Use atomic write if possible (to prevent corruption)

**Testing:**
- Test writing new config
- Test overwriting existing (with confirmation)
- Test with --force flag
- Test write failure (permissions)

**Dependencies:**
- Requires: Task 5.1 (validation)
- Requires: Task 5.2 (generation)

---

### Task 5.4: Integrate with existing SetupInstaller

**Objective:** Use existing shell integration installer.

**Implementation Steps:**

1. Already done in Task 4.2 - verify integration:
```rust
// In run_interactive_wizard and run_quick_setup:

let installer = crate::setup::SetupInstaller::new()?;
installer.install()?;
```

2. Ensure shell type is passed correctly (if needed):
```rust
// SetupInstaller already auto-detects shell
// No changes needed - it uses its own detection
```

**Key Considerations:**
- Reuse existing code - don't duplicate
- SetupInstaller handles shell profile modification
- Let it auto-detect shell (it already does)

**Testing:**
- Verify shell profile is modified correctly
- Verify xvn.sh is copied to ~/.xvn/bin/
- Test with both bash and zsh

**Dependencies:**
- Requires: Existing `setup::SetupInstaller`

---

### Task 6.1: Implement wizard header and branding

**Objective:** Print an attractive header for the wizard.

**Implementation Steps:**

1. Add to `src/init/wizard.rs`:
```rust
/// Print wizard header
fn print_wizard_header() {
    println!();
    crate::output::print_header();
    println!();
    println!("Welcome! Let's set up xvn for your environment.");
    println!();
    println!("This wizard will guide you through configuration:");
    println!("  • Shell detection and integration");
    println!("  • Version manager selection");
    println!("  • Installation preferences");
    println!("  • Version file configuration");
    println!();
    println!("Press Ctrl+C at any time to cancel.");
    println!();
}
```

**Key Considerations:**
- Use output::print_header() for branding
- Explain what wizard will do
- Set expectations
- Mention how to cancel

**Testing:**
- Visual inspection of output
- Verify branding matches other commands

**Dependencies:**
- Requires: output module

---

### Task 6.2: Implement success message

**Objective:** Print helpful success message with next steps.

**Implementation Steps:**

1. Add to `src/init/wizard.rs`:
```rust
use crate::init::prompts::ConfigSummary;

/// Print success message after setup
fn print_success_message(summary: &ConfigSummary) -> Result<()> {
    println!();
    crate::output::success("Setup complete!");
    println!();

    crate::output::info("Configuration:");
    println!("  Shell:        {}", summary.shell.name());
    println!("  Profile:      {}", summary.profile_path.display());
    println!("  Config file:  {}", summary.config_path.display());
    println!();

    crate::output::info("To start using xvn:");
    println!("  1. Restart your shell, or run:");
    println!("       source {}", summary.profile_path.display());
    println!("  2. Navigate to a project with a .nvmrc file");
    println!("  3. xvn will automatically activate the correct Node.js version");
    println!();

    crate::output::info("Useful commands:");
    println!("  xvn status              Show current configuration");
    println!("  xvn activate            Manually activate for a directory");
    println!("  xvn init                Re-run this wizard to modify config");
    println!();

    crate::output::info(&format!(
        "Your config file is at: {}",
        summary.config_path.display()
    ));
    println!();

    Ok(())
}
```

**Key Considerations:**
- Clear next steps
- Show config file location
- Provide useful commands
- Encourage testing

**Testing:**
- Visual inspection
- Verify all info is accurate

**Dependencies:**
- Requires: output module

---

### Task 6.3: Implement educational help text

**Objective:** Add helpful explanations throughout wizard.

**Implementation Steps:**

1. Review all prompts and add help text:
```rust
// Already added in prompt functions via:
.with_help_message("explanation here")

// Ensure each prompt has:
// - Clear question
// - Help text explaining the option
// - Default value indication
```

2. Add inline explanations:
```rust
// In prompt_auto_install:
println!("  What this means:");
println!("  • Prompt: Safe - you control what gets installed");
println!("  • Always: Convenient - automatic but slower");
println!("  • Never: Manual - you install versions yourself");
```

**Key Considerations:**
- Keep text concise but helpful
- Explain implications of choices
- Use examples where helpful

**Testing:**
- Read through wizard
- Verify text is clear
- Get feedback from users

**Dependencies:**
- All prompt functions

---

### Task 6.4: Implement error messages

**Objective:** Handle errors gracefully with helpful messages.

**Implementation Steps:**

1. Add error handling wrapper in `src/init/wizard.rs`:
```rust
use std::io::{self, ErrorKind};

/// Handle wizard errors gracefully
pub fn handle_wizard_error(err: anyhow::Error) {
    // Check if user cancelled (Ctrl+C)
    if let Some(io_err) = err.downcast_ref::<io::Error>() {
        if io_err.kind() == ErrorKind::Interrupted {
            println!();
            crate::output::warning("Setup cancelled by user");
            println!();
            crate::output::info("Run 'xvn init' again when ready");
            return;
        }
    }

    // Other errors
    crate::output::error(&format!("Setup failed: {}", err));

    // Try to provide helpful context
    if err.to_string().contains("version manager") {
        println!();
        crate::output::info("Install a version manager first:");
        println!("  • nvm: https://github.com/nvm-sh/nvm");
        println!("  • fnm: https://github.com/Schniz/fnm");
    }

    println!();
    crate::output::info("For help, visit: https://github.com/cameronolivier/xvn");
}
```

2. Use in CLI handler:
```rust
// In src/cli.rs:
Some(Commands::Init { .. }) => {
    if let Err(e) = crate::init::init(quick, non_interactive, force) {
        crate::init::wizard::handle_wizard_error(e);
        std::process::exit(1);
    }
    Ok(())
}
```

**Key Considerations:**
- Detect Ctrl+C gracefully
- Provide actionable next steps
- Show helpful links

**Testing:**
- Test cancelling with Ctrl+C
- Test various error conditions
- Verify error messages are helpful

**Dependencies:**
- All wizard functions

---

### Task 7.1: Write unit tests

**Objective:** Achieve comprehensive unit test coverage for all new code.

**Implementation Steps:**

1. Create test file for detection module `src/init/detection.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_detect_shell_from_env() {
        // Set SHELL and verify detection
        env::set_var("SHELL", "/bin/zsh");
        let shell = detect_shell().unwrap();
        assert!(matches!(shell, Shell::Zsh));

        env::set_var("SHELL", "/usr/bin/bash");
        let shell = detect_shell().unwrap();
        assert!(matches!(shell, Shell::Bash));
    }

    #[test]
    fn test_detect_managers_nvm() {
        // Create mock ~/.nvm/nvm.sh
        // Verify nvm is detected
    }

    // Add tests for fnm, n detection
    // Add tests for no managers
}
```

2. Create test file for validation module:
```rust
// In src/init/validation.rs tests:
#[test]
fn test_validate_empty_version_files() {
    let result = validate_version_files(&[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("at least one"));
}

#[test]
fn test_validate_valid_config() {
    let config = Config {
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string()],
    };
    assert!(validate_config(&config).is_ok());
}
```

3. Create test file for wizard module:
```rust
// In src/init/wizard.rs tests:
#[test]
fn test_wizard_state_to_config() {
    let state = WizardState {
        shell: Some(Shell::Zsh),
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Always,
        version_files: vec![".nvmrc".to_string()],
    };
    let config = state.to_config().unwrap();
    assert_eq!(config.plugins, vec!["nvm"]);
}

#[test]
fn test_generate_config_format() {
    let config = Config {
        plugins: vec!["nvm".to_string(), "fnm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string()],
    };
    let yaml = generate_config(&config);

    // Verify YAML structure
    assert!(yaml.contains("plugins:"));
    assert!(yaml.contains("  - nvm"));
    assert!(yaml.contains("  - fnm"));
    assert!(yaml.contains("auto_install: prompt"));

    // Verify it can be parsed back
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
    assert!(parsed["plugins"].is_sequence());
}
```

4. Run tests and check coverage:
```bash
cargo test
cargo tarpaulin --out Html --output-dir coverage
```

5. Fix any failing tests and gaps in coverage

**Key Considerations:**
- Mock filesystem access where possible
- Test both success and error paths
- Test edge cases (empty inputs, invalid values)
- Aim for >85% line coverage

**Testing:**
```bash
cargo test init::  # Run all init module tests
cargo test --all  # Run all tests
```

**Dependencies:**
- Requires: All implementation tasks complete

---

### Task 7.2: Write integration tests

**Objective:** Test complete wizard flows end-to-end.

**Implementation Steps:**

1. Create `tests/wizard_integration_test.rs`:
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_quick_setup_flow() {
    let temp_home = TempDir::new().unwrap();
    let config_path = temp_home.path().join(".xvnrc");

    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.env("HOME", temp_home.path())
       .arg("init")
       .arg("--quick")
       .assert()
       .success()
       .stdout(predicate::str::contains("Setup complete"));

    // Verify config was created
    assert!(config_path.exists());

    // Verify config is valid YAML
    let config_content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_yaml::Value = serde_yaml::from_str(&config_content).unwrap();

    assert!(parsed["plugins"].is_sequence());
    assert!(parsed["version_files"].is_sequence());
}

#[test]
fn test_non_interactive_mode() {
    let temp_home = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.env("HOME", temp_home.path())
       .arg("init")
       .arg("--non-interactive")
       .assert()
       .success();

    let config_path = temp_home.path().join(".xvnrc");
    assert!(config_path.exists());
}

#[test]
fn test_force_overwrite() {
    let temp_home = TempDir::new().unwrap();
    let config_path = temp_home.path().join(".xvnrc");

    // Create existing config
    fs::write(&config_path, "# old config").unwrap();

    // Run with --force
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.env("HOME", temp_home.path())
       .arg("init")
       .arg("--quick")
       .arg("--force")
       .assert()
       .success();

    // Verify config was overwritten
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("Generated by: xvn init"));
}

#[test]
fn test_init_creates_shell_integration() {
    let temp_home = TempDir::new().unwrap();
    let xvn_dir = temp_home.path().join(".xvn/bin");

    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.env("HOME", temp_home.path())
       .arg("init")
       .arg("--quick")
       .assert()
       .success();

    // Verify xvn.sh was copied
    assert!(xvn_dir.join("xvn.sh").exists());
}
```

2. Test backward compatibility with `setup` command:
```rust
#[test]
fn test_setup_alias_works() {
    let temp_home = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.env("HOME", temp_home.path())
       .arg("setup")
       .arg("--force")
       .assert()
       .success();

    // Should create same files as init
    let config_path = temp_home.path().join(".xvnrc");
    assert!(config_path.exists());
}
```

3. Run integration tests:
```bash
cargo test --test wizard_integration_test
```

**Key Considerations:**
- Use temporary directories for isolation
- Test happy paths and edge cases
- Verify file system side effects
- Test CLI flags and arguments

**Testing:**
```bash
cargo test --tests  # Run all integration tests
```

**Dependencies:**
- Requires: All implementation complete
- Requires: `tempfile` and `assert_cmd` crates

---

### Task 7.3: Manual testing

**Objective:** Verify wizard works correctly in real-world scenarios.

**Implementation Steps:**

1. Create manual test script `scripts/test-wizard.sh`:
```bash
#!/bin/bash
# Manual testing script for wizard

set -e

echo "=== XVN Wizard Manual Test Suite ==="
echo ""

# Clean state
rm -f ~/.xvnrc
rm -rf ~/.xvn

echo "Test 1: Interactive wizard (manual)"
echo "Please complete the wizard interactively..."
cargo run -- init
echo "✓ Test 1 complete"
echo ""

# Clean state
rm -f ~/.xvnrc
rm -rf ~/.xvn

echo "Test 2: Quick mode"
cargo run -- init --quick
if [ -f ~/.xvnrc ]; then
    echo "✓ Config created"
else
    echo "✗ Config missing"
    exit 1
fi
echo ""

echo "Test 3: Non-interactive mode"
cargo run -- init --non-interactive --force
echo "✓ Non-interactive complete"
echo ""

echo "Test 4: Setup alias"
cargo run -- setup --force
echo "✓ Setup alias works"
echo ""

echo "=== All manual tests passed ==="
```

2. Create test checklist:
```markdown
# Manual Testing Checklist for Milestone 7

## Environment Setup
- [ ] Clean system (no ~/.xvnrc, no ~/.xvn)
- [ ] nvm installed
- [ ] fnm NOT installed

## Interactive Wizard Tests
- [ ] Run `xvn init`
- [ ] Verify header displays correctly
- [ ] Step 1: Confirm shell detection (press Enter)
- [ ] Step 2: Select nvm (should be pre-selected)
- [ ] Step 3: Choose "Prompt" mode
- [ ] Step 4: Keep default version files
- [ ] Step 5: Review looks correct
- [ ] Confirm final step
- [ ] Verify success message shows config path
- [ ] Check ~/.xvnrc exists and is valid YAML
- [ ] Check ~/.xvn/bin/xvn.sh exists
- [ ] Check shell profile was modified

## Quick Mode Tests
- [ ] Run `xvn init --quick`
- [ ] Verify completes in <5 seconds
- [ ] No prompts appear
- [ ] Config created correctly
- [ ] All defaults used

## Re-run Tests
- [ ] Run `xvn init` again (config exists)
- [ ] Verify prompts to overwrite
- [ ] Cancel (n)
- [ ] Config preserved
- [ ] Run `xvn init --force`
- [ ] Verify overwrites without prompt

## Edge Cases
- [ ] Run in directory without write permissions
- [ ] Run with no version managers installed
- [ ] Run in non-TTY environment
- [ ] Press Ctrl+C during wizard
- [ ] Verify graceful cancellation

## Backward Compatibility
- [ ] Run `xvn setup --force`
- [ ] Verify works same as `init --quick`

## Post-Setup Verification
- [ ] Source shell profile
- [ ] Create test directory with .nvmrc
- [ ] cd into directory
- [ ] Verify auto-activation works
- [ ] Run `xvn status`
- [ ] Verify shows correct config
```

3. Execute manual tests:
```bash
chmod +x scripts/test-wizard.sh
./scripts/test-wizard.sh
```

4. Go through checklist systematically

5. Document any issues found

**Key Considerations:**
- Test on clean system for accurate results
- Test with different shells (bash and zsh)
- Test with various version manager combinations
- Get feedback from another person

**Testing:**
- Run through complete checklist
- Fix any issues found
- Re-test until all items pass

**Dependencies:**
- Requires: All implementation complete

---

### Task 7.4: Update documentation

**Objective:** Update README and docs to explain new init command.

**Implementation Steps:**

1. Update `README.md` installation section:
```markdown
## Installation

### Quick Start

```bash
npm install -g @olvrcc/xvn
xvn init
```

The `xvn init` command will guide you through an interactive setup wizard.

For quick setup with defaults:
```bash
xvn init --quick
```

### What the wizard does

The wizard helps you configure:
- **Shell integration** (bash or zsh)
- **Version managers** (nvm, fnm, n)
- **Auto-install behavior** (prompt, always, or never)
- **Version files** (.nvmrc, .node-version, etc.)

### Manual Setup

If you prefer manual configuration, create `~/.xvnrc`:

```yaml
plugins:
  - nvm
auto_install: prompt
version_files:
  - .nvmrc
  - .node-version
```

Then run: `xvn setup --force`
```

2. Add new section to `README.md`:
```markdown
## Configuration

xvn is configured via `~/.xvnrc`. The easiest way to create or modify
your configuration is to run the interactive wizard:

```bash
xvn init
```

### Configuration Options

**plugins** - Version managers in priority order
```yaml
plugins:
  - nvm   # Try nvm first
  - fnm   # Fall back to fnm
```

**auto_install** - What to do when version not found
```yaml
auto_install: prompt  # Options: prompt, always, never
```

**version_files** - Which files to check for version
```yaml
version_files:
  - .nvmrc
  - .node-version
```

### Re-configuring

Run `xvn init` again to modify your configuration through the wizard.
```

3. Update command reference:
```markdown
## Commands

### `xvn init`

Interactive setup wizard. Guides you through configuration with
auto-detection and helpful explanations.

**Options:**
- `--quick` - Skip wizard, use smart defaults
- `--force` - Overwrite existing configuration
- `--non-interactive` - For CI/automation (same as --quick)

**Examples:**
```bash
xvn init                    # Interactive wizard
xvn init --quick            # Quick setup with defaults
xvn init --force            # Overwrite existing config
```

### `xvn setup` (deprecated)

Alias for `xvn init --quick`. Maintained for backward compatibility.
Use `xvn init` for better experience.
```

4. Add troubleshooting section:
```markdown
## Troubleshooting

### Wizard doesn't start
- Make sure you're in an interactive terminal (not a script)
- Use `xvn init --quick` for non-interactive setup
- Check that stdin is a TTY

### No version managers detected
The wizard will warn you if no version managers are found. Install one:
- [nvm](https://github.com/nvm-sh/nvm)
- [fnm](https://github.com/Schniz/fnm)
- [n](https://github.com/tj/n)

### Configuration not working
1. Check `~/.xvnrc` is valid YAML
2. Run `xvn init --force` to regenerate
3. Verify shell profile was modified
4. Restart your shell
```

5. Add screenshots (if applicable):
```bash
# Take screenshots of wizard steps
# Add to docs/images/wizard-*.png
# Reference in README
```

**Key Considerations:**
- Keep documentation concise but complete
- Provide examples for common use cases
- Add troubleshooting for expected issues
- Update all references to old `setup` command

**Testing:**
- Follow documentation as a new user would
- Verify all commands work as documented
- Check all links are valid

**Dependencies:**
- Requires: Implementation complete and tested

---

### Task 7.5: Update CHANGELOG

**Objective:** Document changes for v0.8.0 release.

**Implementation Steps:**

1. Add entry to `CHANGELOG.md`:
```markdown
# Changelog

All notable changes to this project will be documented in this file.

## [0.8.0] - 2025-10-XX

### Added

- **Interactive Setup Wizard** - New `xvn init` command with guided configuration
  - Auto-detects shell (bash/zsh) and version managers (nvm/fnm/n)
  - Educational prompts explaining each configuration option
  - Configuration preview before saving
  - Shows config file location and next steps
  - Supports `--quick` flag for fast setup with defaults
  - Supports `--non-interactive` for CI/automation

- **Enhanced Configuration**
  - New plugin priority system
  - Configurable version file priority
  - Clear auto-install mode selection

- **Improved User Experience**
  - Branded output with "xvn - automatic node version switching" header
  - Colored success/error messages throughout wizard
  - Helpful error messages with actionable next steps
  - Config file location clearly communicated

### Changed

- `xvn setup` is now an alias for `xvn init --quick` (backward compatible)
- Configuration file includes helpful comments and timestamp
- Setup process provides more feedback and explanation

### Fixed

- Word-splitting bug in zsh for version file detection
- Shell integration now works correctly in both bash and zsh

### Dependencies

- Added `inquire 0.7` for interactive terminal prompts

---

## [0.7.0] - 2025-10-02

### Added

- Colored CLI output with branded "xvn:" prefix
- ...
```

2. Update version links at bottom:
```markdown
[0.8.0]: https://github.com/cameronolivier/xvn/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/cameronolivier/xvn/compare/v0.6.1...v0.7.0
```

**Key Considerations:**
- Follow Keep a Changelog format
- Group changes by Added/Changed/Fixed/Removed
- Be specific about user-facing changes
- Include upgrade notes if needed

**Testing:**
- Verify changelog is accurate
- Check links work
- Ensure all major changes documented

**Dependencies:**
- Requires: All implementation and testing complete

---

### Task 8.1: Code review and refinement

**Objective:** Ensure code quality before release.

**Implementation Steps:**

1. Run clippy with strict settings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

2. Fix all clippy warnings:
- Unused imports
- Unnecessary clones
- Inefficient patterns
- Missing documentation

3. Review error handling:
```rust
// Ensure all errors have context
.context("Failed to write config file")?

// Ensure user-facing errors are helpful
Err(ActivationError::NoPluginsAvailable)
```

4. Review code organization:
- Check module boundaries are clear
- Verify functions are appropriately sized
- Look for code duplication
- Ensure consistent naming

5. Run formatter:
```bash
cargo fmt --all
```

6. Check documentation:
```bash
cargo doc --open
```
- Verify all public items documented
- Check examples are clear
- Fix any broken doc links

7. Performance check:
```bash
time cargo run -- init --quick
# Should complete in <5 seconds
```

**Code Quality Checklist:**
- [ ] No clippy warnings
- [ ] All code formatted
- [ ] All public items documented
- [ ] Error messages are user-friendly
- [ ] No TODOs in production code
- [ ] Tests pass
- [ ] No println! in production code (use log!)

**Key Considerations:**
- Focus on user-facing quality
- Don't over-optimize
- Consistency matters

**Testing:**
```bash
make check  # Run all quality checks
```

**Dependencies:**
- Requires: Implementation complete

---

### Task 8.2: Update Makefile

**Objective:** Add convenience commands for wizard testing.

**Implementation Steps:**

1. Add to `Makefile`:
```makefile
# Testing
wizard-test:
	@echo "Running wizard test (interactive)..."
	@cargo run -- init

wizard-test-quick:
	@echo "Running quick setup test..."
	@cargo run -- init --quick --force

wizard-test-clean:
	@echo "Cleaning test artifacts..."
	@rm -f ~/.xvnrc
	@rm -rf ~/.xvn
	@echo "✅ Test environment cleaned"

# Development
test-wizard: wizard-test-clean wizard-test-quick wizard-test
	@echo "✅ Wizard tests complete"
```

2. Update help text:
```makefile
help:
	@echo "xvn development commands:"
	@echo ""
	@echo "Development:"
	@echo "  make dev              Build and install for local development"
	@echo "  make wizard-test      Test interactive wizard"
	@echo "  make wizard-test-quick Test quick setup"
	# ... rest of help
```

**Key Considerations:**
- Make testing easy
- Don't break existing commands
- Document new commands

**Testing:**
```bash
make wizard-test-quick
make wizard-test-clean
```

**Dependencies:**
- None

---

### Task 8.3: Prepare release

**Objective:** Build and tag release for v0.8.0.

**Implementation Steps:**

1. Verify all tests pass:
```bash
cargo test --all-features
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
```

2. Update version using script:
```bash
./scripts/version.sh minor
# Or manually if at 0.7.0 -> 0.8.0
```

3. Verify version updated in:
- `Cargo.toml`
- `package.json`
- `tests/cli_test.rs`

4. Build release binaries:
```bash
cargo build --release
```

5. Test release binary:
```bash
./target/release/xvn --version
./target/release/xvn init --quick
```

6. Create git commit and tag:
```bash
git add -A
git commit -m "chore: bump version to v0.8.0"
git tag -a "v0.8.0" -m "Release v0.8.0

### Added
- Interactive setup wizard (xvn init)
- Auto-detection of shell and version managers
- Configuration preview and validation
- Educational prompts

### Changed
- xvn setup is now alias for xvn init

### Fixed
- zsh word-splitting bug in shell integration"
```

7. Verify tag:
```bash
git tag -l -n9 v0.8.0
```

**Release Checklist:**
- [ ] All tests pass
- [ ] Version updated in all files
- [ ] CHANGELOG.md updated
- [ ] README.md updated
- [ ] Git tag created
- [ ] Release binary tested
- [ ] CI build passes

**Key Considerations:**
- Don't push tags until everything verified
- Test release binary thoroughly
- Double-check version numbers

**Testing:**
```bash
# Install from source
cargo install --path .
xvn --version  # Should show 0.8.0
xvn init
```

**Dependencies:**
- Requires: All tasks complete
- Requires: Documentation updated

---

### Task 8.4: Gather feedback

**Objective:** Collect user feedback before public release.

**Implementation Steps:**

1. Identify beta testers (5-10 people):
- Developers using Node.js
- Mix of macOS/Linux users
- Mix of nvm/fnm users
- Include someone unfamiliar with xvn

2. Create beta testing guide:
```markdown
# XVN v0.8.0 Beta Testing Guide

Thank you for helping test xvn v0.8.0!

## Installation

```bash
cargo install --git https://github.com/cameronolivier/xvn --tag v0.8.0-beta
```

## What to Test

1. **Initial Setup**
   - Run `xvn init`
   - Complete the wizard
   - Report any confusing prompts

2. **Functionality**
   - Test auto-activation works
   - Try different version files
   - Report any issues

## Feedback Form

Please provide feedback on:
- [ ] Was the wizard easy to understand?
- [ ] Were any prompts confusing?
- [ ] Did auto-detection work correctly?
- [ ] Did you encounter any errors?
- [ ] What would make it better?

Submit feedback: [GitHub Issues](https://github.com/cameronolivier/xvn/issues)
```

3. Send to beta testers with deadline (1 week)

4. Collect feedback via:
- GitHub issues
- Direct messages
- Feedback form

5. Categorize feedback:
- **Critical** - Blocks release, must fix
- **Important** - Should fix before release
- **Nice-to-have** - Can defer to next version

6. Address critical and important feedback:
```bash
# Create issues for each item
# Fix critical bugs
# Make UX improvements
# Update documentation
```

7. Iterate if needed:
```bash
# Make fixes
# Create v0.8.0-beta2
# Re-test with same users
```

**Feedback Categories to Watch:**
- Confusing prompts or instructions
- Unexpected behavior
- Missing options
- Error messages not helpful
- Performance issues

**Key Considerations:**
- Give testers enough time
- Make it easy to provide feedback
- Be responsive to issues
- Don't over-iterate (diminishing returns)

**Testing:**
- Verify all critical feedback addressed
- Ensure no regressions from changes

**Dependencies:**
- Requires: Release candidate ready
- Requires: Beta testers identified

---

## Integration Points

### Reused Components
- `setup::SetupInstaller` - Shell integration installation
- `setup::shell_detection::Shell` - Shell type enum
- `config::Config` - Configuration structure
- `config::AutoInstallMode` - Auto-install enum
- `output` module - Colored output functions

### New Dependencies
- `inquire` crate - Interactive prompts
- `chrono` (optional) - Timestamps in config

### CLI Integration
- `Commands::Init` - New primary command
- `Commands::Setup` - Alias for backward compatibility
- Flags: `--quick`, `--force`, `--non-interactive`, `--shell`

## Testing Strategy

### Unit Tests
- Test each detection function with mocked environment
- Test validation functions with various configs
- Test state management and conversions
- Test config generation and parsing

### Integration Tests
```rust
// tests/wizard_test.rs
#[test]
fn test_quick_setup() {
    // Test quick mode end-to-end
    let result = run_quick_setup(true);
    assert!(result.is_ok());

    // Verify config was written
    let config_path = home_dir().unwrap().join(".xvnrc");
    assert!(config_path.exists());

    // Verify config is valid
    let config = Config::load().unwrap();
    assert!(!config.version_files.is_empty());
}
```

### Manual Testing Checklist
- [ ] Run wizard on clean system
- [ ] Run wizard with existing config
- [ ] Test each option in each prompt
- [ ] Test cancelling at different steps
- [ ] Test `--quick` mode
- [ ] Test `--non-interactive` mode
- [ ] Test with nvm installed
- [ ] Test with fnm installed
- [ ] Test with no version managers
- [ ] Test in non-TTY environment
- [ ] Verify generated config works
- [ ] Test re-running wizard to modify

## Success Criteria

- [ ] Wizard completes without errors
- [ ] All prompts are clear and understandable
- [ ] Auto-detection works correctly
- [ ] Generated config is valid YAML
- [ ] Config file location is clearly shown
- [ ] Shell integration works after wizard
- [ ] Quick mode completes in <5 seconds
- [ ] Non-interactive mode works in CI
- [ ] Error messages are helpful
- [ ] Code coverage >85%
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Ready for v0.8.0 release

## Timeline Estimate

**Week 1:**
- Tasks 1.1-1.3: Foundation (1 day)
- Tasks 2.1-2.3: Detection (2 days)
- Tasks 3.1-3.3: First prompts (2 days)

**Week 2:**
- Tasks 3.4-3.5: Remaining prompts (1 day)
- Tasks 4.1-4.4: Wizard orchestration (2 days)
- Tasks 5.1-5.4: Config handling (2 days)

**Week 3:**
- Tasks 6.1-6.4: Output and polish (1 day)
- Tasks 7.1-7.5: Testing and documentation (3 days)
- Task 8.1-8.4: Review and release (1 day)

**Total:** ~15 working days (3 weeks)
