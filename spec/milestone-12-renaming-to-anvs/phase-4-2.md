# Phase 4-2: Rust Source Code - Commands & Init Modules

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 4-2 is the second part of the Rust source code update, focusing on the **Commands** and **Init** modules. These modules handle user-facing CLI commands and the interactive setup wizard.

**Why Phase 4-2 is Important:**
- Commands module contains user-facing CLI commands with many messages
- Init module runs the interactive setup wizard with prompts
- These modules have extensive user-facing text that must reference "anvs"
- Config file paths in these modules must match Phase 4-1 updates

**Modules Covered:**
- `src/commands/` - CLI commands (set, uninstall)
- `src/init/` - Setup wizard and validation

**⚠️ IMPORTANT**: Phase 4-2 should only begin after Phase 4-1 is complete.

---

## Commands Module Updates

### Task 4-2.1: Update Set Command

**File**: `src/commands/set.rs`

**Changes Required**:

1. **Update config file path**:
   - Change `~/.xvnrc` → `~/.anvsrc`

2. **Update user-facing messages**:
   - Change messages referencing config file path
   - Update prompts and confirmation messages

3. **Update help text**:
   - Update command description if it mentions config file

**Example changes**:
```rust
// Before:
let config_path = dirs::home_dir()
    .ok_or_else(|| anyhow!("Could not find home directory"))?
    .join(".xvnrc");

println!("Updating config at: ~/.xvnrc");

// After:
let config_path = dirs::home_dir()
    .ok_or_else(|| anyhow!("Could not find home directory"))?
    .join(".anvsrc");

println!("Updating config at: ~/.anvsrc");
```

**Commands**:
```bash
# Review config path usage
grep -n "xvnrc\|xvn" src/commands/set.rs

# After changes
grep -i "xvn" src/commands/set.rs  # Should return no results
grep -i "anvsrc" src/commands/set.rs  # Should show new path
```

**Actions**:
- [ ] Update config path: `.join(".xvnrc")` → `.join(".anvsrc")`
- [ ] Update all user messages referencing config file
- [ ] Update interactive prompts
- [ ] Update success/error messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.2: Update Uninstall Command

**File**: `src/commands/uninstall.rs`

**Changes Required**:

1. **Update directory and config file paths**:
   - Change `~/.xvn/` → `~/.anvs/`
   - Change `~/.xvnrc` → `~/.anvsrc`

2. **Update user-facing messages**:
   - Change uninstall prompts and confirmations
   - Update success messages
   - Update error messages

3. **Update shell profile cleanup**:
   - Update patterns to detect and remove anvs shell integration
   - Look for `anvs.sh` not `xvn.sh`

**Example changes**:
```rust
// Before:
let install_dir = home_dir.join(".xvn");
let config_file = home_dir.join(".xvnrc");

println!("The following will be removed:");
println!("  - XVN installation: ~/.xvn/");
println!("  - Configuration file: ~/.xvnrc");

// After:
let install_dir = home_dir.join(".anvs");
let config_file = home_dir.join(".anvsrc");

println!("The following will be removed:");
println!("  - ANVS installation: ~/.anvs/");
println!("  - Configuration file: ~/.anvsrc");
```

**Commands**:
```bash
# Review uninstall paths
grep -n "\.xvn\|xvnrc\|xvn\.sh" src/commands/uninstall.rs

# After changes
grep -i "\.xvn\|xvnrc\|xvn\.sh" src/commands/uninstall.rs  # Should return no results
grep -i "\.anvs\|anvsrc\|anvs\.sh" src/commands/uninstall.rs  # Should show new paths
```

**Actions**:
- [ ] Update installation directory path: `.join(".xvn")` → `.join(".anvs")`
- [ ] Update config file path: `.join(".xvnrc")` → `.join(".anvsrc")`
- [ ] Update shell profile removal patterns: match `anvs.sh` lines
- [ ] Update shell script path references: `xvn.sh` → `anvs.sh`
- [ ] Update all user-facing messages
- [ ] Update confirmation prompts: "Remove XVN?" → "Remove ANVS?"
- [ ] Update success messages
- [ ] Update error messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.3: Update Commands Module

