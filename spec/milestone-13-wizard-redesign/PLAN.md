# Milestone 13: Wizard Redesign - Implementation Plan

## Overview

This milestone redesigns the `anvs init` wizard to be faster, cleaner, and more visually appealing—inspired by modern CLI tools like Vite and ShadCN. The goal is to optimize for speed (< 30 seconds) with smart defaults while maintaining clarity through excellent visual design.

**Key Changes:**
- Quick mode as default (auto-detect + single confirmation)
- Timeline-style progress indicators with box-drawing characters
- Minimal prompts (2-3 maximum) with inline detection display
- Advanced mode available via `--advanced` flag
- Modern color scheme with lime green branding

**Version Target**: v2.1.0

## Prerequisites

Before starting implementation:

1. **Understand current wizard flow**: Read `src/init/wizard.rs`, `src/init/prompts.rs`, `src/init/detection.rs`
2. **Review visual references**: Check out Vite CLI, ShadCN CLI, and inquire crate examples
3. **Terminal testing setup**: Have access to both dark and light terminal themes
4. **Dependencies verified**: Confirm `inquire = "0.7"` and `owo-colors = "4.0"` are in Cargo.toml
5. **Add brand color constant**: Ensure `BRAND_COLOR` is defined in `src/output.rs`:
   ```rust
   pub const BRAND_COLOR: owo_colors::Rgb = owo_colors::Rgb(50, 205, 50); // Lime green
   ```

## Implementation Tasks

---

## Phase 1: Visual Components

### Task 1: Create Timeline Module

**Objective:** Build a reusable module for drawing timeline-style progress indicators with box-drawing characters.

**Implementation Steps:**

1. **Create the module file**:
   ```bash
   touch src/init/timeline.rs
   ```

2. **Define box-drawing character constants**:
   ```rust
   // src/init/timeline.rs

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
   ```

3. **Create `Step` struct with states**:
   ```rust
   use owo_colors::OwoColorize;

   #[derive(Debug, Clone, PartialEq)]
   pub enum StepState {
       Pending,
       Active,
       Complete,
   }

   #[derive(Debug, Clone)]
   pub struct Step {
       pub label: String,
       pub state: StepState,
       pub details: Option<String>, // Optional extra info
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
   ```

4. **Implement timeline rendering functions**:
   ```rust
   use crate::output::BRAND_COLOR;

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
       steps.iter()
           .map(render_step)
           .collect::<Vec<_>>()
           .join("\n")
   }

   /// Render a box-style container with title and items
   pub fn render_box(title: &str, items: &[(&str, &str)]) -> String {
       let mut output = format!("{} {}\n", chars::TOP_LEFT, title.bold());
       output.push_str(&format!("{}\n", chars::VERTICAL));

       for (i, (key, value)) in items.iter().enumerate() {
           let prefix = if i == items.len() - 1 {
               chars::BRANCH_LAST
           } else {
               chars::BRANCH_RIGHT
           };
           output.push_str(&format!("{} {}: {}\n", prefix, key.dimmed(), value));
       }

       output
   }
   ```

5. **Add module export to `src/init/mod.rs`**:
   ```rust
   pub mod timeline;
   ```

**Code Structure:**
- File: `src/init/timeline.rs`
  - Character constants for box drawing
  - `Step` struct with `StepState` enum
  - `render_step()`, `render_timeline()`, `render_box()` functions

**Key Considerations:**
- Use consistent color scheme: lime green for brand, cyan for active, green for complete
- Ensure proper spacing and alignment for visual clarity
- Handle optional details gracefully
- Keep functions pure (no side effects, just string generation)

**Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_rendering() {
        let step = Step::new("Test Step")
            .with_details("Found: test value");
        assert!(render_step(&step).contains("Test Step"));
        assert!(render_step(&step).contains("Found: test value"));
    }

    #[test]
    fn test_timeline_rendering() {
        let steps = vec![
            Step { label: "Step 1".into(), state: StepState::Complete, details: None },
            Step { label: "Step 2".into(), state: StepState::Active, details: None },
        ];
        let output = render_timeline(&steps);
        assert!(output.contains("✓"));
        assert!(output.contains("◆"));
    }

    #[test]
    fn test_box_rendering() {
        let items = vec![("Shell", "zsh"), ("Plugin", "nvm")];
        let output = render_box("Configuration", &items);
        assert!(output.contains("Configuration"));
        assert!(output.contains("Shell"));
    }
}
```

**Dependencies:**
- Requires: None (foundation task)
- Enables: All other visual tasks (summary, prompts, progress)

---

### Task 2: Create Summary Display Module

**Objective:** Build functions to format detection results, configuration previews, and next steps messages.

**Implementation Steps:**

1. **Create the module file**:
   ```bash
   touch src/init/summary.rs
   ```

2. **Define detection results structure**:
   ```rust
   // src/init/summary.rs

   use crate::config::AutoInstallMode;
   use crate::setup::shell_detection::Shell;
   use crate::init::timeline;
   use owo_colors::OwoColorize;

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
   ```

3. **Implement detection results formatter**:
   ```rust
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
           items.push(("Version manager", "Not detected".dimmed().to_string()));
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
       let items_ref: Vec<(&str, &str)> = items.iter()
           .map(|(k, v)| (k.as_str(), v.as_str()))
           .collect();

       timeline::render_box("Initializing anvs", &items_ref)
   }
   ```

4. **Create configuration preview function**:
   ```rust
   use crate::config::Config;

   pub fn format_config_preview(config: &Config, shell: &Shell) -> String {
       let items = vec![
           ("Shell", shell.name()),
           ("Version manager", config.plugins.join(", ").as_str()),
           ("Auto-install", format_auto_install(&config.auto_install).as_str()),
           ("Config", "~/.anvsrc"),
       ];

       timeline::render_box("Configuration Summary", &items)
   }

   fn format_auto_install(mode: &AutoInstallMode) -> String {
       match mode {
           AutoInstallMode::Always => "Always".to_string(),
           AutoInstallMode::Prompt => "Prompt".to_string(),
           AutoInstallMode::Never => "Never".to_string(),
       }
   }
   ```

5. **Add "Next Steps" message builder**:
   ```rust
   pub fn format_next_steps(shell: &Shell) -> String {
       let shell_rc = match shell {
           Shell::Zsh => "~/.zshrc",
           Shell::Bash => "~/.bashrc",
           Shell::Unknown(_) => "your shell config",
       };

       let mut output = String::new();
       output.push_str(&"Next steps:".bold().to_string());
       output.push_str("\n");
       output.push_str(&format!("  1. Restart your shell or run: {}\n",
           format!("source {}", shell_rc).cyan()));
       output.push_str("  2. Navigate to a project with .nvmrc\n");
       output.push_str("  3. Watch anvs activate automatically!\n");

       output
   }
   ```

6. **Add module export**:
   ```rust
   // src/init/mod.rs
   pub mod summary;
   ```

**Code Structure:**
- File: `src/init/summary.rs`
  - `DetectionResults` struct for holding detection data
  - `format_detection_summary()` for initial detection display
  - `format_config_preview()` for final configuration summary
  - `format_next_steps()` for completion message

**Key Considerations:**
- Handle missing/undetected values gracefully with dimmed "Not detected" text
- Keep formatting consistent with timeline module
- Make next steps actionable and concise (3 items max)
- Use color coding: cyan for commands, dimmed for optional info

**Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_summary_with_all_detected() {
        let mut results = DetectionResults::new();
        results.shell = Some(Shell::Zsh);
        results.shell_path = Some("/bin/zsh".to_string());
        results.version_managers = vec!["nvm".to_string()];

        let output = format_detection_summary(&results);
        assert!(output.contains("zsh"));
        assert!(output.contains("nvm"));
    }

    #[test]
    fn test_detection_summary_with_nothing_detected() {
        let results = DetectionResults::new();
        let output = format_detection_summary(&results);
        assert!(output.contains("Not detected"));
    }

    #[test]
    fn test_next_steps_contains_source_command() {
        let output = format_next_steps(&Shell::Zsh);
        assert!(output.contains("source"));
        assert!(output.contains("~/.zshrc"));
    }
}
```

