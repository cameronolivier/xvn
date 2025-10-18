# Phase 2: Installation & Binary Files

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 2 updates all installation scripts and the binary wrapper to use the new `anvs` naming. This phase is critical for ensuring that the package installs correctly to the new `~/.anvs/` directory and that the `anvs` binary wrapper points to the correct location.

**Why Phase 2 is Critical:**
- Installation scripts control where binaries are placed on user systems
- The npm postinstall script must create the new `~/.anvs/` directory structure
- The binary wrapper (`bin/anvs`) is what users invoke from their PATH
- These files contain hardcoded paths that must be updated for the new naming

**⚠️ IMPORTANT**: Phase 2 should only begin after Phase 1 (Core Configuration & Build Files) is complete, as it depends on the updated `package.json` and `Cargo.toml` configurations.

---

## Implementation Tasks

### Task 2.1: Update JavaScript Installation Script

**File**: `install.js`

**Changes Required**:

1. **Update constant names** (lines ~10-15):
   - Change `XVN_DIR` to `ANVS_DIR`
   - Update any other constants referencing `xvn`

2. **Update directory paths**:
   - Change `'.xvn'` to `'.anvs'` wherever it appears
   - Update installation directory: `~/.xvn` → `~/.anvs`
   - Update subdirectories: `~/.xvn/bin` → `~/.anvs/bin`, `~/.xvn/versions` → `~/.anvs/versions`

3. **Update binary name references**:
   - Change binary filename from `xvn` to `anvs`
   - Update all paths like `bin/xvn` → `bin/anvs`

4. **Update user-facing messages and console output**:
   - Change all log messages referencing "xvn" to "anvs"
   - Update success messages, error messages, and info logs
   - Update any ASCII art or banners if present

5. **Update function names and comments**:
   - Rename functions like `installXvn` → `installAnvs` (if applicable)
   - Update all code comments referencing xvn

**Example changes**:
```javascript
// Before:
const XVN_DIR = path.join(os.homedir(), '.xvn');
const BIN_DIR = path.join(XVN_DIR, 'bin');
console.log('Installing xvn to', XVN_DIR);

// After:
const ANVS_DIR = path.join(os.homedir(), '.anvs');
const BIN_DIR = path.join(ANVS_DIR, 'bin');
console.log('Installing anvs to', ANVS_DIR);
```

**Commands**:
```bash
# Review current file first
cat install.js | grep -i "xvn"

# After making changes, verify
cat install.js | grep -i "xvn"  # Should return no results
cat install.js | grep -i "anvs" # Should show new references
```

**Actions**:
- [ ] Update constant `XVN_DIR` to `ANVS_DIR`
- [ ] Change directory path from `.xvn` to `.anvs`
- [ ] Update binary name from `xvn` to `anvs`
- [ ] Update all console.log() messages referencing xvn
- [ ] Update all console.error() messages referencing xvn
- [ ] Update all console.info() messages referencing xvn
- [ ] Update all comments and documentation strings
- [ ] Verify no hardcoded "xvn" strings remain (except in deprecation messages)
- [ ] Test syntax: `node install.js --help` (if applicable)

---

### Task 2.2: Update JavaScript Uninstall Script

**File**: `uninstall.js`

**Changes Required**:

1. **Update constant names** (lines ~5-10):
   - Change `XVN_DIR` to `ANVS_DIR`
   - Change `XVN_CONFIG` to `ANVS_CONFIG`
   - Update any other constants

2. **Update file and directory paths**:
   - Change `~/.xvn` → `~/.anvs`
   - Change `~/.xvnrc` → `~/.anvsrc`
   - Change `.xvn.yaml` → `.anvs.yaml`
   - Update all path constants and variables

3. **Update user-facing messages**:
   - Change all console output referencing "xvn" to "anvs"
   - Update prompts: "Remove xvn?" → "Remove anvs?"
   - Update confirmation messages
   - Update error messages