**File**: `src/commands/mod.rs`

**Changes Required**:

1. **Update module documentation**:
   - Change references from "xvn" to "anvs"
   - Update command descriptions

**Commands**:
```bash
# Review module docs
head -20 src/commands/mod.rs

# After changes
grep -i "xvn" src/commands/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update command descriptions in comments
- [ ] Update any example usage
- [ ] Verify compilation: `cargo build --lib`

---

## Init Module Updates

### Task 4-2.4: Update Wizard

**File**: `src/init/wizard.rs`

**Changes Required**:

1. **Update config file path references**:
   - Change `~/.xvnrc` → `~/.anvsrc`

2. **Update wizard prompts and messages**:
   - Change all welcome messages referencing "xvn" → "anvs"
   - Update setup completion messages
   - Update progress indicators

**Example changes**:
```rust
// Before:
println!("Welcome to XVN setup wizard!");
println!("This will configure xvn for your system.\n");

let config_path = home_dir.join(".xvnrc");
println!("Creating config at: ~/.xvnrc");

// After:
println!("Welcome to ANVS setup wizard!");
println!("This will configure anvs for your system.\n");

let config_path = home_dir.join(".anvsrc");
println!("Creating config at: ~/.anvsrc");
```

**Commands**:
```bash
# Review wizard messages
grep -n "xvn\|XVN" src/init/wizard.rs

# After changes
grep -i "xvn" src/init/wizard.rs  # Should return no results
```

**Actions**:
- [ ] Update welcome messages: "XVN" → "ANVS"
- [ ] Update tool name references: "xvn" → "anvs"
- [ ] Update config path: `.join(".xvnrc")` → `.join(".anvsrc")`
- [ ] Update setup progress messages
- [ ] Update completion messages
- [ ] Update success messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.5: Update Prompts

**File**: `src/init/prompts.rs`

**Changes Required**:

1. **Update interactive prompt text**:
   - Change all prompts referencing "xvn" → "anvs"
   - Update help text and descriptions

2. **Update prompt labels and hints**:
   - Change option labels that reference the tool name

**Example changes**:
```rust
// Before:
let prompt = Select::new("Select your default version manager for xvn:")
    .with_options(vec!["nvm", "fnm"])
    .prompt()?;

println!("xvn will use {} to manage Node.js versions", selection);

// After:
let prompt = Select::new("Select your default version manager for anvs:")
    .with_options(vec!["nvm", "fnm"])
    .prompt()?;

println!("anvs will use {} to manage Node.js versions", selection);
```

**Commands**:
```bash
# Review prompt text
grep -n "xvn\|XVN" src/init/prompts.rs

# After changes
grep -i "xvn" src/init/prompts.rs  # Should return no results
```

**Actions**:
- [ ] Update all prompt questions and labels
- [ ] Update help text in prompts
- [ ] Update option descriptions
- [ ] Update confirmation messages
- [ ] Update explanation text
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.6: Update Detection

**File**: `src/init/detection.rs`

**Changes Required**:

1. **Update binary name checks** (if any):
   - Change detection logic that looks for "xvn" binary

2. **Update detection messages**:
   - Change log messages referencing "xvn"

**Commands**:
```bash
# Review detection logic
grep -n "xvn" src/init/detection.rs

# After changes
grep -i "xvn" src/init/detection.rs  # Should return no results
```

**Actions**:
- [ ] Update binary name in detection logic (if present)
- [ ] Update log/debug messages
- [ ] Update info messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.7: Update Validation

**File**: `src/init/validation.rs`

**Changes Required**:

1. **Update validation messages**:
   - Change error messages referencing "xvn" → "anvs"
   - Update warning messages

2. **Update path validation**:
   - Update any hardcoded path checks

**Commands**:
```bash
# Review validation messages
grep -n "xvn\|XVN" src/init/validation.rs

# After changes
grep -i "xvn" src/init/validation.rs  # Should return no results
```

**Actions**:
- [ ] Update error messages
- [ ] Update warning messages
- [ ] Update info messages
- [ ] Update path validation logic if needed
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.8: Update Init Module

**File**: `src/init/mod.rs`

**Changes Required**:

1. **Update module documentation**:
   - Change references from "xvn" to "anvs"
   - Update module description

**Commands**:
```bash
# Review module docs
head -20 src/init/mod.rs

