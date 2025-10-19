# Phase 12: Build, Test, and Publish

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 1-2 hours + CI/CD wait time

## Overview

Phase 12 is the **final deployment phase** that builds, tests, and publishes the renamed `anvs` package to npm and Homebrew. This phase is critical because it makes the renamed project publicly available and creates the v2.0.0 release.

**Why Phase 12 is Critical:**
- Makes the `anvs` package available to users for the first time
- Publishes breaking v2.0.0 release with the new name
- Validates that all previous rename work is correct and functional
- Sets up Homebrew tap for `anvs` installation
- Creates official GitHub release for v2.0.0

**⚠️ CHECKPOINT**: Before starting this phase, ensure:
- Phases 0-11 are 100% complete
- All code changes committed and reviewed
- Local testing confirmed working
- No uncommitted changes in working directory

---

## Implementation Tasks

### Task 12.1: Pre-Publication Testing - Full Test Suite

**Goal**: Verify all tests pass with the renamed codebase.

**Commands**:
```bash
# Run complete test suite
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test categories
cargo test --test integration
cargo test --lib
```

**Expected Output**:
```
running 45 tests
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Actions**:
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] No test failures or panics
- [ ] Test coverage remains >85%
- [ ] Verify test output shows `anvs` (not `xvn`) in messages

**Troubleshooting**:
- If tests fail, review the error messages
- Check that all file paths were updated (`.xvn` → `.anvs`)
- Verify environment variables renamed (`XVN_*` → `ANVS_*`)
- Ensure binary name updated in test assertions

---

### Task 12.2: Pre-Publication Testing - Linting

**Goal**: Ensure code quality and no warnings.

**Commands**:
```bash
# Run clippy with strict warnings
cargo clippy -- -D warnings

# Run clippy on all targets
cargo clippy --all-targets -- -D warnings
```

**Expected Output**:
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

**Actions**:
- [ ] Clippy passes with zero warnings
- [ ] No deprecated code usage
- [ ] No unused imports or variables
- [ ] Code quality checks pass

**Troubleshooting**:
- Fix any clippy warnings before proceeding
- Review suggestions for best practices
- Ensure renamed identifiers follow Rust naming conventions

---

### Task 12.3: Pre-Publication Testing - Code Formatting

**Goal**: Verify code is properly formatted.

**Commands**:
```bash
# Check formatting (doesn't modify files)
cargo fmt -- --check

# If changes needed, format code
cargo fmt
```

**Expected Output**:
```
# If properly formatted:
(no output)

# If formatting needed:
Diff in /path/to/file.rs at line X
```

**Actions**:
- [ ] All Rust files properly formatted
- [ ] No formatting violations
- [ ] Code style consistent

---

### Task 12.4: Pre-Publication Testing - Release Build

**Goal**: Build optimized release binary.

**Commands**:
```bash
# Build release binary
cargo build --release

# Check binary size
ls -lh target/release/anvs

# Verify binary exists and is executable
file target/release/anvs
```

**Expected Output**:
```
    Compiling anvs v2.0.0 (/Users/cam/nona-mac/dev/tooling/xvn)
    Finished release [optimized] target(s) in 45.23s

# Binary size should be <5MB compressed
-rwxr-xr-x  1 user  staff   3.2M Oct 19 12:00 target/release/anvs

# File type
target/release/anvs: Mach-O 64-bit executable arm64
```

**Actions**:
- [ ] Release build succeeds
- [ ] Binary created at `target/release/anvs`
- [ ] Binary size is reasonable (<5MB compressed)
- [ ] No build errors or warnings

---

### Task 12.5: Pre-Publication Testing - Local Installation

**Goal**: Install and test the binary locally.

**Commands**:
```bash
# Install from local path
cargo install --path .

# Verify installation
which anvs

# Check installed version
anvs --version

# Verify binary location
ls -lh ~/.anvs/bin/anvs 2>/dev/null || ls -lh ~/.cargo/bin/anvs
```

**Expected Output**:
```
  Installing anvs v2.0.0 (/Users/cam/nona-mac/dev/tooling/xvn)
    Finished release [optimized] target(s) in 0.23s
  Installing /Users/cam/.cargo/bin/anvs

# which anvs
/Users/cam/.cargo/bin/anvs

# anvs --version
anvs 2.0.0
```

**Actions**:
- [ ] Local installation succeeds
- [ ] `anvs` command available in PATH
- [ ] Version shows `2.0.0`
- [ ] Binary works: `anvs --help` shows correct commands

---

### Task 12.6: Pre-Publication Testing - Shell Integration

**Goal**: Test `anvs setup` and shell integration.

**Commands**:
```bash
# Run setup (this will modify your shell profile)
anvs setup

# Check what was added to shell profile
tail -20 ~/.zshrc  # or ~/.bashrc

# Source the profile to activate
source ~/.zshrc  # or source ~/.bashrc

# Verify shell integration loaded
echo $ANVS_SHELL_LOADED
```

**Expected Output**:
```bash
# In ~/.zshrc (or ~/.bashrc):
# ANVS shell integration
[ -s "$HOME/.anvs/bin/anvs.sh" ] && source "$HOME/.anvs/bin/anvs.sh"