4. **Update function names and logic**:
   - Rename functions like `removeXvn` → `removeAnvs` (if applicable)
   - Update shell profile cleanup (removing old `source ~/.xvn/bin/xvn.sh` lines)
   - Ensure it only removes anvs-related content, not xvn

**Example changes**:
```javascript
// Before:
const XVN_DIR = path.join(os.homedir(), '.xvn');
const XVN_CONFIG = path.join(os.homedir(), '.xvnrc');
console.log('Removing xvn from', XVN_DIR);

// After:
const ANVS_DIR = path.join(os.homedir(), '.anvs');
const ANVS_CONFIG = path.join(os.homedir(), '.anvsrc');
console.log('Removing anvs from', ANVS_DIR);
```

**Commands**:
```bash
# Review current file
cat uninstall.js | grep -i "xvn"

# After making changes, verify
cat uninstall.js | grep -i "xvn"  # Should return no results
cat uninstall.js | grep -i "anvs" # Should show new references
```

**Actions**:
- [ ] Update constant `XVN_DIR` to `ANVS_DIR`
- [ ] Update constant `XVN_CONFIG` to `ANVS_CONFIG`
- [ ] Change directory path `.xvn` → `.anvs`
- [ ] Change config file path `.xvnrc` → `.anvsrc`
- [ ] Change project config `.xvn.yaml` → `.anvs.yaml`
- [ ] Update all console messages (log, error, info, warn)
- [ ] Update interactive prompts if present
- [ ] Update shell profile cleanup logic (should remove anvs lines)
- [ ] Update all comments and documentation
- [ ] Verify no hardcoded "xvn" strings remain
- [ ] Test syntax: `node uninstall.js --help` (if applicable)

---

### Task 2.3: Rename and Update Binary Wrapper

**File**: `bin/xvn` → `bin/anvs`

**Step 1: Rename the file**:
```bash
# Rename the binary wrapper
git mv bin/xvn bin/anvs

# Verify rename
ls -la bin/
```

**Step 2: Update file contents**:

**Changes Required**:

1. **Update shebang and header** (lines 1-5):
   - Keep shebang as `#!/usr/bin/env bash`
   - Update file comment/description if present

2. **Update variable names** (lines ~10-15):
   - Change `XVN_BINARY` to `ANVS_BINARY`
   - Update any other variable names

3. **Update binary path** (line ~15):
   - Change `$HOME/.xvn/bin/xvn` to `$HOME/.anvs/bin/anvs`

4. **Update error messages** (lines ~20-30):
   - Change all error messages referencing "xvn" to "anvs"
   - Update installation instructions in errors
   - Update debug messages if present

**Example changes**:
```bash
# Before:
#!/usr/bin/env bash
# XVN binary wrapper

XVN_BINARY="$HOME/.xvn/bin/xvn"

if [ ! -f "$XVN_BINARY" ]; then
    echo "Error: xvn binary not found at $XVN_BINARY" >&2
    echo "Run: npm install -g @olvrcc/xvn" >&2
    exit 1
fi

exec "$XVN_BINARY" "$@"

# After:
#!/usr/bin/env bash
# ANVS binary wrapper

ANVS_BINARY="$HOME/.anvs/bin/anvs"

if [ ! -f "$ANVS_BINARY" ]; then
    echo "Error: anvs binary not found at $ANVS_BINARY" >&2
    echo "Run: npm install -g anvs" >&2
    exit 1
fi

exec "$ANVS_BINARY" "$@"
```

**Commands**:
```bash
# After renaming and updating, verify
cat bin/anvs

# Check that file is executable
ls -la bin/anvs

# Search for any remaining "xvn" references
grep -i "xvn" bin/anvs  # Should return no results

# Verify shebang is correct
head -n 1 bin/anvs
```

**Expected output**:
```
#!/usr/bin/env bash
```

