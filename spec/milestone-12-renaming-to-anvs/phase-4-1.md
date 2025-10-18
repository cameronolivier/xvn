# Phase 4-1: Rust Source Code - Configuration & Setup Modules

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 4-1 is the first part of the Rust source code update, focusing on the **Configuration** and **Setup** modules. These modules handle config file loading, installation paths, and shell integration setup.

**Why Phase 4-1 is Critical:**
- The config module controls where anvs looks for configuration files
- The setup module controls installation directory and shell integration
- These paths are fundamental to the entire application
- All other modules depend on correct config and installation paths

**Modules Covered:**
- `src/config/` - Configuration loading and schema
- `src/setup/` - Installation and shell integration setup

**⚠️ IMPORTANT**: Phase 4-1 should only begin after Phases 1-3 are complete.

---

## Implementation Tasks

### Task 4-1.1: Update Library Root Documentation

**File**: `src/lib.rs`

**Changes Required**:

1. **Update module-level documentation** (lines 1-10):
   - Change library name from "xvn" to "anvs"
   - Update description to "Automatic Node Version Switcher"
   - Update any example code or usage instructions

**Example changes**:
```rust
// Before:
//! # XVN - Extreme Version Switcher
//!
//! A fast, automatic Node.js version switcher written in Rust.
//!
//! XVN automatically switches Node.js versions when you change directories,
//! reading from `.nvmrc`, `.node-version`, or `package.json` files.

// After:
//! # ANVS - Automatic Node Version Switcher
//!
//! A fast, automatic Node.js version switcher written in Rust.
//!
//! ANVS automatically switches Node.js versions when you change directories,
//! reading from `.nvmrc`, `.node-version`, or `package.json` files.
```

**Commands**:
```bash
# Review current documentation
head -20 src/lib.rs

# After making changes, verify
grep -i "xvn" src/lib.rs  # Should return no results
grep -i "anvs" src/lib.rs # Should show new references
```

**Actions**:
- [ ] Update crate documentation header to reference "ANVS"
- [ ] Update description to "Automatic Node Version Switcher"
- [ ] Update any example code in documentation
- [ ] Update module-level comments
- [ ] Verify documentation compiles: `cargo doc --no-deps`

---

## Configuration Module Updates

### Task 4-1.2: Update Config Loader

**File**: `src/config/loader.rs`

**Changes Required**:

1. **Update config file path constants** (~lines 10-20):
   - Change `".xvnrc"` to `".anvsrc"`
   - Change `".xvn.yaml"` to `".anvs.yaml"`
   - Update any variable names containing "xvn"

2. **Update file discovery logic**:
   - Change global config path: `~/.xvnrc` → `~/.anvsrc`
   - Change project config filename: `.xvn.yaml` → `.anvs.yaml`

3. **Update error messages and logging**:
   - Change all error messages referencing config file names
   - Update debug log messages

4. **Update documentation comments**:
   - Update all doc comments referencing config files
   - Update examples in documentation

**Example changes**:
```rust
// Before:
const GLOBAL_CONFIG_FILE: &str = ".xvnrc";
const PROJECT_CONFIG_FILE: &str = ".xvn.yaml";

/// Load configuration from ~/.xvnrc and ./.xvn.yaml
pub fn load_config() -> Result<Config> {
    let global_config_path = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".xvnrc");
    // ...
}

// After:
const GLOBAL_CONFIG_FILE: &str = ".anvsrc";
const PROJECT_CONFIG_FILE: &str = ".anvs.yaml";

/// Load configuration from ~/.anvsrc and ./.anvs.yaml
pub fn load_config() -> Result<Config> {
    let global_config_path = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".anvsrc");
    // ...
}
```

**Commands**:
```bash
# Review current config paths
grep -n "xvnrc\|xvn.yaml" src/config/loader.rs

# After making changes, verify
grep -i "xvn" src/config/loader.rs  # Should return no results
grep -i "anvsrc\|anvs.yaml" src/config/loader.rs  # Should show new paths
```

**Actions**:
- [ ] Update constant `GLOBAL_CONFIG_FILE` to `".anvsrc"`
- [ ] Update constant `PROJECT_CONFIG_FILE` to `".anvs.yaml"`
- [ ] Update all path construction using `join(".xvnrc")` → `join(".anvsrc")`
- [ ] Update all path construction using `".xvn.yaml"` → `".anvs.yaml"`
- [ ] Update error messages referencing config file names
- [ ] Update debug/info log messages
- [ ] Update doc comments and examples
- [ ] Update function documentation
- [ ] Test compilation: `cargo build --lib`

---

### Task 4-1.3: Update Config Schema

**File**: `src/config/schema.rs`

**Changes Required**:

1. **Update struct documentation**:
   - Update doc comments for `Config` struct
   - Update field documentation if it references file paths

2. **Update any default values** (if applicable):
   - Check if any defaults reference "xvn"

