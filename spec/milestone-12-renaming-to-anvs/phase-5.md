# Phase 5: Test Files Updates

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 5 updates all test files to use the new `anvs` naming conventions. While Phase 4 updated the Rust source code, the test suite still contains many references to the old `xvn` names in test data, assertions, config paths, and URLs.

**Why Phase 5 is Important:**
- Tests verify that the application works correctly with new naming
- Test data must match production config paths and file names
- Assertions must check for correct binary names and error messages
- Shell integration tests must verify the renamed shell script
- Ensures no regressions in the renamed codebase

**Modules Covered:**
- `tests/*.rs` - All Rust integration and unit tests
- `tests/shell/*.sh` - Shell script integration tests

**⚠️ IMPORTANT**: Phase 5 should only begin after Phase 4 (all sub-phases) is complete.

---

## Implementation Tasks

### Task 5.1: Update Config Tests

**File**: `tests/config_test.rs`

**Changes Required**:

1. **Update config file paths in tests** (~10+ occurrences):
   - Change `.xvnrc` → `.anvsrc` in all test paths
   - Update temporary config file names

**Example changes**:
```rust
// Before:
let config_path = temp.path().join(".xvnrc");

// After:
let config_path = temp.path().join(".anvsrc");
```

**Specific line changes** (approximate):
- Line 8: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`
- Line 25: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`
- Line 38: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`
- Line 95: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`
- Line 125: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`
- Lines 153-174: `.xvnrc1`, `.xvnrc2`, etc. → `.anvsrc1`, `.anvsrc2`, etc.
- Line 209: `let config_path = temp.path().join(".xvnrc");` → `".anvsrc"`

**Commands**:
```bash
# Review current config paths
grep -n "xvnrc" tests/config_test.rs

# After changes, verify
grep -i "xvn" tests/config_test.rs  # Should return no results
grep -i "anvsrc" tests/config_test.rs  # Should show new paths
```

**Actions**:
- [ ] Update all `.xvnrc` references to `.anvsrc`
- [ ] Update numbered config files: `.xvnrc1` → `.anvsrc1`, etc.
- [ ] Verify no hardcoded "xvn" strings remain
- [ ] Run config tests: `cargo test config_test`
- [ ] Verify tests pass

---

### Task 5.2: Update Error Tests

**File**: `tests/error_test.rs`

**Changes Required**:

1. **Update error type name** (~5+ occurrences):
   - Change `XvnError` → `AnvsError`
   - Update type annotations and bindings

**Example changes**:
```rust
// Before:
let xvn_error: XvnError = io_error.into();
let message = xvn_error.to_string();

// After:
let anvs_error: AnvsError = io_error.into();
let message = anvs_error.to_string();
```

**Specific line changes** (approximate):
- Line 68: `let xvn_error: XvnError` → `let anvs_error: AnvsError`
- Line 70: `let message = xvn_error.to_string();` → `anvs_error.to_string();`
- Line 79: `let xvn_error: XvnError` → `let anvs_error: AnvsError`
- Line 81: `let message = xvn_error.to_string();` → `anvs_error.to_string();`
- Line 218: `let xvn_error = XvnError::ConfigError` → `let anvs_error = AnvsError::ConfigError`
- Line 223: `let _anyhow_error: anyhow::Error = xvn_error.into();` → `anvs_error.into();`

**Commands**:
```bash
# Review current error type usage
grep -n "XvnError\|xvn_error" tests/error_test.rs

# After changes, verify
grep -i "xvnerror\|xvn_error" tests/error_test.rs  # Should return no results
grep -i "anvserror\|anvs_error" tests/error_test.rs  # Should show new type
```

**Actions**:
- [ ] Update type annotations: `XvnError` → `AnvsError`
- [ ] Update variable names: `xvn_error` → `anvs_error`
- [ ] Update error variant references if needed
- [ ] Run error tests: `cargo test error_test`
- [ ] Verify tests pass

---

### Task 5.3: Update Installer Tests

**File**: `tests/installer_test.rs`

**Changes Required**:

1. **Update config file comment** (~line 16-17):
   - Change config file header comment
   - Update GitHub URL reference

**Example changes**:
```rust
// Before:
let default_config = r#"# xvn configuration file
# See https://github.com/cameronolivier/xvn for documentation

// After:
let default_config = r#"# anvs configuration file
# See https://github.com/olvrcc/anvs for documentation
```

**Commands**:
```bash
# Review installer test config
grep -n "xvn" tests/installer_test.rs