**Dependencies:**
- Requires: Task 1 (Timeline Module)
- Enables: Task 4 (Auto-Detection Summary), Task 7 (Completion Messages)

---

### Task 3: Refactor Prompts Module for Compact Display

**Objective:** Simplify existing prompts to be more concise, add inline detection display, and remove verbose help text.

**Implementation Steps:**

1. **Review current prompts**:
   - Read `src/init/prompts.rs` to understand existing prompt structure
   - Identify verbose help text and explanations to remove/shorten

2. **Create prompt template with detected values**:
   ```rust
   // src/init/prompts.rs

   use inquire::{Select, Confirm};
   use crate::setup::shell_detection::Shell;

   /// Shell selection prompt with inline detection
   pub fn prompt_shell(detected: Option<&Shell>) -> Result<Shell> {
       let options = if let Some(shell) = detected {
           vec![
               format!("{} (detected, recommended)", shell.name()),
               "bash".to_string(),
               "zsh".to_string(),
               "Custom path".to_string(),
           ]
       } else {
           vec![
               "zsh".to_string(),
               "bash".to_string(),
               "Custom path".to_string(),
           ]
       };

       let message = if detected.is_some() {
           "Which shell?"
       } else {
           "Which shell? (auto-detection failed)"
       };

       let selected = Select::new(message, options).prompt()?;

       // Parse selection back to Shell
       if selected.contains("zsh") {
           Ok(Shell::Zsh)
       } else if selected.contains("bash") {
           Ok(Shell::Bash)
       } else {
           // Handle custom path...
           prompt_custom_shell_path()
       }
   }
   ```

3. **Update version manager prompt**:
   ```rust
   /// Version manager selection with detection
   pub fn prompt_version_manager(detected: Vec<String>) -> Result<Vec<String>> {
       let mut options = Vec::new();

       if detected.contains(&"nvm".to_string()) {
           options.push("nvm (detected, recommended)");
       } else {
           options.push("nvm");
       }

       if detected.contains(&"fnm".to_string()) {
           options.push("fnm (detected)");
       } else {
           options.push("fnm");
       }

       options.push("Multiple (advanced)");

       let selected = Select::new("Which version manager?", options).prompt()?;

       if selected.contains("nvm") {
           Ok(vec!["nvm".to_string()])
       } else if selected.contains("fnm") {
           Ok(vec!["fnm".to_string()])
       } else {
           // Multiple selection...
           prompt_multiple_version_managers()
       }
   }
   ```

4. **Create quick mode confirmation prompt**:
   ```rust
   use crate::init::summary::DetectionResults;

   #[derive(Debug, Clone)]
   pub enum QuickModeChoice {
       Proceed,
       Customize,
       Cancel,
   }

   pub fn prompt_quick_mode_confirmation(results: &DetectionResults) -> Result<QuickModeChoice> {
       let options = vec![
           "Yes, continue",
           "Customize settings",
           "Cancel",
       ];

       let choice = Select::new("Proceed with this configuration?", options)
           .prompt()?;

       match choice {
           "Yes, continue" => Ok(QuickModeChoice::Proceed),
           "Customize settings" => Ok(QuickModeChoice::Customize),
           _ => Ok(QuickModeChoice::Cancel),
       }
   }
   ```

5. **Simplify auto-install prompt**:
   ```rust
   use crate::config::AutoInstallMode;

   pub fn prompt_auto_install() -> Result<AutoInstallMode> {
       let options = vec![
           "Prompt (recommended) - Ask before installing",
           "Always - Install automatically",
           "Never - Manual installation only",
       ];

       let selected = Select::new("Auto-install missing versions?", options)
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

6. **Remove verbose help text**:
   - Remove `.with_help_message()` from prompts where help is obvious
   - Keep help only for complex/ambiguous choices
   - Move detailed explanations to `--help` text instead

**Code Structure:**
- File: `src/init/prompts.rs` (refactor existing)
  - `prompt_shell()` with inline detection
  - `prompt_version_manager()` with detection
  - `prompt_quick_mode_confirmation()` new function
  - `prompt_auto_install()` simplified
  - `QuickModeChoice` enum for quick mode flow

**Key Considerations:**
- Pre-select detected values in prompts
- Show "(detected, recommended)" label for detected options
- Keep prompt messages short (< 50 chars)
- Use `inquire::Select` for single choice, avoid custom formatting
- Maintain backward compatibility with existing wizard flow

**Testing:**
- Manual testing required (interactive prompts)
- Test with various detection scenarios (detected vs not detected)
- Verify keyboard navigation works smoothly
- Test cancellation behavior

**Dependencies:**
- Requires: Task 2 (Summary Module) for `DetectionResults`
- Enables: Task 4 (Quick Mode), Task 8 (Advanced Mode)

---

## Phase 2: Quick Mode Implementation

### Task 4: Implement Auto-Detection Summary Screen

**Objective:** Create a single-screen summary showing all detected values with a confirmation prompt.

**Implementation Steps:**

1. **Update detection module to collect all results**:
   ```rust
   // src/init/detection.rs

   use crate::init::summary::DetectionResults;

   /// Run all detection steps and return comprehensive results
   pub fn detect_all() -> Result<DetectionResults> {
       let mut results = DetectionResults::new();

       // Detect shell
       if let Ok(shell) = detect_shell() {
           results.shell = Some(shell);
           if let Ok(path) = get_shell_path(&shell) {
               results.shell_path = Some(path);
           }
       }

       // Detect version managers
       results.version_managers = detect_version_managers()?;

       // Set defaults
       results.config_path = get_config_path();
       results.auto_install = AutoInstallMode::Prompt; // Default

       Ok(results)
   }

   fn get_shell_path(shell: &Shell) -> Result<String> {
       // Implementation to get shell binary path
       std::env::var("SHELL")
   }

   fn get_config_path() -> String {
       dirs::home_dir()
           .map(|h| h.join(".anvsrc").display().to_string())
           .unwrap_or_else(|| "~/.anvsrc".to_string())
   }
   ```

2. **Create quick mode wizard function**:
   ```rust
   // src/init/wizard.rs

   use crate::init::summary::{format_detection_summary, DetectionResults};
   use crate::init::prompts::{prompt_quick_mode_confirmation, QuickModeChoice};
   use crate::output;

   /// Run quick mode wizard (default)
   pub fn run_quick_wizard() -> Result<(Config, Shell)> {
       // Print header
       println!();
       output::brand("⚡ Automatic Node Version Switcher");
       println!();

       // Run detection
       let results = crate::init::detection::detect_all()?;

       // Show summary
       println!("{}", format_detection_summary(&results));
       println!();

       // Single confirmation prompt
       match prompt_quick_mode_confirmation(&results)? {
           QuickModeChoice::Proceed => {
               // User accepted defaults
               let shell = results.shell
                   .ok_or_else(|| anyhow::anyhow!("Shell detection failed"))?;
               let config = results_to_config(&results)?;
               Ok((config, shell))
           }
           QuickModeChoice::Customize => {
               // Drop into advanced mode
               run_advanced_wizard()
           }
           QuickModeChoice::Cancel => {
               Err(anyhow::anyhow!("Setup cancelled by user"))
           }
       }
   }

   fn results_to_config(results: &DetectionResults) -> Result<Config> {
       Ok(Config {
           plugins: if results.version_managers.is_empty() {
               vec!["nvm".to_string()] // Default fallback
           } else {
               results.version_managers.clone()
           },
           auto_install: results.auto_install.clone(),
           version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
           use_default: true,
       })
   }
   ```

3. **Add error handling for missing detections**:
   ```rust
   // If shell detection fails in quick mode, show error and offer to customize
   pub fn run_quick_wizard() -> Result<(Config, Shell)> {
       // ... detection code ...

       // Check if critical detection failed
       if results.shell.is_none() {
           output::warning("⚠️  Shell auto-detection failed");
           println!();
           output::info("Please use advanced mode to configure manually:");
           output::info("  anvs init --advanced");
           return Err(anyhow::anyhow!("Shell detection failed"));
       }

       if results.version_managers.is_empty() {
           output::warning("⚠️  No version managers detected");
           output::info("Please install nvm or fnm first, or use --advanced mode");
           // Continue anyway with nvm as default (will fail at activation if not installed)
       }

       // ... rest of quick mode ...
   }
   ```

4. **Wire up to CLI command**:
   ```rust
   // src/commands/init.rs (or wherever init is handled)

   pub fn handle_init(quick: bool, advanced: bool, non_interactive: bool, force: bool) -> Result<()> {
       let mode = if advanced {
           WizardMode::Advanced
       } else if non_interactive {
           WizardMode::NonInteractive
       } else {
           // Default is now quick mode!
           WizardMode::Quick
       };

       let (config, shell) = match mode {
           WizardMode::Quick => run_quick_wizard()?,
           WizardMode::Advanced => run_advanced_wizard()?,
           WizardMode::NonInteractive => run_non_interactive_wizard()?,
       };

       // Continue with installation...
       install_config(config, shell, force)?;

       Ok(())
   }
   ```

**Code Structure:**
- File: `src/init/detection.rs`
  - `detect_all()` function collecting comprehensive results
  - Helper functions for shell path and config path
- File: `src/init/wizard.rs`
  - `run_quick_wizard()` function
  - `results_to_config()` helper
  - Error handling for detection failures

**Key Considerations:**
- Must detect shell successfully or fail gracefully
- Version manager detection can fail (use default with warning)
- Single confirmation prompt is the ONLY user interaction in quick mode
- "Customize" choice should seamlessly transition to advanced mode
- Config path should use actual home directory, not hardcoded "~"

**Testing:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_detect_all_with_shell() {
        let results = detect_all().unwrap();
        assert!(results.shell.is_some());
    }

    #[test]
    fn test_results_to_config_defaults() {
        let results = DetectionResults::new();
        let config = results_to_config(&results).unwrap();
        assert_eq!(config.plugins, vec!["nvm".to_string()]);
        assert_eq!(config.auto_install, AutoInstallMode::Prompt);
    }
}
```

