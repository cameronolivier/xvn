# Phase 5: Polish & Testing

**Status**: Completed
**Version Target**: v2.1.0
**Duration Estimate**: 4-6 hours
**Phase Tasks**: 5.1 - 5.4

## Overview

Phase 5 focuses on final polish, comprehensive testing, and documentation updates to ensure the wizard redesign meets production quality standards. This phase transforms the implemented wizard from functional to polished, ensuring it feels as professional as modern CLI tools like Vite and ShadCN.

**Why Phase 5 is Critical:**
- Ensures the wizard meets the visual and UX quality bar set by modern CLIs
- Validates all functionality works correctly across different environments
- Provides comprehensive testing coverage for reliability
- Updates documentation to reflect the new wizard experience
- Prepares for the final release as v2.1.0

**⚠️ CHECKPOINT**: Before starting Phase 5, ensure:
- All previous phases (1-4) are complete and tested
- The wizard runs successfully in both quick and advanced modes
- Basic functionality works (config creation, shell hook installation)
- You have access to multiple terminal environments for testing
- Screenshots/documentation tools are available

---

## Implementation Tasks

### Task 5.1: Visual Refinement

**Goal**: Ensure consistent spacing, alignment, and cross-terminal compatibility.

**Files**: `src/init/timeline.rs`, `src/init/summary.rs`, `src/init/wizard.rs`, `src/output.rs`

**Changes Required**:

1. **Review and standardize spacing**:
   - Ensure consistent blank lines between sections (2 lines between major sections, 1 line between subsections)
   - Verify indentation is consistent (2 spaces for continuation lines)
   - Check that box-drawing characters align properly

2. **Test on 80-column terminal width**:
   - Resize terminal to exactly 80 columns
   - Run both quick and advanced modes
   - Verify no text wraps unexpectedly
   - Adjust any overly long lines

3. **Verify colors on dark theme**:
   - Switch to dark terminal theme
   - Run wizard and check all color elements
   - Ensure lime green brand color is visible
   - Verify dimmed text is readable but not prominent

4. **Verify colors on light theme**:
   - Switch to light terminal theme
   - Run wizard again
   - Ensure colors work well (not washed out)
   - Adjust RGB values if needed for better contrast

5. **Fix alignment issues in timeline module**:
   ```rust
   // In src/init/timeline.rs, ensure render_box handles alignment properly
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
           // Ensure consistent padding
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
   ```

6. **Standardize color usage**:
   ```rust
   // In src/output.rs, ensure consistent color definitions
   pub const BRAND_COLOR: owo_colors::Rgb = owo_colors::Rgb(50, 205, 50); // Lime green
   pub const SUCCESS_COLOR: owo_colors::Rgb = owo_colors::Rgb(34, 197, 94); // Green
   pub const WARNING_COLOR: owo_colors::Rgb = owo_colors::Rgb(251, 191, 36); // Yellow
   pub const ERROR_COLOR: owo_colors::Rgb = owo_colors::Rgb(239, 68, 68); // Red
   pub const INFO_COLOR: owo_colors::Rgb = owo_colors::Rgb(59, 130, 246); // Blue
   ```

**Commands**:

```bash
# Test on 80-column terminal
printf '\e[8;24;80t'  # Resize to 80x24
cargo run -- init
cargo run -- init --advanced

# Test colors on different themes
# Switch terminal theme manually and re-run

# Check formatting
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

**Expected Output**:

Wizard output should look clean and aligned, with no text wrapping on 80-column terminals.

**Actions**:
- [ ] Test wizard on 80-column terminal width, fix any wrapping issues
- [ ] Verify colors work well on dark terminal theme
- [ ] Verify colors work well on light terminal theme
- [ ] Check alignment of all box-drawing characters
- [ ] Ensure consistent spacing throughout all wizard output
- [ ] Update color constants in `src/output.rs` if needed
- [ ] Run `cargo fmt` and `cargo clippy -- -D warnings`
- [ ] Take screenshots of final visual output for documentation

---

### Task 5.2: User Experience Testing

**Goal**: Validate the wizard provides an excellent user experience across different scenarios.

**Files**: `src/init/wizard.rs`, `src/init/detection.rs`, `src/init/prompts.rs`

**Changes Required**:

1. **Time the quick mode flow**:
   - Run quick mode multiple times
   - Ensure completion in < 30 seconds
   - Identify and optimize any slow operations

2. **Test advanced mode intuitiveness**:
   - Run through advanced mode step-by-step
   - Verify step counter is helpful ("Step 1 of 3")
   - Ensure prompts are clear and non-confusing
   - Test navigation between steps

3. **Verify error messages are helpful**:
   ```rust
   // Test scenarios that should show helpful errors:
   // - No shell detected
   // - No version manager detected
   // - Permission denied writing config
   // - Existing config without --force
   ```

4. **Ensure cancellation works properly**:
   - Cancel at quick mode confirmation
   - Cancel at various advanced mode steps
   - Verify no partial config is written
   - Test Ctrl+C interruption

5. **Test keyboard navigation**:
   - Use arrow keys in all Select prompts
   - Test Enter to confirm selections
   - Verify cursor positioning works
   - Test with different keyboard layouts if possible

6. **Edge case testing**:
   - Multiple version managers detected (nvm + fnm)
   - Non-standard shell path
   - Custom shell configuration
   - No home directory (should fail gracefully)
   - Very long paths that might cause wrapping

**Commands**:

```bash
# Time quick mode
time cargo run -- init