# After changes, verify
grep -i "xvn" tests/installer_test.rs  # Should return no results
```

**Actions**:
- [ ] Update config file header comment to reference "anvs"
- [ ] Update GitHub URL: `cameronolivier/xvn` → `olvrcc/anvs`
- [ ] Update any other documentation references
- [ ] Run installer tests: `cargo test installer_test`
- [ ] Verify tests pass

---

### Task 5.4: Update Integration Tests

**File**: `tests/integration.rs`

**Changes Required**:

1. **Update file header comment** (~line 1):
   - Change comment to reference anvs

2. **Update config file paths** (~line 254+):
   - Change `.xvnrc` → `.anvsrc` in test setup

**Example changes**:
```rust
// Before:
// Integration tests for xvn

let config_path = temp.path().join(".xvnrc");

// After:
// Integration tests for anvs

let config_path = temp.path().join(".anvsrc");
```

**Commands**:
```bash
# Review integration tests
grep -n "xvn" tests/integration.rs

# After changes, verify
grep -i "xvn" tests/integration.rs  # Should return no results
```

**Actions**:
- [ ] Update file header comment
- [ ] Update all `.xvnrc` references to `.anvsrc`
- [ ] Update any test description strings
- [ ] Run integration tests: `cargo test integration`
- [ ] Verify tests pass

---

### Task 5.5: Update CLI Tests

**File**: `tests/cli_test.rs`

**Changes Required**:

1. **Update CLI assertions**:
   - Check for any hardcoded "xvn" strings in assertions
   - Update expected help text if tested
   - Update binary name references

**Commands**:
```bash
# Review CLI tests
grep -n "xvn\|XVN" tests/cli_test.rs

# After changes, verify
grep -i "xvn" tests/cli_test.rs  # Should return no results
```

**Actions**:
- [ ] Review all test assertions
- [ ] Update any "xvn" string comparisons to "anvs"
- [ ] Update help text expectations if present
- [ ] Run CLI tests: `cargo test cli_test`
- [ ] Verify tests pass

---

### Task 5.6: Update Plugin Tests

**File**: `tests/plugin_test.rs`

**Changes Required**:

1. **Update plugin test documentation and assertions**:
   - Check for any "xvn" references in test descriptions
   - Update any error message assertions

**Commands**:
```bash
# Review plugin tests
grep -n "xvn\|XVN" tests/plugin_test.rs

# After changes, verify
grep -i "xvn" tests/plugin_test.rs  # Should return no results
```

**Actions**:
- [ ] Review test documentation strings
- [ ] Update any "xvn" references in assertions
- [ ] Run plugin tests: `cargo test plugin_test`
- [ ] Verify tests pass

---

### Task 5.7: Update Plugin Loading Tests

**File**: `tests/plugin_loading_test.rs`

**Changes Required**:

1. **Update plugin loading test assertions**:
   - Check for "xvn" in test descriptions
   - Update any plugin path references if they include "xvn"

**Commands**:
```bash
# Review plugin loading tests
grep -n "xvn\|XVN" tests/plugin_loading_test.rs

# After changes, verify
grep -i "xvn" tests/plugin_loading_test.rs  # Should return no results
```

**Actions**:
- [ ] Review test descriptions
- [ ] Update any "xvn" references
- [ ] Run plugin loading tests: `cargo test plugin_loading_test`
- [ ] Verify tests pass

---

### Task 5.8: Update Security Tests

**File**: `tests/security_test.rs`

**Changes Required**:

1. **Update security test documentation**:
   - Check for "xvn" in test descriptions
   - Update any path references or security assertions

**Commands**:
```bash
# Review security tests
grep -n "xvn\|XVN" tests/security_test.rs

# After changes, verify
grep -i "xvn" tests/security_test.rs  # Should return no results
```

**Actions**:
- [ ] Review test documentation
- [ ] Update any "xvn" references
- [ ] Run security tests: `cargo test security_test`
- [ ] Verify tests pass

---

### Task 5.9: Update Shell Integration Tests

**File**: `tests/shell_integration.rs`

**Changes Required**:

1. **Update shell script path references**:
   - Change `xvn.sh` → `anvs.sh`
   - Update shell function name references
   - Update environment variable names in assertions

**Example changes**:
```rust
// Before:
let shell_script = "shell/xvn.sh";
assert!(output.contains("__xvn_activate"));

// After:
let shell_script = "shell/anvs.sh";
assert!(output.contains("__anvs_activate"));
```

**Commands**:
```bash
# Review shell integration tests
grep -n "xvn" tests/shell_integration.rs

