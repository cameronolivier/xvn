# Milestone 10: Version-Independent Installation - Tasks

**Status:** Planning
**Target:** v1.2.0

---

## Phase 1: Core Infrastructure (Days 1-3)

### M10.1: Update install.js for ~/.xvn Installation

**Priority:** CRITICAL
**Estimated Time:** 4 hours

- [ ] Create directory structure helper
  - [ ] Create `~/.xvn/` directory
  - [ ] Create `~/.xvn/bin/` directory
  - [ ] Create `~/.xvn/versions/v{VERSION}/` directory
  - [ ] Create `~/.xvn/versions/v{VERSION}/bin/` directory
  - [ ] Create `~/.xvn/versions/v{VERSION}/lib/` directory

- [ ] Modify binary download logic
  - [ ] Download binary to `~/.xvn/versions/v{VERSION}/bin/xvn`
  - [ ] Make binary executable (chmod +x)
  - [ ] Verify binary works (`xvn --version`)

- [ ] Copy shell integration scripts
  - [ ] Copy `shell/xvn.sh` to `~/.xvn/versions/v{VERSION}/lib/xvn.sh`
  - [ ] Copy `shell/xvn.ps1` to `~/.xvn/versions/v{VERSION}/lib/xvn.ps1` (Windows)
  - [ ] Preserve file permissions

- [ ] Create symlinks
  - [ ] Create/update `~/.xvn/bin/xvn` → `versions/v{VERSION}/bin/xvn`
  - [ ] Create/update `~/.xvn/current` → `versions/v{VERSION}/`
  - [ ] Handle existing symlinks (unlink before creating)
  - [ ] Test symlink resolution

- [ ] Add upgrade detection
  - [ ] Detect if `~/.xvn/` already exists
  - [ ] Determine if this is fresh install or upgrade
  - [ ] Keep previous version for rollback
  - [ ] Clean up versions older than last 2

- [ ] Add error handling
  - [ ] Handle permission errors
  - [ ] Handle disk space issues
  - [ ] Provide clear error messages
  - [ ] Add installation troubleshooting output

- [ ] Update postinstall messaging
  - [ ] Show installation location (`~/.xvn/versions/v{VERSION}`)
  - [ ] Prompt user to run `xvn setup`
  - [ ] Show manual PATH setup instructions
  - [ ] Indicate if this is an upgrade

**Files to modify:**
- `install.js`

**Testing:**
```bash
# Test fresh install
rm -rf ~/.xvn
npm install -g @olvrcc/xvn
test -f ~/.xvn/bin/xvn
test -L ~/.xvn/current
~/.xvn/bin/xvn --version

# Test upgrade
npm install -g @olvrcc/xvn@1.2.0
test -d ~/.xvn/versions/v1.1.2  # Old version kept
test -d ~/.xvn/versions/v1.2.0  # New version installed
```

---

### M10.2: Update Setup Command for PATH Export

**Priority:** CRITICAL
**Estimated Time:** 3 hours

- [ ] Update setup command to add PATH
  - [ ] Add `export PATH="$HOME/.xvn/bin:$PATH"` to shell profile
  - [ ] Update shell hook source path to use `$XVN_DIR/current/lib/xvn.sh`
  - [ ] Detect if PATH is already configured
  - [ ] Handle different shell profiles (bash, zsh)

- [ ] Add verification step
  - [ ] Check that `~/.xvn/bin/xvn` exists
  - [ ] Verify symlink is valid
  - [ ] Test that xvn is in PATH after setup
  - [ ] Show success message with next steps

- [ ] Add migration detection
  - [ ] Detect old-style shell integration (without PATH)
  - [ ] Offer to update shell profile
  - [ ] Backup shell profile before modification
  - [ ] Show diff of changes

- [ ] Update setup output
  - [ ] Show PATH modification
  - [ ] Show shell hook location
  - [ ] Provide restart shell instructions
  - [ ] Add verification command (`which xvn`)

**Files to modify:**
- `src/commands/setup.rs`

