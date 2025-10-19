# Phase 3: Advanced Mode Refinement

**Status**: Not Started
**Version Target**: v2.1.0
**Duration Estimate**: 2-3 hours
**Phase Tasks**: 3.1 - 3.3

## Overview

Phase 3 implements the advanced mode wizard—a streamlined 3-step flow for users who want to customize their configuration. This mode shows detected values inline with prompts, allows easy overrides, and provides a final confirmation summary before applying changes.

**Why Phase 3 is Critical:**
- Provides power users with full customization while maintaining simplicity
- Completes the wizard redesign by implementing the "Customize settings" option from quick mode
- Demonstrates inline detection display, making it clear what was auto-detected vs. user-selected
- Reduces advanced wizard from ~5-7 steps (old) to exactly 3 steps (new)

**⚠️ CHECKPOINT**: Before starting Phase 3, ensure:
- Phase 2 is 100% complete (quick mode working)
- The `run_quick_wizard()` function exists and works correctly
- Prompt functions in `src/init/prompts.rs` include detection-aware versions
- The `format_config_preview()` function exists in `src/init/summary.rs`

---

## Implementation Tasks

### Task 3.1: Implement Step-by-Step Advanced Wizard

**Goal**: Create a clean 3-step advanced wizard that shows detected values inline and allows customization.

**File**: `src/init/wizard.rs` (existing file)

**Changes Required**:

1. **Replace the `run_advanced_wizard()` placeholder function** (currently at line ~665):

Find this:
```rust
/// Run advanced mode wizard (placeholder for Phase 3)
pub fn run_advanced_wizard() -> Result<(Config, Shell)> {
    // TODO: Implement in Phase 3
    Err(anyhow!(
        "Advanced mode not yet implemented. Coming in Phase 3."
    ))
}
```

Replace with:
```rust
/// Run advanced mode wizard (3-step customization flow)
///
/// This provides full customization with inline detection:
/// 1. Shell selection (with detected value pre-selected)
/// 2. Version manager selection (with detected values)
/// 3. Auto-install behavior
pub fn run_advanced_wizard() -> Result<(Config, Shell)> {
    use crate::init::prompts::{
        prompt_auto_install_mode, prompt_shell_with_detection,
        prompt_version_manager_with_detection,
    };
    use crate::init::summary::format_config_preview;

    // Print header
    println!();
    output::brand("⚡ Automatic Node Version Switcher");
    println!();
    output::info("Advanced Setup - Customize your configuration");
    println!();

    // Run detection for defaults
    log::debug!("Running detection for advanced mode defaults...");
    let results = detect_all()?;

    // Step 1: Shell selection
    println!();
    println!("{} {}", chars::STEP_ACTIVE, "Step 1 of 3: Shell Configuration".bold());
    let shell = prompt_shell_with_detection(results.shell.as_ref())?;
    log::debug!("Selected shell: {:?}", shell);

    // Step 2: Version manager selection
    println!();
    println!("{} {}", chars::STEP_ACTIVE, "Step 2 of 3: Version Manager".bold());
    let version_managers = prompt_version_manager_with_detection(results.version_managers.clone())?;
    log::debug!("Selected version managers: {:?}", version_managers);

    // Step 3: Auto-install behavior
    println!();
    println!("{} {}", chars::STEP_ACTIVE, "Step 3 of 3: Auto-Install Behavior".bold());
    let auto_install = prompt_auto_install_mode()?;
    log::debug!("Selected auto-install mode: {:?}", auto_install);

    // Create config from selections
    let config = Config {
        plugins: version_managers,
        auto_install,
        version_files: vec![
            ".nvmrc".to_string(),
            ".node-version".to_string(),
            "package.json".to_string(),
        ],
        use_default: true,
    };

    // Show configuration preview and confirm
    println!();
    println!("{}", format_config_preview(&config, &shell));
    println!();

    let confirmed = inquire::Confirm::new("Apply this configuration?")
        .with_default(true)
        .with_help_message("Select 'No' to cancel setup")
        .prompt()?;

    if !confirmed {
        return Err(anyhow!("Setup cancelled by user"));
    }

    Ok((config, shell))
}
```