# After changes, verify
grep -i "xvn" tests/shell_integration.rs  # Should return no results
```

**Actions**:
- [ ] Update shell script path: `xvn.sh` → `anvs.sh`
- [ ] Update function name references: `__xvn_*` → `__anvs_*`
- [ ] Update env var references: `XVN_*` → `ANVS_*`
- [ ] Run shell integration tests: `cargo test shell_integration`
- [ ] Verify tests pass

---

### Task 5.10: Update Version File Tests

**File**: `tests/version_file_test.rs`

**Changes Required**:

1. **Update test documentation**:
   - Check for "xvn" in comments or test descriptions
   - Likely minimal changes needed

**Commands**:
```bash
# Review version file tests
grep -n "xvn\|XVN" tests/version_file_test.rs

# After changes, verify
grep -i "xvn" tests/version_file_test.rs  # Should return no results
```

**Actions**:
- [ ] Review test documentation
- [ ] Update any "xvn" references if found
- [ ] Run version file tests: `cargo test version_file_test`
- [ ] Verify tests pass

---

### Task 5.11: Verify Shell Script Test

**File**: `tests/shell/test_anvs_sh.sh`

**Status**: Already renamed from `test_xvn_sh.sh` to `test_anvs_sh.sh` ✅

**Changes Required**: **NONE** - File already updated in Phase 4-4

**Verification**:
```bash
# Verify file exists and is updated
ls -la tests/shell/test_anvs_sh.sh

# Check that it tests anvs functions
grep "__anvs_" tests/shell/test_anvs_sh.sh | head -5

# Run shell script test
bash tests/shell/test_anvs_sh.sh
```

**Expected output**:
```
Testing anvs.sh shell integration...
✓ Test 1: Checking function definitions...
✓ All functions defined
✓ Test 2: Checking ANVS_SHELL_LOADED...
✓ ANVS_SHELL_LOADED is set
...
```

**Actions**:
- [ ] Verify `tests/shell/test_anvs_sh.sh` exists
- [ ] Verify old `tests/shell/test_xvn_sh.sh` is deleted
- [ ] Run shell test: `bash tests/shell/test_anvs_sh.sh`
- [ ] Verify test passes

---

### Task 5.12: Comprehensive Test Search

**Search all test files for remaining references**:

```bash
# Search for all case-insensitive "xvn" references in tests
grep -ri "xvn" tests/

# Search for config file references
grep -r "\.xvnrc" tests/

# Search for old error type
grep -r "XvnError" tests/

# All should return no results
```

**Actions**:
- [ ] No "xvn" string literals in test files
- [ ] No `.xvnrc` config references (should be `.anvsrc`)
- [ ] No `XvnError` type references (should be `AnvsError`)
- [ ] No old shell script references (should be `anvs.sh`)
- [ ] Document any intentional exceptions with explanation

---

### Task 5.13: Full Test Suite Run

**Run complete test suite with all changes**:

```bash
# Run all tests
cargo test

# Run with verbose output to see individual test names
cargo test -- --nocapture

# Run specific test categories
cargo test config
cargo test error
cargo test installer
cargo test integration
cargo test cli
cargo test plugin
cargo test shell
cargo test version_file
cargo test security
```

**Expected Results**:
- All tests pass
- No panics or failures
- Test output references "anvs" not "xvn"

**Actions**:
- [ ] All tests pass: `cargo test`
- [ ] No test failures
- [ ] No panics or errors
- [ ] Test output mentions "anvs" appropriately
- [ ] Shell script tests pass

---

### Task 5.14: Test Coverage Verification

**Generate and review test coverage**:

```bash
# Generate coverage report
./scripts/coverage.sh

# Review coverage report
open coverage/index.html

# Verify coverage is still >85%
```

**Actions**:
- [ ] Coverage script runs successfully
- [ ] Coverage report generates
- [ ] Code coverage is still >85%
- [ ] No regressions in coverage

---

### Task 5.15: Integration Testing

**Manual integration testing**:

```bash
# Build release binary
cargo build --release

# Test binary
./target/release/anvs --version  # Should show 2.0.0

# Run init (in a test shell)
./target/release/anvs init --quick

# Test activation
cd /tmp
mkdir -p test-anvs-project
cd test-anvs-project
echo "18.20.0" > .nvmrc

# Source shell integration
source ~/.anvs/bin/anvs.sh

# Test cd activation (if nvm/fnm installed)
cd .
# Should trigger activation
```

**Actions**:
- [ ] Binary builds successfully
- [ ] Init command works
- [ ] Shell integration sources without errors
- [ ] Version file detection works
- [ ] Activation triggers correctly

---

### Task 5.16: Commit Changes

**Stage and commit Phase 5 changes**:

```bash
# Check status
git status

