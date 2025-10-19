# Phase 2: Quick Mode Implementation

**Status**: Not Started
**Version Target**: v2.1.0
**Duration Estimate**: 2-3 hours
**Phase Tasks**: 2.1 - 2.3

## Overview

Phase 2 implements the quick mode wizard—the new default experience for `anvs init`. This mode auto-detects the user's shell and version manager, displays a summary, and requires just one confirmation prompt to complete setup in under 30 seconds.

**Why Phase 2 is Critical:**
- Establishes the new default user experience for first-time setup
- Reduces setup time from ~2 minutes (old wizard) to <30 seconds
- Demonstrates the visual improvements built in Phase 1
- Sets the foundation for the advanced mode (Phase 3) to build upon

**⚠️ CHECKPOINT**: Before starting Phase 2, ensure:
- Phase 1 is 100% complete (all visual components working)
- All Phase 1 tests pass: `cargo test --lib timeline summary`
- You've visually verified timeline and summary rendering on your terminal
- The `src/init/detection.rs` file exists (for shell/version manager detection)

---

## Implementation Tasks

### Task 2.1: Implement Auto-Detection Summary Screen

**Goal**: Create comprehensive detection that runs once and displays all results in a single summary screen.

**File**: `src/init/detection.rs` (existing file)

**Changes Required**:

1. **Add new imports at the top**:
   ```rust
   use crate::init::summary::DetectionResults;
   use crate::config::AutoInstallMode;
   use std::env;
   ```

2. **Add new function: detect_all** (add after existing detection functions):
   ```rust
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

       // Set paths and defaults
       results.config_path = get_config_path();
       results.auto_install = AutoInstallMode::Prompt; // Default

       Ok(results)
   }

   /// Get the path to the shell binary
   fn get_shell_path(shell: &Shell) -> Result<String> {
       env::var("SHELL").map_err(|e| anyhow::anyhow!("Failed to get shell path: {}", e))
   }

   /// Get the configuration file path
   fn get_config_path() -> String {
       dirs::home_dir()
           .map(|h| h.join(".anvsrc").display().to_string())
           .unwrap_or_else(|| "~/.anvsrc".to_string())
   }
   ```

3. **Add tests at the bottom of the file** (in `#[cfg(test)] mod tests`):
   ```rust
   #[test]
   fn test_detect_all_returns_results() {
       let results = detect_all();
       // Should not panic, may or may not detect shell
       assert!(results.is_ok());
   }

   #[test]
   fn test_get_config_path_not_empty() {
       let path = get_config_path();
       assert!(!path.is_empty());
       assert!(path.contains(".anvsrc"));
   }
   ```

**Commands**:

```bash
# Verify compilation
cargo check

# Run detection tests
cargo test --lib detection

# Expected output:
# running 2+ tests (existing + new tests)
# test init::detection::tests::test_detect_all_returns_results ... ok
# test init::detection::tests::test_get_config_path_not_empty ... ok
```

**Actions**:
- [ ] Open `src/init/detection.rs` in your editor
- [ ] Add the new imports at the top
- [ ] Add `detect_all()`, `get_shell_path()`, and `get_config_path()` functions
- [ ] Add the new tests in the tests module
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo test --lib detection` and ensure tests pass
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`

---

**File**: `src/init/wizard.rs` (existing file)

**Changes Required**:

1. **Add new imports at the top**:
   ```rust
   use crate::init::summary::{format_detection_summary, DetectionResults};
   use crate::init::prompts::{prompt_quick_mode_confirmation, QuickModeChoice};
   use crate::init::detection::detect_all;
   use crate::output;
   use anyhow::{anyhow, Result};
   ```

2. **Add helper function: results_to_config**:
   ```rust
   /// Convert detection results to a Config object
   fn results_to_config(results: &DetectionResults) -> Result<Config> {
       Ok(Config {
           plugins: if results.version_managers.is_empty() {
               // Default to nvm if nothing detected
               vec!["nvm".to_string()]
           } else {
               results.version_managers.clone()
           },
           auto_install: results.auto_install.clone(),
           version_files: vec![
               ".nvmrc".to_string(),
               ".node-version".to_string(),
               "package.json".to_string(),
           ],
           use_default: true,
       })
   }
   ```

