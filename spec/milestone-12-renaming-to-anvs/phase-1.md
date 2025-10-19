# Phase 1: Core Configuration & Build Files

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes

## Overview

Phase 1 updates the fundamental package and build configuration files to reflect the rename from `xvn` to `anvs`. This includes updating package metadata, binary names, and repository references across the core configuration files.

**Why Phase 1 is Critical:**
- Establishes the new package identity in build system
- Updates all references that affect distribution and installation
- Ensures build system generates correct artifacts
- Foundation for all subsequent code changes

**⚠️ CHECKPOINT**: Ensure Phase 0 is complete and published before starting Phase 1!

---

## Implementation Tasks

### Task 1.1: Update package.json

**File**: `package.json` (existing file)

**Changes Required**:
- Change `name` from `"@olvrcc/xvn"` to `"anvs"`
- Update `version` to `"2.0.0"`
- Update `description` to `"Automatic Node Version Switcher for Node.js"`
- Change `bin` entry from `"xvn"` to `"anvs"`
- Update repository URLs from `olvrcc/xvn` to `olvrcc/anvs`
- Update `homepage` and `bugs` URLs
- Update any other references to old package name

**Before/After Examples**:
```json
// Before
{
  "name": "@olvrcc/xvn",
  "version": "1.7.0",
  "description": "Extreme Version Switcher for Node.js",
  "bin": {
    "xvn": "./bin/xvn"
  },
  "repository": {
    "type": "git",
    "url": "git+https://github.com/olvrcc/xvn.git"
  },
  "homepage": "https://github.com/olvrcc/xvn#readme",
  "bugs": {
    "url": "https://github.com/olvrcc/xvn/issues"
  }
}

// After
{
  "name": "anvs",
  "version": "2.0.0",
  "description": "Automatic Node Version Switcher for Node.js",
  "bin": {
    "anvs": "./bin/anvs"
  },
  "repository": {
    "type": "git",
    "url": "git+https://github.com/olvrcc/anvs.git"
  },
  "homepage": "https://github.com/olvrcc/anvs#readme",
  "bugs": {
    "url": "https://github.com/olvrcc/anvs/issues"
  }
}
```

**Commands**:
```bash
# Verify current package.json
cat package.json | grep -E '"name"|"version"|"description"|"bin"|"repository"|"homepage"|"bugs"'
```

**Actions**:
- [ ] Update `name` to `"anvs"`
- [ ] Update `version` to `"2.0.0"`
- [ ] Update `description` to `"Automatic Node Version Switcher for Node.js"`
- [ ] Change `bin` entry to `"anvs": "./bin/anvs"`
- [ ] Update repository URL to `olvrcc/anvs`
- [ ] Update homepage URL to `olvrcc/anvs`
- [ ] Update bugs URL to `olvrcc/anvs`
- [ ] Verify no other references to old name remain

---

### Task 1.2: Update Cargo.toml

**File**: `Cargo.toml` (existing file)

**Changes Required**:
- Change package `name` to `"anvs"`
- Update `version` to `"2.0.0"`
- Update `description` to `"Automatic Node Version Switcher for Node.js"`
- Change `[[bin]]` name from `"xvn"` to `"anvs"`

**Before/After Examples**:
```toml
# Before
[package]
name = "xvn"
version = "1.7.0"
description = "Extreme Version Switcher for Node.js"

[[bin]]
name = "xvn"
path = "src/main.rs"

# After
[package]
name = "anvs"
version = "2.0.0"
description = "Automatic Node Version Switcher for Node.js"

[[bin]]
name = "anvs"
path = "src/main.rs"
```

**Commands**:
```bash
# Verify current Cargo.toml
grep -E '^name =|^version =|^description =|^\[\[bin\]\]|name = "xvn"' Cargo.toml
```

**Actions**:
- [ ] Update package `name` to `"anvs"`
- [ ] Update `version` to `"2.0.0"`
- [ ] Update `description` to `"Automatic Node Version Switcher for Node.js"`
- [ ] Change `[[bin]]` name to `"anvs"`
- [ ] Verify no other references to old name remain

