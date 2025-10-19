# Phase 6: Final Review & Release

**Status**: Not Started | **Version**: v2.1.0 | **Duration**: 2-3 hours + CI time

## Overview
Phase 6 completes Milestone 13 by conducting final code review, performance validation, comprehensive testing across platforms, and preparing the release. This phase ensures the wizard redesign meets quality standards and is ready for production deployment.

**Why Phase 6 is Critical:**
- Ensures code quality and consistency before release
- Validates performance improvements (quick mode < 30 seconds)
- Confirms cross-platform compatibility
- Prepares all release artifacts and documentation
- Provides final quality gate before v2.1.0 release

**⚠️ CHECKPOINT**: Before starting Phase 6, verify:
- All previous phases (1-5) are complete and tested
- `cargo test` passes all tests
- `cargo check` compiles without errors
- Basic wizard functionality works (quick and advanced modes)

---

## Implementation Tasks

### Task 6.1: Code Review and Quality Checks

**File**: Multiple files (review all wizard-related code)

**Content Requirements** (for any fixes needed):
```rust
// Example: Add missing documentation
//! Final review and quality assurance for wizard redesign
```

**Changes Required** (for EXISTING files only):
- Review `src/init/timeline.rs`: Ensure all functions are well-documented, error handling consistent
- Review `src/init/summary.rs`: Check for unused imports, consistent formatting
- Review `src/init/prompts.rs`: Verify all prompts handle edge cases gracefully
- Review `src/init/wizard.rs`: Ensure logging is appropriate, no debug prints left
- Review `src/cli.rs`: Confirm help text is accurate and concise
- Check all new files for consistent error handling patterns

**Commands**:
```bash
# Run comprehensive quality checks
make check

# Run clippy with strict warnings
cargo clippy -- -D warnings

# Format all code
cargo fmt --check

# Check for unused dependencies
cargo +nightly udeps

# Review documentation
cargo doc --open
```

**Expected Output**:
```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Finished dev [unoptimized + debuginfo] target(s) in 1.23s
```

**Actions**:
- [ ] Review all wizard code for clarity and consistency
- [ ] Ensure error messages are user-friendly and actionable
- [ ] Verify logging uses appropriate levels (info, warn, error)
- [ ] Check for any TODO comments or debug code left behind
- [ ] Remove any unused imports or dead code
- [ ] Run `make check` and ensure it passes
- [ ] Run `cargo clippy -- -D warnings` and fix any issues
- [ ] Run `cargo fmt --check` and format if needed
- [ ] Review generated documentation for completeness
- [ ] Commit any fixes: `fix: code review improvements for wizard redesign`

---

### Task 6.2: Performance Validation

**File**: `src/init/wizard.rs`, `src/init/detection.rs`

**Content Requirements** (for optimizations if needed):
```rust
// Example: Add timing measurement
use std::time::Instant;

let start = Instant::now();
// ... wizard logic ...
let duration = start.elapsed();
log::info!("Wizard completed in {:.2}s", duration.as_secs_f64());
```

**Changes Required** (for EXISTING files only):
- Add timing measurements to wizard functions if not present
- Optimize any slow detection logic found during profiling
- Ensure quick mode consistently completes in < 30 seconds

**Commands**:
```bash
# Profile wizard startup time
time cargo run -- init --non-interactive

# Check binary size
ls -lh target/release/anvs

# Run with timing in debug mode
RUST_LOG=info cargo run -- init --non-interactive

# Test on slower system simulation (if available)
# Or manually time on target hardware
```

**Expected Output**:
```
real    0m12.345s
user    0m8.901s
sys     0m1.234s
```

**Actions**:
- [ ] Time quick mode execution: should be < 30 seconds
- [ ] Time advanced mode execution: should be < 60 seconds
- [ ] Profile detection logic for bottlenecks
- [ ] Check binary size hasn't grown excessively (> 10MB is concerning)
- [ ] Test on slower hardware if available
- [ ] Add timing logs if helpful for debugging
- [ ] Optimize any slow code paths identified
- [ ] Verify performance meets requirements
- [ ] Commit optimizations: `perf: optimize wizard performance for < 30s quick mode`

---

### Task 6.3: Cross-Platform Quality Assurance

**File**: Various (test on different systems)

**Content Requirements** (for platform-specific fixes if needed):
```bash
# Example: Platform-specific shell detection
#[cfg(target_os = "macos")]
fn detect_shell_macos() -> Result<Shell> {
    // macOS-specific logic
}
```

**Changes Required** (for EXISTING files only):
- Fix any platform-specific issues discovered during testing
- Ensure shell detection works on both Intel and ARM Macs
- Verify Linux compatibility if applicable
- Test with different shell configurations

**Commands**:
```bash
# Test on macOS Intel (if available)
# Test on macOS ARM (M1/M2)
# Test on Linux (if available)

# Test different shell scenarios
export SHELL=/bin/bash && cargo run -- init --non-interactive
export SHELL=/bin/zsh && cargo run -- init --non-interactive

# Test with nvm installed
which nvm && cargo run -- init --non-interactive

# Test with fnm installed
which fnm && cargo run -- init --non-interactive

# Test error case: no version manager
# (uninstall nvm/fnm temporarily)
cargo run -- init --non-interactive

# Test re-initialization
cargo run -- init --force --non-interactive
```

**Expected Output**:
```
✓ Setup complete!
Completed in 8.45s
```