3. **Add main function: run_quick_wizard**:
   ```rust
   /// Run quick mode wizard (default)
   ///
   /// This is the new default wizard experience:
   /// 1. Auto-detect shell and version manager
   /// 2. Display summary of detected values
   /// 3. Single confirmation prompt
   /// 4. Done!
   pub fn run_quick_wizard() -> Result<(Config, Shell)> {
       // Print header
       println!();
       output::brand("⚡ Automatic Node Version Switcher");
       println!();

       // Run detection
       log::debug!("Running auto-detection...");
       let results = detect_all()?;
       log::debug!("Detection complete: shell={:?}, version_managers={:?}",
           results.shell, results.version_managers);

       // Show summary
       println!("{}", format_detection_summary(&results));
       println!();

       // Check if critical detection failed
       if results.shell.is_none() {
           output::warning("⚠️  Shell auto-detection failed");
           println!();
           output::info("Please use advanced mode to configure manually:");
           output::info("  anvs init --advanced");
           return Err(anyhow!("Shell detection failed. Use --advanced mode or specify --shell flag."));
       }

       if results.version_managers.is_empty() {
           output::warning("⚠️  No version managers detected");
           output::info("anvs will default to nvm. Ensure nvm or fnm is installed.");
           println!();
           // Continue anyway with nvm as default
       }

       // Single confirmation prompt
       match prompt_quick_mode_confirmation(&results)? {
           QuickModeChoice::Proceed => {
               log::debug!("User accepted quick mode configuration");
               // User accepted defaults
               let shell = results.shell
                   .ok_or_else(|| anyhow!("Shell not detected"))?;
               let config = results_to_config(&results)?;
               Ok((config, shell))
           }
           QuickModeChoice::Customize => {
               log::debug!("User chose to customize settings");
               // Drop into advanced mode
               println!();
               output::info("Switching to advanced mode...");
               println!();
               run_advanced_wizard()
           }
           QuickModeChoice::Cancel => {
               log::debug!("User cancelled setup");
               Err(anyhow!("Setup cancelled by user"))
           }
       }
   }
   ```