**Dependencies:**
- Requires: Task 2 (Summary Module), Task 3 (Compact Prompts)
- Enables: Task 5 (Progress Indicators), Task 6 (CLI Integration)

---

### Task 5: Add Installation Progress Indicators

**Objective:** Show clear visual feedback during the installation process using timeline-style progress.

**Implementation Steps:**

1. **Define installation steps**:
   ```rust
   // src/init/wizard.rs or src/setup/installer.rs

   use crate::init::timeline::{Step, StepState, render_step};

   struct InstallationProgress {
       steps: Vec<Step>,
   }

   impl InstallationProgress {
       fn new() -> Self {
           Self {
               steps: vec![
                   Step::new("Creating config at ~/.anvsrc"),
                   Step::new("Installing shell hook"),
                   Step::new("Validating installation"),
                   Step::new("Testing activation"),
               ],
           }
       }

       fn mark_complete(&mut self, index: usize) {
           if let Some(step) = self.steps.get_mut(index) {
               step.set_state(StepState::Complete);
           }
       }

       fn mark_active(&mut self, index: usize) {
           if let Some(step) = self.steps.get_mut(index) {
               step.set_state(StepState::Active);
           }
       }

       fn render(&self) -> String {
           use crate::init::timeline::chars;
           let mut output = String::new();
           output.push_str(&format!("{}  Installing\n", chars::STEP_ACTIVE));

           for (i, step) in self.steps.iter().enumerate() {
               let prefix = if i == self.steps.len() - 1 {
                   chars::BRANCH_LAST
               } else {
                   chars::BRANCH_RIGHT
               };
               output.push_str(&format!("{} {}\n", prefix, render_step(step)));
           }

           output
       }
   }
   ```

2. **Integrate progress display into installation**:
   ```rust
   // Update existing installation function
   // DECISION: Print each step on new line (simpler, more reliable)
   // NOT using \r in-place updates to avoid terminal buffering issues

   pub fn install_config(config: Config, shell: Shell, force: bool) -> Result<()> {
       println!();
       output::brand("⚡ Automatic Node Version Switcher");
       println!();

       let mut progress = InstallationProgress::new();

       // Print header
       println!("{}  Installing", chars::STEP_ACTIVE);

       // Step 1: Create config
       progress.mark_active(0);
       crate::config::save_config(&config)?;
       progress.mark_complete(0);
       println!("{}  {}", chars::BRANCH_RIGHT, render_step(&progress.steps[0]));

       // Step 2: Install shell hook
       progress.mark_active(1);
       install_shell_hook(&shell, force)?;
       progress.mark_complete(1);
       println!("{}  {}", chars::BRANCH_RIGHT, render_step(&progress.steps[1]));

       // Step 3: Validate
       progress.mark_active(2);
       validate_installation(&shell)?;
       progress.mark_complete(2);
       println!("{}  {}", chars::BRANCH_RIGHT, render_step(&progress.steps[2]));

       // Step 4: Test
       progress.mark_active(3);
       test_activation()?;
       progress.mark_complete(3);
       println!("{}  {}", chars::BRANCH_LAST, render_step(&progress.steps[3]));

       Ok(())
   }
   ```

3. **Add dynamic step details**:
   ```rust
   // Update steps with dynamic details based on execution
   fn install_shell_hook(&shell: &Shell, force: bool) -> Result<()> {
       let profile_path = get_profile_path(shell)?;

       // Update progress step with actual path
       // This requires passing progress by reference...

       // Actual hook installation logic
       modify_shell_profile(&profile_path, force)?;

       Ok(())
   }
   ```

4. **Handle errors in progress display**:
   ```rust
   // Show error state in timeline
   pub fn install_config(config: Config, shell: Shell, force: bool) -> Result<()> {
       let mut progress = InstallationProgress::new();

       for i in 0..progress.steps.len() {
           progress.mark_active(i);
           print!("\r{}", progress.render());

           let result = match i {
               0 => crate::config::save_config(&config),
               1 => install_shell_hook(&shell, force),
               2 => validate_installation(&shell),
               3 => test_activation(),
               _ => Ok(()),
           };

           if let Err(e) = result {
               // Mark as failed (could add Failed state)
               output::error(&format!("✗ Installation failed at step {}: {}", i + 1, e));
               return Err(e);
           }

           progress.mark_complete(i);
       }

       println!("{}", progress.render());
       Ok(())
   }
   ```

5. **Add timing information (optional)**:
   ```rust
   use std::time::Instant;

   pub fn install_config(config: Config, shell: Shell, force: bool) -> Result<()> {
       let start = Instant::now();

       // ... installation steps ...

       let duration = start.elapsed();
       output::success(&format!("✓ Setup complete! (in {:.1}s)", duration.as_secs_f64()));

       Ok(())
   }
   ```

**Code Structure:**
- File: `src/init/wizard.rs` or `src/setup/installer.rs`
  - `InstallationProgress` struct
  - Updated `install_config()` function with progress display
  - Error handling integrated with progress

**Key Considerations:**
- Use `\r` (carriage return) to update progress in-place if desired
- Or print each step completion on a new line for simplicity
- Handle installation errors gracefully (show which step failed)
- Keep step labels concise but descriptive
- Consider terminal buffering issues (flush stdout if needed)

**Testing:**
- Manual testing required (visual inspection)
- Test error scenarios (permission denied, file exists, etc.)
- Verify timing display is reasonable
- Test on slow connections/systems

**Dependencies:**
- Requires: Task 1 (Timeline Module)
- Enables: Task 7 (Completion Messages)

---

### Task 6: Create Completion Screen

**Objective:** Design a clean, helpful completion message with next steps.

**Implementation Steps:**