**Actions**:
- [ ] Test on macOS Intel architecture
- [ ] Test on macOS ARM architecture (M1/M2)
- [ ] Test on Linux if available
- [ ] Test with bash shell environment
- [ ] Test with zsh shell environment
- [ ] Test with nvm installed and detected
- [ ] Test with fnm installed and detected
- [ ] Test error case: no version manager detected
- [ ] Test re-initialization with existing config
- [ ] Document any platform-specific issues found
- [ ] Fix any compatibility problems discovered
- [ ] Commit fixes: `fix: cross-platform compatibility improvements`

---

### Task 6.4: Release Preparation

**File**: `Cargo.toml`, `CHANGELOG.md`, `package.json`, `homebrew/anvs.rb`

**Content Requirements** (for version updates):
```toml
# Cargo.toml
[package]
name = "anvs"
version = "2.1.0"
```

```json
// package.json
{
  "name": "@olvrcc/anvs",
  "version": "2.1.0"
}
```

**Changes Required** (for EXISTING files only):
- Update version to 2.1.0 in Cargo.toml
- Update version in package.json
- Add changelog entry for wizard improvements
- Update Homebrew formula version
- Create release notes highlighting UX improvements

**Commands**:
```bash
# Update version numbers
sed -i '' 's/version = "2.0.0"/version = "2.1.0"/' Cargo.toml
sed -i '' 's/"version": "2.0.0"/"version": "2.1.0"/' package.json

# Build release binaries
cargo build --release

# Test release binary
./target/release/anvs --version

# Create git tag
git tag v2.1.0

# Build npm package
npm run build

# Test npm package
npm pack --dry-run
```

**Expected Output**:
```
anvs 2.1.0
```

**Actions**:
- [ ] Update version to 2.1.0 in Cargo.toml
- [ ] Update version in package.json
- [ ] Update CHANGELOG.md with wizard redesign features
- [ ] Create detailed release notes
- [ ] Build and test release binaries
- [ ] Update Homebrew formula with new version
- [ ] Tag release: `git tag v2.1.0`
- [ ] Test npm package build
- [ ] Prepare release announcement
- [ ] Commit version updates: `release: bump version to v2.1.0`

---

### Task 6.5: Final Integration Testing

**File**: `tests/` (run all tests)

**Content Requirements** (for new tests if needed):
```rust
#[test]
fn test_wizard_quick_mode_performance() {
    // Test that quick mode completes within time limits
}
```

**Changes Required** (for EXISTING files only):
- Ensure all existing tests pass
- Add performance regression tests if needed
- Verify integration tests cover new wizard flows

**Commands**:
```bash
# Run full test suite
make test

# Run integration tests specifically
cargo test --test integration

# Run with coverage
make coverage

# Test wizard flows
cargo test wizard
```

**Expected Output**:
```
running 42 tests
test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Actions**:
- [ ] Run `make test` and ensure all tests pass
- [ ] Run integration tests covering wizard flows
- [ ] Generate coverage report and verify >85% coverage
- [ ] Test all wizard modes (quick, advanced, non-interactive)
- [ ] Verify no regressions in existing functionality
- [ ] Add any missing tests for edge cases
- [ ] Ensure CI will pass with these changes
- [ ] Commit test improvements: `test: add final integration tests for wizard`

---

## Verification Checklist

Before releasing v2.1.0, verify:
- [ ] All code review issues resolved
- [ ] Performance requirements met (< 30s quick mode)
- [ ] Cross-platform testing completed
- [ ] All tests pass (unit + integration)
- [ ] Release binaries build successfully
- [ ] Version numbers updated consistently
- [ ] Changelog and release notes prepared
- [ ] Git tag created (v2.1.0)
- [ ] Homebrew formula updated
- [ ] NPM package builds correctly

---

## Success Criteria

1. ✅ Code passes all quality checks (clippy, fmt, udeps)
2. ✅ Quick mode consistently completes in < 30 seconds
3. ✅ Cross-platform compatibility verified (macOS Intel/ARM, Linux)
4. ✅ All tests pass with good coverage
5. ✅ Release artifacts prepared (binaries, npm, homebrew)
6. ✅ Version bumped to v2.1.0 with proper changelog
7. ✅ Git tag created and ready for push

---

## Next Steps

1. Push release branch and create PR for final review
2. After approval, merge and push git tag
3. Publish to npm registry
4. Update Homebrew tap
5. Announce release on relevant channels
6. **Milestone 13 Complete!** 🎉

---

## Rollback Plan

1. If critical issues found post-release:
   ```bash
   # Delete git tag
   git tag -d v2.1.0
   git push origin :refs/tags/v2.1.0

   # Revert version bump
   git revert HEAD~1
   ```
2. For npm rollback:
   ```bash
   npm unpublish @olvrcc/anvs@2.1.0
   ```
3. For Homebrew rollback: Update formula to previous version

---

## Notes

- **Performance Baseline**: Quick mode should complete in 8-15 seconds on modern hardware
- **Platform Priority**: macOS is primary platform, Linux secondary
- **Testing Coverage**: Focus on happy path + key error scenarios
- **Release Process**: Follow existing release workflow in CONTRIBUTING.md
- **Communication**: Highlight UX improvements in release notes
- **Monitoring**: Watch for issues post-release, especially around wizard flows

---

## Estimated Time Breakdown

- Task 6.1: Code Review (45 min)
- Task 6.2: Performance (30 min)
- Task 6.3: QA Testing (60 min)
- Task 6.4: Release Prep (30 min)
- Task 6.5: Integration Tests (30 min)
- **Total: 3-4 hours**