4. **Add placeholder for advanced wizard** (if it doesn't exist yet):
   ```rust
   /// Run advanced mode wizard (placeholder for Phase 3)
   pub fn run_advanced_wizard() -> Result<(Config, Shell)> {
       // TODO: Implement in Phase 3
       Err(anyhow!("Advanced mode not yet implemented. Coming in Phase 3."))
   }
   ```

**Commands**:

```bash
# Check compilation
cargo check

# Run wizard tests (if any exist)
cargo test --lib wizard

# Expected output:
# Compiling anvs v2.1.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 3.45s
```

**Actions**:
- [ ] Open `src/init/wizard.rs` in your editor
- [ ] Add new imports at the top
- [ ] Add `results_to_config()` helper function
- [ ] Add `run_quick_wizard()` function
- [ ] Add `run_advanced_wizard()` placeholder (if not present)
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`
- [ ] Manually test the quick wizard (Task 2.3 will integrate with CLI):
  ```rust
  // In main.rs temporarily
  use anvs::init::wizard::run_quick_wizard;

  fn main() {
      if let Ok((config, shell)) = run_quick_wizard() {
          println!("Config: {:?}", config);
          println!("Shell: {:?}", shell);
      }
  }
  ```
- [ ] Run `cargo run` and verify:
  - Header displays correctly
  - Detection summary shows up with your actual shell/version manager
  - Prompt appears with 3 options
  - Selecting "Yes, continue" returns config and shell
  - Selecting "Cancel" returns error
- [ ] Remove temporary test from `main.rs`

---

### Task 2.2: Add Installation Progress Indicators

**Goal**: Show clear visual feedback during the installation process using timeline-style progress.

**File**: `src/init/wizard.rs` (existing file, continue editing)

**Changes Required**:

1. **Add new imports**:
   ```rust
   use crate::init::timeline::{Step, StepState, chars};
   use std::time::Instant;
   ```

2. **Create InstallationProgress struct** (add before wizard functions):
   ```rust
   /// Installation progress tracker for visual feedback
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

       fn get(&self, index: usize) -> Option<&Step> {
           self.steps.get(index)
       }
   }
   ```

3. **Add install_config function** (or refactor existing one):
   ```rust
   /// Install configuration and shell hook with progress indicators
   pub fn install_config(config: Config, shell: Shell, force: bool) -> Result<()> {
       use crate::init::timeline::render_step;

       println!();
       output::brand("⚡ Automatic Node Version Switcher");
       println!();

       let mut progress = InstallationProgress::new();

       // Print header
       println!("{}  Installing", chars::STEP_ACTIVE);

       // Step 1: Create config
       progress.mark_active(0);
       crate::config::save_config(&config)
           .map_err(|e| anyhow!("Failed to create config: {}", e))?;
       progress.mark_complete(0);
       if let Some(step) = progress.get(0) {
           println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
       }

       // Step 2: Install shell hook
       progress.mark_active(1);
       install_shell_hook(&shell, force)
           .map_err(|e| anyhow!("Failed to install shell hook: {}", e))?;
       progress.mark_complete(1);
       if let Some(step) = progress.get(1) {
           println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
       }

       // Step 3: Validate
       progress.mark_active(2);
       validate_installation(&shell)
           .map_err(|e| anyhow!("Validation failed: {}", e))?;
       progress.mark_complete(2);
       if let Some(step) = progress.get(2) {
           println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
       }

       // Step 4: Test activation (optional, may skip)
       progress.mark_active(3);
       // Test activation is optional and may not be implemented yet
       match test_activation() {
           Ok(_) => {
               progress.mark_complete(3);
               if let Some(step) = progress.get(3) {
                   println!("{}  {}", chars::BRANCH_LAST, render_step(step));
               }
           }
           Err(_) => {
               // Skip test activation if not implemented
               log::debug!("Skipping activation test (not implemented)");
               if let Some(step) = progress.get(3) {
                   println!("{}  {} (skipped)", chars::BRANCH_LAST, step.label.dimmed());
               }
           }
       }

       Ok(())
   }

   // Placeholder functions (implement or use existing)
   fn install_shell_hook(shell: &Shell, force: bool) -> Result<()> {
       // Use existing shell profile modification logic
       crate::setup::modify_shell_profile(shell, force)
   }

   fn validate_installation(shell: &Shell) -> Result<()> {
       // Basic validation: check that config file exists
       let config_path = dirs::home_dir()
           .ok_or_else(|| anyhow!("Could not find home directory"))?
           .join(".anvsrc");

       if !config_path.exists() {
           return Err(anyhow!("Config file not created at {:?}", config_path));
       }

       log::debug!("Installation validated successfully");
       Ok(())
   }

   fn test_activation() -> Result<()> {
       // Placeholder - may not be implemented yet
       log::debug!("Activation test not implemented");
       Ok(())
   }
   ```

**Commands**:

```bash
# Check compilation
cargo check

# Build and test manually
cargo build

# Expected output:
# Compiling anvs v2.1.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 4.23s
```

**Actions**:
- [ ] Open `src/init/wizard.rs` (continuing from Task 2.1)
- [ ] Add timeline imports
- [ ] Add `InstallationProgress` struct and implementation
- [ ] Add `install_config()` function with progress indicators
- [ ] Add helper functions: `install_shell_hook()`, `validate_installation()`, `test_activation()`
- [ ] Note: You may need to adjust function signatures based on your existing codebase
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`
- [ ] If you have existing installation logic in `src/setup/`, integrate it here instead of creating new functions

---

### Task 2.3: Create Completion Screen

**Goal**: Display a clean, helpful completion message with next steps and timing information.

**File**: `src/init/wizard.rs` (existing file, continue editing)

**Changes Required**:

1. **Add function: show_completion_message**:
   ```rust
   /// Display completion message with next steps
   fn show_completion_message(shell: &Shell, duration: std::time::Duration) -> Result<()> {
       use crate::init::summary::format_next_steps;

       println!();
       output::success("✓ Setup complete!");

       // Show timing if < 60 seconds
       if duration.as_secs() < 60 {
           output::info(&format!("Completed in {:.1}s", duration.as_secs_f64()));
       } else {
           output::info(&format!("Completed in {}m {}s",
               duration.as_secs() / 60,
               duration.as_secs() % 60));
       }

       println!();
       println!("{}", format_next_steps(shell));

       Ok(())
   }
   ```

2. **Add full integration function: handle_init** (or update existing):
   ```rust
   /// Handle the complete init flow (detection -> wizard -> install -> completion)
   pub fn handle_init(quick: bool, advanced: bool, force: bool) -> Result<()> {
       let start = Instant::now();

       // Determine mode (for now, always use quick mode)
       let (config, shell) = if advanced {
           run_advanced_wizard()?
       } else {
           run_quick_wizard()?
       };

       log::debug!("Wizard completed, proceeding with installation");

       // Install
       install_config(config, shell.clone(), force)?;

       // Show completion
       show_completion_message(&shell, start.elapsed())?;

       Ok(())
   }
   ```

**Commands**:

```bash
# Check compilation
cargo check

# Run a full end-to-end test (manual)
cargo run -- init --help

# Expected output:
# Should show updated help text with quick/advanced modes
```

**Actions**:
- [ ] Add `show_completion_message()` function to `src/init/wizard.rs`
- [ ] Add `handle_init()` function (or update existing one)
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`

---

**File**: `src/commands/mod.rs` or wherever `init` command is handled (existing file)

**Changes Required**:

Wire up the new wizard to the CLI command handler.

1. **Find the init command handler** (likely in `src/commands/mod.rs` or `src/main.rs`):
   - Look for where `Commands::Init { ... }` is matched

2. **Update the handler to call the new wizard**:
   ```rust
   // Example location: src/commands/mod.rs or src/main.rs
   use crate::init::wizard::handle_init;

   // In the match statement for CLI commands:
   Commands::Init { quick, advanced, force, .. } => {
       log::debug!("Running init command: quick={}, advanced={}, force={}", quick, advanced, force);
       handle_init(quick, advanced, force)?;
   }
   ```

**Commands**:

```bash
# Test the full flow
cargo build --release

# Run init in quick mode
./target/release/anvs init