# ANVS_SHELL_LOADED should be set
1
```

**Actions**:
- [ ] `anvs setup` completes successfully
- [ ] Shell profile modified correctly
- [ ] `~/.anvsrc` config file created (if not exists)
- [ ] `~/.anvs/bin/anvs.sh` script exists
- [ ] Shell integration loads without errors
- [ ] `ANVS_SHELL_LOADED` environment variable set

**Troubleshooting**:
- If setup fails, check error messages
- Verify `~/.anvs/` directory permissions
- Check that shell script was created: `ls -la ~/.anvs/bin/`
- Review shell profile changes: `cat ~/.zshrc | grep anvs`

---

### Task 12.7: Pre-Publication Testing - Activation Test

**Goal**: Test automatic version switching in a real project.

**Commands**:
```bash
# Create test directory with .nvmrc
mkdir -p /tmp/anvs-test-activation
cd /tmp/anvs-test-activation
echo "22" > .nvmrc

# Trigger activation (via cd in new shell, or manually)
anvs activate

# Check status
anvs status

# Test leaving directory (should return to default)
cd /tmp
anvs activate
```

**Expected Output**:
```bash
# When entering project directory:
Switched to Node.js v22.x.x

# anvs status should show:
ANVS Configuration:
  Config file: ~/.anvsrc
  Active version: 22.x.x
  Last activation: /tmp/anvs-test-activation
```

**Actions**:
- [ ] Manual activation works: `anvs activate`
- [ ] Correct Node.js version activated
- [ ] `anvs status` shows correct information
- [ ] Leaving directory returns to default version
- [ ] No errors in activation process

**Cleanup**:
```bash
# Remove test directory
rm -rf /tmp/anvs-test-activation
```

---

### Task 12.8: Pre-Publication Testing - Auto-Activation Test

**Goal**: Verify automatic activation on `cd`.

**Setup**: This requires shell integration to be active. Open a **new terminal window** to ensure shell hooks are loaded.

**Commands**:
```bash
# In new terminal, create test directory
mkdir -p /tmp/anvs-auto-test
cd /tmp/anvs-auto-test
echo "20" > .nvmrc

# Exit and re-enter directory to trigger auto-activation
cd ..
cd anvs-auto-test

# Should automatically switch to Node.js v20
node --version
```

**Expected Output**:
```bash
# When cd into directory:
Switched to Node.js v20.x.x

# node --version
v20.x.x
```

**Actions**:
- [ ] Auto-activation triggers on `cd`
- [ ] Correct version activated automatically
- [ ] No errors during auto-activation
- [ ] chpwd hook working correctly

**Cleanup**:
```bash
rm -rf /tmp/anvs-auto-test
```

**Troubleshooting**:
- If auto-activation doesn't work:
  - Check `echo $ANVS_SHELL_LOADED` (should be `1`)
  - Verify shell profile: `cat ~/.zshrc | grep anvs`
  - Check shell script exists: `ls ~/.anvs/bin/anvs.sh`
  - Enable debug: `export ANVS_DEBUG=1` and retry
  - Source profile again: `source ~/.zshrc`

---

### Task 12.9: Commit All Changes

**Goal**: Commit all rename changes with comprehensive message.

**Pre-commit Checklist**:
- [ ] All phases 1-11 complete
- [ ] All tests passing
- [ ] Code formatted and linted
- [ ] No uncommitted changes left behind
- [ ] Version is `2.0.0` in package.json and Cargo.toml

**Commands**:
```bash
# Check status
git status

# Review what will be committed
git diff --cached

# If not staged, stage all changes
git add .

# Review changed files
git status

# Create comprehensive commit message
git commit -m "$(cat <<'EOF'
feat: rename xvn to anvs - v2.0.0 breaking change

BREAKING CHANGE: Project renamed from xvn to anvs (Automatic Node Version Switcher)

Changes:
- Package name: @olvrcc/xvn → anvs
- Binary name: xvn → anvs
- Install path: ~/.xvn/ → ~/.anvs/
- Config files: .xvnrc → .anvsrc, .xvn.yaml → .anvs.yaml
- Environment variables: XVN_* → ANVS_*
- Shell script: xvn.sh → anvs.sh
- All documentation updated
- GitHub workflows updated for new artifact names
- Homebrew tap updated (olvrcc/homebrew-anvs)

Migration:
- Existing xvn users must manually migrate (see docs/XVN_TO_ANVS_MIGRATION.md)
- xvn@1.7.0 published with deprecation notice
- Both packages will coexist during transition

