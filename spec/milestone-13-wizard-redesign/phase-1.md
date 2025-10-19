# Phase 1: Visual Components

**Status**: Not Started
**Version Target**: v2.1.0
**Duration Estimate**: 2-3 hours
**Phase Tasks**: 1.1 - 1.3

## Overview

Phase 1 establishes the visual foundation for the redesigned wizard by creating reusable components for timeline-style progress indicators, summary displays, and compact prompts. These components will be used throughout the wizard to create a modern, polished CLI experience inspired by Vite and ShadCN.

**Why Phase 1 is Essential:**
- Creates the visual building blocks needed by all other phases
- Establishes consistent styling and color scheme across the wizard
- Provides reusable functions that simplify later implementation
- Enables early visual testing to ensure the design works on different terminals

**⚠️ CHECKPOINT**: Before starting Phase 1, ensure:
- You have access to both dark and light terminal themes for testing
- The `inquire` and `owo-colors` crates are in `Cargo.toml`
- You've reviewed the Vite CLI and ShadCN CLI for visual reference
- The `BRAND_COLOR` constant exists in `src/output.rs` (if not, add it in Task 1.1)

---

## Implementation Tasks

### Task 1.1: Create Timeline Module with Box-Drawing Characters

**Goal**: Build a reusable module for rendering timeline-style progress indicators.

**File**: `src/init/timeline.rs` (new file)

**Content Requirements**:

Create the following complete module structure:

```rust
//! Timeline rendering for wizard progress display
//!
//! Provides box-drawing characters and functions to render
//! timeline-style progress indicators with colored step states.

use owo_colors::OwoColorize;
use crate::output::BRAND_COLOR;

/// Box-drawing characters for timeline display
pub mod chars {
    pub const STEP_PENDING: &str = "◇";
    pub const STEP_ACTIVE: &str = "◆";
    pub const STEP_COMPLETE: &str = "✓";
    pub const VERTICAL: &str = "│";
    pub const BRANCH_RIGHT: &str = "├─";
    pub const BRANCH_LAST: &str = "└─";
    pub const TOP_LEFT: &str = "┌─";
    pub const HORIZONTAL: &str = "─";
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepState {
    Pending,
    Active,
    Complete,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub label: String,
    pub state: StepState,
    pub details: Option<String>,
}

impl Step {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: StepState::Pending,
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn set_state(&mut self, state: StepState) {
        self.state = state;
    }
}

/// Render a single step in the timeline
pub fn render_step(step: &Step) -> String {
    let symbol = match step.state {
        StepState::Pending => chars::STEP_PENDING,
        StepState::Active => chars::STEP_ACTIVE,
        StepState::Complete => chars::STEP_COMPLETE,
    };

    let label = match step.state {
        StepState::Active => step.label.color(BRAND_COLOR).bold().to_string(),
        StepState::Complete => step.label.green().to_string(),
        StepState::Pending => step.label.dimmed().to_string(),
    };

    let mut output = format!("{} {}", symbol, label);

    if let Some(details) = &step.details {
        output.push('\n');
        output.push_str(&format!("{}  {}", chars::VERTICAL, details.dimmed()));
    }

    output
}

/// Render a timeline with multiple steps
pub fn render_timeline(steps: &[Step]) -> String {
    steps
        .iter()
        .map(render_step)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render a box-style container with title and items
pub fn render_box(title: &str, items: &[(&str, &str)]) -> String {
    // Calculate max key length for alignment
    let max_key_len = items.iter().map(|(k, _)| k.len()).max().unwrap_or(0);

    let mut output = format!("{} {}\n", chars::TOP_LEFT, title.bold());
    output.push_str(&format!("{}\n", chars::VERTICAL));

    for (i, (key, value)) in items.iter().enumerate() {
        let prefix = if i == items.len() - 1 {
            chars::BRANCH_LAST
        } else {
            chars::BRANCH_RIGHT
        };
        output.push_str(&format!(
            "{} {:width$}: {}\n",
            prefix,
            key.dimmed(),
            value,
            width = max_key_len
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_new() {
        let step = Step::new("Test Step");
        assert_eq!(step.label, "Test Step");
        assert_eq!(step.state, StepState::Pending);
        assert!(step.details.is_none());
    }

    #[test]
    fn test_step_with_details() {
        let step = Step::new("Test Step").with_details("Found: test value");
        assert!(step.details.is_some());
        assert_eq!(step.details.unwrap(), "Found: test value");
    }

    #[test]
    fn test_step_rendering() {
        let step = Step::new("Test Step").with_details("Found: test value");
        let output = render_step(&step);
        assert!(output.contains("Test Step"));
        assert!(output.contains("Found: test value"));
    }

    #[test]
    fn test_timeline_rendering() {
        let steps = vec![
            Step {
                label: "Step 1".into(),
                state: StepState::Complete,
                details: None,
            },
            Step {
                label: "Step 2".into(),
                state: StepState::Active,
                details: None,
            },
            Step {
                label: "Step 3".into(),
                state: StepState::Pending,
                details: None,
            },
        ];
        let output = render_timeline(&steps);
        assert!(output.contains("✓"));
        assert!(output.contains("◆"));
        assert!(output.contains("◇"));
    }

    #[test]
    fn test_box_rendering() {
        let items = vec![("Shell", "zsh"), ("Plugin", "nvm")];
        let output = render_box("Configuration", &items);
        assert!(output.contains("Configuration"));
        assert!(output.contains("Shell"));
        assert!(output.contains("zsh"));
        assert!(output.contains("┌─"));
        assert!(output.contains("└─"));
    }

    #[test]
    fn test_box_rendering_alignment() {
        let items = vec![
            ("Shell", "zsh"),
            ("Version manager", "nvm"),
            ("X", "short"),
        ];
        let output = render_box("Test", &items);
        // Verify alignment by checking structure
        assert!(output.contains("Version manager"));
    }
}
```