2. **Add the required imports at the top** (if not already present):

Check if these imports exist at the top of the file, and add any that are missing:
```rust
use crate::init::timeline::chars;
use owo_colors::OwoColorize;
use inquire;
```

**Commands**:

```bash
# Verify compilation
cargo check

# Expected output:
# Checking anvs v2.0.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.8s
```

**Actions**:
- [ ] Open `src/init/wizard.rs` in your editor
- [ ] Locate the `run_advanced_wizard()` placeholder function (around line 665)
- [ ] Replace it with the complete implementation above
- [ ] Check that required imports are present at the top of the file
- [ ] Add any missing imports (chars, OwoColorize, inquire)
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`

---

### Task 3.2: Update Prompt Functions for Advanced Mode

**Goal**: Ensure all prompt functions needed by advanced mode exist with correct signatures.

**File**: `src/init/prompts.rs` (existing file)

**Changes Required**:

The Phase 1 implementation added some prompt functions. We need to verify they exist and add any missing ones:

1. **Verify `prompt_shell_with_detection` exists** (should be around line 31):
   - If it exists, no changes needed
   - If it's missing or has a different name, add it:

```rust
/// Shell selection prompt with inline detection
pub fn prompt_shell_with_detection(detected: Option<&Shell>) -> Result<Shell> {
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

    let selected = Select::new(&message, options)
        .with_starting_cursor(0) // Pre-select first option
        .prompt()?;

    // Parse selection
    if selected.contains("zsh") {
        Ok(Shell::Zsh)
    } else if selected.contains("bash") {
        Ok(Shell::Bash)
    } else if let Some(shell) = detected {
        Ok(*shell)
    } else {
        Err(anyhow::anyhow!("Invalid shell selection"))
    }
}
```

2. **Verify `prompt_version_manager_with_detection` exists** (should be around line 70):
   - If it exists, no changes needed
   - If missing, add it:

```rust
/// Version manager selection with detection
pub fn prompt_version_manager_with_detection(detected: Vec<String>) -> Result<Vec<String>> {
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
    let options = vec!["nvm", "fnm"];
    let defaults = detected
        .iter()
        .filter(|d| options.contains(&d.as_str()))
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    let selected = MultiSelect::new("Select version managers:", options)
        .with_default(&defaults)
        .prompt()?;

    if selected.is_empty() {
        Err(anyhow::anyhow!(
            "At least one version manager must be selected"
        ))
    } else {
        Ok(selected.iter().map(|s| s.to_string()).collect())
    }
}
```

3. **Add or verify `prompt_auto_install_mode` function**:

```rust
/// Auto-install mode selection
pub fn prompt_auto_install_mode() -> Result<AutoInstallMode> {
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
# Check compilation
cargo check

# Run tests
cargo test --lib prompts

# Expected output:
# Checking anvs v2.0.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.8s
```

**Actions**:
- [ ] Open `src/init/prompts.rs` in your editor
- [ ] Check if `prompt_shell_with_detection` exists (search for it)
- [ ] If missing, add the function as shown above
- [ ] Check if `prompt_version_manager_with_detection` exists
- [ ] If missing, add it along with the helper function `prompt_multiple_version_managers`
- [ ] Check if `prompt_auto_install_mode` exists
- [ ] If missing, add it as shown above
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`

---

### Task 3.3: Test Advanced Mode End-to-End

**Goal**: Verify the advanced mode wizard works correctly with all scenarios.

**Manual Testing Steps**:

1. **Test with full detection (happy path)**:
```bash
# Build release version
cargo build --release

# Run advanced mode
./target/release/anvs init --advanced

# Expected flow:
# 1. Shows header "⚡ Automatic Node Version Switcher"
# 2. Shows "Advanced Setup - Customize your configuration"
# 3. Step 1: Shell prompt with detected value pre-selected
# 4. Step 2: Version manager prompt with detected values
# 5. Step 3: Auto-install behavior prompt
# 6. Shows configuration preview box
# 7. Confirmation prompt
# 8. If confirmed, runs installation with progress indicators
# 9. Shows completion message
```

2. **Test switching from quick mode to advanced**:
```bash
# Run quick mode and select "Customize settings"
./target/release/anvs init

# When prompted, select "Customize settings"
# Should transition to advanced mode wizard
# Verify all 3 steps appear
```

3. **Test cancellation**:
```bash
# Run advanced mode
./target/release/anvs init --advanced

# Complete all 3 steps
# When asked "Apply this configuration?", select "No"
# Should cancel without creating config
# Verify: ls ~/.anvsrc (should not exist if this was first run)
```

4. **Test with no detection**:
```bash
# Temporarily rename shell env or test in clean environment
# Run advanced mode
./target/release/anvs init --advanced

# Verify prompts handle missing detection gracefully
# Should show "auto-detection failed" messages
# Should still allow manual selection
```

**Commands**:

```bash
# Run all tests to ensure nothing broke
cargo test

# Expected output:
# test result: ok. [all tests pass]

# Build release for manual testing
cargo build --release

# Test advanced mode
./target/release/anvs init --advanced

# Test quick->advanced transition
./target/release/anvs init
# (select "Customize settings" when prompted)
```

**Actions**:
- [ ] Build release version: `cargo build --release`
- [ ] Test Scenario 1: Full detection (happy path)
  - [ ] Run `./target/release/anvs init --advanced`
  - [ ] Verify header displays correctly
  - [ ] Verify "Step 1 of 3" shows for shell selection
  - [ ] Verify detected shell is pre-selected
  - [ ] Verify "Step 2 of 3" shows for version manager
  - [ ] Verify detected version managers show "(detected, recommended)"
  - [ ] Verify "Step 3 of 3" shows for auto-install
  - [ ] Verify configuration preview box appears
  - [ ] Verify confirmation prompt works
  - [ ] Verify installation runs with progress indicators
  - [ ] Verify completion message appears
- [ ] Test Scenario 2: Quick->Advanced transition
  - [ ] Run `./target/release/anvs init`
  - [ ] Select "Customize settings"
  - [ ] Verify transitions to advanced mode
  - [ ] Verify all 3 steps appear
- [ ] Test Scenario 3: Cancellation
  - [ ] Run advanced mode
  - [ ] Complete all steps
  - [ ] Select "No" at confirmation
  - [ ] Verify no config created
- [ ] Test Scenario 4: Override detection
  - [ ] Run advanced mode
  - [ ] Select different shell than detected
  - [ ] Select different version manager than detected
  - [ ] Verify choices are respected in config
- [ ] Clean up test configs: `rm ~/.anvsrc` (if testing)
- [ ] Run full test suite: `cargo test`
- [ ] Verify all tests still pass

---

## Verification Checklist

Before proceeding to Phase 4, verify ALL of the following:

- [ ] File `src/init/wizard.rs` has complete `run_advanced_wizard()` implementation
- [ ] File `src/init/prompts.rs` has `prompt_shell_with_detection()` function
- [ ] File `src/init/prompts.rs` has `prompt_version_manager_with_detection()` function
- [ ] File `src/init/prompts.rs` has `prompt_auto_install_mode()` function
- [ ] `cargo check` completes without errors
- [ ] `cargo test` passes all tests
- [ ] `cargo fmt` has been run on all modified files
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] Manual test: `anvs init --advanced` runs successfully
- [ ] Manual test: Advanced mode shows "Step 1 of 3", "Step 2 of 3", "Step 3 of 3"
- [ ] Manual test: Detected values show "(detected, recommended)" labels
- [ ] Manual test: Detected values are pre-selected in prompts
- [ ] Manual test: Configuration preview displays before confirmation
- [ ] Manual test: Selecting "No" at confirmation cancels without creating config
- [ ] Manual test: Selecting "Yes" at confirmation proceeds with installation
- [ ] Manual test: Quick mode -> "Customize settings" -> transitions to advanced mode
- [ ] Manual test: Advanced mode completes successfully and creates `~/.anvsrc`
- [ ] Manual test: Advanced mode shows progress indicators during installation
- [ ] Manual test: Advanced mode shows completion message with next steps
- [ ] Visual test: All output looks good on dark theme
- [ ] Visual test: All output looks good on light theme