1. **Create completion message function**:
   ```rust
   // src/init/wizard.rs

   use crate::init::summary::format_next_steps;
   use crate::output;

   fn show_completion_message(shell: &Shell, duration: std::time::Duration) -> Result<()> {
       println!();
       output::success(&format!("✓ Setup complete!"));

       if duration.as_secs() < 60 {
           output::info(&format!("Completed in {:.1}s", duration.as_secs_f64()));
       }

       println!();
       println!("{}", format_next_steps(shell));

       Ok(())
   }
   ```

2. **Add example usage hint**:
   ```rust
   // Update format_next_steps in summary.rs
   pub fn format_next_steps(shell: &Shell) -> String {
       let shell_rc = match shell {
           Shell::Zsh => "~/.zshrc",
           Shell::Bash => "~/.bashrc",
           Shell::Unknown(_) => "your shell config",
       };

       let mut output = String::new();
       output.push_str(&"Next steps:".bold().to_string());
       output.push_str("\n");
       output.push_str(&format!("  1. Restart your shell or run: {}\n",
           format!("source {}", shell_rc).cyan()));
       output.push_str("  2. Navigate to a project with .nvmrc\n");
       output.push_str("  3. Watch anvs activate automatically!\n");
       output.push_str("\n");
       output.push_str(&format!("Example: {}\n",
           "cd ~/my-project && anvs status".dimmed()));

       output
   }
   ```

3. **Integrate into installation flow**:
   ```rust
   pub fn handle_init(...) -> Result<()> {
       let start = Instant::now();

       // Run wizard
       let (config, shell) = run_quick_wizard()?;

       // Install
       install_config(config, shell.clone(), force)?;

       // Show completion
       show_completion_message(&shell, start.elapsed())?;

       Ok(())
   }
   ```

**Code Structure:**
- File: `src/init/wizard.rs`
  - `show_completion_message()` function
- File: `src/init/summary.rs`
  - Enhanced `format_next_steps()` with example

**Key Considerations:**
- Keep next steps to 3 items maximum
- Include actual shell-specific command (source .zshrc vs .bashrc)
- Add example usage to help first-time users
- Show timing only if < 60 seconds (avoid showing minutes)
- Use success color (green) for "Setup complete!"

**Testing:**
- Manual testing (visual inspection)
- Test with both bash and zsh
- Verify example command is helpful

**Dependencies:**
- Requires: Task 2 (Summary Module), Task 5 (Progress Indicators)
- Enables: Phase completion

---

## Phase 3: Advanced Mode Refinement

### Task 7: Implement Step-by-Step Advanced Flow

**Objective:** Simplify the existing wizard to exactly 3 steps with step counter display.

**Implementation Steps:**

1. **Define WizardState struct** (if not already present in wizard.rs):
   ```rust
   // src/init/wizard.rs

   use crate::config::{Config, AutoInstallMode};
   use crate::setup::shell_detection::Shell;

   /// Wizard state - collects configuration through steps
   #[derive(Debug, Clone, Default)]
   pub struct WizardState {
       pub shell: Option<Shell>,
       pub plugins: Vec<String>,
       pub auto_install: AutoInstallMode,
       pub version_files: Vec<String>,
   }

   impl WizardState {
       pub fn new() -> Self {
           Self {
               shell: None,
               plugins: Vec::new(),
               auto_install: AutoInstallMode::Prompt,
               version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
           }
       }

       pub fn get_shell(&self) -> Result<Shell> {
           self.shell.clone()
               .ok_or_else(|| anyhow::anyhow!("Shell not configured"))
       }

       pub fn to_config(&self) -> Result<Config> {
           Ok(Config {
               plugins: if self.plugins.is_empty() {
                   vec!["nvm".to_string()]
               } else {
                   self.plugins.clone()
               },
               auto_install: self.auto_install.clone(),
               version_files: self.version_files.clone(),
               use_default: true,
           })
       }
   }
   ```

2. **Create advanced mode wizard function**:
   ```rust
   /// Run advanced mode wizard with full customization
   pub fn run_advanced_wizard() -> Result<(Config, Shell)> {
       println!();
       output::brand("⚡ Automatic Node Version Switcher");
       output::info("Advanced Setup (3 steps)");
       println!();

       // Run detection for defaults
       let detected = crate::init::detection::detect_all()?;

       let mut state = WizardState::new();

       // Step 1: Shell
       step_shell(&mut state, detected.shell.as_ref())?;

       // Step 2: Version Manager
       step_version_manager(&mut state, &detected.version_managers)?;

       // Step 3: Auto-install
       step_auto_install(&mut state)?;

       // Summary confirmation
       confirm_configuration(&state)?;

       let shell = state.get_shell()?;
       let config = state.to_config()?;

       Ok((config, shell))
   }
   ```

2. **Implement Step 1: Shell Configuration**:
   ```rust
   use crate::init::timeline::chars;

   fn step_shell(state: &mut WizardState, detected: Option<&Shell>) -> Result<()> {
       println!("{}  Step 1 of 3: Shell Configuration", chars::STEP_ACTIVE);
       println!("{}", chars::VERTICAL);

       let shell = crate::init::prompts::prompt_shell(detected)?;
       state.shell = Some(shell);

       println!();
       Ok(())
   }
   ```

3. **Implement Step 2: Version Manager**:
   ```rust
   fn step_version_manager(state: &mut WizardState, detected: &[String]) -> Result<()> {
       println!("{}  Step 2 of 3: Version Manager", chars::STEP_ACTIVE);
       println!("{}", chars::VERTICAL);

       let plugins = crate::init::prompts::prompt_version_manager(detected.to_vec())?;
       state.plugins = plugins;

       println!();
       Ok(())
   }
   ```

4. **Implement Step 3: Auto-install Behavior**:
   ```rust
   fn step_auto_install(state: &mut WizardState) -> Result<()> {
       println!("{}  Step 3 of 3: Configuration", chars::STEP_ACTIVE);
       println!("{}", chars::VERTICAL);

       let mode = crate::init::prompts::prompt_auto_install()?;
       state.auto_install = mode;

       println!();
       Ok(())
   }
   ```

5. **Add configuration summary confirmation**:
   ```rust
   use crate::init::summary::format_config_preview;

   fn confirm_configuration(state: &WizardState) -> Result<()> {
       let shell = state.get_shell()?;
       let config = state.to_config()?;

       println!("{}", format_config_preview(&config, &shell));
       println!();

       let confirmed = inquire::Confirm::new("Apply this configuration?")
           .with_default(true)
           .prompt()?;

       if !confirmed {
           return Err(anyhow::anyhow!("Configuration cancelled by user"));
       }

       Ok(())
   }
   ```

**Code Structure:**
- File: `src/init/wizard.rs`
  - `run_advanced_wizard()` function
  - `step_shell()`, `step_version_manager()`, `step_auto_install()` helpers
  - `confirm_configuration()` final confirmation

**Key Considerations:**
- Exactly 3 steps, no more, no less
- Show step counter on each step (Step 1 of 3, etc.)
- Use detected values as defaults in prompts
- Allow user to go back (if inquire supports it, otherwise just proceed)
- Final confirmation before applying changes

**Testing:**
- Manual testing required (interactive)
- Test with detected values
- Test with no detected values
- Test cancellation at various points

**Dependencies:**
- Requires: Task 3 (Compact Prompts), Task 2 (Summary Module)
- Enables: Task 8 (Inline Detection), Task 9 (Configuration Summary)

---

### Task 8: Add Inline Detection Display to Prompts

**Objective:** Show detected values directly in prompt messages with "(detected, recommended)" labels.

**Implementation Steps:**

1. **Update shell prompt**:
   ```rust
   // src/init/prompts.rs

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

       let selected = Select::new(&message, options).prompt()?;

       // Parse selection
       if selected.contains("zsh") {
           Ok(Shell::Zsh)
       } else if selected.contains("bash") {
           Ok(Shell::Bash)
       } else if selected.contains("Custom") {
           prompt_custom_shell_path()
       } else if let Some(shell) = detected {
           Ok(shell.clone())
       } else {
           Err(anyhow::anyhow!("Invalid shell selection"))
       }
   }
   ```