**Actions**:
- [ ] Rename file: `git mv bin/xvn bin/anvs`
- [ ] Update file header/comment
- [ ] Change variable `XVN_BINARY` to `ANVS_BINARY`
- [ ] Update binary path from `$HOME/.xvn/bin/xvn` to `$HOME/.anvs/bin/anvs`
- [ ] Update error message: "xvn binary not found" → "anvs binary not found"
- [ ] Update installation help message: reference `npm install -g anvs`
- [ ] Update all other error messages and debug output
- [ ] Update all comments in the file
- [ ] Verify file is executable: `chmod +x bin/anvs`
- [ ] Test wrapper (will fail until binary is built, but syntax should be valid)

---

### Task 2.4: Update package.json bin Reference

**File**: `package.json`

**Note**: This should already be done in Phase 1, but verify here.

**Verify Change**:
```json
{
  "bin": {
    "anvs": "./bin/anvs"
  }
}
```

**Commands**:
```bash
# Verify bin entry
cat package.json | grep -A 2 '"bin"'
```

**Expected output**:
```json
"bin": {
  "anvs": "./bin/anvs"
},
```

**Actions**:
- [ ] Verify `package.json` bin entry points to `./bin/anvs`
- [ ] Verify bin entry uses `"anvs"` as the command name (not "xvn")
- [ ] If incorrect, update and commit: `fix: correct bin entry in package.json`

---

### Task 2.5: Search for Remaining References

**Search entire installation directory**:

```bash
# Search all JavaScript files for "xvn" references
grep -r "xvn" install.js uninstall.js bin/anvs

# Search for ".xvn" directory references
grep -r "\.xvn" install.js uninstall.js bin/anvs

# Search for "XVN" constant references
grep -r "XVN" install.js uninstall.js bin/anvs

# All of the above should return no results (or only in comments explaining migration)
```

**Actions**:
- [ ] No "xvn" string literals found in install.js
- [ ] No "xvn" string literals found in uninstall.js
- [ ] No "xvn" string literals found in bin/anvs
- [ ] No ".xvn" path references found (except in migration/cleanup logic)
- [ ] No "XVN_" constants found in any file
- [ ] Document any intentional exceptions (e.g., migration messages)

---

### Task 2.6: Test Installation Scripts Locally

**Test installation script syntax**:

```bash
# Dry run of install.js (if supported)
node install.js --dry-run

# Or just run it in a test directory
mkdir -p /tmp/test-anvs-install
cd /tmp/test-anvs-install

# Copy package files
cp -r /Users/cam/nona-mac/dev/tooling/xvn/{package.json,install.js,bin,native} .

# Run install script (will fail without built binaries, but should show correct paths)
node install.js

# Check that it tries to create ~/.anvs/ not ~/.xvn/
# Check that messages reference "anvs" not "xvn"
```

**Expected behavior**:
- Script should reference `~/.anvs/` directory
- Error messages should mention "anvs" not "xvn"
- Script should look for `native/*/anvs` binaries (not `xvn`)

**Actions**:
- [ ] install.js runs without syntax errors
- [ ] install.js references correct directory (`~/.anvs/`)
- [ ] install.js references correct binary name (`anvs`)
- [ ] Error messages are accurate and helpful
- [ ] Cleanup test directory: `rm -rf /tmp/test-anvs-install`

---

### Task 2.7: Test Uninstall Script Locally

**Test uninstall script syntax**:

```bash
# Dry run of uninstall.js
node uninstall.js --dry-run

# Or test in isolated environment
# (Be careful not to uninstall actual anvs if you have it)
cd /tmp
node /Users/cam/nona-mac/dev/tooling/xvn/uninstall.js --dry-run
```

**Expected behavior**:
- Script should reference `~/.anvs/` directory
- Script should reference `~/.anvsrc` config file
- Messages should reference "anvs" not "xvn"