---

## Success Criteria

Phase 3 is complete when:

1. ✅ Advanced mode wizard has exactly 3 steps
2. ✅ Each step shows "Step X of 3" counter
3. ✅ Detected values appear inline with "(detected, recommended)" labels
4. ✅ Detected values are pre-selected in prompts
5. ✅ Configuration preview box displays before final confirmation
6. ✅ "Apply this configuration?" confirmation works correctly
7. ✅ Advanced mode integrates with installation flow (progress indicators, completion)
8. ✅ Quick mode "Customize settings" transitions to advanced mode
9. ✅ Advanced mode can be triggered via `anvs init --advanced`
10. ✅ All tests pass
11. ✅ Visual output is polished and consistent

---

## Next Steps

After completing Phase 3:

1. Run a final full test of both quick and advanced modes
2. Verify the complete wizard flow (quick -> customize -> advanced -> install)
3. Commit your changes:
   ```bash
   git add src/init/wizard.rs src/init/prompts.rs
   git commit -m "feat(init): implement advanced mode wizard (Phase 3)

   - Implement 3-step advanced wizard with step counters
   - Add inline detection display with (detected, recommended) labels
   - Add configuration preview before applying changes
   - Support quick->advanced mode transition
   - All prompt functions handle detection gracefully

   Files changed:
   - src/init/wizard.rs (run_advanced_wizard implementation)
   - src/init/prompts.rs (detection-aware prompt functions)"
   ```