2. **Update version manager prompt**:
   ```rust
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

       let selected = Select::new(&message, options).prompt()?;

       if selected.contains("nvm") {
           Ok(vec!["nvm".to_string()])
       } else if selected.contains("fnm") {
           Ok(vec!["fnm".to_string()])
       } else {
           prompt_multiple_version_managers(&detected)
       }
   }
   ```

3. **Pre-select detected values**:
   ```rust
   // Use inquire's initial selection feature
   use inquire::Select;

   pub fn prompt_shell(detected: Option<&Shell>) -> Result<Shell> {
       // ... build options as above ...

       let mut select = Select::new(&message, options);

       // Pre-select detected option (first in list)
       if detected.is_some() {
           select = select.with_starting_cursor(0);
       }

       let selected = select.prompt()?;

       // ... parse selection ...
   }
   ```

4. **Add visual indicators**:
   ```rust
   // Consider using symbols for detected values
   pub fn prompt_version_manager(detected: Vec<String>) -> Result<Vec<String>> {
       let has_nvm = detected.contains(&"nvm".to_string());

       let mut options = vec![];

       if has_nvm {
           options.push("● nvm (detected, recommended)"); // ● for selected/detected
       } else {
           options.push("○ nvm"); // ○ for available
       }

       // ... rest of options ...
   }
   ```

**Code Structure:**
- File: `src/init/prompts.rs` (refactor existing functions)
  - Updated `prompt_shell()` with inline detection
  - Updated `prompt_version_manager()` with inline detection
  - Pre-selection logic for detected values

**Key Considerations:**
- Detected values should be first in the list and pre-selected
- Use "(detected, recommended)" label consistently
- If detection fails, show "(auto-detection failed)" in message
- Keep prompt messages concise even with detection info
- Parse selections correctly with added labels

**Testing:**
- Test with detected shell and version manager
- Test with no detection (all options available)
- Test with partial detection (e.g., nvm but not fnm)
- Verify pre-selection works

**Dependencies:**
- Requires: Task 3 (Compact Prompts)
- Enables: Task 7 (Advanced Flow) to use enhanced prompts

---

### Task 9: Add Final Configuration Summary

**Objective:** Show all selected options in a box layout before applying.

**Implementation Steps:**

1. **Use existing format_config_preview function**:
   ```rust
   // Already implemented in Task 2, just wire it up

   fn confirm_configuration(state: &WizardState) -> Result<()> {
       let shell = state.get_shell()?;
       let config = state.to_config()?;

       println!("{}", format_config_preview(&config, &shell));
       println!();

       let confirmed = inquire::Confirm::new("Apply this configuration?")
           .with_default(true)
           .prompt()?;

       if !confirmed {
           return Err(anyhow::anyhow!("Configuration cancelled by user"));
       }

       Ok(())
   }
   ```

2. **Add "go back to edit" option (optional)**:
   ```rust
   // If inquire supports navigation, allow going back
   // Otherwise, just proceed or cancel

   fn confirm_configuration(state: &WizardState) -> Result<()> {
       let shell = state.get_shell()?;
       let config = state.to_config()?;

       println!("{}", format_config_preview(&config, &shell));
       println!();

       let options = vec!["Apply", "Cancel"];
       let choice = Select::new("Ready to proceed?", options).prompt()?;

       if choice == "Cancel" {
           return Err(anyhow::anyhow!("Configuration cancelled"));
       }

       Ok(())
   }
   ```

3. **Ensure summary shows all settings**:
   ```rust
   // Update format_config_preview to be comprehensive
   pub fn format_config_preview(config: &Config, shell: &Shell) -> String {
       let items = vec![
           ("Shell", shell.name()),
           ("Version manager", config.plugins.join(", ").as_str()),
           ("Auto-install", format_auto_install(&config.auto_install).as_str()),
           ("Version files", config.version_files.join(", ").as_str()),
           ("Config", "~/.anvsrc"),
       ];

       timeline::render_box("Configuration Summary", &items)
   }
   ```

**Code Structure:**
- File: `src/init/wizard.rs`
  - `confirm_configuration()` function (already in Task 7)
- File: `src/init/summary.rs`
  - Enhanced `format_config_preview()` to show all settings

**Key Considerations:**
- Show ALL configuration options, not just the main ones
- Use same box-style layout as detection summary
- Make it visually distinct from detection summary
- Confirmation should be clear (Yes/No or Apply/Cancel)

**Testing:**
- Test with various configuration combinations
- Verify all fields are displayed correctly
- Test cancellation

**Dependencies:**
- Requires: Task 2 (Summary Module), Task 7 (Advanced Flow)
- Enables: Advanced mode completion

---

## Phase 4: CLI Integration

### Task 10: Update CLI with `--advanced` Flag

**Objective:** Add the `--advanced` flag to the `init` command and update default behavior.

**Implementation Steps:**

1. **Update CLI enum**:
   ```rust
   // src/cli.rs

   #[derive(Subcommand, Debug)]
   pub enum Commands {
       Init {
           /// Skip wizard and use sensible defaults (same as quick mode)
           #[arg(short, long)]
           quick: bool,

           /// Advanced setup with full customization
           #[arg(long)]
           advanced: bool,

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
       // ... other commands ...
   }
   ```

2. **Create WizardMode enum**:
   ```rust
   // src/init/wizard.rs

   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum WizardMode {
       Quick,
       Advanced,
       NonInteractive,
   }

   impl WizardMode {
       pub fn from_flags(quick: bool, advanced: bool, non_interactive: bool) -> Self {
           if non_interactive {
               WizardMode::NonInteractive
           } else if advanced {
               WizardMode::Advanced
           } else {
               // Default is Quick mode
               WizardMode::Quick
           }
       }
   }
   ```

3. **Update init command handler**:
   ```rust
   // In the main command handler (src/main.rs or wherever)

   use crate::init::wizard::{WizardMode, run_wizard};

   Commands::Init { quick, advanced, force, shell, non_interactive } => {
       let mode = WizardMode::from_flags(quick, advanced, non_interactive);

       log::debug!("Running wizard in mode: {:?}", mode);

       let (config, detected_shell) = run_wizard(mode)?;

       // Override shell if specified via CLI
       let final_shell = if let Some(shell_name) = shell {
           parse_shell_from_string(&shell_name)?
       } else {
           detected_shell
       };

       install_config(config, final_shell, force)?;
   }
   ```

4. **Implement mode routing**:
   ```rust
   // src/init/wizard.rs

   pub fn run_wizard(mode: WizardMode) -> Result<(Config, Shell)> {
       match mode {
           WizardMode::Quick => run_quick_wizard(),
           WizardMode::Advanced => run_advanced_wizard(),
           WizardMode::NonInteractive => run_non_interactive_wizard(),
       }
   }

   fn run_non_interactive_wizard() -> Result<(Config, Shell)> {
       // Detect everything, use defaults, no prompts
       let results = crate::init::detection::detect_all()?;

       // Require shell detection to succeed
       let shell = results.shell.ok_or_else(|| {
           anyhow::anyhow!(
               "Shell detection failed. Use --shell flag or run in interactive mode.\n\
                Example: anvs init --non-interactive --shell zsh"
           )
       })?;

       // Warn if no version manager detected, but continue with default
       if results.version_managers.is_empty() {
           log::warn!("No version managers detected, defaulting to nvm");
           log::warn!("Ensure nvm is installed before using anvs");
       }

       let config = results_to_config(&results)?;

       // Log configuration for debugging
       log::info!("Non-interactive setup:");
       log::info!("  Shell: {}", shell.name());
       log::info!("  Plugins: {:?}", config.plugins);
       log::info!("  Auto-install: {:?}", config.auto_install);

       Ok((config, shell))
   }
   ```