Files changed:
- Core config: package.json, Cargo.toml, Cargo.lock
- Installation: install.js, uninstall.js, bin/anvs
- Shell: shell/anvs.sh
- Rust source: src/**/*.rs (all modules)
- Tests: tests/**/*.rs
- Docs: README.md, CLAUDE.md, CHANGELOG.md, CONTRIBUTING.md, docs/**
- Scripts: scripts/*.sh
- CI/CD: .github/workflows/*.yml
- Homebrew: Formula/anvs.rb

See: docs/XVN_TO_ANVS_MIGRATION.md
See: spec/milestone-12-renaming-to-anvs/RENAME_PLAN.md
EOF
)"
```

**Expected Output**:
```
[main abc1234] feat: rename xvn to anvs - v2.0.0 breaking change
 150 files changed, 2500 insertions(+), 2500 deletions(-)
 rename bin/{xvn => anvs} (95%)
 rename shell/{xvn.sh => anvs.sh} (98%)
 ...
```

**Actions**:
- [ ] All changes staged
- [ ] Commit message is comprehensive
- [ ] Commit includes file list
- [ ] Conventional commit format used (`feat:`)
- [ ] Breaking change noted in message
- [ ] Migration guide referenced

---

### Task 12.10: Create Git Tag

**Goal**: Tag the release as v2.0.0.

**Commands**:
```bash
# Create annotated tag
git tag -a v2.0.0 -m "$(cat <<'EOF'
v2.0.0 - Rename to anvs

BREAKING CHANGE: Project renamed from xvn to anvs

This is a major version bump due to the package rename.

Key changes:
- New package name: anvs (unnamespaced on npm)
- New binary name: anvs
- New config paths: ~/.anvsrc, .anvs.yaml
- All xvn references renamed to anvs

Migration guide: docs/XVN_TO_ANVS_MIGRATION.md
Previous package: @olvrcc/xvn@1.7.0 (deprecated)

See full changelog for details.
EOF
)"

# Verify tag created
git tag -l -n9 v2.0.0

# View tag details
git show v2.0.0 --stat
```

**Expected Output**:
```
v2.0.0          v2.0.0 - Rename to anvs

                BREAKING CHANGE: Project renamed from xvn to anvs
                ...
```

**Actions**:
- [ ] Tag `v2.0.0` created
- [ ] Tag is annotated (not lightweight)
- [ ] Tag message includes breaking change notice
- [ ] Tag includes migration guide reference

---

### Task 12.11: Push to GitHub

**Goal**: Push commits and tags to trigger CI/CD.

**Commands**:
```bash
# Push commits first
git push origin main

# Push tag (this triggers CI/CD)
git push origin v2.0.0

# Or push all tags
git push --tags
```

**Expected Output**:
```
Enumerating objects: 200, done.
Counting objects: 100% (200/200), done.
...
To github.com:olvrcc/anvs.git
   abc1234..def5678  main -> main
 * [new tag]         v2.0.0 -> v2.0.0
```

**Actions**:
- [ ] Commits pushed to GitHub
- [ ] Tag v2.0.0 pushed
- [ ] GitHub Actions triggered
- [ ] No push errors

---

### Task 12.12: Monitor CI/CD Build

**Goal**: Wait for GitHub Actions to build all platform binaries.

**Commands**:
```bash
# List recent workflow runs
gh run list --limit 5

# Watch the current run
gh run watch

# View detailed run information
gh run view

# If you need to check specific job
gh run view --log
```

**Expected Platforms**:
- ✅ Linux x86_64 (x86_64-unknown-linux-gnu)
- ✅ Linux ARM64 (aarch64-unknown-linux-gnu)
- ✅ macOS x86_64 (x86_64-apple-darwin)
- ✅ macOS ARM64 (aarch64-apple-darwin)

**Monitoring**:
- [ ] Build workflow triggered (`.github/workflows/build.yml`)
- [ ] All 4 platform builds started
- [ ] Builds progressing without errors
- [ ] Wait for completion (typically 10-15 minutes)

**Expected Output**:
```bash
# gh run watch output:
✓ build-linux-x64    Build and test (x86_64-unknown-linux-gnu)
✓ build-linux-arm64  Build and test (aarch64-unknown-linux-gnu)
✓ build-macos-x64    Build and test (x86_64-apple-darwin)
✓ build-macos-arm64  Build and test (aarch64-apple-darwin)
✓ create-release     Create GitHub Release
```

**Actions**:
- [ ] All platform builds succeed
- [ ] No build failures
- [ ] Artifacts generated for all platforms
- [ ] GitHub Release created automatically

**Troubleshooting**:
- If builds fail:
  - Click on failed job to view logs
  - Check for artifact naming issues (`anvs-*` vs `xvn-*`)
  - Verify Cargo.toml has correct binary name
  - Check shell script path in workflow (should be `shell/anvs.sh`)
  - Verify workflow YAML files updated correctly

---

### Task 12.13: Download Release Artifacts

**Goal**: Download built binaries for all platforms.

**Commands**:
```bash
# Download artifacts for v2.0.0
./scripts/download-artifacts.sh v2.0.0

# Artifacts will be in a temporary directory like:
# /tmp/anvs-v2.0.0-artifacts/

# Check downloaded artifacts
ls -lh /tmp/anvs-v2.0.0-artifacts/
```

**Expected Output**:
```
total 40M
drwxr-xr-x  2 user  wheel   64B Oct 19 12:00 anvs-aarch64-apple-darwin/
drwxr-xr-x  2 user  wheel   64B Oct 19 12:00 anvs-aarch64-unknown-linux-gnu/
drwxr-xr-x  2 user  wheel   64B Oct 19 12:00 anvs-x86_64-apple-darwin/
drwxr-xr-x  2 user  wheel   64B Oct 19 12:00 anvs-x86_64-unknown-linux-gnu/

# Each directory should contain a .tar.gz file:
-rw-r--r--  1 user  wheel   3.2M Oct 19 12:00 anvs-aarch64-apple-darwin.tar.gz
-rw-r--r--  1 user  wheel   3.5M Oct 19 12:00 anvs-aarch64-unknown-linux-gnu.tar.gz
-rw-r--r--  1 user  wheel   3.4M Oct 19 12:00 anvs-x86_64-apple-darwin.tar.gz
-rw-r--r--  1 user  wheel   3.6M Oct 19 12:00 anvs-x86_64-unknown-linux-gnu.tar.gz
```

**Actions**:
- [ ] Download script succeeds
- [ ] All 4 platform artifacts downloaded
- [ ] Archive files present (.tar.gz)
- [ ] File sizes reasonable (3-5MB each)

**Troubleshooting**:
- If download fails:
  - Check GitHub CLI is authenticated: `gh auth status`
  - Verify release exists: `gh release view v2.0.0`
  - List release assets: `gh release view v2.0.0 --json assets`
  - Check artifact names match expected format

---

### Task 12.14: Extract Binaries

**Goal**: Extract binaries from archives and place in `native/` directory.

**Commands**:
```bash
# Extract binaries from downloaded artifacts
./scripts/extract-binaries.sh v2.0.0

# Verify extraction
ls -lh native/*/anvs

