# Phase 4-4: Rust Source Code - CLI, Plugins & Remaining Modules

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 4-4 is the **final** part of the Rust source code update, covering the **CLI**, **Plugins**, **Version File**, and remaining utility modules. This phase completes all Rust source code changes.

**Why Phase 4-4 is Critical:**
- CLI module defines the main command-line interface and help text
- This is what users see when they run `anvs --help`
- Plugins and version_file modules need documentation updates
- Completing this phase means all Rust code is renamed

**Modules Covered:**
- `src/cli.rs` - Command-line interface definition
- `src/plugins/` - Version manager plugins
- `src/version_file/` - Version file detection
- `src/error.rs`, `src/output.rs`, `src/main.rs` - Utility modules

**⚠️ IMPORTANT**: Phase 4-4 should only begin after Phase 4-3 is complete.

---

## CLI Module Updates

### Task 4-4.1: Update CLI Module

**File**: `src/cli.rs`

**Changes Required**:

1. **Update CLI application name** (~line 20):
   - Change `name("xvn")` to `name("anvs")`
   - Change application description

2. **Update about text and description**:
   - Change from "Extreme Version Switcher" to "Automatic Node Version Switcher"
   - Update "XVN" → "ANVS"

3. **Update command descriptions and help text**:
   - Update `activate` command description
   - Update `setup` command description and long help
   - Update `status` command description
   - Update `set` command description
   - Update `uninstall` command description

4. **Update examples in help text**:
   - Change all examples from `xvn <cmd>` to `anvs <cmd>`

5. **Update version string formatting**:
   - Verify version display is correct

**Example changes**:
```rust
// Before:
#[derive(Parser)]
#[command(
    name = "xvn",
    about = "XVN - Extreme Version Switcher for Node.js",
    long_about = "Automatic Node.js version switching based on project configuration",
    version
)]
pub struct Cli {
    // ...
}

#[derive(Subcommand)]
pub enum Commands {
    /// Setup xvn on your system
    #[command(about = "Configure shell integration for xvn")]
    Setup,

    /// Activate Node.js version for current directory
    #[command(about = "Manually activate version for a directory")]
    Activate { path: Option<PathBuf> },
    // ...
}

// After:
#[derive(Parser)]
#[command(
    name = "anvs",
    about = "ANVS - Automatic Node Version Switcher for Node.js",
    long_about = "Automatic Node.js version switching based on project configuration",
    version
)]
pub struct Cli {
    // ...
}

#[derive(Subcommand)]
pub enum Commands {
    /// Setup anvs on your system
    #[command(about = "Configure shell integration for anvs")]
    Setup,

    /// Activate Node.js version for current directory
    #[command(about = "Manually activate version for a directory")]
    Activate { path: Option<PathBuf> },
    // ...
}
```

**Commands**:
```bash
# Review CLI definitions
grep -n "xvn\|XVN" src/cli.rs

# After changes
grep -i "xvn" src/cli.rs  # Should return no results

# Test CLI help output (after compilation)
cargo build && ./target/debug/anvs --help
cargo build && ./target/debug/anvs setup --help
```

**Expected help output**:
```
ANVS - Automatic Node Version Switcher for Node.js

Automatic Node.js version switching based on project configuration

Usage: anvs <COMMAND>

Commands:
  activate    Manually activate version for a directory
  setup       Configure shell integration for anvs
  status      Show current configuration and status
  set         Interactively configure anvs settings
  uninstall   Remove anvs from your system
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**Actions**:
- [ ] Update CLI `name` attribute to `"anvs"`
- [ ] Update `about` text: "ANVS - Automatic Node Version Switcher"
- [ ] Update `long_about` text if present
- [ ] Update each command's description and help text:
  - [ ] `Setup` command: "Configure shell integration for anvs"
  - [ ] `Activate` command (verify description)
  - [ ] `Status` command (verify description)
  - [ ] `Set` command: "Interactively configure anvs settings"
  - [ ] `Uninstall` command: "Remove anvs from your system"
- [ ] Update all example commands in long help text
- [ ] Update any inline comments
- [ ] Update doc comments
- [ ] Build and test help output: `cargo build && ./target/debug/anvs --help`
- [ ] Verify each subcommand help: `./target/debug/anvs <cmd> --help`

---

## Plugins Module Updates

### Task 4-4.2: Update Plugins Module

**Files**: `src/plugins/` (all files)

**Files to check**:
- `src/plugins/mod.rs`
- `src/plugins/trait_def.rs`
- `src/plugins/registry.rs`
- `src/plugins/nvm.rs`
- `src/plugins/fnm.rs`
- `src/plugins/mock.rs` (test mock, if exists)

**Commands**:
```bash
# Search all plugin files
grep -rn "xvn\|XVN" src/plugins/