4. **Proceed to Phase 4**: CLI Integration

---

## Rollback Plan

If issues are discovered in Phase 3:

1. To rollback wizard changes:
   ```bash
   git checkout HEAD -- src/init/wizard.rs
   ```

2. To rollback prompts changes:
   ```bash
   git checkout HEAD -- src/init/prompts.rs
   ```

3. To rollback all Phase 3 changes:
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

- **Detection Graceful Degradation**: If detection fails (e.g., no shell detected), prompts should still work and show appropriate messages like "auto-detection failed". Users can manually select options.

- **Pre-selection**: The detected values are pre-selected by using `.with_starting_cursor(0)` in inquire Select prompts, and the detected option is placed first in the options list.

- **Step Counter**: The "Step X of 3" display uses the timeline `STEP_ACTIVE` character with bold text for visual consistency with the rest of the wizard.

- **Configuration Preview**: The `format_config_preview()` function from Phase 1 (summary module) is reused here to show a box-style summary before confirmation.

- **Quick->Advanced Transition**: This already works from Phase 2. When users select "Customize settings" in quick mode, it calls `run_advanced_wizard()`. Phase 3 makes that function fully functional.

- **Auto-Install Options**: The three options are:
  1. **Prompt (recommended)**: Ask each time a version is missing
  2. **Always**: Automatically install without asking
  3. **Never**: Error if version is missing (manual installation required)

- **Version Files**: The default version files (`.nvmrc`, `.node-version`, `package.json`) are hardcoded in advanced mode. Future phases could make this customizable.

- **Dependencies**: Phase 3 depends on Phase 1 (visual components) and Phase 2 (detection and quick mode) being complete.

- **Estimated Time**:
  - Task 3.1: 45-60 minutes (wizard implementation)
  - Task 3.2: 30-45 minutes (prompt functions verification/additions)
  - Task 3.3: 45-60 minutes (thorough testing of all scenarios)
  - Total: 2-3 hours

- **Testing Priority**: Manual testing is critical for this phase. The advanced mode is all about UX—the flow, prompts, and visual feedback need to feel smooth and intuitive.

- **Existing Functions**: The Phase 1 implementation already added prompt functions with detection. Task 3.2 is mainly verification that they exist with correct names/signatures. If they're already there, no code changes needed—just verify.