---

### Task 1.3: Update Cargo.lock

**File**: `Cargo.lock` (auto-generated)

**Changes Required**:
- This file is auto-generated and will be updated when we rebuild
- The package name change in Cargo.toml will propagate here

**Commands**:
```bash
# Rebuild to update Cargo.lock
cargo build --release

# Verify the change
grep -E '^name =|^version =' Cargo.lock | head -10
```

**Expected Output**:
```
name = "anvs"
version = "2.0.0"
```

**Actions**:
- [ ] Run `cargo build --release` to update Cargo.lock
- [ ] Verify package name shows as `"anvs"` in Cargo.lock
- [ ] Verify version shows as `"2.0.0"` in Cargo.lock
- [ ] Confirm build succeeds with new binary name

---

### Task 1.4: Verify Build System

**Test that the build system works with new configuration**:

**Commands**:
```bash
# Clean previous build
cargo clean

# Build release version
cargo build --release

# Check that binary is created with new name
ls -lh target/release/anvs

# Test the binary
./target/release/anvs --version
```

**Expected Output**:
```
anvs 2.0.0
```

**Actions**:
- [ ] Build succeeds without errors
- [ ] Binary created as `target/release/anvs` (not `xvn`)
- [ ] Version command shows `anvs 2.0.0`
- [ ] Binary is functional (basic commands work)

---

### Task 1.5: Update Git Ignore Patterns (if needed)

**File**: `.gitignore` (check if needed)

**Changes Required**:
- Check if there are any references to old binary name in .gitignore
- Update any patterns that reference `xvn` binaries or artifacts

**Commands**:
```bash
# Check for references to old name
grep -i xvn .gitignore || echo "No xvn references found in .gitignore"
```

**Actions**:
- [ ] Check .gitignore for old binary name references
- [ ] Update any patterns if found (unlikely, but check)
- [ ] No changes needed if no references exist

---

## Verification Checklist

Before proceeding to Phase 2, verify ALL of the following:

- [ ] `package.json` name changed to `"anvs"`
- [ ] `package.json` version updated to `"2.0.0"`
- [ ] `package.json` bin entry changed to `"anvs"`
- [ ] `package.json` repository URLs updated to `olvrcc/anvs`
- [ ] `Cargo.toml` name changed to `"anvs"`
- [ ] `Cargo.toml` version updated to `"2.0.0"`
- [ ] `Cargo.toml` bin name changed to `"anvs"`
- [ ] `Cargo.lock` updated after rebuild (shows anvs 2.0.0)
- [ ] Build succeeds: `cargo build --release`
- [ ] Binary created as `target/release/anvs`
- [ ] Binary version shows `anvs 2.0.0`
- [ ] Binary is executable and functional

---

## Success Criteria

Phase 1 is complete when:

1. ✅ Package configuration files reflect new `anvs` identity
2. ✅ Build system generates `anvs` binary successfully
3. ✅ Version reporting shows `2.0.0`
4. ✅ No references to old `xvn` package name remain in core config
5. ✅ All repository URLs point to new `anvs` repository

---

## Next Steps

After completing Phase 1:

1. **Test locally**: Install the binary and verify basic functionality
2. **Commit changes**: `feat: rename package to anvs v2.0.0`
3. **Proceed to Phase 2**: Update installation and binary wrapper files

---

## Rollback Plan

If issues are discovered:

1. **Revert package.json**: Change name back to `"@olvrcc/xvn"`, version to `"1.7.0"`
2. **Revert Cargo.toml**: Change name back to `"xvn"`, version to `"1.7.0"`
3. **Rebuild**: `cargo build --release` to update Cargo.lock
4. **Test**: Verify old configuration works

---

## Notes

- These changes are breaking - version bump to 2.0.0 is appropriate
- Build system must work correctly before proceeding to code changes
- Repository URLs will be invalid until Phase 9 (repo rename)
- Keep old working version available during development