**Changes Required in Other Files**:

1. **File**: `src/init/mod.rs`
   - Add: `pub mod timeline;` to export the new module

2. **File**: `src/output.rs` (if BRAND_COLOR doesn't exist)
   - Add near the top:
     ```rust
     pub const BRAND_COLOR: owo_colors::Rgb = owo_colors::Rgb(50, 205, 50); // Lime green
     ```

**Commands**:

```bash
# Create the new file
touch src/init/timeline.rs

# Run tests to verify implementation
cargo test --lib timeline

# Check formatting
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

**Expected Output**:

```
running 6 tests
test init::timeline::tests::test_box_rendering ... ok
test init::timeline::tests::test_box_rendering_alignment ... ok
test init::timeline::tests::test_step_new ... ok
test init::timeline::tests::test_step_rendering ... ok
test init::timeline::tests::test_step_with_details ... ok
test init::timeline::tests::test_timeline_rendering ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Actions**:
- [ ] Create `src/init/timeline.rs` with the complete module code
- [ ] Add `pub mod timeline;` to `src/init/mod.rs`
- [ ] Verify `BRAND_COLOR` exists in `src/output.rs`, add if missing
- [ ] Run `cargo test --lib timeline` and ensure all 6 tests pass
- [ ] Run `cargo fmt` to format the code
- [ ] Run `cargo clippy -- -D warnings` and fix any issues
- [ ] Manually test rendering by adding a temporary test in `main.rs`:
  ```rust
  use anvs::init::timeline::{Step, StepState, render_timeline, render_box};

  fn main() {
      let steps = vec![
          Step { label: "Shell detection".into(), state: StepState::Complete, details: Some("Found: zsh".into()) },
          Step { label: "Version manager".into(), state: StepState::Active, details: None },
      ];
      println!("{}", render_timeline(&steps));

      let items = vec![("Shell", "zsh"), ("Version manager", "nvm")];
      println!("\n{}", render_box("Configuration", &items));
  }
  ```
- [ ] Run `cargo run` and visually verify output looks good
- [ ] Test on both dark and light terminal themes
- [ ] Remove the temporary test from `main.rs`

---

### Task 1.2: Create Summary Display Module

**Goal**: Build functions to format detection results, configuration previews, and next steps messages.

**File**: `src/init/summary.rs` (new file)

**Content Requirements**:

```rust
//! Summary and status display formatting for the wizard
//!
//! Provides functions to format detection results, configuration previews,
//! and completion messages using the timeline module.

use crate::config::{AutoInstallMode, Config};
use crate::init::timeline;
use crate::setup::shell_detection::Shell;
use owo_colors::OwoColorize;

/// Results from auto-detection of shell and version managers
#[derive(Debug, Clone)]
pub struct DetectionResults {
    pub shell: Option<Shell>,
    pub shell_path: Option<String>,
    pub version_managers: Vec<String>,
    pub config_path: String,
    pub auto_install: AutoInstallMode,
}

impl DetectionResults {
    pub fn new() -> Self {
        Self {
            shell: None,
            shell_path: None,
            version_managers: Vec::new(),
            config_path: "~/.anvsrc".to_string(),
            auto_install: AutoInstallMode::Prompt,
        }
    }
}

impl Default for DetectionResults {
    fn default() -> Self {
        Self::new()
    }
}

/// Format detection results as a box-style summary
pub fn format_detection_summary(results: &DetectionResults) -> String {
    let mut items = Vec::new();

    // Shell
    if let Some(shell) = &results.shell {
        let shell_info = if let Some(path) = &results.shell_path {
            format!("{} ({})", shell.name(), path)
        } else {
            shell.name().to_string()
        };
        items.push(("Shell", shell_info));
    } else {
        items.push(("Shell", "Not detected".dimmed().to_string()));
    }

    // Version manager
    if !results.version_managers.is_empty() {
        let vm_list = results.version_managers.join(", ");
        items.push(("Version manager", vm_list));
    } else {
        items.push((
            "Version manager",
            "Not detected".dimmed().to_string(),
        ));
    }

    // Config location
    items.push(("Config location", results.config_path.clone()));

    // Auto-install mode
    let mode_str = match results.auto_install {
        AutoInstallMode::Always => "Always",
        AutoInstallMode::Prompt => "Prompt when needed",
        AutoInstallMode::Never => "Never",
    };
    items.push(("Auto-install", mode_str.to_string()));

    // Convert to vec of string tuples for render_box
    let items_ref: Vec<(&str, &str)> = items
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    timeline::render_box("Initializing anvs", &items_ref)
}

/// Format a configuration preview before applying
pub fn format_config_preview(config: &Config, shell: &Shell) -> String {
    let items = vec![
        ("Shell", shell.name()),
        ("Version manager", &config.plugins.join(", ")),
        ("Auto-install", &format_auto_install(&config.auto_install)),
        ("Config", "~/.anvsrc"),
    ];

    timeline::render_box("Configuration Summary", &items)
}

/// Format auto-install mode as a string
fn format_auto_install(mode: &AutoInstallMode) -> String {
    match mode {
        AutoInstallMode::Always => "Always".to_string(),
        AutoInstallMode::Prompt => "Prompt".to_string(),
        AutoInstallMode::Never => "Never".to_string(),
    }
}

/// Format next steps message after successful setup
pub fn format_next_steps(shell: &Shell) -> String {
    let shell_rc = match shell {
        Shell::Zsh => "~/.zshrc",
        Shell::Bash => "~/.bashrc",
        Shell::Unknown(_) => "your shell config",
    };

    let mut output = String::new();
    output.push_str(&"Next steps:".bold().to_string());
    output.push('\n');
    output.push_str(&format!(
        "  1. Restart your shell or run: {}\n",
        format!("source {}", shell_rc).cyan()
    ));
    output.push_str("  2. Navigate to a project with .nvmrc\n");
    output.push_str("  3. Watch anvs activate automatically!\n");
    output.push('\n');
    output.push_str(&format!(
        "Example: {}\n",
        "cd ~/my-project && anvs status".dimmed()
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_results_new() {
        let results = DetectionResults::new();
        assert!(results.shell.is_none());
        assert!(results.version_managers.is_empty());
        assert_eq!(results.config_path, "~/.anvsrc");
    }

    #[test]
    fn test_detection_summary_with_all_detected() {
        let mut results = DetectionResults::new();
        results.shell = Some(Shell::Zsh);
        results.shell_path = Some("/bin/zsh".to_string());
        results.version_managers = vec!["nvm".to_string()];

        let output = format_detection_summary(&results);
        assert!(output.contains("zsh"));
        assert!(output.contains("nvm"));
        assert!(output.contains("Initializing anvs"));
    }

    #[test]
    fn test_detection_summary_with_nothing_detected() {
        let results = DetectionResults::new();
        let output = format_detection_summary(&results);
        assert!(output.contains("Not detected"));
    }

    #[test]
    fn test_config_preview() {
        let config = Config {
            plugins: vec!["nvm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string()],
            use_default: true,
        };
        let shell = Shell::Zsh;

        let output = format_config_preview(&config, &shell);
        assert!(output.contains("zsh"));
        assert!(output.contains("nvm"));
        assert!(output.contains("Prompt"));
        assert!(output.contains("Configuration Summary"));
    }

    #[test]
    fn test_next_steps_zsh() {
        let output = format_next_steps(&Shell::Zsh);
        assert!(output.contains("Next steps:"));
        assert!(output.contains("source ~/.zshrc"));
        assert!(output.contains("Navigate to a project"));
    }

    #[test]
    fn test_next_steps_bash() {
        let output = format_next_steps(&Shell::Bash);
        assert!(output.contains("source ~/.bashrc"));
    }

    #[test]
    fn test_format_auto_install() {
        assert_eq!(format_auto_install(&AutoInstallMode::Always), "Always");
        assert_eq!(format_auto_install(&AutoInstallMode::Prompt), "Prompt");
        assert_eq!(format_auto_install(&AutoInstallMode::Never), "Never");
    }
}
```

**Changes Required in Other Files**:

1. **File**: `src/init/mod.rs`
   - Add: `pub mod summary;` to export the new module

**Commands**:

```bash
# Create the new file
touch src/init/summary.rs

# Run tests to verify implementation
cargo test --lib summary

# Check formatting
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

**Expected Output**:

```
running 7 tests
test init::summary::tests::test_config_preview ... ok
test init::summary::tests::test_detection_results_new ... ok
test init::summary::tests::test_detection_summary_with_all_detected ... ok
test init::summary::tests::test_detection_summary_with_nothing_detected ... ok
test init::summary::tests::test_format_auto_install ... ok
test init::summary::tests::test_next_steps_bash ... ok
test init::summary::tests::test_next_steps_zsh ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Actions**:
- [ ] Create `src/init/summary.rs` with the complete module code
- [ ] Add `pub mod summary;` to `src/init/mod.rs`
- [ ] Run `cargo test --lib summary` and ensure all 7 tests pass
- [ ] Run `cargo fmt` to format the code
- [ ] Run `cargo clippy -- -D warnings` and fix any issues
- [ ] Manually test rendering by adding to the temporary test in `main.rs`:
  ```rust
  use anvs::init::summary::{DetectionResults, format_detection_summary, format_next_steps};
  use anvs::setup::shell_detection::Shell;

  fn test_summary() {
      let mut results = DetectionResults::new();
      results.shell = Some(Shell::Zsh);
      results.shell_path = Some("/bin/zsh".into());
      results.version_managers = vec!["nvm".into()];

      println!("{}", format_detection_summary(&results));
      println!("\n{}", format_next_steps(&Shell::Zsh));
  }
  ```
- [ ] Run `cargo run` and visually verify formatting
- [ ] Test on both dark and light terminal themes
- [ ] Remove the temporary test

---

### Task 1.3: Refactor Prompts Module for Compact Display

**Goal**: Simplify existing prompts to show detected values inline with cleaner formatting.

**File**: `src/init/prompts.rs` (existing file - refactor)

**Changes Required**:

1. **Add new imports at the top**:
   ```rust
   use crate::init::summary::DetectionResults;
   use inquire::Select;
   use anyhow::Result;
   ```

2. **Add QuickModeChoice enum** (before existing code):
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq)]
   pub enum QuickModeChoice {
       Proceed,
       Customize,
       Cancel,
   }
   ```

3. **Add new function: prompt_quick_mode_confirmation**:
   ```rust
   /// Quick mode confirmation prompt
   pub fn prompt_quick_mode_confirmation(
       results: &DetectionResults,
   ) -> Result<QuickModeChoice> {
       let options = vec!["Yes, continue", "Customize settings", "Cancel"];

       let choice = Select::new("Proceed with this configuration?", options).prompt()?;

       match choice {
           "Yes, continue" => Ok(QuickModeChoice::Proceed),
           "Customize settings" => Ok(QuickModeChoice::Customize),
           _ => Ok(QuickModeChoice::Cancel),
       }
   }
   ```

4. **Add new function: prompt_shell** (or refactor existing if present):
   ```rust
   /// Shell selection prompt with inline detection
   pub fn prompt_shell(detected: Option<&Shell>) -> Result<Shell> {
       let message = if let Some(shell) = detected {
           format!("Which shell? (detected: {})", shell.name())
       } else {
           "Which shell? (auto-detection failed)".to_string()
       };

       let mut options = vec![];

       // Add detected shell first if available
       if let Some(shell) = detected {
           options.push(format!("{} (recommended)", shell.name()));
       }

       // Add other options
       if detected.is_none() || !matches!(detected, Some(Shell::Zsh)) {
           options.push("zsh".to_string());
       }
       if detected.is_none() || !matches!(detected, Some(Shell::Bash)) {
           options.push("bash".to_string());
       }
       options.push("Custom path".to_string());

       let selected = Select::new(&message, options)
           .with_starting_cursor(0) // Pre-select first option
           .prompt()?;

       // Parse selection
       if selected.contains("zsh") {
           Ok(Shell::Zsh)
       } else if selected.contains("bash") {
           Ok(Shell::Bash)
       } else if selected.contains("Custom") {
           // Handle custom path - you may need to implement this
           prompt_custom_shell_path()
       } else if let Some(shell) = detected {
           Ok(shell.clone())
       } else {
           Err(anyhow::anyhow!("Invalid shell selection"))
       }
   }

   fn prompt_custom_shell_path() -> Result<Shell> {
       use inquire::Text;
       let path = Text::new("Enter shell path:").prompt()?;
       Ok(Shell::Unknown(path))
   }
   ```

5. **Add new function: prompt_version_manager**:
   ```rust
   /// Version manager selection with detection
   pub fn prompt_version_manager(detected: Vec<String>) -> Result<Vec<String>> {
       let has_nvm = detected.contains(&"nvm".to_string());
       let has_fnm = detected.contains(&"fnm".to_string());

       let message = if !detected.is_empty() {
           format!("Which version manager? (detected: {})", detected.join(", "))
       } else {
           "Which version manager?".to_string()
       };

       let mut options = vec![];

       if has_nvm {
           options.push("nvm (detected, recommended)");
       } else {
           options.push("nvm");
       }

       if has_fnm {
           options.push("fnm (detected)");
       } else {
           options.push("fnm");
       }

       options.push("Multiple (advanced)");

       let selected = Select::new(&message, options)
           .with_starting_cursor(0)
           .prompt()?;

       if selected.contains("nvm") {
           Ok(vec!["nvm".to_string()])
       } else if selected.contains("fnm") {
           Ok(vec!["fnm".to_string()])
       } else {
           prompt_multiple_version_managers(&detected)
       }
   }

   fn prompt_multiple_version_managers(detected: &[String]) -> Result<Vec<String>> {
       use inquire::MultiSelect;

       let options = vec!["nvm", "fnm"];
       let defaults = detected.iter()
           .filter(|d| options.contains(&d.as_str()))
           .map(|s| s.as_str())
           .collect::<Vec<_>>();

       let selected = MultiSelect::new("Select version managers:", options)
           .with_default(&defaults)
           .prompt()?;

       if selected.is_empty() {
           Err(anyhow::anyhow!("At least one version manager must be selected"))
       } else {
           Ok(selected.iter().map(|s| s.to_string()).collect())
       }
   }
   ```

6. **Add new function: prompt_auto_install**:
   ```rust
   /// Auto-install mode selection
   pub fn prompt_auto_install() -> Result<AutoInstallMode> {
       let options = vec![
           "Prompt (recommended) - Ask before installing",
           "Always - Install automatically",
           "Never - Manual installation only",
       ];

       let selected = Select::new("Auto-install missing versions?", options)
           .with_starting_cursor(0) // Default to Prompt
           .prompt()?;

       if selected.contains("Always") {
           Ok(AutoInstallMode::Always)
       } else if selected.contains("Never") {
           Ok(AutoInstallMode::Never)
       } else {
           Ok(AutoInstallMode::Prompt)
       }
   }
   ```

**Commands**:

```bash
# Check that the file compiles
cargo check