# After changes
grep -ri "xvn" src/plugins/  # Should return no results
```

**Changes Needed**:
1. **Update documentation comments** referencing "xvn"
2. **Update log/debug messages** if they reference the tool name
3. **Update error messages** if applicable

**Actions**:
- [ ] Review all files in `src/plugins/`
- [ ] Update `mod.rs` - module documentation
- [ ] Update `trait_def.rs` - trait documentation
- [ ] Update `registry.rs` - registry docs and logs
- [ ] Update `nvm.rs` - plugin implementation docs
- [ ] Update `fnm.rs` - plugin implementation docs
- [ ] Update `mock.rs` - test mock docs (if exists)
- [ ] Update log/error messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

## Version File Module Updates

### Task 4-4.3: Update Version File Module

**Files**: `src/version_file/` (all files)

**Files to check**:
- `src/version_file/mod.rs`
- `src/version_file/finder.rs`
- `src/version_file/package_json.rs`
- `src/version_file/semver.rs`

**Commands**:
```bash
# Search all version_file module files
grep -rn "xvn\|XVN" src/version_file/

# After changes
grep -ri "xvn" src/version_file/  # Should return no results
```

**Changes Needed**:
1. **Update documentation comments**
2. **Update log messages** if present

**Actions**:
- [ ] Review all files in `src/version_file/`
- [ ] Update `mod.rs` - module documentation
- [ ] Update `finder.rs` - finder documentation
- [ ] Update `package_json.rs` - parser documentation
- [ ] Update `semver.rs` - semver documentation
- [ ] Update log messages if present
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

## Remaining Utility Modules

### Task 4-4.4: Update Error Module

**File**: `src/error.rs`

**Changes Required**:

1. **Update error type documentation**:
   - Change references from "xvn" to "anvs"

2. **Update error messages** (if any hardcoded):
   - Update any Display implementations

**Commands**:
```bash
# Search for references
grep -n "xvn\|XVN" src/error.rs

# After changes
grep -i "xvn" src/error.rs  # Should return no results
```

**Actions**:
- [ ] Update error type documentation
- [ ] Update error messages if hardcoded
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-4.5: Update Output Module

**File**: `src/output.rs`

**Changes Required**:

1. **Update output formatting**:
   - Check if any formatted output references "xvn"

2. **Update documentation**:
   - Change doc comments referencing "xvn"

**Commands**:
```bash
# Search for references
grep -n "xvn\|XVN" src/output.rs

# After changes
grep -i "xvn" src/output.rs  # Should return no results
```

**Actions**:
- [ ] Update output formatting if needed
- [ ] Update banner/header text if present
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-4.6: Update Main Module

**File**: `src/main.rs`

**Changes Required**:

1. **Update main function documentation**:
   - Change references from "xvn" to "anvs"

2. **Update any startup messages** (if any):
   - Update debug/logging output

**Commands**:
```bash
# Search for references
grep -n "xvn\|XVN" src/main.rs

# After changes
grep -i "xvn" src/main.rs  # Should return no results
```

**Actions**:
- [ ] Update main function documentation
- [ ] Update startup messages if present
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build`

---

## Final Verification

### Task 4-4.7: Comprehensive Source Search

**Search entire src/ directory for any remaining references**:

```bash
# Search for all case-insensitive "xvn" references
grep -ri "xvn" src/

# Search for config file references
grep -r "\.xvnrc\|xvn\.yaml" src/

# Search for directory path references
grep -r "\.xvn/" src/

# Search for constant names
grep -r "XVN_" src/

# All of the above should return no results
```

**Actions**:
- [ ] No "xvn" string literals found in any source file
- [ ] No ".xvnrc" config references found (should be ".anvsrc")
- [ ] No ".xvn/" directory paths found (should be ".anvs/")
- [ ] No "XVN_" constants found (should be "ANVS_")
- [ ] Document any intentional exceptions with explanation

---

### Task 4-4.8: Full Build and Test

**Build the entire project with all changes**:

```bash
# Clean build
cargo clean

# Build library
cargo build --lib

# Build binary
cargo build

# Build release
cargo build --release

# Run all tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check

# Generate documentation
cargo doc --no-deps --open
```

**Expected Results**:
- All builds succeed without errors
- All tests pass
- No clippy warnings
- Code is properly formatted
- Documentation generates without warnings

**Actions**:
- [ ] `cargo clean` completes
- [ ] `cargo build --lib` succeeds
- [ ] `cargo build` succeeds
- [ ] `cargo build --release` succeeds
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] `cargo fmt -- --check` passes
- [ ] `cargo doc --no-deps` generates documentation
- [ ] Binary exists at `./target/debug/anvs`
- [ ] Binary exists at `./target/release/anvs`

---

### Task 4-4.9: Manual Binary Testing

**Test the compiled binary**:

```bash
# Test version
./target/debug/anvs --version

# Expected output:
# anvs 2.0.0

# Test help
./target/debug/anvs --help

# Test subcommand help
./target/debug/anvs activate --help
./target/debug/anvs setup --help
./target/debug/anvs status --help
./target/debug/anvs set --help
./target/debug/anvs uninstall --help

# Test status command (should show config paths)
./target/debug/anvs status

# Expected status output should reference:
# - ~/.anvsrc
# - .anvs.yaml
# - ~/.anvs/bin/anvs
```

**Verify Output**:
- All help text references "anvs" not "xvn"
- Config paths show `~/.anvsrc` and `.anvs.yaml`
- Installation path shows `~/.anvs/`
- No "xvn" references in any output