**Shell Profile Format:**
```bash
# xvn shell integration
export XVN_DIR="$HOME/.xvn"
export PATH="$XVN_DIR/bin:$PATH"
[ -s "$XVN_DIR/current/lib/xvn.sh" ] && . "$XVN_DIR/current/lib/xvn.sh"
```

**Testing:**
```bash
# Test setup on fresh system
xvn setup
source ~/.zshrc
which xvn  # Should be ~/.xvn/bin/xvn
xvn --version

# Test setup with existing profile
# (should detect and not duplicate)
xvn setup
grep -c "xvn shell integration" ~/.zshrc  # Should be 1
```

---

### M10.3: Add Migration Support for Existing Users

**Priority:** HIGH
**Estimated Time:** 3 hours

- [ ] Detect old installation in postinstall
  - [ ] Check if xvn exists in npm global bin
  - [ ] Check if shell profile has old-style integration
  - [ ] Determine if migration is needed

- [ ] Implement automatic migration
  - [ ] Detect current installation location
  - [ ] Move/copy configuration to ~/.xvn
  - [ ] Update shell profile to new format
  - [ ] Backup old installation

- [ ] Add migration prompt
  - [ ] Show what will be migrated
  - [ ] Ask for confirmation
  - [ ] Provide manual migration option
  - [ ] Show rollback instructions

- [ ] Create migration verification
  - [ ] Test old binary still works (for rollback)
  - [ ] Verify new binary is accessible
  - [ ] Check PATH is correctly configured
  - [ ] Validate shell integration works

**Files to modify:**
- `install.js` (migration detection)
- New file: `scripts/migrate.js` (migration logic)

**Migration Flow:**
```javascript
// install.js
if (detectOldInstallation()) {
  console.log('Detected old xvn installation');
  console.log('Would you like to migrate to ~/.xvn? (Y/n)');

  if (confirmed) {
    await migrate();
  } else {
    console.log('Manual migration steps: ...');
  }
}
```

---

### M10.4: Update Documentation

**Priority:** MEDIUM
**Estimated Time:** 2 hours

- [ ] Update README.md
  - [ ] Document new installation location (`~/.xvn/`)
  - [ ] Explain PATH modification requirement
  - [ ] Add troubleshooting section
  - [ ] Update installation instructions

- [ ] Create migration guide
  - [ ] Instructions for users upgrading from v1.1.x
  - [ ] Manual migration steps
  - [ ] Rollback procedure
  - [ ] FAQ section

- [ ] Update ROADMAP.md
  - [ ] Mark Milestone 10 as complete
  - [ ] Update version number references
  - [ ] Add completed date

**Files to modify:**
- `README.md`
- `ROADMAP.md`
- New file: `docs/MIGRATION.md`

---

## Phase 2: Testing & Validation (Days 4-5)

### M10.5: Fresh Installation Testing

**Priority:** HIGH
**Estimated Time:** 2 hours

- [ ] Test on clean system (no Node.js versions)
  - [ ] Install Node.js 20
  - [ ] Install xvn via npm
  - [ ] Run `xvn setup`
  - [ ] Verify xvn works

- [ ] Test with existing nvm installation
  - [ ] Multiple Node.js versions (16, 18, 20)
  - [ ] Install xvn while on Node 20
  - [ ] Switch to Node 18, verify xvn available
  - [ ] Switch to Node 16, verify xvn available

- [ ] Test with existing fnm installation
  - [ ] Multiple Node.js versions
  - [ ] Verify xvn availability after switches

**Test Script:**
```bash
# test-fresh-install.sh
#!/bin/bash
set -e

echo "Testing fresh installation..."

# Clean slate
rm -rf ~/.xvn

# Install xvn
npm install -g @olvrcc/xvn

# Verify installation
test -f ~/.xvn/bin/xvn || exit 1
test -L ~/.xvn/current || exit 1

# Setup
xvn setup

# Source profile
source ~/.zshrc

# Verify xvn in PATH
which xvn | grep -q "/.xvn/bin/xvn" || exit 1

# Test version switching
cd /tmp
echo "18.20.0" > .nvmrc
xvn activate .
which xvn | grep -q "/.xvn/bin/xvn" || exit 1  # Should still be available

echo "✓ Fresh installation test passed"
```