# Check binary permissions
file native/*/anvs
```

**Expected Output**:
```
# ls -lh native/*/anvs
-rwxr-xr-x  1 user  wheel   3.2M Oct 19 12:00 native/aarch64-apple-darwin/anvs
-rwxr-xr-x  1 user  wheel   3.5M Oct 19 12:00 native/aarch64-unknown-linux-gnu/anvs
-rwxr-xr-x  1 user  wheel   3.4M Oct 19 12:00 native/x86_64-apple-darwin/anvs
-rwxr-xr-x  1 user  wheel   3.6M Oct 19 12:00 native/x86_64-unknown-linux-gnu/anvs

# file output
native/aarch64-apple-darwin/anvs:          Mach-O 64-bit executable arm64
native/aarch64-unknown-linux-gnu/anvs:     ELF 64-bit LSB executable, ARM aarch64
native/x86_64-apple-darwin/anvs:           Mach-O 64-bit executable x86_64
native/x86_64-unknown-linux-gnu/anvs:      ELF 64-bit LSB executable, x86-64
```

**Actions**:
- [ ] Extraction script succeeds
- [ ] All 4 binaries extracted to `native/` directory
- [ ] Binaries are executable (chmod +x)
- [ ] Correct binary formats for each platform
- [ ] Binary names are `anvs` (not `xvn`)

---

### Task 12.15: Verify Binary Versions

**Goal**: Confirm all binaries report version 2.0.0.

**Commands**:
```bash
# Test each binary (on compatible platform)
for dir in native/*darwin*; do
    echo "=== $dir ==="
    "$dir/anvs" --version
done

# For Linux binaries (if on Linux):
for dir in native/*linux*; do
    echo "=== $dir ==="
    "$dir/anvs" --version
done
```

**Expected Output**:
```
=== native/aarch64-apple-darwin ===
anvs 2.0.0

=== native/x86_64-apple-darwin ===
anvs 2.0.0

=== native/aarch64-unknown-linux-gnu ===
anvs 2.0.0

=== native/x86_64-unknown-linux-gnu ===
anvs 2.0.0
```

**Actions**:
- [ ] All binaries report version `2.0.0`
- [ ] Binary name shows as `anvs` (not `xvn`)
- [ ] No version mismatches
- [ ] At least one binary tested on current platform

---

### Task 12.16: Create npm Package

**Goal**: Create tarball for npm publication.

**Commands**:
```bash
# Create package tarball
npm pack

# Should create: anvs-2.0.0.tgz
ls -lh anvs-2.0.0.tgz

# Verify package contents
tar -tzf anvs-2.0.0.tgz | head -30

# Check for critical files
tar -tzf anvs-2.0.0.tgz | grep -E "(package.json|bin/anvs|native/|install.js|README.md)"
```

**Expected Output**:
```
# Package size
-rw-r--r--  1 user  wheel    15M Oct 19 12:00 anvs-2.0.0.tgz

# Package contents (excerpt):
package/package.json
package/bin/anvs
package/install.js
package/uninstall.js
package/README.md
package/CHANGELOG.md
package/LICENSE
package/native/aarch64-apple-darwin/anvs
package/native/aarch64-unknown-linux-gnu/anvs
package/native/x86_64-apple-darwin/anvs
package/native/x86_64-unknown-linux-gnu/anvs
package/shell/anvs.sh
...
```

**Actions**:
- [ ] Tarball created: `anvs-2.0.0.tgz`
- [ ] Package size reasonable (<20MB)
- [ ] All binaries included in package
- [ ] package.json included
- [ ] bin/anvs wrapper included
- [ ] install.js and uninstall.js included
- [ ] Shell script included (shell/anvs.sh)
- [ ] Documentation included (README.md, etc.)
- [ ] No `xvn` references in package (spot check)

---

### Task 12.17: Verify Package Contents

**Goal**: Deep inspection of package contents.

**Commands**:
```bash
# Extract to temporary location for inspection
mkdir -p /tmp/anvs-package-verify
cd /tmp/anvs-package-verify
tar -xzf ~/path/to/anvs-2.0.0.tgz

# Check package.json
cat package/package.json | grep -E "(name|version|bin)"

# Verify no xvn references
grep -r "xvn" package/ --exclude-dir=node_modules 2>/dev/null || echo "✅ No xvn references found"

# Check binary wrapper
cat package/bin/anvs | grep -E "(ANVS_BINARY|.anvs)"

# Verify shell script
cat package/shell/anvs.sh | grep -E "(ANVS_|__anvs_)"

# Check for XVN environment variables (should not exist)
grep -r "XVN_" package/ --exclude-dir=node_modules 2>/dev/null && echo "❌ Found XVN_ references" || echo "✅ No XVN_ references"
```

**Actions**:
- [ ] package.json has name "anvs"
- [ ] package.json has version "2.0.0"
- [ ] package.json bin points to "anvs"
- [ ] No references to "xvn" in code
- [ ] No references to "XVN_" environment variables
- [ ] Binary wrapper uses ANVS_ variables
- [ ] Shell script uses ANVS_ variables and __anvs_ functions
- [ ] Config paths use .anvs (not .xvn)

**Cleanup**:
```bash
rm -rf /tmp/anvs-package-verify
cd ~/path/to/project
```

---

### Task 12.18: Publish to npm

**Goal**: Publish `anvs` package to npm registry.

**Pre-publication Checklist**:
- [ ] Logged into npm: `npm whoami`
- [ ] Package verified (Task 12.17 complete)
- [ ] Version is 2.0.0
- [ ] No test failures
- [ ] Ready for public release

**Commands**:
```bash
# Verify npm login
npm whoami

# If not logged in:
# npm login

# Publish package (requires 2FA)
npm publish --otp=<YOUR_2FA_CODE>

# Note: Since this is unnamespaced, no --access public needed
# (unless you've published scoped packages before and npm defaults to restricted)

# If needed:
# npm publish --access public --otp=<YOUR_2FA_CODE>
```

**Expected Output**:
```
npm notice
npm notice 📦  anvs@2.0.0
npm notice === Tarball Contents ===
npm notice 150 files, 15.2MB unpacked
npm notice === Tarball Details ===
npm notice name:          anvs
npm notice version:       2.0.0
npm notice filename:      anvs-2.0.0.tgz
npm notice package size:  14.5 MB
npm notice unpacked size: 15.2 MB
npm notice total files:   150
npm notice
+ anvs@2.0.0
```

**Actions**:
- [ ] npm publish succeeds
- [ ] Package published as `anvs` (unnamespaced)
- [ ] Version 2.0.0 shows in npm registry
- [ ] No publication errors

**⚠️ Important Notes**:
- This cannot be easily undone - npm discourages unpublishing
- Ensure everything is correct before publishing
- Have 2FA codes ready
- Double-check package name and version

---

### Task 12.19: Verify npm Publication

**Goal**: Confirm package is accessible on npm.

**Commands**:
```bash
# View package on npm
npm view anvs

# Check specific version
npm view anvs@2.0.0

# View package metadata
npm view anvs version
npm view anvs description
npm view anvs bin

# Check package page
echo "Visit: https://www.npmjs.com/package/anvs"
```

**Expected Output**:
```
anvs@2.0.0 | MIT | deps: none | versions: 1
Automatic Node Version Switcher for Node.js

https://github.com/olvrcc/anvs#readme

dist
.tarball: https://registry.npmjs.org/anvs/-/anvs-2.0.0.tgz
.shasum: abc123...
.integrity: sha512-...

bin: anvs
```

**Web Verification**:
Visit https://www.npmjs.com/package/anvs and verify:
- [ ] Package page loads
- [ ] Version shows as `2.0.0`
- [ ] README displays correctly
- [ ] No deprecation warnings
- [ ] Installation command shown: `npm install -g anvs`
- [ ] Repository link correct: github.com/olvrcc/anvs

**Actions**:
- [ ] `npm view anvs` shows version 2.0.0
- [ ] Package metadata correct
- [ ] README visible on npm
- [ ] Links work correctly
- [ ] Package publicly accessible

---

### Task 12.20: Test npm Installation

**Goal**: Verify installation from npm works.

**Commands**:
```bash
# Install from npm in a clean environment
# (Best done in a container or separate user account)

# Remove any local anvs installation first
which anvs && cargo uninstall anvs

# Install from npm
npm install -g anvs

# Verify installation
which anvs
anvs --version
anvs --help

# Check binary location
ls -lh $(which anvs)

# Verify it's the npm version (should be in ~/.npm or similar)
readlink -f $(which anvs)
```

**Expected Output**:
```
# npm install -g anvs
added 1 package in 5s

# which anvs
/Users/username/.nvm/versions/node/v22.20.0/bin/anvs

# anvs --version
anvs 2.0.0

# Binary should link to npm global install
/Users/username/.nvm/versions/node/v22.20.0/lib/node_modules/anvs/bin/anvs
```

**Full Installation Test**:
```bash
# Run setup
anvs setup

# Verify setup created config
ls -la ~/.anvsrc

# Verify shell integration
cat ~/.zshrc | grep anvs

# Test in new shell
# (open new terminal window)
cd /tmp
mkdir anvs-npm-test
cd anvs-npm-test
echo "20" > .nvmrc

# Should trigger activation
# Test activation
anvs activate
```

**Actions**:
- [ ] npm installation succeeds
- [ ] `anvs` command available globally
- [ ] Version is 2.0.0
- [ ] `anvs setup` works
- [ ] Config file created
- [ ] Shell integration works
- [ ] Activation works in test directory

**Cleanup**:
```bash
rm -rf /tmp/anvs-npm-test
```

---

### Task 12.21: Update Homebrew Tap

**Goal**: Update Homebrew formula for `anvs`.

**Background**: GitHub Actions should automatically update the Homebrew tap when a release is created. Verify this happened, or run manually if needed.

**Verify Automatic Update**:
```bash
# Check Homebrew update workflow
gh run list --workflow=update-homebrew.yml --limit 3

# View the run for v2.0.0
gh run view <run-id>

# Check homebrew-anvs repository
cd /path/to/homebrew-anvs
git pull
cat Formula/anvs.rb
```

**If Automatic Update Worked**:
- [ ] Workflow ran successfully
- [ ] Formula/anvs.rb updated with v2.0.0
- [ ] Download URLs point to v2.0.0 release assets
- [ ] SHA256 checksums updated

**If Manual Update Needed**:
```bash
# Run setup script
./scripts/setup-homebrew-tap.sh

# This should:
# 1. Download release artifacts
# 2. Calculate SHA256 checksums
# 3. Update Formula/anvs.rb
# 4. Commit and push to homebrew-anvs repo
```

**Actions**:
- [ ] Homebrew tap workflow triggered
- [ ] Formula updated to v2.0.0
- [ ] SHA256 checksums calculated
- [ ] Formula committed to homebrew-anvs repo

---

### Task 12.22: Verify Homebrew Formula

**Goal**: Inspect and test the Homebrew formula.

**Commands**:
```bash
# Clone or navigate to homebrew-anvs repo
cd /path/to/homebrew-anvs

# View formula
cat Formula/anvs.rb

# Check for critical elements
grep "class Anvs" Formula/anvs.rb
grep "url.*v2.0.0" Formula/anvs.rb
grep "sha256" Formula/anvs.rb
grep "bin.install.*anvs" Formula/anvs.rb
```

**Expected Formula Structure**:
```ruby
class Anvs < Formula
  desc "Automatic Node Version Switcher for Node.js"
  homepage "https://github.com/olvrcc/anvs"
  url "https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-x86_64-apple-darwin.tar.gz"
  sha256 "abc123..."  # Checksum for x86_64-apple-darwin
  version "2.0.0"

  # Platform-specific downloads
  if Hardware::CPU.arm?
    url "https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-aarch64-apple-darwin.tar.gz"
    sha256 "def456..."
  end

  def install
    bin.install "anvs"
    # Shell integration
    (prefix/"shell").install "anvs.sh"
  end

  test do
    assert_match "anvs 2.0.0", shell_output("#{bin}/anvs --version")
  end
end
```

**Actions**:
- [ ] Class name is `Anvs` (not `Xvn`)
- [ ] Version is `2.0.0`
- [ ] URLs point to `anvs` release assets (not `xvn`)
- [ ] SHA256 checksums present for all platforms
- [ ] Binary name is `anvs`
- [ ] Test command uses `anvs`
- [ ] Formula valid Ruby syntax: `ruby -c Formula/anvs.rb`

---

### Task 12.23: Test Homebrew Installation

**Goal**: Install and test via Homebrew.

**⚠️ Note**: This test will modify your system. Consider using a VM or test environment.

**Commands**:
```bash
# Add tap (if not already added)
brew tap olvrcc/anvs

# Update tap
brew update

# Install anvs
brew install anvs

# Verify installation
which anvs
anvs --version

# Check where it was installed
brew --prefix anvs
ls -lh $(brew --prefix anvs)/bin/anvs

# Test setup
anvs setup

# Check shell integration
cat ~/.zshrc | grep anvs
```

**Expected Output**:
```
# brew install anvs
==> Downloading https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-...
==> Installing anvs from olvrcc/anvs
🍺  /usr/local/Cellar/anvs/2.0.0: 3 files, 3.2MB, built in 2 seconds

# which anvs
/usr/local/bin/anvs

# anvs --version
anvs 2.0.0
```

**Actions**:
- [ ] Tap added successfully
- [ ] `brew install anvs` succeeds
- [ ] Binary installed to Homebrew location
- [ ] Version is 2.0.0
- [ ] `anvs setup` works
- [ ] Shell integration configured

**Test Uninstall** (optional):
```bash
# Uninstall via Homebrew
brew uninstall anvs

# Should remove binary
which anvs  # Should return nothing (unless npm version still installed)
```

---

### Task 12.24: Verify GitHub Release

**Goal**: Check that GitHub Release was created properly.

**Commands**:
```bash
# View release
gh release view v2.0.0

# View release on web
echo "Visit: https://github.com/olvrcc/anvs/releases/tag/v2.0.0"

# List release assets
gh release view v2.0.0 --json assets -q '.assets[].name'
```

**Expected Assets**:
```
anvs-aarch64-apple-darwin.tar.gz
anvs-aarch64-unknown-linux-gnu.tar.gz
anvs-x86_64-apple-darwin.tar.gz
anvs-x86_64-unknown-linux-gnu.tar.gz
```

**Web Verification**:
Visit https://github.com/olvrcc/anvs/releases/tag/v2.0.0 and verify:
- [ ] Release title: "v2.0.0"
- [ ] Release marked as "Latest"
- [ ] All 4 platform binaries attached as assets
- [ ] Asset names correct (anvs-*, not xvn-*)
- [ ] Release notes present (auto-generated or manual)
- [ ] No draft or pre-release status

**Actions**:
- [ ] GitHub Release created
- [ ] All platform assets present
- [ ] Asset names correct
- [ ] Release is public and marked as latest

---

### Task 12.25: Update Release Notes

**Goal**: Add detailed release notes to GitHub Release.

**Edit the release** on GitHub or via CLI:

```bash
gh release edit v2.0.0 --notes "$(cat <<'EOF'
# v2.0.0 - Rename to ANVS 🎉

**BREAKING CHANGE**: This project has been renamed from `xvn` to `anvs` (Automatic Node Version Switcher).

## What's New

- **New Package Name**: Now available as `anvs` (unnamespaced on npm)
- **Better Name**: "Automatic Node Version Switcher" clearly describes the tool's purpose
- **Same Great Features**: All functionality from xvn carried over
- **Improved Discoverability**: Easier to find on npm and Homebrew

## Breaking Changes

This is a major version bump due to the rename. Changes include:

| Old (xvn)                | New (anvs)               |
|--------------------------|--------------------------|
| `@olvrcc/xvn`            | `anvs`                   |
| `xvn` binary             | `anvs` binary            |
| `~/.xvn/` directory      | `~/.anvs/` directory     |
| `~/.xvnrc` config        | `~/.anvsrc` config       |
| `.xvn.yaml` project file | `.anvs.yaml` project file|
| `XVN_*` env vars         | `ANVS_*` env vars        |

## Installation

### New Users

**npm**:
```bash
npm install -g anvs
anvs setup
```

**Homebrew**:
```bash
brew install olvrcc/anvs/anvs
anvs setup
```

### Migrating from XVN

See the [Migration Guide](https://github.com/olvrcc/anvs/blob/main/docs/XVN_TO_ANVS_MIGRATION.md) for detailed instructions.

**Quick migration**:
```bash
# Uninstall old xvn
xvn uninstall

# Install new anvs
npm install -g anvs
anvs setup

# Copy config (if you had custom settings)
cp ~/.xvnrc ~/.anvsrc
```

## What Happened to XVN?

- The old `@olvrcc/xvn` package remains available and will continue to work
- Version 1.7.0 includes a deprecation notice
- No new features will be added to `xvn`
- All future development will be on `anvs`

## Support

- **Documentation**: https://github.com/olvrcc/anvs
- **Issues**: https://github.com/olvrcc/anvs/issues
- **Migration Help**: https://github.com/olvrcc/anvs/blob/main/docs/XVN_TO_ANVS_MIGRATION.md

## Full Changelog

See [CHANGELOG.md](https://github.com/olvrcc/anvs/blob/main/CHANGELOG.md) for detailed changes.

---

Thank you for using xvn! We're excited to see you on anvs! 🚀
EOF
)"
```

**Actions**:
- [ ] Release notes updated
- [ ] Breaking changes clearly documented
- [ ] Installation instructions included
- [ ] Migration guide linked
- [ ] Old package status explained

---

## Verification Checklist

Before proceeding to Phase 13, verify ALL of the following:

### Testing
- [ ] All tests pass: `cargo test`
- [ ] Linting passes: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt -- --check`
- [ ] Release build succeeds
- [ ] Local installation works
- [ ] Shell integration works
- [ ] Manual activation works
- [ ] Auto-activation works (cd trigger)

### Git & CI/CD
- [ ] All changes committed
- [ ] Git tag v2.0.0 created
- [ ] Changes pushed to GitHub
- [ ] Tag pushed to GitHub
- [ ] GitHub Actions workflows passed
- [ ] All 4 platform builds succeeded
- [ ] Artifacts downloaded successfully
- [ ] Binaries extracted and verified

### npm
- [ ] Package created: `anvs-2.0.0.tgz`
- [ ] Package contents verified (no xvn references)
- [ ] Published to npm: `npm publish` succeeded
- [ ] Package visible: https://www.npmjs.com/package/anvs
- [ ] npm installation works: `npm install -g anvs`
- [ ] Installed package works correctly

### Homebrew
- [ ] Homebrew workflow ran (or manual update done)
- [ ] Formula updated to v2.0.0
- [ ] Formula syntax valid: `ruby -c Formula/anvs.rb`
- [ ] SHA256 checksums correct
- [ ] Homebrew installation works: `brew install anvs`
- [ ] Installed binary works correctly

### GitHub
- [ ] GitHub Release created for v2.0.0
- [ ] All platform assets attached
- [ ] Asset names correct (anvs-*, not xvn-*)
- [ ] Release notes comprehensive
- [ ] Release marked as latest

### Final Checks
- [ ] `anvs --version` shows `2.0.0` (from all sources)
- [ ] No references to `xvn` in published package
- [ ] No references to `XVN_` env vars in published package
- [ ] Migration guide available and linked
- [ ] Documentation accurate

---

## Success Criteria

Phase 12 is complete when:

1. ✅ All tests passing locally
2. ✅ Version 2.0.0 built and tested
3. ✅ Committed and tagged as v2.0.0
4. ✅ GitHub Actions built all platform binaries successfully
5. ✅ Published to npm as `anvs@2.0.0`
6. ✅ npm installation verified working
7. ✅ Homebrew formula updated and tested
8. ✅ GitHub Release created with comprehensive notes
9. ✅ All verification checklist items passed
10. ✅ No `xvn` references in published artifacts

---

## Next Steps

After completing Phase 12:

1. **Monitor npm downloads**: Check that users can install successfully
2. **Monitor GitHub issues**: Watch for installation or migration problems
3. **Test in production**: Try installing on a fresh system
4. **Proceed to Phase 13**: Final deprecation and announcements

**Proceed to Phase 13**: [Final Deprecation & Announcement](./phase-13.md) (to be created)

---

## Rollback Plan

If critical issues are discovered after publication:

### For npm
1. **Do not unpublish** - npm discourages unpublishing
2. **Fix the issue** in the codebase
3. **Publish patch version**: v2.0.1 with fixes
4. **Deprecate broken version**: `npm deprecate anvs@2.0.0 "Issue found, use 2.0.1"`

### For Homebrew
1. **Update formula** to point to fixed version
2. **Push update** to homebrew-anvs repository
3. **Users can update**: `brew upgrade anvs`

### For GitHub
1. **Create new release** with fixes (v2.0.1)
2. **Update release notes** to reference the fix
3. **Keep old release** for transparency

### Emergency Rollback
If package is completely broken:
1. Publish emergency patch (v2.0.1) that fixes critical issue
2. Add deprecation notice to v2.0.0: `npm deprecate anvs@2.0.0 "Broken, use 2.0.1"`
3. Update Homebrew formula immediately
4. Post issue on GitHub explaining the problem

---

## Common Issues & Troubleshooting

### Issue: Tests fail after rename
**Solution**:
- Check that all file paths updated (`.xvn` → `.anvs`)
- Verify environment variables renamed (`XVN_*` → `ANVS_*`)
- Search for any remaining `xvn` references: `grep -r "xvn" src/ tests/`

### Issue: CI/CD builds fail
**Solution**:
- Check workflow YAML files for artifact names (`anvs-*` not `xvn-*`)
- Verify Cargo.toml has `[[bin]] name = "anvs"`
- Check shell script path in workflow: `shell/anvs.sh`
- Review build logs for specific errors

### Issue: npm publish fails
**Solution**:
- Verify logged in: `npm whoami`
- Check package name not taken: `npm view anvs` (should 404 before first publish)
- Ensure 2FA code is fresh (<30 seconds old)
- Check package.json has correct name and version

### Issue: Homebrew formula broken
**Solution**:
- Verify SHA256 checksums are correct
- Test formula syntax: `ruby -c Formula/anvs.rb`
- Check download URLs are accessible
- Ensure binary name is `anvs` in install section

### Issue: Package contains xvn references
**Solution**:
- Search and replace any remaining references
- Rebuild package: `npm pack`
- Verify contents: `tar -tzf anvs-2.0.0.tgz | xargs tar -xzf anvs-2.0.0.tgz -O | grep -i xvn`
- If found, fix and republish as patch version

---

## Notes

- **This is the public launch** - ensure everything is perfect before publishing
- **Cannot easily undo npm publish** - double-check everything
- **Test in multiple environments** if possible (different OS, Node versions)
- **Keep old xvn package available** - users rely on it during transition
- **Monitor closely after launch** - be ready to fix issues quickly
- **Communicate clearly** - update all documentation and help users migrate
- **Phase 12 is critical** - take time to verify each step thoroughly