# Review changes
git diff tests/

# Stage changes
git add tests/

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
test: update all test files for anvs rename

- Update config tests: .xvnrc → .anvsrc
- Update error tests: XvnError → AnvsError
- Update installer tests: config comments and URLs
- Update integration tests: file paths and descriptions
- Update CLI tests: binary name references
- Update plugin tests: test descriptions
- Update shell integration tests: script paths and function names
- Update version file tests: documentation
- Verify shell script test already renamed

Files changed:
- tests/config_test.rs - config file paths
- tests/error_test.rs - error type name
- tests/installer_test.rs - config comments
- tests/integration.rs - integration test paths
- tests/cli_test.rs - CLI assertions
- tests/plugin_test.rs - plugin test docs
- tests/plugin_loading_test.rs - loading test docs
- tests/security_test.rs - security test docs
- tests/shell_integration.rs - shell script references
- tests/version_file_test.rs - version file test docs

Part of Phase 5: Test Files Updates
All tests passing
EOF
)"
```

**Actions**:
- [ ] All test changes staged
- [ ] Commit message follows conventional commit format
- [ ] Commit message lists all changed files
- [ ] Changes committed to current branch

---

## Verification Checklist

Before proceeding to Phase 6, verify ALL of the following:

**Test Files Updated:**
- [ ] `tests/config_test.rs` - uses `.anvsrc`
- [ ] `tests/error_test.rs` - uses `AnvsError`
- [ ] `tests/installer_test.rs` - references "anvs"
- [ ] `tests/integration.rs` - references "anvs"
- [ ] `tests/cli_test.rs` - tests "anvs" CLI
- [ ] `tests/plugin_test.rs` - updated
- [ ] `tests/plugin_loading_test.rs` - updated
- [ ] `tests/security_test.rs` - updated
- [ ] `tests/shell_integration.rs` - tests `anvs.sh`
- [ ] `tests/version_file_test.rs` - updated
- [ ] `tests/shell/test_anvs_sh.sh` - exists and works

**Code Quality:**
- [ ] No "xvn" references in test files: `grep -ri "xvn" tests/` returns nothing
- [ ] No `.xvnrc` references: `grep -r "\.xvnrc" tests/` returns nothing
- [ ] No `XvnError` references: `grep -r "XvnError" tests/` returns nothing
- [ ] All tests pass: `cargo test`
- [ ] Shell tests pass: `bash tests/shell/test_anvs_sh.sh`
- [ ] No test failures or panics

**Integration:**
- [ ] Binary builds: `cargo build --release`
- [ ] Binary version correct: `./target/release/anvs --version` shows `2.0.0`
- [ ] Test coverage maintained: `./scripts/coverage.sh` shows >85%

**Git:**
- [ ] Changes committed with proper message
- [ ] Commit follows conventional commit format

---

## Success Criteria

Phase 5 is complete when:

1. ✅ All test files updated to use "anvs" naming
2. ✅ Config file paths changed: `.xvnrc` → `.anvsrc`
3. ✅ Error type changed: `XvnError` → `AnvsError`
4. ✅ Shell script references changed: `xvn.sh` → `anvs.sh`
5. ✅ Function names changed: `__xvn_*` → `__anvs_*`
6. ✅ All tests pass without errors
7. ✅ Shell integration tests pass
8. ✅ Test coverage maintained (>85%)
9. ✅ No "xvn" references remain in test files
10. ✅ Changes committed to git

---

## Next Steps

After completing Phase 5:

1. **Proceed to Phase 6**: Documentation Files
2. **Verify test stability**: Run tests multiple times to ensure consistency
3. **Track progress**: All code changes complete (Phases 1-5)
4. **Prepare for docs**: Next phase updates README, CLAUDE.md, CHANGELOG, etc.

---

## Rollback Plan

If issues are discovered:

1. **Revert git commit**: `git revert <commit-hash>`
2. **Restore test files**: `git checkout HEAD~1 tests/`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify with `cargo test` and manual testing
5. **Re-commit**: Commit fixes before proceeding

---

## Notes

- Test files are critical - they verify the application works correctly
- Be thorough with search and replace to avoid missing references
- Run tests frequently during this phase to catch issues early
- Some tests may need adjustments beyond simple string replacement
- Shell script test was already renamed in Phase 4-4 (verify only)
- Config path tests are especially important - they verify correct file locations
- Error type tests verify the error system works with new naming
- Integration tests provide end-to-end verification
- Total estimated time: 45-60 minutes of focused work
- This phase completes all code updates (source + tests)