# Expected behavior:
# 1. Shows header "⚡ Automatic Node Version Switcher"
# 2. Shows detection summary box
# 3. Shows confirmation prompt
# 4. After confirming, shows progress indicators
# 5. Shows completion message with next steps
```

**Actions**:
- [ ] Find where `Commands::Init` is handled in your codebase
- [ ] Update the handler to call `handle_init(quick, advanced, force)`
- [ ] Ensure the CLI flags are passed correctly
- [ ] Run `cargo check`
- [ ] Build: `cargo build --release`
- [ ] Test the full flow: `./target/release/anvs init`
- [ ] Verify each step displays correctly:
  - [ ] Header shows
  - [ ] Detection summary shows your actual shell/version manager
  - [ ] Confirmation prompt appears
  - [ ] Progress indicators show during installation
  - [ ] Completion message shows with next steps
  - [ ] Timing information displays
- [ ] Test cancellation: run again and select "Cancel"
- [ ] Verify no config is created when cancelled

---

## Verification Checklist

Before proceeding to Phase 3, verify ALL of the following:

- [ ] File `src/init/detection.rs` has `detect_all()` function
- [ ] File `src/init/wizard.rs` has `run_quick_wizard()` function
- [ ] File `src/init/wizard.rs` has `install_config()` with progress indicators
- [ ] File `src/init/wizard.rs` has `show_completion_message()` function
- [ ] File `src/init/wizard.rs` has `handle_init()` integration function
- [ ] CLI command handler calls `handle_init()` correctly
- [ ] `cargo check` completes without errors
- [ ] `cargo test` passes all tests (including new detection tests)
- [ ] `cargo fmt` has been run on all modified files
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] Manual test: `anvs init` completes successfully in quick mode
- [ ] Manual test: Detection summary shows correct shell and version manager
- [ ] Manual test: Confirmation prompt offers 3 choices (Yes/Customize/Cancel)
- [ ] Manual test: Selecting "Yes, continue" proceeds with installation
- [ ] Manual test: Progress indicators display for all 4 steps
- [ ] Manual test: Completion message shows with next steps
- [ ] Manual test: Timing information is accurate (should be < 30 seconds)
- [ ] Manual test: Cancellation works (no partial config created)
- [ ] Visual test: All output looks good on dark theme
- [ ] Visual test: All output looks good on light theme
- [ ] Config file `~/.anvsrc` is created correctly
- [ ] Shell hook is installed in `~/.zshrc` or `~/.bashrc`

---

## Success Criteria

Phase 2 is complete when:

1. ✅ Quick mode wizard runs as the default for `anvs init`
2. ✅ Auto-detection runs once and displays comprehensive summary
3. ✅ Single confirmation prompt offers Proceed/Customize/Cancel
4. ✅ Installation shows 4 progress steps with visual indicators
5. ✅ Completion message displays next steps clearly
6. ✅ Entire flow completes in < 30 seconds
7. ✅ "Customize settings" transitions to advanced mode (shows placeholder for now)
8. ✅ Cancellation works correctly without creating partial config
9. ✅ All tests pass
10. ✅ Visual output is polished and consistent

---

## Next Steps

After completing Phase 2:

1. Run a final full test of the quick mode flow
2. Time the flow to ensure it's < 30 seconds
3. Commit your changes:
   ```bash
   git add src/init/detection.rs src/init/wizard.rs src/commands/
   git commit -m "feat(init): implement quick mode wizard (Phase 2)

   - Add comprehensive auto-detection (detect_all)
   - Implement quick mode wizard with single confirmation
   - Add installation progress indicators (4 steps)
   - Add completion screen with next steps
   - Wire up to CLI command handler
   - Quick mode completes in < 30 seconds

   Files changed:
   - src/init/detection.rs (detect_all function)
   - src/init/wizard.rs (quick mode implementation)
   - src/commands/mod.rs (CLI integration)"
   ```
4. **Proceed to Phase 3**: Advanced Mode Refinement

---

## Rollback Plan

If issues are discovered in Phase 2:

1. To rollback wizard changes:
   ```bash
   git checkout HEAD -- src/init/wizard.rs
   ```

2. To rollback detection changes:
   ```bash
   git checkout HEAD -- src/init/detection.rs
   ```

3. To rollback all Phase 2 changes:
   ```bash
   git reset --hard HEAD~1  # If committed
   # OR
   git checkout HEAD -- src/init/  # If not committed
   ```

4. To restore working state:
   ```bash
   cargo clean
   cargo build
   cargo test
   ```

---

## Notes

- **Error Handling**: The quick mode wizard fails gracefully if shell detection fails. Users are directed to use `--advanced` mode or specify `--shell` flag. This is intentional to keep quick mode simple and fast.

- **Version Manager Detection**: If no version manager is detected, the wizard defaults to `nvm` with a warning. This allows setup to proceed but informs the user they need to install a version manager.

- **Timing Goal**: The < 30 second goal includes user interaction time (reading summary and confirming). The actual code execution should be < 5 seconds.

- **Progress Indicators**: The 4 installation steps are:
  1. Create config file (~/.anvsrc)
  2. Install shell hook (modify ~/.zshrc or ~/.bashrc)
  3. Validate installation (check files exist)
  4. Test activation (optional, may be skipped if not implemented)

- **Customization**: Selecting "Customize settings" will transition to advanced mode (Phase 3). For now, it shows a placeholder error. This will be implemented in Phase 3.

- **Dependencies**: Phase 2 depends entirely on Phase 1 being complete. The timeline and summary modules are heavily used here.

- **Estimated Time**:
  - Task 2.1: 60-75 minutes (detection + quick wizard)
  - Task 2.2: 45-60 minutes (progress indicators)
  - Task 2.3: 30-45 minutes (completion screen + integration)
  - Total: 2-3 hours

- **Testing Priority**: Manual testing is critical for this phase. The visual output and user experience are the primary deliverables. Automated tests are secondary.