# Run all tests
cargo test --lib prompts

# Format
cargo fmt

# Clippy
cargo clippy -- -D warnings
```

**Expected Output**:

```
Checking anvs v2.0.0 (/path/to/anvs)
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
```

**Actions**:
- [ ] Open `src/init/prompts.rs` in your editor
- [ ] Add the new imports at the top of the file
- [ ] Add the `QuickModeChoice` enum
- [ ] Add all the new prompt functions: `prompt_quick_mode_confirmation`, `prompt_shell`, `prompt_version_manager`, `prompt_auto_install`
- [ ] Add helper functions: `prompt_custom_shell_path`, `prompt_multiple_version_managers`
- [ ] Run `cargo check` to verify it compiles
- [ ] Run `cargo test --lib prompts` (may need to add tests later)
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`
- [ ] Manually test prompts (optional at this stage, will be tested in Phase 2):
  ```rust
  // In main.rs temporarily
  use anvs::init::prompts::*;
  use anvs::setup::shell_detection::Shell;

  fn test_prompts() {
      let detected = Some(&Shell::Zsh);
      if let Ok(shell) = prompt_shell(detected) {
          println!("Selected shell: {:?}", shell);
      }
  }
  ```

---

## Verification Checklist

Before proceeding to Phase 2, verify ALL of the following:

- [ ] File `src/init/timeline.rs` exists with complete implementation
- [ ] File `src/init/summary.rs` exists with complete implementation
- [ ] File `src/init/prompts.rs` has been updated with new prompt functions
- [ ] File `src/init/mod.rs` exports both `timeline` and `summary` modules
- [ ] File `src/output.rs` contains `BRAND_COLOR` constant
- [ ] All timeline tests pass (6 tests)
- [ ] All summary tests pass (7 tests)
- [ ] `cargo check` completes without errors
- [ ] `cargo fmt` has been run on all modified files
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] Visual rendering has been tested on dark terminal theme
- [ ] Visual rendering has been tested on light terminal theme
- [ ] Box-drawing characters render correctly (no mojibake)
- [ ] Colors are visible and pleasant in both themes
- [ ] No compilation errors or warnings

---

## Success Criteria

Phase 1 is complete when:

1. ✅ Timeline module (`src/init/timeline.rs`) is fully implemented with tests passing
2. ✅ Summary module (`src/init/summary.rs`) is fully implemented with tests passing
3. ✅ Prompts module (`src/init/prompts.rs`) has new compact prompt functions
4. ✅ All modules are properly exported in `src/init/mod.rs`
5. ✅ Visual output has been manually verified on both dark and light themes
6. ✅ Code is formatted and passes clippy checks
7. ✅ All verification checklist items are completed