# After changes
grep -i "xvn" src/init/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update example usage in comments
- [ ] Update module description
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-2.9: Search Commands and Init Modules

**Comprehensive search for remaining references**:

```bash
# Search commands module
grep -ri "xvn" src/commands/

# Search init module
grep -ri "xvn" src/init/

# Both should return no results
```

**Actions**:
- [ ] No "xvn" references in `src/commands/`
- [ ] No "xvn" references in `src/init/`
- [ ] No `.xvnrc` references (should be `.anvsrc`)
- [ ] No `.xvn/` directory references (should be `.anvs/`)
- [ ] No `xvn.sh` references (should be `anvs.sh`)

---

### Task 4-2.10: Build and Test

**Build the modules with changes**:

```bash
# Build library only
cargo build --lib

# Run tests for these modules
cargo test --lib commands
cargo test --lib init

# Run clippy on these modules
cargo clippy --lib -- -D warnings
```

**Actions**:
- [ ] `cargo build --lib` succeeds
- [ ] Commands module tests pass
- [ ] Init module tests pass
- [ ] No clippy warnings

---

### Task 4-2.11: Commit Changes

**Stage and commit Phase 4-2 changes**:

```bash
# Check status
git status

# Review changes
git diff src/commands/ src/init/

# Stage changes
git add src/commands/ src/init/

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
feat(rust): update commands and init modules for anvs rename

- Update commands module: config paths and messages
- Update init module: wizard prompts and validation messages
- Update all user-facing text to reference "anvs"
- Update config file paths: .xvnrc → .anvsrc
- Update directory paths: ~/.xvn/ → ~/.anvs/
- Update shell script references: xvn.sh → anvs.sh

Files changed:
- src/commands/set.rs - config file path
- src/commands/uninstall.rs - uninstall paths and messages
- src/commands/mod.rs - module docs
- src/init/wizard.rs - setup wizard messages
- src/init/prompts.rs - interactive prompts
- src/init/detection.rs - detection logic
- src/init/validation.rs - validation messages
- src/init/mod.rs - module docs

Part of Phase 4-2: Commands & Init Modules
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

Before proceeding to Phase 4-3, verify ALL of the following:

- [ ] Commands module uses correct config/directory paths
- [ ] Uninstall command references `~/.anvs/` and `~/.anvsrc`
- [ ] Init wizard welcome message says "ANVS"
- [ ] All prompts reference "anvs" not "xvn"
- [ ] No "xvn" references in `src/commands/`: `grep -ri "xvn" src/commands/`
- [ ] No "xvn" references in `src/init/`: `grep -ri "xvn" src/init/`
- [ ] `cargo build --lib` succeeds
- [ ] Commands tests pass: `cargo test --lib commands`
- [ ] Init tests pass: `cargo test --lib init`
- [ ] No clippy warnings
- [ ] Changes committed to git

---

## Success Criteria

Phase 4-2 is complete when:

1. ✅ Commands module updated with new paths and messages
2. ✅ Init module updated with new wizard prompts
3. ✅ All user-facing text references "anvs"
4. ✅ All config file paths use `.anvsrc` and `.anvs.yaml`
5. ✅ All modules compile without errors
6. ✅ All module tests pass
7. ✅ Changes committed to git

---

## Next Steps

After completing Phase 4-2:

1. **Proceed to Phase 4-3**: Activation & Shell Modules
2. **Verify integration**: Ensure commands work with Phase 4-1 config updates
3. **Track progress**: Mark tasks complete as you go

---

## Rollback Plan

If issues are discovered:

1. **Revert git commit**: `git revert <commit-hash>`
2. **Restore files**: `git checkout HEAD~1 src/commands/ src/init/`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify with `cargo build --lib` and `cargo test`

---

## Notes

- Focus on commands and init modules only in this phase
- These modules have many user-facing messages - check them all carefully
- The wizard prompts are what users see first - make them clear
- Test compilation after each file to catch errors early
- Take a break after this phase before starting Phase 4-3
- Estimated time: 45-60 minutes of focused work
