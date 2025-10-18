# Phase 4-3: Rust Source Code - Activation & Shell Modules

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes

## Overview

Phase 4-3 is the third part of the Rust source code update, focusing on the **Activation** and **Shell** modules. These modules handle version activation orchestration and shell communication.

**Why Phase 4-3 is Important:**
- Activation module coordinates version switching with user prompts
- Shell module handles FD3 protocol for parent shell communication
- Error messages in activation are frequently seen by users
- These modules tie together config, plugins, and version detection

**Modules Covered:**
- `src/activation/` - Version activation and orchestration
- `src/shell/` - Shell integration and FD3 protocol

**⚠️ IMPORTANT**: Phase 4-3 should only begin after Phase 4-2 is complete.

---

## Activation Module Updates

### Task 4-3.1: Update Activation Errors

**File**: `src/activation/errors.rs`

**Changes Required**:

1. **Update error messages that mention config paths**:
   - Change references to `~/.xvnrc` → `~/.anvsrc`
   - Change references to `.xvn.yaml` → `.anvs.yaml`

2. **Update error variants' display messages**:
   - Change any error text referencing "xvn"

**Example changes**:
```rust
// Before:
impl fmt::Display for ActivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigNotFound => write!(f, "Config file not found at ~/.xvnrc"),
            Self::InvalidConfig => write!(f, "Invalid configuration in .xvn.yaml"),
            // ...
        }
    }
}

// After:
impl fmt::Display for ActivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigNotFound => write!(f, "Config file not found at ~/.anvsrc"),
            Self::InvalidConfig => write!(f, "Invalid configuration in .anvs.yaml"),
            // ...
        }
    }
}
```

**Commands**:
```bash
# Review error messages
grep -n "xvn\|XVN" src/activation/errors.rs

# After changes
grep -i "xvn" src/activation/errors.rs  # Should return no results
```

**Actions**:
- [ ] Update error message strings referencing config paths
- [ ] Update error display implementations
- [ ] Update any error constructors with hardcoded messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.2: Update Orchestrator

**File**: `src/activation/orchestrator.rs`

**Changes Required**:

1. **Update activation logic documentation**:
   - Change doc comments referencing "xvn" → "anvs"

2. **Update any log messages**:
   - Change debug/info messages

**Commands**:
```bash
# Review orchestrator
grep -n "xvn\|XVN" src/activation/orchestrator.rs

# After changes
grep -i "xvn" src/activation/orchestrator.rs  # Should return no results
```

**Actions**:
- [ ] Update function documentation
- [ ] Update log messages (debug, info, warn, error)
- [ ] Update comments explaining the flow
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.3: Update User Prompts

**File**: `src/activation/user_prompt.rs`

**Changes Required**:

1. **Update auto-install prompts**:
   - Change prompt text referencing "xvn" → "anvs"
   - Update confirmation messages

2. **Update informational messages**:
   - Change all messages displayed to users

**Example changes**:
```rust
// Before:
println!("XVN can automatically install Node.js {} for you.", version);
let install = Confirm::new("Would you like xvn to install it?")
    .with_default(true)
    .prompt()?;

println!("xvn is installing Node.js {}...", version);

// After:
println!("ANVS can automatically install Node.js {} for you.", version);
let install = Confirm::new("Would you like anvs to install it?")
    .with_default(true)
    .prompt()?;

println!("anvs is installing Node.js {}...", version);
```

**Commands**:
```bash
# Review prompts
grep -n "xvn\|XVN" src/activation/user_prompt.rs

# After changes
grep -i "xvn" src/activation/user_prompt.rs  # Should return no results
```

**Actions**:
- [ ] Update installation prompts: "xvn" → "anvs", "XVN" → "ANVS"
- [ ] Update confirmation questions
- [ ] Update progress messages
- [ ] Update success messages
- [ ] Update error messages
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.4: Update Activation Module

**File**: `src/activation/mod.rs`

**Changes Required**:

1. **Update module documentation**:
   - Change references from "xvn" to "anvs"
   - Update module description

**Commands**:
```bash
# Review module docs
head -20 src/activation/mod.rs

# After changes
grep -i "xvn" src/activation/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update any example usage
- [ ] Update module description
- [ ] Verify compilation: `cargo build --lib`

---

## Shell Module Updates

### Task 4-3.5: Update FD3 Protocol

**File**: `src/shell/fd3.rs`

**Changes Required**:

1. **Update protocol documentation comments**:
   - Change any references to "xvn" in comments explaining the protocol

2. **Update debug/log messages** (if any):
   - Change messages referencing "xvn"

**Commands**:
```bash
# Review FD3 implementation
grep -n "xvn\|XVN" src/shell/fd3.rs