**Actions**:
- [ ] uninstall.js runs without syntax errors
- [ ] uninstall.js references correct directory (`~/.anvs/`)
- [ ] uninstall.js references correct config (`~/.anvsrc`)
- [ ] Error messages are accurate and helpful
- [ ] Dry run mode works (if implemented)

---

### Task 2.8: Commit Changes

**Stage and commit all changes from Phase 2**:

```bash
# Check status
git status

# Review changes
git diff install.js
git diff uninstall.js
git diff bin/anvs

# Stage all changes
git add install.js uninstall.js bin/anvs package.json

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
feat(install): update installation scripts for anvs rename

- Rename bin/xvn → bin/anvs
- Update install.js: XVN_DIR → ANVS_DIR, paths .xvn → .anvs
- Update uninstall.js: XVN_CONFIG → ANVS_CONFIG, paths .xvnrc → .anvsrc
- Update all user-facing messages to reference "anvs"
- Update binary wrapper to point to ~/.anvs/bin/anvs

Files changed:
- install.js
- uninstall.js
- bin/xvn → bin/anvs
- package.json (verified bin entry)

Part of Phase 2: Installation & Binary Files
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

Before proceeding to Phase 3, verify ALL of the following:

- [ ] `install.js` contains no references to "xvn" (except in migration comments)
- [ ] `install.js` references `ANVS_DIR` constant pointing to `~/.anvs`
- [ ] `install.js` console messages reference "anvs" not "xvn"
- [ ] `uninstall.js` contains no references to "xvn" (except in migration comments)
- [ ] `uninstall.js` references `ANVS_DIR` and `ANVS_CONFIG` constants
- [ ] `uninstall.js` removes `~/.anvs/` directory and `~/.anvsrc` file
- [ ] `bin/anvs` exists (renamed from `bin/xvn`)
- [ ] `bin/anvs` references `ANVS_BINARY` variable
- [ ] `bin/anvs` points to `$HOME/.anvs/bin/anvs`
- [ ] `bin/anvs` is executable (`chmod +x`)
- [ ] `package.json` bin entry points to `./bin/anvs`
- [ ] No syntax errors in JavaScript files (`node --check install.js uninstall.js`)
- [ ] No shell syntax errors in bin/anvs (`bash -n bin/anvs`)
- [ ] All changes committed with descriptive message

---

## Success Criteria

Phase 2 is complete when:

1. ✅ All installation scripts updated to use `anvs` naming
2. ✅ Binary wrapper renamed to `bin/anvs` and updated
3. ✅ Installation directory changed from `~/.xvn` to `~/.anvs`
4. ✅ Configuration files changed from `~/.xvnrc` to `~/.anvsrc`
5. ✅ All user-facing messages reference "anvs" not "xvn"
6. ✅ Scripts run without syntax errors
7. ✅ Changes are committed to git

---

## Next Steps

After completing Phase 2:

1. **Verify Phase 1 completion**: Ensure `package.json` and `Cargo.toml` are updated
2. **Proceed to Phase 3**: Shell Integration (update `shell/xvn.sh` → `shell/anvs.sh`)
3. **Integration testing**: After Phase 3, test that installation and shell integration work together

---

## Rollback Plan

If issues are discovered:

1. **Revert git commits**: `git revert <commit-hash>` for Phase 2 changes
2. **Restore files**: `git checkout HEAD~1 install.js uninstall.js bin/anvs`
3. **Fix issues**: Address problems in a new commit
4. **Re-test**: Verify fixes before proceeding

---

## Notes

- The binary wrapper (`bin/anvs`) is a critical file that npm uses to create the global command
- Installation paths are hardcoded and must be exact (typos will break installation)
- These scripts run during `npm install -g anvs`, so they must be correct before publishing
- The uninstall script should only remove anvs-related files, not any user data
- Consider adding backwards-compatible cleanup (remove old `~/.xvn/` if found) in a future phase
- Test thoroughly before proceeding - installation errors are difficult to debug for users
