# Milestone 13: Wizard Redesign - Task Checklist

Track progress on redesigning the `anvs init` wizard for speed, clarity, and visual excellence.

---

## Phase 1: Visual Components

### Timeline Module
- [ ] Create `src/init/timeline.rs` module
- [ ] Implement box drawing character constants (◇ ◆ │ ├ └ ┌ ─)
- [ ] Create `Step` struct with states (pending, active, complete)
- [ ] Implement timeline rendering functions
- [ ] Add color support (lime green for brand, cyan for steps)
- [ ] Write unit tests for timeline formatting

### Summary Display
- [ ] Create `src/init/summary.rs` module
- [ ] Implement detection results formatter
- [ ] Create configuration preview function
- [ ] Add "Next Steps" message builder
- [ ] Implement box-style summary layout
- [ ] Test summary display with various configs

### Compact Prompts
- [ ] Refactor `src/init/prompts.rs` for inline detection
- [ ] Create prompt templates with detected values
- [ ] Remove verbose help text (keep concise)
- [ ] Update confirmation prompts for new style
- [ ] Add "Customize settings" option to quick prompts
- [ ] Test prompts with inquire crate updates

---

## Phase 2: Quick Mode Implementation

### Auto-Detection Summary
- [ ] Update `src/init/detection.rs` to collect all detections at once
- [ ] Create summary screen showing all detected values
- [ ] Add single confirmation prompt (Yes/Customize/Cancel)
- [ ] Implement "Proceed with defaults" flow
- [ ] Add transition to advanced mode on "Customize"
- [ ] Test detection with various environments

### Progress Indicators
- [ ] Add installation progress timeline
- [ ] Show steps: Create config, Install hook, Validate, Test
- [ ] Use ✓ symbols for completed steps
- [ ] Update status messages during installation
- [ ] Add error handling with clear visual feedback
- [ ] Test progress display timing

### Completion Messages
- [ ] Design clean completion screen
- [ ] Show "Setup complete!" with checkmark
- [ ] Display concise "Next steps" (3 items max)
- [ ] Include example usage hint
- [ ] Add timing info (optional: "Completed in 3s")
- [ ] Test completion screen formatting

---

## Phase 3: Advanced Mode Refinement

### Step-by-Step Flow
- [ ] Reduce wizard to exactly 3 steps maximum
- [ ] Step 1: Shell configuration (with detected value)
- [ ] Step 2: Version manager selection (with detected value)
- [ ] Step 3: Auto-install behavior
- [ ] Add step counter display (e.g., "Step 1 of 3")
- [ ] Test navigation between steps

### Inline Detection Display
- [ ] Update shell prompt to show detected shell
- [ ] Update version manager prompt with detection
- [ ] Add "(detected, recommended)" labels
- [ ] Ensure detected values are pre-selected
- [ ] Allow easy override of detected values
- [ ] Test with various detection scenarios

### Configuration Summary
- [ ] Create final summary before applying
- [ ] Show all selected options in box layout
- [ ] Add "Apply configuration?" confirmation
- [ ] Allow going back to edit if needed
- [ ] Test summary accuracy
- [ ] Verify configuration is applied correctly

---

## Phase 4: CLI Integration

### Command Updates
- [ ] Update `src/cli.rs` - add `--advanced` flag
- [ ] Make quick mode the new default for `anvs init`
- [ ] Keep `--quick` flag for explicit quick mode
- [ ] Implement `--advanced` flag for advanced mode
- [ ] Update `--non-interactive` to use new quick mode
- [ ] Test all flag combinations

### Help Text Updates
- [ ] Update `anvs init --help` text
- [ ] Document quick mode (default)
- [ ] Document `--advanced` flag
- [ ] Update examples in help
- [ ] Ensure help is concise but clear
- [ ] Test help output formatting

### Wizard Mode Enum
- [ ] Create `WizardMode` enum (Quick, Advanced)
- [ ] Implement mode detection from flags
- [ ] Route to appropriate wizard function
- [ ] Add mode logging for debugging
- [ ] Test mode switching logic

---

## Phase 5: Polish & Testing

### Visual Refinement
- [ ] Ensure consistent spacing throughout
- [ ] Align all box drawing characters properly
- [ ] Test on 80-column terminal width
- [ ] Verify colors on dark theme
- [ ] Verify colors on light theme
- [ ] Fix any alignment issues

### User Experience
- [ ] Time the quick mode flow (should be < 30 seconds)
- [ ] Ensure advanced mode is clear and intuitive
- [ ] Test with first-time users (if possible)
- [ ] Verify error messages are helpful
- [ ] Ensure cancellation works properly
- [ ] Test keyboard navigation

### Testing
- [ ] Update integration tests for new wizard flow
- [ ] Add tests for quick mode
- [ ] Add tests for advanced mode
- [ ] Test auto-detection logic
- [ ] Test configuration application
- [ ] Ensure all existing tests still pass

### Documentation
- [ ] Update README.md with new init flow
- [ ] Add screenshots/examples of new wizard
- [ ] Update CLAUDE.md with wizard references
- [ ] Document quick vs advanced mode differences
- [ ] Add troubleshooting section if needed
- [ ] Update migration guide if applicable

---

## Phase 6: Final Review & Release

### Code Review
- [ ] Review all new code for clarity
- [ ] Ensure consistent error handling
- [ ] Verify logging is appropriate
- [ ] Check for unused code/imports
- [ ] Run clippy with strict warnings
- [ ] Run cargo fmt

### Performance
- [ ] Profile wizard startup time
- [ ] Ensure quick mode completes in < 30 seconds
- [ ] Optimize any slow detection logic
- [ ] Verify binary size hasn't grown significantly
- [ ] Test on slower systems if possible

### Quality Assurance
- [ ] Test on macOS (both Intel and ARM)
- [ ] Test on Linux (if applicable)
- [ ] Test with bash shell
- [ ] Test with zsh shell
- [ ] Test with nvm installed
- [ ] Test with fnm installed
- [ ] Test with no version manager (error case)
- [ ] Test with existing config (re-init)

### Release Preparation
- [ ] Update version to v2.1.0
- [ ] Update CHANGELOG.md with wizard improvements
- [ ] Create release notes highlighting new UX
- [ ] Tag release: `git tag v2.1.0`
- [ ] Build and test release binaries
- [ ] Publish to npm
- [ ] Update Homebrew formula
- [ ] Announce the update

---

## Post-Release

### Monitoring
- [ ] Watch for GitHub issues about new wizard
- [ ] Collect user feedback
- [ ] Track completion times (if metrics added)
- [ ] Identify common pain points

### Iteration
- [ ] Address any critical bugs
- [ ] Consider minor UX improvements
- [ ] Plan for future enhancements (from SPEC.md)

---

## Success Criteria

Milestone 13 is complete when:

- ✅ Quick mode is default and completes in < 30 seconds
- ✅ Visual design uses timeline/progress indicators
- ✅ Prompts are concise and well-formatted
- ✅ Detection results shown inline with prompts
- ✅ Advanced mode available via `--advanced` flag
- ✅ Installation progress shows clear feedback
- ✅ Completion message is helpful and concise
- ✅ All tests pass
- ✅ Documentation updated
- ✅ Wizard feels as polished as Vite/ShadCN CLIs
- ✅ Published as v2.1.0

---

## Notes

- Focus on visual polish - this is about UX, not new functionality
- Test frequently during development
- Keep backward compatibility with existing flags
- Maintain the quality bar set by the Vite/ShadCN examples
- Wizard should feel fast, clean, and effortless