5. **Add logging for debugging**:
   ```rust
   pub fn run_wizard(mode: WizardMode) -> Result<(Config, Shell)> {
       log::info!("Starting wizard in {:?} mode", mode);

       let result = match mode {
           WizardMode::Quick => {
               log::debug!("Running quick wizard");
               run_quick_wizard()
           }
           WizardMode::Advanced => {
               log::debug!("Running advanced wizard");
               run_advanced_wizard()
           }
           WizardMode::NonInteractive => {
               log::debug!("Running non-interactive wizard");
               run_non_interactive_wizard()
           }
       }?;

       log::info!("Wizard completed successfully");
       Ok(result)
   }
   ```

**Code Structure:**
- File: `src/cli.rs`
  - Updated `Commands::Init` with `advanced` flag
- File: `src/init/wizard.rs`
  - `WizardMode` enum and `from_flags()` method
  - `run_wizard()` routing function
  - `run_non_interactive_wizard()` implementation

**Key Considerations:**
- Default mode is now Quick (breaking change from verbose wizard)
- `--quick` flag is explicit but redundant (same as default)
- `--advanced` flag for full customization
- `--non-interactive` for CI/automation (no prompts at all)
- CLI `--shell` flag should override detected shell
- Add logging to help debug mode selection

**Testing:**
```bash
# Test all flag combinations
anvs init                    # Should run quick mode
anvs init --quick            # Should run quick mode (explicit)
anvs init --advanced         # Should run advanced mode
anvs init --non-interactive  # Should run non-interactive mode
anvs init --advanced --quick # Conflict? Advanced takes precedence
anvs init --shell zsh        # Should override detection
```

**Dependencies:**
- Requires: Task 4 (Quick Mode), Task 7 (Advanced Mode)
- Enables: Task 11 (Help Text Updates)

---

### Task 11: Update Help Text

**Objective:** Update `anvs init --help` to document quick and advanced modes clearly.

**Implementation Steps:**

1. **Update main `about` text**:
   ```rust
   // src/cli.rs

   Init {
       /// Skip wizard and use sensible defaults (same as quick mode)
       #[arg(short, long)]
       quick: bool,

       /// Advanced setup with full customization
       #[arg(long)]
       advanced: bool,

       /// Force overwrite existing configuration
       #[arg(short, long)]
       force: bool,

       /// Shell to configure (bash, zsh, or auto-detect)
       #[arg(short, long)]
       shell: Option<String>,

       /// Non-interactive mode for automation
       #[arg(long)]
       non_interactive: bool,
   }
   ```

2. **Update command-level help**:
   ```rust
   #[derive(Subcommand, Debug)]
   pub enum Commands {
       /// Initialize anvs with interactive configuration wizard
       ///
       /// By default, runs in quick mode with auto-detection and minimal prompts.
       /// Detects your shell and version manager, then asks for confirmation.
       ///
       /// For full customization, use --advanced to step through all options.
       /// For automation/CI, use --non-interactive to skip all prompts.
       ///
       /// Examples:
       ///   anvs init              Quick setup (default, < 30 seconds)
       ///   anvs init --advanced   Full customization wizard
       ///   anvs init --non-interactive   Automated setup for scripts
       Init {
           // ... fields ...
       },
       // ... other commands ...
   }
   ```

3. **Update top-level help**:
   ```rust
   #[command(
       about = "ANVS - Automatic Node Version Switcher for Node.js",
       long_about = r#"
   anvs automatically switches your Node.js version when you cd into a directory
   with a .nvmrc or .node-version file. When you leave a project directory, anvs
   automatically returns to your default Node.js version.

   After installation, run 'anvs init' for quick setup with auto-detection, or
   'anvs init --advanced' for full customization.

   Examples:
     anvs init               Quick setup (recommended, < 30 seconds)
     anvs init --advanced    Advanced setup with full control
     anvs activate           Manually activate for current directory
     anvs status             Show configuration and test activation
     anvs set                Change configuration settings
     anvs uninstall          Completely remove anvs

   For more information, visit: https://github.com/olvrcc/anvs
   "#
   )]
   ```

4. **Verify help output**:
   ```bash
   anvs --help
   anvs init --help
   ```

5. **Ensure concise but clear**:
   - Each flag should have a one-line description
   - Command-level help should explain modes
   - Examples should show common use cases
   - Don't over-explain (keep it concise)

**Code Structure:**
- File: `src/cli.rs`
  - Updated doc comments on `Init` command
  - Updated doc comments on flags
  - Updated top-level `long_about`

**Key Considerations:**
- Help text should guide users to quick mode (default)
- Mention advanced mode for those who need it
- Keep examples realistic and helpful
- Use consistent terminology (quick mode, advanced mode, non-interactive)

**Testing:**
```bash
cargo run -- --help
cargo run -- init --help
```

**Dependencies:**
- Requires: Task 10 (CLI Updates)
- Enables: Documentation updates

---

## Phase 5: Polish & Testing

### Task 12: Visual Refinement

**Objective:** Ensure consistent spacing, alignment, colors, and terminal compatibility.

**Implementation Steps:**

1. **Check spacing consistency**:
   - Review all timeline rendering for consistent indentation
   - Ensure blank lines between sections are uniform
   - Verify box drawing alignment

2. **Test on 80-column terminal**:
   ```bash
   # Resize terminal to 80 columns
   printf '\e[8;24;80t'

   # Run wizard and check for line wrapping
   anvs init
   anvs init --advanced
   ```

3. **Verify colors on dark theme**:
   - Open dark terminal theme
   - Run wizard, check all steps
   - Verify lime green is visible and pleasant
   - Check dimmed text is readable

4. **Verify colors on light theme**:
   - Open light terminal theme
   - Run wizard again
   - Ensure colors work well (not washed out)
   - Adjust if needed

5. **Fix alignment issues**:
   ```rust
   // If box drawing characters misalign, check:
   // - Are we using tabs vs spaces inconsistently?
   // - Are all box chars the same width?
   // - Is terminal font monospace?

   // Example fix:
   pub fn render_box(title: &str, items: &[(&str, &str)]) -> String {
       // Calculate max key length for alignment
       let max_key_len = items.iter()
           .map(|(k, _)| k.len())
           .max()
           .unwrap_or(0);

       let mut output = format!("{} {}\n", chars::TOP_LEFT, title.bold());
       output.push_str(&format!("{}\n", chars::VERTICAL));

       for (i, (key, value)) in items.iter().enumerate() {
           let prefix = if i == items.len() - 1 {
               chars::BRANCH_LAST
           } else {
               chars::BRANCH_RIGHT
           };
           // Pad key for alignment
           output.push_str(&format!("{} {:width$}: {}\n",
               prefix,
               key.dimmed(),
               value,
               width = max_key_len
           ));
       }

       output
   }
   ```

6. **Ensure consistent color usage**:
   ```rust
   // Check that we use the same colors everywhere
   // Brand: lime green (RGB 50, 205, 50)
   // Info: cyan
   // Success: green
   // Warning: yellow
   // Error: red
   // Dimmed: gray

   // Update output.rs if needed to standardize
   ```

**Code Structure:**
- Files: `src/init/timeline.rs`, `src/init/summary.rs`, `src/output.rs`
  - Alignment fixes
  - Color consistency
  - Spacing adjustments

**Key Considerations:**
- Test on actual terminals, not just IDE embedded terminals
- Different terminal emulators may render differently
- Monospace font is assumed (standard for terminals)
- Box drawing characters should be Unicode (not ASCII art)

**Testing:**
- Visual inspection on multiple terminal themes
- Test on iTerm2, Terminal.app (macOS), gnome-terminal (Linux)
- Check with both zsh and bash
- Verify no broken characters or garbled output

**Dependencies:**
- Requires: All visual tasks (1, 2, 5)
- Enables: Final quality assurance

---

### Task 13: User Experience Testing

**Objective:** Ensure the wizard is intuitive, fast, and handles edge cases well.

**Implementation Steps:**

1. **Time the quick mode flow**:
   ```bash
   time anvs init
   # Should complete in < 30 seconds (goal)
   ```