**Example changes**:
```rust
// Before:
/// Configuration for XVN
///
/// Loaded from ~/.xvnrc (global) and ./.xvn.yaml (project-specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ...
}

// After:
/// Configuration for ANVS
///
/// Loaded from ~/.anvsrc (global) and ./.anvs.yaml (project-specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ...
}
```

**Commands**:
```bash
# Review documentation
grep -n "xvn\|XVN" src/config/schema.rs

# After changes
grep -i "xvn" src/config/schema.rs  # Should return no results
```

**Actions**:
- [ ] Update `Config` struct documentation
- [ ] Update field documentation referencing paths
- [ ] Update any example values in docs
- [ ] Update comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-1.4: Update Config Module

**File**: `src/config/mod.rs`

**Changes Required**:

1. **Update module-level documentation**:
   - Change references from "xvn" to "anvs"
   - Update config file path references

**Commands**:
```bash
# Review module docs
head -20 src/config/mod.rs

# After changes
grep -i "xvn" src/config/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update any example code in comments
- [ ] Update re-export documentation if present
- [ ] Verify compilation: `cargo build --lib`

---

## Setup Module Updates

### Task 4-1.5: Update Installer

**File**: `src/setup/installer.rs`

**Changes Required**:

1. **Update installation directory constants** (~lines 10-15):
   - Change `".xvn"` to `".anvs"`
   - Update binary name from `"xvn"` to `"anvs"`

2. **Update directory creation logic**:
   - Change installation path: `~/.xvn/` → `~/.anvs/`
   - Change subdirectories: `~/.xvn/bin` → `~/.anvs/bin`

3. **Update binary installation**:
   - Change binary filename from `xvn` to `anvs`
   - Update binary path construction

4. **Update shell script installation**:
   - Change script filename: `xvn.sh` → `anvs.sh`
   - Update script installation path

5. **Update user-facing messages**:
   - Change all success/error messages referencing paths
   - Update installation progress messages

**Example changes**:
```rust
// Before:
const INSTALL_DIR: &str = ".xvn";
const BIN_SUBDIR: &str = "bin";
const BINARY_NAME: &str = "xvn";

pub fn install() -> Result<()> {
    let install_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".xvn");

    info!("Installing xvn to {}", install_dir.display());
    // ...
}

// After:
const INSTALL_DIR: &str = ".anvs";
const BIN_SUBDIR: &str = "bin";
const BINARY_NAME: &str = "anvs";

pub fn install() -> Result<()> {
    let install_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".anvs");

    info!("Installing anvs to {}", install_dir.display());
    // ...
}
```

**Commands**:
```bash
# Review installation paths
grep -n "\.xvn\|xvn" src/setup/installer.rs

# After changes
grep -i "\.xvn\|\"xvn\"" src/setup/installer.rs  # Should return no results
grep -i "\.anvs\|\"anvs\"" src/setup/installer.rs  # Should show new paths
```

**Actions**:
- [ ] Update constant `INSTALL_DIR` to `".anvs"`
- [ ] Update constant `BINARY_NAME` to `"anvs"`
- [ ] Update shell script filename reference: `"xvn.sh"` → `"anvs.sh"`
- [ ] Update all path construction using `.join(".xvn")` → `.join(".anvs")`
- [ ] Update binary installation path
- [ ] Update all info/error messages referencing "xvn" → "anvs"
- [ ] Update all log messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-1.6: Update Profile Modification

**File**: `src/setup/profile_modification.rs`

**Changes Required**:

1. **Update shell integration source line**:
   - Change `source ~/.xvn/bin/xvn.sh` → `source ~/.anvs/bin/anvs.sh`

2. **Update detection logic** (for idempotency):
   - Update regex/patterns that detect existing installation
   - Look for both old and new patterns (for migration scenarios)

3. **Update error messages**:
   - Change all messages referencing shell script path

**Example changes**:
```rust
// Before:
const SHELL_INTEGRATION_LINE: &str = "[ -f ~/.xvn/bin/xvn.sh ] && source ~/.xvn/bin/xvn.sh";

fn is_already_installed(profile_content: &str) -> bool {
    profile_content.contains("xvn.sh")
}

// After:
const SHELL_INTEGRATION_LINE: &str = "[ -f ~/.anvs/bin/anvs.sh ] && source ~/.anvs/bin/anvs.sh";

fn is_already_installed(profile_content: &str) -> bool {
    profile_content.contains("anvs.sh")
}
```

**Commands**:
```bash
# Review shell integration strings
grep -n "xvn\.sh\|\.xvn" src/setup/profile_modification.rs

# After changes
grep -i "xvn" src/setup/profile_modification.rs  # Should return no results
grep -i "anvs\.sh\|\.anvs" src/setup/profile_modification.rs  # Should show new paths
```

**Actions**:
- [ ] Update shell source line constant/string
- [ ] Update shell script path: `~/.xvn/bin/xvn.sh` → `~/.anvs/bin/anvs.sh`
- [ ] Update detection patterns for existing installation
- [ ] Update error messages referencing shell script
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-1.7: Update Shell Detection

**File**: `src/setup/shell_detection.rs`

**Changes Required**:

1. **Update shell script path references**:
   - Change any references to `xvn.sh` → `anvs.sh`

2. **Update error messages**:
   - Change messages that reference the shell script name

**Commands**:
```bash
# Review shell script references
grep -n "xvn" src/setup/shell_detection.rs

