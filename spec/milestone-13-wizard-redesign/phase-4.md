# Phase 4: CLI Integration & Mode Switching

**Status**: Not Started
**Version Target**: v2.1.0
**Duration Estimate**: 1-2 hours
**Phase Tasks**: 4.1 - 4.3

## Overview

Phase 4 completes the CLI integration by refining the mode detection logic, updating help text to reflect the new wizard behavior, and ensuring all flag combinations work correctly. This phase ensures users can easily access both quick and advanced modes, understand the difference through clear help text, and have a smooth experience regardless of their chosen path.

**Why Phase 4 is Critical:**
- Finalizes the user-facing interface for the new wizard system
- Ensures backward compatibility with existing flags (`--quick`, `--non-interactive`)
- Provides clear documentation through help text so users understand their options
- Completes the mode switching logic so quick->advanced transitions work seamlessly

**⚠️ CHECKPOINT**: Before starting Phase 4, ensure:
- Phase 3 is 100% complete (`run_advanced_wizard()` fully implemented)
- Phase 2 is complete (`run_quick_wizard()` fully implemented)
- Both quick and advanced modes work when tested manually
- `anvs init` currently defaults to quick mode

---

## Implementation Tasks

### Task 4.1: Refine Mode Detection and Routing Logic

**Goal**: Clean up the mode detection logic in `handle_init()` and `init()` to ensure proper routing based on flags.

**File**: `src/init/wizard.rs` (existing file)

**Changes Required**:

1. **Update the `handle_init()` function** (currently at line ~773):

Find this:
```rust
/// Handle the complete init flow (detection -> wizard -> install -> completion)
pub fn handle_init(_quick: bool, advanced: bool, force: bool) -> Result<()> {
    use std::time::Instant;
    let start = Instant::now();

    // Determine mode (for now, always use quick mode)
    let (config, shell) = if advanced {
        run_advanced_wizard()?
    } else {
        run_quick_wizard()?
    };

    log::debug!("Wizard completed, proceeding with installation");

    // Install
    install_config(config, shell, force)?;

    // Show completion
    show_completion_message(&shell, start.elapsed())?;

    Ok(())
}
```

Replace with:
```rust
/// Handle the complete init flow (detection -> wizard -> install -> completion)
pub fn handle_init(quick: bool, advanced: bool, force: bool) -> Result<()> {
    use std::time::Instant;
    let start = Instant::now();

    // Determine wizard mode
    let mode = if advanced {
        log::debug!("Running advanced mode (--advanced flag)");
        WizardMode::Advanced
    } else if quick {
        log::debug!("Running quick mode (--quick flag)");
        WizardMode::Quick
    } else {
        // Default to quick mode when no flags provided
        log::debug!("Running quick mode (default behavior)");
        WizardMode::Quick
    };

    // Check for installation conflicts before proceeding
    check_installation_conflicts()?;

    // Run appropriate wizard
    let (config, shell) = match mode {
        WizardMode::Quick => run_quick_wizard()?,
        WizardMode::Advanced => run_advanced_wizard()?,
    };

    log::debug!("Wizard completed, proceeding with installation");

    // Install
    install_config(config, shell, force)?;

    // Show completion
    show_completion_message(&shell, start.elapsed())?;

    Ok(())
}
```

2. **Add `WizardMode` enum** at the top of the file (after imports, around line 11):

```rust
/// Wizard mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardMode {
    /// Quick mode with auto-detection and single confirmation
    Quick,
    /// Advanced mode with full customization (3-step flow)
    Advanced,
}
```

**File**: `src/init/mod.rs` (existing file)

**Changes Required**:

Update the `init()` function to properly pass the quick flag:

Find this (around line 18):
```rust
/// Main entry point for the init command
pub fn init(quick: bool, advanced: bool, non_interactive: bool, force: bool) -> Result<()> {
    if non_interactive {
        run_non_interactive(force)
    } else if advanced || quick {
        // Use the new quick/advanced mode wizard
        handle_init(quick, advanced, force)
    } else {
        // Default to quick mode (new behavior)
        handle_init(false, false, force)
    }
}
```

Replace with:
```rust
/// Main entry point for the init command
pub fn init(quick: bool, advanced: bool, non_interactive: bool, force: bool) -> Result<()> {
    if non_interactive {
        run_non_interactive(force)
    } else {
        // Route to appropriate wizard mode
        // Default behavior (no flags): quick mode
        handle_init(quick, advanced, force)
    }
}
```

**Commands**:

```bash
# Verify compilation
cargo check

# Expected output:
# Checking anvs v2.0.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 1.2s

# Run linter
cargo clippy -- -D warnings

# Expected output:
# Checking anvs v2.0.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 1.0s
```

**Actions**:
- [ ] Open `src/init/wizard.rs` in your editor
- [ ] Add the `WizardMode` enum after the imports (around line 11)
- [ ] Locate `handle_init()` function (around line 773)
- [ ] Update `handle_init()` to use the `WizardMode` enum
- [ ] Add mode detection logic (advanced -> quick -> default to quick)
- [ ] Ensure `check_installation_conflicts()` is called before wizard
- [ ] Update variable name from `_quick` to `quick` (remove underscore)
- [ ] Add logging statements for debugging
- [ ] Open `src/init/mod.rs` in your editor
- [ ] Simplify the `init()` function routing logic
- [ ] Remove redundant conditionals (all paths go to `handle_init`)
- [ ] Run `cargo check` to verify compilation
- [ ] Run `cargo clippy -- -D warnings` to check for warnings
- [ ] Run `cargo fmt` to format code

---

### Task 4.2: Update CLI Help Text

**Goal**: Update the CLI help text to clearly document quick mode as the default, and explain the `--advanced` flag.

**File**: `src/cli.rs` (existing file)

**Changes Required**:

1. **Update the main `about` text** (lines 10-28):

Find this:
```rust
#[command(
    about = "ANVS - Automatic Node Version Switcher for Node.js",
    long_about = r#"
anvs automatically switches your Node.js version when you cd into a directory
with a .nvmrc or .node-version file. When you leave a project directory, anvs
automatically returns to your default Node.js version.

After installation, run 'anvs init' to configure your shell with an interactive
wizard, or 'anvs init --quick' for automatic setup with sensible defaults.

Examples:
  anvs init               Interactive setup wizard (recommended)
  anvs init --quick       Quick setup with defaults
  anvs activate           Manually activate for current directory
  anvs status             Show configuration and test activation
  anvs set                Change configuration settings
  anvs uninstall          Completely remove anvs

For more information, visit: https://github.com/olvrcc/anvs
"#
)]
```

Replace with:
```rust
#[command(
    about = "ANVS - Automatic Node Version Switcher for Node.js",
    long_about = r#"
anvs automatically switches your Node.js version when you cd into a directory
with a .nvmrc or .node-version file. When you leave a project directory, anvs
automatically returns to your default Node.js version.

After installation, run 'anvs init' to set up your shell with a fast, guided
wizard. The wizard auto-detects your shell and version managers, then completes
setup in under 30 seconds.

Examples:
  anvs init               Quick setup with auto-detection (default)
  anvs init --advanced    Advanced setup with full customization
  anvs activate           Manually activate for current directory
  anvs status             Show configuration and test activation
  anvs set                Change configuration settings
  anvs uninstall          Completely remove anvs

For more information, visit: https://github.com/olvrcc/anvs
"#
)]
```

2. **Update the `Init` command documentation** (lines 42-49):

Find this:
```rust
/// Initialize anvs with interactive configuration wizard
///
/// This command guides you through initial setup with auto-detection
/// and configuration of shell integration, version managers, and preferences.
///
/// For quick setup with defaults: anvs init --quick
/// For automation/CI: anvs init --non-interactive
Init {
```

Replace with:
```rust
/// Initialize anvs with guided configuration wizard
///
/// By default, anvs init runs a quick setup that auto-detects your shell
/// and version managers, then confirms before applying. Setup completes in
/// under 30 seconds.
///
/// Use --advanced for full customization with a 3-step wizard that lets you
/// override detected values and configure all settings.
///
/// Examples:
///   anvs init              Quick setup (default)
///   anvs init --advanced   Full customization
///   anvs init --quick      Explicit quick mode
Init {
```

3. **Update the `--quick` flag help text** (around line 51):

Find this:
```rust
/// Skip wizard and use sensible defaults
#[arg(short, long)]
quick: bool,
```

Replace with:
```rust
/// Use quick mode with auto-detection (default behavior)
#[arg(short, long)]
quick: bool,
```

4. **Update the `--advanced` flag help text** (around line 55):

Find this:
```rust
/// Advanced setup with full customization
#[arg(long)]
advanced: bool,
```

Replace with:
```rust
/// Use advanced mode with 3-step customization wizard
#[arg(long)]
advanced: bool,
```

5. **Update the `--non-interactive` flag help text** (around line 67):

Find this:
```rust
/// Non-interactive mode for automation
#[arg(long)]
non_interactive: bool,
```