# After changes
grep -i "xvn" src/shell/fd3.rs  # Should return no results
```

**Actions**:
- [ ] Update documentation comments
- [ ] Update protocol explanation comments
- [ ] Update log messages if present
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.6: Update JSON Writer

**File**: `src/shell/json_writer.rs`

**Changes Required**:

1. **Check JSON field names** (if any reference "xvn"):
   - Review if field names need updating (be careful - breaking change)
   - Likely only documentation needs updating

2. **Update documentation**:
   - Change doc comments referencing "xvn"

**Commands**:
```bash
# Review JSON output
grep -n "xvn\|XVN" src/shell/json_writer.rs

# After changes
grep -i "xvn" src/shell/json_writer.rs  # Should return no results
```

**Actions**:
- [ ] Review JSON field names (likely no changes needed)
- [ ] Update documentation comments
- [ ] Update example JSON in docs
- [ ] Update doc comments
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.7: Update Shell Module

**File**: `src/shell/mod.rs`

**Changes Required**:

1. **Update module documentation**:
   - Change references from "xvn" to "anvs"

**Commands**:
```bash
# Review module docs
head -20 src/shell/mod.rs

# After changes
grep -i "xvn" src/shell/mod.rs  # Should return no results
```

**Actions**:
- [ ] Update module documentation header
- [ ] Update any example usage
- [ ] Verify compilation: `cargo build --lib`

---

### Task 4-3.8: Search Activation and Shell Modules

**Comprehensive search for remaining references**:

```bash
# Search activation module
grep -ri "xvn" src/activation/

# Search shell module
grep -ri "xvn" src/shell/

# Both should return no results
```

**Actions**:
- [ ] No "xvn" references in `src/activation/`
- [ ] No "xvn" references in `src/shell/`
- [ ] No config path references to old files
- [ ] All user-facing messages updated

---

### Task 4-3.9: Build and Test

**Build the modules with changes**:

```bash
# Build library only
cargo build --lib

# Run tests for these modules
cargo test --lib activation
cargo test --lib shell

# Run clippy on these modules
cargo clippy --lib -- -D warnings
```

**Actions**:
- [ ] `cargo build --lib` succeeds
- [ ] Activation module tests pass
- [ ] Shell module tests pass
- [ ] No clippy warnings

---

### Task 4-3.10: Commit Changes

**Stage and commit Phase 4-3 changes**:

```bash
# Check status
git status

# Review changes
git diff src/activation/ src/shell/

# Stage changes
git add src/activation/ src/shell/

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
feat(rust): update activation and shell modules for anvs rename

- Update activation module: error messages and user prompts
- Update shell module: FD3 protocol and JSON writer docs
- Update all user-facing text to reference "anvs"
- Update config path references in error messages
- Update installation prompts

Files changed:
- src/activation/errors.rs - error messages with paths
- src/activation/orchestrator.rs - orchestration docs
- src/activation/user_prompt.rs - installation prompts
- src/activation/mod.rs - module docs
- src/shell/fd3.rs - FD3 protocol docs
- src/shell/json_writer.rs - JSON output docs
- src/shell/mod.rs - module docs

Part of Phase 4-3: Activation & Shell Modules
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

Before proceeding to Phase 4-4, verify ALL of the following:

- [ ] Activation errors reference correct config paths
- [ ] User prompts say "anvs" not "xvn"
- [ ] Shell module documentation updated
- [ ] No "xvn" references in `src/activation/`: `grep -ri "xvn" src/activation/`
- [ ] No "xvn" references in `src/shell/`: `grep -ri "xvn" src/shell/`
- [ ] `cargo build --lib` succeeds
- [ ] Activation tests pass: `cargo test --lib activation`
- [ ] Shell tests pass: `cargo test --lib shell`
- [ ] No clippy warnings
- [ ] Changes committed to git

---

## Success Criteria

Phase 4-3 is complete when:

1. ✅ Activation module updated with new messages and prompts
2. ✅ Shell module updated with new documentation
3. ✅ All user-facing text references "anvs"
4. ✅ All error messages reference correct paths
5. ✅ All modules compile without errors
6. ✅ All module tests pass
7. ✅ Changes committed to git

---

## Next Steps

After completing Phase 4-3:

1. **Proceed to Phase 4-4**: CLI, Plugins, and Remaining Modules (final Rust phase)
2. **Verify progress**: Check that 3 out of 4 Rust phases are complete
3. **Track progress**: Mark tasks complete as you go

---

## Rollback Plan

If issues are discovered:

1. **Revert git commit**: `git revert <commit-hash>`
2. **Restore files**: `git checkout HEAD~1 src/activation/ src/shell/`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify with `cargo build --lib` and `cargo test`

---

## Notes

- Focus on activation and shell modules only in this phase
- User prompts in activation are critical - users see them frequently
- Shell module mostly needs documentation updates
- Test compilation after each file to catch errors early
- Take a break after this phase before starting Phase 4-4 (the final Rust phase)
- Estimated time: 30-45 minutes of focused work