# After changes
grep -i "xvn" src/setup/shell_detection.rs  # Should return no results
```

**Actions**:
- [ ] Update shell script filename references
- [ ] Update error/warning messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-1.8: Update Setup Module

**File**: `src/setup/mod.rs`

**Changes Required**:

1. **Update module documentation**:
   - Change references from "xvn" to "anvs"
   - Update path examples in documentation

**Commands**:
```bash
# Review module docs
head -20 src/setup/mod.rs

# After changes
grep -i "xvn" src/setup/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update example paths in comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-1.9: Search Config and Setup Modules

**Comprehensive search for remaining references**:

```bash
# Search config module
grep -ri "xvn" src/config/

# Search setup module
grep -ri "xvn" src/setup/

# Both should return no results
```

**Actions**:
- [ ] No "xvn" references in `src/config/`
- [ ] No "xvn" references in `src/setup/`
- [ ] No `.xvnrc` references (should be `.anvsrc`)
- [ ] No `.xvn.yaml` references (should be `.anvs.yaml`)
- [ ] No `.xvn/` directory references (should be `.anvs/`)

---

### Task 4-1.10: Build and Test

**Build the modules with changes**:

```bash
# Build library only
cargo build --lib

# Run tests for these modules
cargo test --lib config
cargo test --lib setup

# Run clippy on these modules
cargo clippy --lib -- -D warnings
```

**Actions**:
- [ ] `cargo build --lib` succeeds
- [ ] Config module tests pass
- [ ] Setup module tests pass
- [ ] No clippy warnings

---

### Task 4-1.11: Commit Changes

**Stage and commit Phase 4-1 changes**:

```bash
# Check status
git status

# Review changes
git diff src/lib.rs src/config/ src/setup/

# Stage changes
git add src/lib.rs src/config/ src/setup/

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
feat(rust): update config and setup modules for anvs rename

- Update src/lib.rs documentation to reference ANVS
- Update config module: .xvnrc → .anvsrc, .xvn.yaml → .anvs.yaml
- Update setup module: ~/.xvn/ → ~/.anvs/, xvn.sh → anvs.sh
- Update all user-facing messages in setup
- Update shell integration paths

Files changed:
- src/lib.rs - library documentation
- src/config/loader.rs - config file paths
- src/config/schema.rs - config struct docs
- src/config/mod.rs - module docs
- src/setup/installer.rs - installation paths and binary name
- src/setup/profile_modification.rs - shell integration paths
- src/setup/shell_detection.rs - shell script references
- src/setup/mod.rs - module docs

Part of Phase 4-1: Configuration & Setup Modules
EOF
)"
```

**Actions**:
- [ ] All changes staged
- [ ] Commit message follows conventional commit format
- [ ] Commit message lists all changed files
- [ ] Changes committed to current branch

---

## Verification Checklist

Before proceeding to Phase 4-2, verify ALL of the following:

- [ ] `src/lib.rs` documentation references "ANVS"
- [ ] Config module uses `.anvsrc` and `.anvs.yaml`
- [ ] Setup module uses `~/.anvs/` directory
- [ ] Setup module uses `anvs` binary name
- [ ] Setup module uses `anvs.sh` shell script
- [ ] No "xvn" references in `src/config/`: `grep -ri "xvn" src/config/`
- [ ] No "xvn" references in `src/setup/`: `grep -ri "xvn" src/setup/`
- [ ] `cargo build --lib` succeeds
- [ ] Config tests pass: `cargo test --lib config`
- [ ] Setup tests pass: `cargo test --lib setup`
- [ ] No clippy warnings
- [ ] Changes committed to git

---

## Success Criteria

Phase 4-1 is complete when:

1. ✅ Library root documentation updated
2. ✅ Config module updated with new file paths
3. ✅ Setup module updated with new directory and binary names
4. ✅ All user-facing messages reference "anvs"
5. ✅ All modules compile without errors
6. ✅ All module tests pass
7. ✅ Changes committed to git

---

## Next Steps

After completing Phase 4-1:

1. **Proceed to Phase 4-2**: Commands & Init Modules
2. **Keep testing**: Ensure compilation continues to work as you progress
3. **Track progress**: Mark tasks complete as you go

---

## Rollback Plan

If issues are discovered:

1. **Revert git commit**: `git revert <commit-hash>`
2. **Restore files**: `git checkout HEAD~1 src/lib.rs src/config/ src/setup/`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify with `cargo build --lib` and `cargo test`

---

## Notes

- Focus on config and setup modules only in this phase
- These are foundational modules - get them right first
- Test compilation after each file to catch errors early
- The binary won't function fully until all phases are complete
- Take a break after this phase before starting Phase 4-2
- Estimated time: 45-60 minutes of focused work