Replace with:
```rust
/// Non-interactive mode for scripts/CI (uses all defaults)
#[arg(long)]
non_interactive: bool,
```

**Commands**:

```bash
# Check help text output
cargo build --release

# View main help
./target/release/anvs --help

# Expected output should include:
# Examples:
#   anvs init               Quick setup with auto-detection (default)
#   anvs init --advanced    Advanced setup with full customization

# View init command help
./target/release/anvs init --help

# Expected output should include:
# By default, anvs init runs a quick setup that auto-detects your shell
# and version managers, then confirms before applying. Setup completes in
# under 30 seconds.
#
# Use --advanced for full customization...
```

**Actions**:
- [ ] Open `src/cli.rs` in your editor
- [ ] Update the main `long_about` text (lines 10-28)
- [ ] Change "Interactive setup wizard" to "Quick setup with auto-detection"
- [ ] Update example descriptions to reflect quick mode default
- [ ] Update the `Init` command doc comment (lines 42-49)
- [ ] Add explanation of quick vs advanced mode
- [ ] Add examples showing `anvs init` and `anvs init --advanced`
- [ ] Update `--quick` flag help text to indicate it's the default
- [ ] Update `--advanced` flag help text to mention "3-step wizard"
- [ ] Update `--non-interactive` flag help text to clarify use case
- [ ] Build release binary: `cargo build --release`
- [ ] Test help output: `./target/release/anvs --help`
- [ ] Test init help: `./target/release/anvs init --help`
- [ ] Verify help text is clear, concise, and accurate
- [ ] Run `cargo fmt` to format code

---

### Task 4.3: Test All Flag Combinations

**Goal**: Thoroughly test all CLI flag combinations to ensure correct routing and behavior.

**Test Matrix**:

| Command                              | Expected Mode      | Expected Behavior                              |
|--------------------------------------|-------------------|------------------------------------------------|
| `anvs init`                          | Quick             | Auto-detect, single confirmation, install      |
| `anvs init --quick`                  | Quick             | Same as `anvs init`                            |
| `anvs init --advanced`               | Advanced          | 3-step wizard, configuration preview           |
| `anvs init --non-interactive`        | Non-interactive   | No prompts, use all defaults                   |
| `anvs init --quick --advanced`       | Advanced          | Advanced wins (explicit choice)                |
| `anvs init --force`                  | Quick             | Overwrites existing config without asking      |
| `anvs init --advanced --force`       | Advanced          | 3-step wizard, overwrites config               |
| `anvs init --non-interactive --force`| Non-interactive   | Overwrites config without prompts              |

**Manual Testing Steps**:

1. **Test default behavior**:
```bash
# Build fresh binary
cargo build --release

# Run without flags
./target/release/anvs init

# Expected:
# 1. Shows "⚡ Automatic Node Version Switcher"
# 2. Shows quick mode detection summary
# 3. Single confirmation prompt with options:
#    - Yes, continue
#    - Customize settings
#    - Cancel
# 4. If "Yes": proceeds with installation
# 5. If "Customize": switches to advanced mode
```

2. **Test explicit quick mode**:
```bash
# Run with --quick flag
./target/release/anvs init --quick

# Expected: Same behavior as `anvs init` (quick mode)
```

3. **Test advanced mode**:
```bash
# Run with --advanced flag
./target/release/anvs init --advanced

# Expected:
# 1. Shows "Advanced Setup - Customize your configuration"
# 2. Shows "Step 1 of 3: Shell Configuration"
# 3. Shows "Step 2 of 3: Version Manager"
# 4. Shows "Step 3 of 3: Auto-Install Behavior"
# 5. Shows configuration preview
# 6. Final confirmation prompt
# 7. Installation with progress indicators
```

4. **Test non-interactive mode**:
```bash
# Run non-interactive mode
./target/release/anvs init --non-interactive

# Expected:
# 1. No prompts
# 2. Uses all defaults
# 3. Creates config immediately
# 4. Shows completion message
```

5. **Test flag conflicts** (advanced wins):
```bash
# Run with both --quick and --advanced
./target/release/anvs init --quick --advanced

# Expected: Advanced mode (advanced flag takes precedence)
# Verify: Should show "Step 1 of 3" (not quick mode summary)
```

6. **Test force flag**:
```bash
# First, create a config
./target/release/anvs init --non-interactive

# Verify config exists
cat ~/.anvsrc

# Now run init again without --force
./target/release/anvs init

# Expected: Should show warning about existing config
# (behavior may vary based on implementation)

# Now run with --force
./target/release/anvs init --force

# Expected: Should overwrite without additional warning
```