2. **Test advanced mode intuitiveness**:
   - Run through advanced mode step-by-step
   - Note any confusing prompts or unclear steps
   - Verify step counter is helpful

3. **Test error messages**:
   ```bash
   # Test with no shell detected (simulate by mocking detection)
   # Test with no version manager detected
   # Test cancellation at various points
   # Test with existing config (should prompt to overwrite)
   ```

4. **Verify cancellation works properly**:
   - Cancel at confirmation prompt in quick mode
   - Cancel at various steps in advanced mode
   - Ensure no partial config is written

5. **Test keyboard navigation**:
   - Use arrow keys in prompts
   - Try tab completion (if applicable)
   - Test Ctrl+C cancellation

6. **Edge case testing**:
   ```rust
   // Test scenarios:
   // - Multiple version managers detected (nvm + fnm)
   // - Non-standard shell path
   // - Custom shell (not bash/zsh)
   // - No home directory (should fail gracefully)
   // - Permission denied writing config (should show clear error)
   ```

**Testing Checklist:**
- [ ] Quick mode completes in < 30 seconds
- [ ] Advanced mode is clear and intuitive
- [ ] Error messages are helpful and actionable
- [ ] Cancellation doesn't leave partial state
- [ ] Keyboard navigation works smoothly
- [ ] Edge cases fail gracefully with clear messages

**Dependencies:**
- Requires: All wizard tasks (4, 7, 10)
- Enables: Final release preparation

---

### Task 14: Update Integration Tests

**Objective:** Ensure all existing tests pass and add tests for new wizard flows.

**Implementation Steps:**

1. **Update existing wizard tests**:
   ```rust
   // tests/integration.rs or tests/wizard_test.rs

   #[test]
   fn test_quick_wizard_with_detection() {
       // Mock detection to return valid shell and version manager
       // Run quick wizard
       // Verify config is created correctly
   }

   #[test]
   fn test_advanced_wizard_flow() {
       // This requires mocking interactive prompts, which is difficult
       // May need to refactor wizard to accept pre-configured state
       // Or use environment variables for testing
   }
   ```

2. **Add tests for auto-detection**:
   ```rust
   #[test]
   fn test_detect_all_returns_results() {
       let results = crate::init::detection::detect_all().unwrap();
       assert!(results.shell.is_some() || results.shell.is_none()); // Just verify no panic
   }

   #[test]
   fn test_results_to_config() {
       let mut results = DetectionResults::new();
       results.version_managers = vec!["nvm".to_string()];

       let config = results_to_config(&results).unwrap();
       assert_eq!(config.plugins, vec!["nvm".to_string()]);
   }
   ```

3. **Add tests for WizardMode**:
   ```rust
   #[test]
   fn test_wizard_mode_from_flags() {
       assert_eq!(
           WizardMode::from_flags(false, false, false),
           WizardMode::Quick
       );
       assert_eq!(
           WizardMode::from_flags(true, false, false),
           WizardMode::Quick
       );
       assert_eq!(
           WizardMode::from_flags(false, true, false),
           WizardMode::Advanced
       );
       assert_eq!(
           WizardMode::from_flags(false, false, true),
           WizardMode::NonInteractive
       );
   }
   ```

4. **Ensure all existing tests still pass**:
   ```bash
   cargo test
   ```

5. **Fix any broken tests**:
   - Update tests that relied on old wizard flow
   - Mock interactive prompts if needed
   - Consider refactoring wizard to be more testable

**Code Structure:**
- Files: `tests/integration.rs`, `tests/wizard_test.rs` (may need to create)
  - Tests for quick wizard
  - Tests for advanced wizard
  - Tests for auto-detection
  - Tests for WizardMode