**Actions**:
- [ ] `--version` shows "anvs 2.0.0"
- [ ] `--help` shows "ANVS - Automatic Node Version Switcher"
- [ ] All subcommand help text is accurate
- [ ] `status` command shows correct config paths
- [ ] No "xvn" references in any command output
- [ ] All error messages reference "anvs"

---

### Task 4-4.10: Commit Changes

**Stage and commit Phase 4-4 changes**:

```bash
# Check status
git status

# Review changes (will be a moderate diff)
git diff src/cli.rs src/plugins/ src/version_file/ src/error.rs src/output.rs src/main.rs

# Stage all changes
git add src/cli.rs src/plugins/ src/version_file/ src/error.rs src/output.rs src/main.rs

# Commit with comprehensive message
git commit -m "$(cat <<'EOF'
feat(rust): update CLI, plugins, and remaining modules for anvs rename

- Update CLI name and help text to reference ANVS
- Update all command descriptions
- Update plugins module documentation
- Update version_file module documentation
- Update error, output, and main modules

Files changed:
- src/cli.rs - CLI name, description, and all help text
- src/plugins/mod.rs - module docs
- src/plugins/trait_def.rs - trait docs
- src/plugins/registry.rs - registry docs
- src/plugins/nvm.rs - nvm plugin docs
- src/plugins/fnm.rs - fnm plugin docs
- src/version_file/mod.rs - module docs
- src/version_file/finder.rs - finder docs
- src/version_file/package_json.rs - parser docs
- src/version_file/semver.rs - semver docs
- src/error.rs - error type docs
- src/output.rs - output formatting docs
- src/main.rs - main function docs

Part of Phase 4-4: CLI, Plugins & Remaining Modules (FINAL RUST PHASE)
EOF
)"
```

**Actions**:
- [ ] All source changes staged
- [ ] Commit message follows conventional commit format
- [ ] Commit message lists all major file categories changed
- [ ] Changes committed to current branch

---

## Verification Checklist

Before proceeding to Phase 5 (Test Files), verify ALL of the following:

**Compilation & Tests:**
- [ ] All Rust source files compile without errors
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt -- --check`
- [ ] Documentation builds: `cargo doc --no-deps`

**Binary Verification:**
- [ ] Binary builds successfully: `cargo build --release`
- [ ] Binary version shows `2.0.0`: `./target/release/anvs --version`
- [ ] CLI help shows "ANVS": `./target/release/anvs --help`
- [ ] All subcommand help is correct

**Path Verification:**
- [ ] Config paths use `.anvsrc` and `.anvs.yaml`
- [ ] Installation paths use `~/.anvs/`
- [ ] Shell integration uses `anvs.sh`

**Code Quality:**
- [ ] No "xvn" references in source code: `grep -ri "xvn" src/` returns nothing
- [ ] All user-facing messages reference "anvs"
- [ ] All error messages reference "anvs"
- [ ] All CLI help text references "anvs"

**Git:**
- [ ] All 4 Rust phases committed (4-1, 4-2, 4-3, 4-4)
- [ ] Changes follow conventional commit format

---

## Success Criteria

Phase 4-4 (and entire Phase 4) is complete when:

1. ✅ CLI module updated with new name and help text
2. ✅ Plugins module documentation updated
3. ✅ Version file module documentation updated
4. ✅ All utility modules updated
5. ✅ Binary name changed to `anvs`
6. ✅ All user-facing messages reference "anvs"
7. ✅ All error messages reference "anvs"
8. ✅ All documentation comments updated
9. ✅ Project compiles without errors
10. ✅ All tests pass
11. ✅ Binary functionality verified
12. ✅ All changes committed to git
13. ✅ **ALL RUST SOURCE CODE UPDATES COMPLETE**

---

## Next Steps

After completing Phase 4-4:

1. **Celebrate!** 🎉 All Rust source code has been renamed from xvn to anvs
2. **Proceed to Phase 5**: Test Files (update all test files with new names)
3. **Integration check**: Verify that binary, installation scripts, and shell integration work together
4. **Documentation review**: Prepare for Phase 6 (Documentation Files)

---

## Rollback Plan

If issues are discovered:

1. **Revert git commits**: `git revert <commit-hash>` for Phase 4-4 changes
2. **Restore files**: `git checkout HEAD~1 src/cli.rs src/plugins/ src/version_file/ src/error.rs src/output.rs src/main.rs`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify fixes with `cargo test` and manual testing
5. **Re-commit**: Commit fixes before proceeding

---

## Notes

- This is the **final** Rust source code phase - take your time
- The CLI help text is what users see first - make it perfect
- Test the binary thoroughly before moving on
- All 4 Rust phases (4-1, 4-2, 4-3, 4-4) together complete Phase 4 from the original plan
- After this phase, only test files and documentation remain for Rust code
- The binary should be fully functional with the new naming
- Consider creating a backup of the working binary: `cp target/release/anvs /tmp/anvs-backup`
- Total estimated time for Phase 4-4: 45-60 minutes of focused work
- Total estimated time for all of Phase 4 (4-1 through 4-4): 2.5-4 hours