---

## Next Steps

After completing Phase 1:

1. Run a final `cargo test` to ensure all tests pass
2. Commit your changes:
   ```bash
   git add src/init/timeline.rs src/init/summary.rs src/init/prompts.rs src/init/mod.rs src/output.rs
   git commit -m "feat(init): add visual components for wizard redesign (Phase 1)

   - Add timeline module with box-drawing characters
   - Add summary display module for detection results
   - Add compact prompts with inline detection
   - All tests passing (13 new tests)

   Files changed:
   - src/init/timeline.rs (new)
   - src/init/summary.rs (new)
   - src/init/prompts.rs (refactored)
   - src/init/mod.rs (exports)
   - src/output.rs (BRAND_COLOR constant)"
   ```
3. **Proceed to Phase 2**: Quick Mode Implementation

---

## Rollback Plan

If issues are discovered in Phase 1:

1. To rollback timeline module:
   ```bash
   git checkout HEAD -- src/init/timeline.rs
   rm src/init/timeline.rs
   # Remove `pub mod timeline;` from src/init/mod.rs
   ```

2. To rollback summary module:
   ```bash
   git checkout HEAD -- src/init/summary.rs
   rm src/init/summary.rs
   # Remove `pub mod summary;` from src/init/mod.rs
   ```

3. To rollback prompts refactor:
   ```bash
   git checkout HEAD -- src/init/prompts.rs
   ```

4. To rollback all Phase 1 changes:
   ```bash
   git reset --hard HEAD~1  # If committed
   # OR
   git checkout HEAD -- src/init/  # If not committed
   ```

---

## Notes

- **Terminal Compatibility**: The unicode box-drawing characters used in this phase are well-supported in modern terminals (iTerm2, Terminal.app, gnome-terminal, konsole). If you encounter rendering issues, verify your terminal font supports these characters.

- **Color Testing**: The lime green brand color (RGB 50, 205, 50) should be tested on both dark and light backgrounds. If it's too bright or hard to read, adjust the RGB values in `BRAND_COLOR`.

- **Prompt Testing**: The new prompt functions won't be fully testable until they're integrated in Phase 2. Focus on ensuring they compile and the logic is correct.

- **Dependencies**: This phase has no external dependencies on other phases. You can complete it independently.

- **Estimated Time**:
  - Task 1.1: 45-60 minutes (timeline module + tests)
  - Task 1.2: 45-60 minutes (summary module + tests)
  - Task 1.3: 30-45 minutes (prompts refactor)
  - Total: 2-3 hours

- **Testing Strategy**: Unit tests cover the core logic. Manual visual testing is required to verify the UI appears correctly. Save screenshots for documentation if desired.