**Key Considerations:**
- Testing interactive CLI is challenging (inquire doesn't have easy mocking)
- Focus on testing logic (detection, config creation) rather than UI
- Integration tests may need to be manual for prompt flow
- Use `#[ignore]` for tests that require manual interaction

**Testing:**
```bash
cargo test
cargo test --test integration
cargo test wizard
```

**Dependencies:**
- Requires: All implementation tasks
- Enables: Release preparation

---

### Task 15: Update Documentation

**Objective:** Update README, CLAUDE.md, and other docs to reflect new wizard behavior.

**Implementation Steps:**

1. **Update README.md installation section**:
   ```markdown
   ## Installation

   After installing via npm or Homebrew:

   ```bash
   anvs init
   ```

   This runs a quick setup wizard (< 30 seconds) that auto-detects your shell and
   version manager. For full customization:

   ```bash
   anvs init --advanced
   ```

   For automated scripts:

   ```bash
   anvs init --non-interactive
   ```
   ```

2. **Add screenshots/examples** (optional):
   - Take a screenshot of quick mode wizard
   - Add to README or docs folder
   - Show before/after (old wizard vs new)

3. **Update CLAUDE.md**:
   ```markdown
   ## Common Development Commands

   ```bash
   # Setup
   anvs init                  # Quick setup wizard (default)
   anvs init --advanced       # Advanced setup with customization
   anvs init --non-interactive # Automated setup for scripts
   ```
   ```

4. **Update migration guide** (if applicable):
   ```markdown
   // docs/MIGRATION.md

   ## Upgrading to v2.1.0

   ### New Wizard Experience

   The `anvs init` wizard has been redesigned for speed and clarity:

   - **Quick mode** is now the default (< 30 seconds)
   - Auto-detects shell and version manager
   - Single confirmation prompt
   - Use `--advanced` flag for full customization

   The old wizard behavior is available via `anvs init --advanced`.
   ```

5. **Update CHANGELOG.md**:
   ```markdown
   ## [2.1.0] - 2025-XX-XX

   ### Changed
   - Redesigned `anvs init` wizard for speed and visual polish
   - Quick mode is now the default (completes in < 30 seconds)
   - Timeline-style progress indicators inspired by Vite
   - Minimal prompts with inline detection display

   ### Added
   - `--advanced` flag for full customization wizard
   - Visual progress indicators during installation
   - Improved completion messages with next steps

   ### Fixed
   - Wizard no longer verbose and wordy
   - Better error messages during setup
   ```

6. **Check for other doc references**:
   ```bash
   # Search for references to old wizard behavior
   grep -r "anvs setup" docs/
   grep -r "setup wizard" README.md
   ```

**Code Structure:**
- Files: `README.md`, `CLAUDE.md`, `docs/MIGRATION.md`, `CHANGELOG.md`
  - Updated installation instructions
  - Added wizard mode documentation
  - Updated examples

**Key Considerations:**
- Keep documentation concise and accurate
- Show examples of both quick and advanced modes
- Explain when to use which mode
- Update any screenshots or GIFs (if present)

**Testing:**
- Read through all docs to verify accuracy
- Test commands shown in examples
- Check links are not broken

**Dependencies:**
- Requires: All implementation tasks
- Enables: Release preparation

---

## Phase 6: Final Review & Release

### Task 16: Code Review and Cleanup

**Objective:** Review all new code for clarity, consistency, and quality.

**Implementation Steps:**

1. **Review all new modules**:
   - `src/init/timeline.rs` - Check for clarity and correctness
   - `src/init/summary.rs` - Verify formatting logic
   - Updated `src/init/wizard.rs` - Review flow control
   - Updated `src/init/prompts.rs` - Check prompt logic

2. **Check for unused code/imports**:
   ```bash
   cargo clippy --all-targets --all-features -- -W unused-imports
   ```

3. **Run clippy with strict warnings**:
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Run cargo fmt**:
   ```bash
   cargo fmt --all
   ```

5. **Check for consistent error handling**:
   - Ensure all errors return `anyhow::Error` or `Result<T>`
   - Add context to errors where helpful: `.context("Failed to detect shell")`
   - Remove any `unwrap()` or `expect()` in production code

6. **Verify logging is appropriate**:
   - Add `log::debug!()` for detailed flow
   - Add `log::info!()` for major steps
   - Remove any excessive logging

**Testing:**
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

**Dependencies:**
- Requires: All implementation complete
- Enables: Release

---

### Task 17: Performance Check

**Objective:** Ensure wizard completes quickly and binary size hasn't grown significantly.

**Implementation Steps:**

1. **Profile wizard startup time**:
   ```bash
   time anvs init --non-interactive
   ```

2. **Ensure quick mode completes in < 30 seconds**:
   ```bash
   time anvs init
   # (Click through prompts quickly)
   ```

3. **Check binary size**:
   ```bash
   cargo build --release
   ls -lh target/release/anvs
   ```

4. **Optimize if needed**:
   - Check if any dependencies added unnecessary bloat
   - Profile with `cargo flamegraph` if slow

**Testing:**
```bash
hyperfine 'anvs activate' # Compare before/after if concerned
```

**Dependencies:**
- Requires: Implementation complete
- Enables: Release

---

### Task 18: Quality Assurance Testing

**Objective:** Test on multiple platforms and environments to ensure reliability.

**Implementation Steps:**

1. **Test on macOS (Intel)**:
   ```bash
   cargo build --release
   ./target/release/anvs init
   ./target/release/anvs init --advanced
   ./target/release/anvs status
   ```

2. **Test on macOS (ARM/M1)**:
   - Same as above on ARM Mac

3. **Test on Linux** (if applicable):
   - Test on Ubuntu/Debian with bash and zsh

4. **Test with bash shell**:
   ```bash
   bash
   anvs init
   source ~/.bashrc
   # Test activation
   ```

5. **Test with zsh shell**:
   ```bash
   zsh
   anvs init
   source ~/.zshrc
   # Test activation
   ```

6. **Test with nvm installed**:
   - Verify detection works
   - Test activation after wizard

7. **Test with fnm installed**:
   - Verify detection works
   - Test with fnm selected

8. **Test with no version manager** (error case):
   - Should show clear error message
   - Should not crash

9. **Test with existing config** (re-init):
   - Should detect existing config
   - Should prompt to overwrite with `--force`

**Testing Checklist:**
- [ ] macOS Intel tested
- [ ] macOS ARM tested
- [ ] Linux tested (optional)
- [ ] bash tested
- [ ] zsh tested
- [ ] nvm tested
- [ ] fnm tested
- [ ] No version manager tested
- [ ] Re-init tested

**Dependencies:**
- Requires: All implementation complete
- Enables: Release

---

### Task 19: Release Preparation

**Objective:** Prepare for v2.1.0 release.

**Implementation Steps:**

1. **Update version to v2.1.0**:
   ```bash
   ./scripts/bump-version.sh minor
   # Or manually update:
   # - Cargo.toml
   # - package.json
   # - Any other version references
   ```

2. **Update CHANGELOG.md**:
   ```markdown
   ## [2.1.0] - 2025-XX-XX

   ### Changed
   - Redesigned `anvs init` wizard for speed and visual polish (#XX)
   - Quick mode is now the default (completes in < 30 seconds)
   - Timeline-style progress indicators inspired by Vite
   - Minimal prompts with inline detection display

   ### Added
   - `--advanced` flag for full customization wizard
   - Visual progress indicators during installation
   - Improved completion messages with next steps
   ```

3. **Create release notes**:
   ```markdown
   # v2.1.0 - Wizard Redesign

   We've completely redesigned the `anvs init` wizard to be faster, cleaner,
   and more visually appealing—inspired by modern CLI tools like Vite and ShadCN.

   ## Highlights

   - **Quick by default**: Setup completes in < 30 seconds with smart auto-detection
   - **Beautiful visuals**: Timeline-style progress indicators and clean layout
   - **Minimal prompts**: Just one confirmation needed for most users
   - **Advanced mode**: Full customization available via `--advanced` flag

   ## Examples

   Quick setup (default):
   ```bash
   anvs init
   ```

   Advanced setup:
   ```bash
   anvs init --advanced
   ```

   ## Breaking Changes

   - Quick mode is now the default (was verbose wizard)
   - Use `--advanced` to access the full wizard
   ```

4. **Tag release**:
   ```bash
   git add .
   git commit -m "chore(release): bump version to v2.1.0"
   git tag v2.1.0
   ```

5. **Build and test release binaries**:
   ```bash
   cargo build --release
   ./target/release/anvs --version
   ./target/release/anvs init --help
   ```

**Code Structure:**
- Files: `Cargo.toml`, `package.json`, `CHANGELOG.md`
  - Updated version numbers
  - Release notes

**Testing:**
```bash
./scripts/version.sh
git tag
```

**Dependencies:**
- Requires: All tasks complete, QA passed
- Enables: Publication

---

### Task 20: Publish Release

**Objective:** Publish v2.1.0 to npm and Homebrew.

**Implementation Steps:**

1. **Push to GitHub**:
   ```bash
   git push origin main
   git push --tags
   ```

2. **Wait for CI to build artifacts**:
   - Monitor GitHub Actions
   - Ensure all platform builds succeed

3. **Download release artifacts**:
   ```bash
   npm run release:download
   ```

4. **Extract binaries**:
   ```bash
   npm run release:extract
   ```

5. **Publish to npm**:
   ```bash
   npm publish
   ```

6. **Update Homebrew formula**:
   ```bash
   cd ../homebrew-anvs
   ./scripts/setup-homebrew-tap.sh
   git add Formula/anvs.rb
   git commit -m "chore: update anvs to v2.1.0"
   git push origin main
   ```

7. **Announce the update**:
   - GitHub Releases page
   - Project README (if applicable)
   - Social media (if applicable)

**Testing:**
```bash
npm view anvs version
brew upgrade anvs
anvs --version
```

**Dependencies:**
- Requires: Task 19 (Release Preparation)
- Enables: Milestone complete!

---

## Integration Points

### How Tasks Work Together

1. **Visual Foundation** (Phase 1):
   - Timeline module provides reusable rendering functions
   - Summary module builds on timeline for detection/config display
   - Prompts module uses detection results for inline display

2. **Quick Mode Flow** (Phase 2):
   - Detection runs first → Summary displays results → Single prompt → Progress indicators → Completion message

3. **Advanced Mode Flow** (Phase 3):
   - Detection provides defaults → 3-step wizard → Configuration summary → Installation

4. **CLI Integration** (Phase 4):
   - Flags determine mode → Mode routes to appropriate wizard → Wizard returns config → Installation proceeds

## Testing Strategy

### Unit Tests
- Timeline rendering functions
- Summary formatting functions
- Detection logic
- Config conversion functions
- WizardMode flag parsing

### Integration Tests
- Quick wizard end-to-end (mocked prompts)
- Advanced wizard flow (mocked prompts)
- Non-interactive mode (no prompts)
- Error scenarios (detection failures)

### Manual Testing
- Visual inspection on multiple terminals
- Interactive prompt navigation
- Performance timing
- Cross-platform testing (macOS, Linux, bash, zsh)

### Regression Testing
- Ensure all existing commands still work (`activate`, `status`, `set`, `uninstall`)
- Verify config file format unchanged
- Test upgrade from v2.0.0 → v2.1.0

## Success Criteria

Milestone 13 is complete when:

1. ✅ Quick mode is default and completes in < 30 seconds
2. ✅ Visual design uses timeline/progress indicators with box-drawing characters
3. ✅ Prompts are concise with inline detection display
4. ✅ Advanced mode available via `--advanced` flag with exactly 3 steps
5. ✅ Installation progress shows clear visual feedback
6. ✅ Completion message is helpful with next steps
7. ✅ All tests pass (unit + integration)
8. ✅ Documentation updated (README, CLAUDE.md, help text)
9. ✅ Wizard feels as polished as Vite/ShadCN CLIs
10. ✅ Published as v2.1.0 to npm and Homebrew

---

## Notes

- This is a UX-focused milestone—no functionality changes, only improved experience
- Focus on visual polish and speed optimization
- Maintain backward compatibility with existing flags
- Test frequently during development for visual feedback
- Keep the quality bar high—compare against Vite/ShadCN for inspiration

**Estimated Total Time**: 4-6 hours for experienced Rust developer