---

### M10.6: Upgrade Path Testing

**Priority:** HIGH
**Estimated Time:** 2 hours

- [ ] Test upgrade from v1.1.2 to v1.2.0
  - [ ] Install v1.1.2 first
  - [ ] Upgrade to v1.2.0
  - [ ] Verify migration works
  - [ ] Verify old version kept for rollback

- [ ] Test upgrade with active usage
  - [ ] Have xvn managing a project (Node 18)
  - [ ] Upgrade to v1.2.0
  - [ ] Verify version switching still works
  - [ ] Check no interruption to workflow

- [ ] Test multiple upgrades
  - [ ] Install v1.1.0
  - [ ] Upgrade to v1.1.2
  - [ ] Upgrade to v1.2.0
  - [ ] Verify only last 2 versions kept

**Test Script:**
```bash
# test-upgrade.sh
#!/bin/bash
set -e

echo "Testing upgrade path..."

# Install old version
npm install -g @olvrcc/xvn@1.1.2

# Verify old installation
~/.xvn/bin/xvn --version | grep -q "1.1.2" || exit 1

# Upgrade to new version
npm install -g @olvrcc/xvn@1.2.0

# Verify upgrade
~/.xvn/bin/xvn --version | grep -q "1.2.0" || exit 1

# Verify old version kept
test -d ~/.xvn/versions/v1.1.2 || exit 1
test -d ~/.xvn/versions/v1.2.0 || exit 1

# Verify symlink updated
readlink ~/.xvn/current | grep -q "v1.2.0" || exit 1

echo "✓ Upgrade test passed"
```

---

### M10.7: Version Switching Validation

**Priority:** CRITICAL
**Estimated Time:** 2 hours

- [ ] Test xvn availability across versions
  - [ ] Install xvn while on Node 20
  - [ ] Switch to Node 18 (via xvn)
  - [ ] Verify `which xvn` still works
  - [ ] Verify `xvn --version` works
  - [ ] Switch to Node 16
  - [ ] Verify xvn still available

- [ ] Test rapid version switching
  - [ ] Switch between 5+ Node.js versions
  - [ ] Verify xvn never becomes unavailable
  - [ ] Check for any PATH issues

- [ ] Test edge cases
  - [ ] Switch to Node version without global packages
  - [ ] Switch to newly installed Node version
  - [ ] Verify xvn works in all cases

**Test Script:**
```bash
# test-version-switching.sh
#!/bin/bash
set -e

echo "Testing version switching..."

# Install multiple Node versions with nvm
nvm install 16
nvm install 18
nvm install 20

# Install xvn while on Node 20
nvm use 20
npm install -g @olvrcc/xvn
xvn setup
source ~/.zshrc

# Initial check
which xvn | grep -q "/.xvn/bin/xvn" || exit 1

# Switch to Node 18
nvm use 18
which xvn | grep -q "/.xvn/bin/xvn" || exit 1
xvn --version || exit 1

# Switch to Node 16
nvm use 16
which xvn | grep -q "/.xvn/bin/xvn" || exit 1
xvn --version || exit 1

# Switch back to Node 20
nvm use 20
which xvn | grep -q "/.xvn/bin/xvn" || exit 1

echo "✓ Version switching test passed"
```

---

### M10.8: Shell Integration Testing

**Priority:** HIGH
**Estimated Time:** 2 hours

- [ ] Test bash integration
  - [ ] Setup on bash
  - [ ] Verify PATH modification
  - [ ] Test shell hook works
  - [ ] Test version switching in bash

- [ ] Test zsh integration
  - [ ] Setup on zsh
  - [ ] Verify PATH modification
  - [ ] Test shell hook works
  - [ ] Test version switching in zsh

- [ ] Test different scenarios
  - [ ] Fresh shell session
  - [ ] After source ~/.zshrc
  - [ ] In subshell
  - [ ] In tmux/screen session