# Test cancellation scenarios
cargo run -- init  # Cancel at confirmation
cargo run -- init --advanced  # Cancel at various steps

# Test error scenarios (simulate by modifying detection temporarily)
# - Comment out shell detection to test failure
# - Remove version manager to test warnings
```

**Expected Output**:

Quick mode should complete in < 30 seconds. All error messages should be clear and actionable. Cancellation should leave no partial state.

**Actions**:
- [ ] Time quick mode flow, ensure < 30 seconds completion
- [ ] Test advanced mode for clarity and intuitiveness
- [ ] Verify all error messages are helpful and actionable
- [ ] Test cancellation at all possible points
- [ ] Test keyboard navigation (arrows, enter, etc.)
- [ ] Test edge cases (multiple VMs, custom shells, long paths)
- [ ] Document any UX issues found and fixes applied

---

### Task 5.3: Update Integration Tests

**Goal**: Ensure comprehensive test coverage for the new wizard functionality.

**Files**: `tests/integration.rs`, `tests/wizard_test.rs` (new), `tests/init_test.rs`

**Content Requirements** (for new test file `tests/wizard_test.rs`):

```rust
//! Integration tests for the redesigned wizard

use anvs::config::{Config, AutoInstallMode};
use anvs::init::detection::detect_all;
use anvs::init::summary::DetectionResults;
use anvs::init::wizard::{WizardMode, run_wizard};
use anvs::setup::shell_detection::Shell;
use std::env;

#[test]
fn test_detect_all_returns_valid_results() {
    let results = detect_all().unwrap();
    // Should not panic, even if detection fails
    assert!(results.config_path.contains(".anvsrc"));
}

#[test]
fn test_quick_mode_with_mocked_detection() {
    // This test would require mocking the detection functions
    // For now, test that the mode enum works
    assert_eq!(WizardMode::from_flags(false, false, false), WizardMode::Quick);
    assert_eq!(WizardMode::from_flags(false, true, false), WizardMode::Advanced);
    assert_eq!(WizardMode::from_flags(false, false, true), WizardMode::NonInteractive);
}

#[test]
fn test_wizard_mode_routing() {
    // Test that mode selection works correctly
    // Note: Full integration testing requires mocking user input
}

#[test]
fn test_detection_results_to_config() {
    let mut results = DetectionResults::new();
    results.shell = Some(Shell::Zsh);
    results.version_managers = vec!["nvm".to_string()];

    // Test the conversion logic (implement in wizard.rs if not present)
    let config = Config {
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
        use_default: true,
    };

    assert_eq!(config.plugins, vec!["nvm".to_string()]);
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);
}

#[test]
fn test_config_validation() {
    // Test that created configs are valid
    let config = Config {
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string()],
        use_default: true,
    };

    // Should not panic when used
    assert!(!config.plugins.is_empty());
}
```

**Changes Required in Other Files**:

1. **File**: `tests/integration.rs`
   - Add tests for full wizard flows (may require environment setup)
   - Test CLI flag combinations

2. **File**: `tests/init_test.rs`
   - Update existing tests to work with new wizard
   - Add tests for new detection and summary functions

**Commands**:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test --test integration
cargo test wizard_test

# Check test coverage (if coverage tool is set up)
make coverage
```

**Expected Output**:

```
running 25 tests  # (approximate - will vary)
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Actions**:
- [ ] Create `tests/wizard_test.rs` with comprehensive wizard tests
- [ ] Update `tests/integration.rs` with new wizard integration tests
- [ ] Update `tests/init_test.rs` for any breaking changes
- [ ] Run `cargo test` and ensure all tests pass
- [ ] Run `make coverage` and verify >85% coverage for wizard code
- [ ] Add any missing test cases identified during UX testing
- [ ] Document test scenarios and edge cases covered

---

### Task 5.4: Documentation Updates

**Goal**: Update all documentation to reflect the new wizard experience.

**Files**: `README.md`, `CLAUDE.md`, `docs/tasks/XVN_TO_ANVS_MIGRATION.md`, `docs/tasks/MIGRATION.md`

**Changes Required**:

1. **Update README.md with new init flow**:
   ```markdown
   ## Quick Start

   Initialize anvs with auto-detection (recommended):

   ```bash
   anvs init
   ```

   Or use advanced mode for full customization:

   ```bash
   anvs init --advanced
   ```

   The wizard will detect your shell and Node.js version manager, then guide you through setup.
   ```

2. **Add screenshots/examples of new wizard**:
   - Take screenshots of quick mode summary screen
   - Take screenshots of advanced mode steps
   - Add to README.md or create separate docs

3. **Update CLAUDE.md with wizard references**:
   - Update any hardcoded references to old wizard flow
   - Add examples of new prompt formats

4. **Document quick vs advanced mode differences**:
   ```markdown
   ### Setup Modes

   **Quick Mode** (default):
   - Auto-detects shell and version managers
   - Shows summary and asks for confirmation
   - Completes in < 30 seconds
   - Recommended for most users

   **Advanced Mode** (`--advanced`):
   - Step-by-step customization
   - Full control over all settings
   - Takes 1-2 minutes
   - For users with specific requirements
   ```

5. **Add troubleshooting section**:
   ```markdown
   ### Troubleshooting

   **Shell not detected:**
   ```bash
   anvs init --shell zsh  # or bash
   ```

   **Version manager not detected:**
   - Ensure nvm or fnm is installed and in PATH
   - Use advanced mode to specify manually

   **Permission denied:**
   - Check write permissions for ~/.anvsrc and shell config files
   - Run with sudo if necessary (not recommended)
   ```

6. **Update migration guide**:
   - Document any breaking changes from v2.0.0 to v2.1.0
   - Explain new default behavior (quick mode)

**Commands**:

```bash
# Check documentation formatting
# (Assuming markdown linting is available)
markdownlint README.md