7. **Test help text**:
```bash
# Check main help
./target/release/anvs --help

# Verify:
# - Examples show "anvs init" as "Quick setup with auto-detection (default)"
# - Examples show "anvs init --advanced" as "Advanced setup..."

# Check init help
./target/release/anvs init --help

# Verify:
# - Description mentions quick mode as default
# - --advanced flag documented clearly
# - --quick flag shows it's the default behavior
```

**Commands**:

```bash
# Build release binary
cargo build --release

# Run all test scenarios above
# (manual execution required)

# After all tests pass, run automated tests
cargo test

# Expected output:
# test result: ok. [all tests pass]

# Check for any clippy warnings
cargo clippy -- -D warnings

# Expected output:
# Checking anvs v2.0.0 (/path/to/anvs)
#     Finished dev [unoptimized + debuginfo] target(s) in 1.0s
```

**Actions**:
- [ ] Build release binary: `cargo build --release`
- [ ] Test Scenario 1: `anvs init` (default quick mode)
  - [ ] Verify quick mode summary appears
  - [ ] Verify single confirmation prompt
  - [ ] Verify "Customize settings" option present
  - [ ] Test "Yes" path (completes installation)
- [ ] Test Scenario 2: `anvs init --quick` (explicit quick)
  - [ ] Verify identical behavior to Scenario 1
- [ ] Test Scenario 3: `anvs init --advanced`
  - [ ] Verify advanced mode header
  - [ ] Verify "Step 1 of 3" through "Step 3 of 3"
  - [ ] Verify configuration preview
  - [ ] Verify final confirmation
  - [ ] Complete installation successfully
- [ ] Test Scenario 4: `anvs init --non-interactive`
  - [ ] Verify no prompts appear
  - [ ] Verify config created with defaults
  - [ ] Verify completion message
- [ ] Test Scenario 5: `anvs init --quick --advanced`
  - [ ] Verify advanced mode wins (shows "Step 1 of 3")
- [ ] Test Scenario 6: Force flag combinations
  - [ ] Create initial config
  - [ ] Test `anvs init` with existing config
  - [ ] Test `anvs init --force` (overwrites)
  - [ ] Test `anvs init --advanced --force`
- [ ] Test Scenario 7: Help text
  - [ ] Run `anvs --help` and verify examples
  - [ ] Run `anvs init --help` and verify descriptions
  - [ ] Verify all flag descriptions are clear
- [ ] Test quick->advanced transition
  - [ ] Run `anvs init`
  - [ ] Select "Customize settings"
  - [ ] Verify transitions to advanced mode
- [ ] Clean up test configs: `rm ~/.anvsrc ~/.zshrc.backup*` (if needed)
- [ ] Run full test suite: `cargo test`
- [ ] Run clippy: `cargo clippy -- -D warnings`
- [ ] Verify all tests pass and no warnings

---

## Verification Checklist

Before proceeding to Phase 5, verify ALL of the following:

- [ ] `WizardMode` enum exists in `src/init/wizard.rs`
- [ ] `handle_init()` uses `WizardMode` enum for routing
- [ ] `init()` in `src/init/mod.rs` properly routes to `handle_init()`
- [ ] Mode detection logic is clear and well-commented
- [ ] CLI help text updated in `src/cli.rs`
- [ ] Main help examples show quick mode as default
- [ ] `anvs init --help` explains quick vs advanced modes
- [ ] All flag help texts are accurate and concise
- [ ] `cargo check` completes without errors
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] `cargo test` passes all tests
- [ ] `cargo fmt` has been run on all modified files
- [ ] Manual test: `anvs init` defaults to quick mode
- [ ] Manual test: `anvs init --quick` works (quick mode)
- [ ] Manual test: `anvs init --advanced` works (advanced mode)
- [ ] Manual test: `anvs init --non-interactive` works (no prompts)
- [ ] Manual test: `anvs init --quick --advanced` uses advanced mode
- [ ] Manual test: `anvs init --force` overwrites existing config
- [ ] Manual test: Help text displays correctly
- [ ] Manual test: All mode transitions work smoothly
- [ ] Visual test: Output looks correct on dark theme
- [ ] Visual test: Output looks correct on light theme

---

## Success Criteria

Phase 4 is complete when:

1. ✅ `WizardMode` enum properly encodes quick/advanced mode selection
2. ✅ `handle_init()` correctly routes based on flags
3. ✅ Quick mode is the default when no flags provided
4. ✅ `--advanced` flag triggers advanced mode (3-step wizard)
5. ✅ `--quick` flag explicitly triggers quick mode (same as default)
6. ✅ `--non-interactive` bypasses all prompts
7. ✅ `--force` flag works with all modes
8. ✅ Flag conflicts resolved correctly (advanced wins)
9. ✅ CLI help text is clear, accurate, and concise
10. ✅ Main help examples reflect new default behavior
11. ✅ `anvs init --help` explains mode differences
12. ✅ All manual test scenarios pass
13. ✅ All automated tests pass
14. ✅ No clippy warnings

---

## Next Steps

After completing Phase 4:

1. Verify the complete user journey:
   - User runs `anvs init` (no flags)
   - Quick mode runs with auto-detection
   - User confirms or customizes
   - Installation completes successfully
   - Completion message shows next steps

2. Test backward compatibility:
   - Existing scripts using `anvs init --quick` still work
   - Existing scripts using `anvs init --non-interactive` still work
   - No breaking changes to CLI interface

3. Commit your changes:
   ```bash
   git add src/cli.rs src/init/mod.rs src/init/wizard.rs
   git commit -m "feat(init): finalize CLI integration for wizard redesign (Phase 4)

   - Add WizardMode enum for clean mode selection
   - Update handle_init() to use enum-based routing
   - Simplify init() routing logic
   - Update CLI help text to reflect quick mode as default
   - Document --advanced flag for 3-step customization
   - Clarify all flag descriptions and examples
   - Test all flag combinations and mode transitions

   Files changed:
   - src/cli.rs (help text updates)
   - src/init/mod.rs (routing simplification)
   - src/init/wizard.rs (WizardMode enum, handle_init refactor)"
   ```

4. **Proceed to Phase 5**: Polish & Testing

---

## Rollback Plan

If issues are discovered in Phase 4:

1. To rollback CLI changes:
   ```bash
   git checkout HEAD -- src/cli.rs
   ```

2. To rollback wizard changes:
   ```bash
   git checkout HEAD -- src/init/wizard.rs
   ```

3. To rollback init module changes:
   ```bash
   git checkout HEAD -- src/init/mod.rs
   ```

4. To rollback all Phase 4 changes:
   ```bash
   git reset --hard HEAD~1  # If committed
   # OR
   git checkout HEAD -- src/cli.rs src/init/mod.rs src/init/wizard.rs  # If not committed
   ```

5. To restore working state:
   ```bash
   cargo clean
   cargo build
   cargo test
   ```

---

## Notes

- **Flag Precedence**: When both `--quick` and `--advanced` are provided, advanced mode takes precedence. This is the expected behavior since `--advanced` is a more specific/explicit choice.

- **Default Behavior Change**: Phase 4 formalizes the change to make quick mode the default. Previously, running `anvs init` might have triggered a different flow. Now it explicitly defaults to quick mode.

- **Backward Compatibility**: All existing flags continue to work as expected:
  - `anvs init --quick` still works (redundant with default, but allowed)
  - `anvs init --non-interactive` still works for CI/scripts
  - `anvs setup` still works (alias for `anvs init`)

- **Mode Selection Logic**:
  1. If `--non-interactive`: bypass wizard entirely (existing behavior)
  2. Else if `--advanced`: use advanced mode
  3. Else if `--quick`: use quick mode (explicit)
  4. Else: use quick mode (default)

- **Help Text Philosophy**: The updated help text emphasizes speed and ease of use ("under 30 seconds", "auto-detection", "quick setup"). This aligns with the milestone goal of optimizing for speed.

- **WizardMode Enum**: The enum makes the code more explicit and easier to maintain. Future modes (e.g., "expert mode", "guided mode") can be added by extending the enum.

- **Testing Priority**: Manual testing is critical for this phase. The user experience depends on correct CLI behavior, so each flag combination must be tested in a real terminal.

- **Estimated Time**:
  - Task 4.1: 20-30 minutes (mode detection refactor)
  - Task 4.2: 20-30 minutes (help text updates)
  - Task 4.3: 30-40 minutes (thorough testing of all scenarios)
  - Total: 1-2 hours

- **Documentation Impact**: Phase 4 changes affect user-facing documentation. Phase 5 will update README.md and other docs to match the new CLI behavior.

- **Log Messages**: The updated `handle_init()` includes log::debug messages for mode selection. This helps with debugging if users report unexpected behavior.

- **No Breaking Changes**: Despite changing the default behavior, there are no breaking changes because:
  1. `anvs init --quick` (old default) still works
  2. `anvs init --non-interactive` (scripts/CI) still works
  3. New default (quick mode) is a superset of old behavior