**Manual Testing Checklist:**
```
[ ] Open new terminal window
[ ] Run: which xvn
    Expected: ~/.xvn/bin/xvn
[ ] Run: xvn --version
    Expected: Shows version
[ ] cd to project with .nvmrc
    Expected: Auto-switches version
[ ] Run: which xvn
    Expected: Still ~/.xvn/bin/xvn
[ ] Run: xvn status
    Expected: Shows active version
```

---

## Phase 3: Documentation & Release (Days 6-7)

### M10.9: Create Migration Documentation

**Priority:** MEDIUM
**Estimated Time:** 2 hours

- [ ] Write migration guide
  - [ ] Upgrading from v1.1.x to v1.2.0
  - [ ] What changes
  - [ ] Why the change is necessary
  - [ ] Step-by-step instructions

- [ ] Document manual migration
  - [ ] For users who want manual control
  - [ ] Backup procedures
  - [ ] Rollback instructions

- [ ] Add troubleshooting section
  - [ ] Common issues during migration
  - [ ] Solutions for each issue
  - [ ] How to verify successful migration

- [ ] Create FAQ
  - [ ] Why is this change needed?
  - [ ] Will my config be preserved?
  - [ ] Can I rollback?
  - [ ] Do I need to reinstall in each Node version?

**File:** `docs/MIGRATION_V1.2.md`

---

### M10.10: Release Preparation

**Priority:** HIGH
**Estimated Time:** 2 hours

- [ ] Version bump
  - [ ] Update version to 1.2.0 in Cargo.toml
  - [ ] Update version in package.json
  - [ ] Update CHANGELOG.md

- [ ] Test release build
  - [ ] Build release binaries for all platforms
  - [ ] Test each binary
  - [ ] Verify GitHub Actions workflow

- [ ] Update release notes
  - [ ] Highlight critical bootstrap fix
  - [ ] Document breaking changes (PATH requirement)
  - [ ] Include migration instructions
  - [ ] Add upgrade instructions

- [ ] Pre-release testing
  - [ ] Test npm tarball locally
  - [ ] Verify postinstall script works
  - [ ] Test on clean VM
  - [ ] Get community beta testers

**Release Checklist:**
```
[ ] All tests passing in CI
[ ] Documentation updated
[ ] Migration guide complete
[ ] CHANGELOG.md updated
[ ] Version numbers bumped
[ ] GitHub release created
[ ] npm package published
[ ] Announcement post prepared
```

---

## Definition of Done

- [ ] xvn binary installed to `~/.xvn/bin/xvn`
- [ ] xvn remains available after switching Node.js versions
- [ ] Setup command adds PATH to shell profile
- [ ] Upgrade from v1.1.x works smoothly
- [ ] Migration is automatic or well-documented
- [ ] All tests passing
- [ ] Documentation complete
- [ ] No "xvn not found" errors after version switches

## Success Metrics

- Zero user reports of "xvn not found after version switch"
- Smooth upgrade experience for existing users (>95% success rate)
- Installation time remains under 10 seconds
- Positive community feedback on bootstrap fix

## Risk Mitigation

### High Risk: Breaking existing installations

**Mitigation:**
- Detect old installations and offer migration
- Keep old version for rollback
- Provide clear rollback instructions
- Test thoroughly before release

### Medium Risk: PATH conflicts

**Mitigation:**
- Prepend `~/.xvn/bin` to PATH (takes precedence)
- Document PATH order importance
- Add verification in setup command

### Low Risk: Symlink issues on Windows

**Mitigation:**
- Test on Windows extensively
- Use directory junctions if symlinks don't work
- Provide alternative installation method

## Dependencies

None - can be implemented immediately

## Timeline

- **Week 1, Days 1-3:** Core infrastructure (M10.1-M10.3)
- **Week 1, Days 4-5:** Testing (M10.5-M10.8)
- **Week 2, Days 6-7:** Documentation and release (M10.9-M10.10)

**Total:** 10-14 days (including buffer for testing and fixes)