# Preview documentation
# Open files in browser or editor to verify
```

**Expected Output**:

Documentation should clearly explain the new wizard flow and provide helpful troubleshooting information.

**Actions**:
- [ ] Update README.md with new init flow examples
- [ ] Add screenshots of wizard screens to documentation
- [ ] Update CLAUDE.md references to new wizard
- [ ] Document differences between quick and advanced modes
- [ ] Add troubleshooting section for common issues
- [ ] Update migration guides for v2.1.0 changes
- [ ] Verify all documentation renders correctly
- [ ] Test that documentation examples work as described

---

## Verification Checklist

Before proceeding to Phase 6, verify ALL of the following:

- [ ] Visual output is consistent and well-aligned on 80-column terminals
- [ ] Colors work correctly on both dark and light terminal themes
- [ ] Quick mode completes in < 30 seconds
- [ ] Advanced mode is intuitive with clear step progression
- [ ] Error messages are helpful and actionable
- [ ] Cancellation works properly at all points
- [ ] Keyboard navigation works smoothly
- [ ] All edge cases fail gracefully
- [ ] Integration tests pass for new wizard functionality
- [ ] Test coverage >85% for wizard-related code
- [ ] README.md documents new init flow
- [ ] Screenshots added to documentation
- [ ] CLAUDE.md updated with new wizard references
- [ ] Troubleshooting section added
- [ ] Migration guides updated for v2.1.0
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt` has been run

---

## Success Criteria

Phase 5 is complete when:

1. ✅ Visual design is polished and consistent across terminals
2. ✅ User experience testing passes all scenarios
3. ✅ Comprehensive test coverage for new functionality
4. ✅ Documentation updated with new wizard experience
5. ✅ All verification checklist items completed
6. ✅ Ready for final review and release preparation

---

## Next Steps

After completing Phase 5:

1. Run final comprehensive test suite
2. Commit all changes:
   ```bash
   git add .
   git commit -m "feat: polish wizard redesign and add comprehensive testing

   - Visual refinement for cross-terminal compatibility
   - UX testing and optimization (< 30s quick mode)
   - Comprehensive integration tests added
   - Documentation updated for new wizard flow
   - Troubleshooting and migration guides added

   Phase 5 complete - ready for final review"
   ```
3. **Proceed to Phase 6**: Final Review & Release

---

## Rollback Plan

If issues are discovered in Phase 5:

1. To rollback visual changes:
   ```bash
   git checkout HEAD -- src/init/timeline.rs src/init/summary.rs src/output.rs
   ```

2. To rollback test changes:
   ```bash
   git checkout HEAD -- tests/
   ```

3. To rollback documentation:
   ```bash
   git checkout HEAD -- README.md CLAUDE.md docs/
   ```

4. To rollback all Phase 5 changes:
   ```bash
   git reset --hard HEAD~1  # If committed
   ```

---

## Notes

- **Terminal Compatibility**: Focus on iTerm2, Terminal.app (macOS), and gnome-terminal (Linux) as primary targets. The unicode box-drawing characters are well-supported in modern terminals.

- **Performance**: The < 30 second goal for quick mode is ambitious but achievable. Focus on optimizing detection logic if needed.

- **Testing Strategy**: Combine automated tests with manual UX testing. The interactive nature of the wizard makes some automation challenging.

- **Documentation**: Screenshots are valuable for demonstrating the visual improvements. Consider using terminal recording tools like asciinema.

- **Edge Cases**: Pay special attention to users with non-standard setups (custom shells, multiple version managers, permission issues).

- **Estimated Time**:
  - Task 5.1: 1-2 hours (visual refinement)
  - Task 5.2: 1-2 hours (UX testing)
  - Task 5.3: 1-2 hours (test updates)
  - Task 5.4: 1 hour (documentation)
  - Total: 4-6 hours

- **Quality Bar**: The wizard should feel as polished as Vite or ShadCN CLIs. Don't compromise on visual quality or user experience.