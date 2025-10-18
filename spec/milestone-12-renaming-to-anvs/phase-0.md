# Phase 0: Final XVN Release (Deprecation Notice)

**Status**: Not Started
**Version**: v1.7.0
**Duration Estimate**: 30-45 minutes + CI/CD time

## Overview

Phase 0 is the **critical first step** before any renaming work begins. This phase publishes a final version of `@olvrcc/xvn` that notifies existing users about the upcoming rename to `anvs` and provides clear migration guidance.

**Why Phase 0 is Essential:**
- Gives existing users advance notice before the breaking change
- Provides a clear migration path to the new package
- Ensures the old package remains functional indefinitely
- Allows graceful deprecation rather than abrupt cutoff

**⚠️ CHECKPOINT**: Do not proceed to Phase 1 until Phase 0 is complete and published to npm!

---

## Implementation Tasks

### Task 0.1: Create Deprecation README

**File**: `README.DEPRECATION.md` (new file)

**Content Requirements**:
```markdown
# XVN Has Been Renamed to ANVS

⚠️ **IMPORTANT NOTICE**: This project has been renamed to **anvs** (Automatic Node Version Switcher)

## Why the Rename?

- **Better Name**: "Automatic Node Version Switcher" is more descriptive
- **Unnamespaced Package**: `anvs` is available without namespace (vs `@olvrcc/xvn`)
- **Clearer Purpose**: Name immediately communicates what the tool does
- **Tribute to avn**: Pays homage to the original project while being distinct

## What's Changing?

| Old (xvn)                | New (anvs)               |
|--------------------------|--------------------------|
| `@olvrcc/xvn`            | `anvs`                   |
| `xvn` binary             | `anvs` binary            |
| `~/.xvn/` directory      | `~/.anvs/` directory     |
| `~/.xvnrc` config        | `~/.anvsrc` config       |
| `.xvn.yaml` project file | `.anvs.yaml` project file|

## Migration Instructions

### For New Users
Install the new package directly:
```bash
npm install -g anvs
anvs setup
```

### For Existing XVN Users

1. **Backup your config** (optional but recommended):
   ```bash
   cp ~/.xvnrc ~/.xvnrc.backup
   ```

2. **Uninstall xvn**:
   ```bash
   xvn uninstall
   # or manually: npm uninstall -g @olvrcc/xvn
   ```

3. **Install anvs**:
   ```bash
   npm install -g anvs
   anvs setup
   ```

4. **Migrate config** (if you had custom settings):
   - Copy settings from `~/.xvnrc.backup` to `~/.anvsrc`
   - Rename project-level `.xvn.yaml` to `.anvs.yaml`

## Timeline

- **v1.7.0** (Current): Final xvn release with deprecation notice
- **v2.0.0**: New `anvs` package published (coming soon)
- **Ongoing**: Both packages will coexist during transition period

## Support

- New package: https://github.com/olvrcc/anvs
- Migration help: https://github.com/olvrcc/anvs/issues
- Full migration guide: [Link will be added when published]

## Will XVN Stop Working?

**No!** Your current `xvn` installation will continue to work indefinitely. However:
- No new features will be added to `xvn`
- Bug fixes and updates will only go to `anvs`
- We recommend migrating when convenient

---

*Thank you for using xvn! We look forward to seeing you on anvs.*
```

**Actions**:
- [ ] Create `README.DEPRECATION.md` with above content
- [ ] Adjust wording as needed for clarity
- [ ] Commit: `docs: add deprecation notice document`

---

### Task 0.2: Update Current README

**File**: `README.md`

**Changes Required**:

1. **Add deprecation banner at the very top** (before title):
   ```markdown
   > **⚠️ DEPRECATION NOTICE**: This package has been renamed to [`anvs`](https://www.npmjs.com/package/anvs) (Automatic Node Version Switcher).
   > Please install the new package: `npm install -g anvs`
   > See [README.DEPRECATION.md](./README.DEPRECATION.md) for migration instructions.
   > This package will continue to work but won't receive updates.

   ---
   ```

2. **Add migration section** (after Installation section):
   ```markdown
   ## Migration to ANVS

   This project has been renamed to **anvs**. For migration instructions, see [README.DEPRECATION.md](./README.DEPRECATION.md).

   **Quick migration**:
   ```bash
   xvn uninstall
   npm install -g anvs
   anvs setup
   ```
   ```

3. **Keep all other documentation intact** - do not remove or modify existing sections

**Actions**:
- [ ] Add deprecation banner at top of README.md
- [ ] Add migration section after Installation
- [ ] Verify banner will be visible on npm package page
- [ ] Commit: `docs: add deprecation notice to README`

---

### Task 0.3: Update CHANGELOG

**File**: `CHANGELOG.md`

**Add new entry at the top**:
```markdown
## [1.7.0] - 2025-10-18

### ⚠️ Deprecation Notice

This is the final release of `xvn` before the project is renamed to `anvs` (Automatic Node Version Switcher).

**What's happening:**
- Project is being renamed to `anvs` for better npm namespace and clarity
- New package will be published as `anvs` (unnamespaced)
- This version (`@olvrcc/xvn@1.7.0`) will continue to work indefinitely
- No new features will be added to `xvn`; future development on `anvs`

**For users:**
- See [README.DEPRECATION.md](./README.DEPRECATION.md) for full migration guide
- Install new package: `npm install -g anvs`
- Your current installation will keep working

### Documentation
- Added deprecation notice to README.md
- Created README.DEPRECATION.md with migration guide
- Updated package metadata to indicate deprecation

---
```

**Actions**:
- [ ] Add v1.7.0 entry to CHANGELOG.md
- [ ] Commit: `docs: add v1.7.0 changelog entry for deprecation`

---

### Task 0.4: Version Bump

**Files**: `package.json`, `Cargo.toml`, `Cargo.lock`

**Version Changes**:
- Current: `1.6.2`
- Target: `1.7.0`

**Commands**:
```bash
# Use version bump script
./scripts/bump-version.sh minor

# Or manually:
# 1. Update package.json version to "1.7.0"
# 2. Update Cargo.toml version to "1.7.0"
# 3. Rebuild to update Cargo.lock
cargo build --release
```

**Actions**:
- [ ] Bump version to `1.7.0` in package.json
- [ ] Bump version to `1.7.0` in Cargo.toml
- [ ] Run `cargo build --release` to update Cargo.lock
- [ ] Verify build succeeds
- [ ] Commit: `chore: bump version to v1.7.0`

---

### Task 0.5: Build and Test Locally

**Pre-publication verification**:

```bash
# Run tests
cargo test

# Run linting
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check

# Build release binary
cargo build --release

# Test binary
./target/release/xvn --version  # Should show 1.7.0

# Install locally and test
cargo install --path .
xvn --version  # Should show 1.7.0
```

**Actions**:
- [ ] All tests pass
- [ ] Clippy passes with no warnings
- [ ] Code is properly formatted
- [ ] Binary builds successfully
- [ ] Version displays as `1.7.0`

---

### Task 0.6: Git Tag and Push

**Create annotated tag and push**:

```bash
# Stage all changes
git add .

# Verify what's staged
git status

# Create git tag
git tag -a v1.7.0 -m "v1.7.0 - Deprecation notice for rename to anvs"

# Push commits
git push

# Push tag (triggers CI/CD)
git push --tags
```

**Actions**:
- [ ] Commit message: `chore: prepare v1.7.0 release with deprecation notice`
- [ ] Create tag: `v1.7.0`
- [ ] Push commits to GitHub
- [ ] Push tag to GitHub (triggers CI/CD)

---

### Task 0.7: Monitor CI/CD Build

**Wait for GitHub Actions to complete**:

```bash
# Monitor workflow status
gh run list --limit 5

# Watch specific run
gh run watch

# View workflow logs if needed
gh run view
```

**Verify**:
- [ ] All platform builds succeed (Linux x64/arm64, macOS x64/arm64)
- [ ] Artifacts are generated
- [ ] GitHub Release is created
- [ ] No build errors

**Estimated wait time**: 10-15 minutes

---

### Task 0.8: Download and Verify Artifacts

**Download release artifacts**:

```bash
# Download artifacts for v1.7.0
./scripts/download-artifacts.sh v1.7.0

# Extract binaries
./scripts/extract-binaries.sh v1.7.0

# Verify binaries exist
ls -lh native/*/xvn

# Check binary versions
for dir in native/*; do
    echo "=== $dir ==="
    "$dir/xvn" --version
done
```

**Expected output**: All binaries should report version `1.7.0`

**Actions**:
- [ ] Artifacts downloaded successfully
- [ ] Binaries extracted to `native/` directory
- [ ] All platform binaries present (4 total)
- [ ] All binaries report version `1.7.0`

---

### Task 0.9: Create and Verify npm Package

**Create package tarball**:

```bash
# Create tarball
npm pack

# Should create: olvrcc-xvn-1.7.0.tgz
ls -lh *.tgz

# Verify package contents
tar -tzf olvrcc-xvn-1.7.0.tgz | head -20

# Check that README.DEPRECATION.md is included
tar -tzf olvrcc-xvn-1.7.0.tgz | grep DEPRECATION

# Check that native binaries are included
tar -tzf olvrcc-xvn-1.7.0.tgz | grep native/
```

**Actions**:
- [ ] Tarball created successfully
- [ ] README.DEPRECATION.md included in package
- [ ] README.md includes deprecation notice
- [ ] Native binaries included
- [ ] package.json shows version 1.7.0

---

### Task 0.10: Publish to npm

**Publish the package**:

```bash
# Make sure you're logged in to npm
npm whoami

# Publish with 2FA code
npm publish --otp=<YOUR_2FA_CODE>

# If scoped package, ensure it's public:
# npm publish --access public --otp=<YOUR_2FA_CODE>
```

**Expected output**:
```
+ @olvrcc/xvn@1.7.0
```

**Actions**:
- [ ] npm publish succeeds
- [ ] Version 1.7.0 appears in npm registry
- [ ] No errors during publication

---

### Task 0.11: Post-Publication Verification

**Verify package is accessible**:

```bash
# View package on npm
npm view @olvrcc/xvn

# Check specific version
npm view @olvrcc/xvn@1.7.0

# Check latest version
npm view @olvrcc/xvn version

# Test installation in a temporary directory
cd /tmp
npm install -g @olvrcc/xvn
xvn --version  # Should show 1.7.0
```

**Verify on npm website**:
- [ ] Visit https://www.npmjs.com/package/@olvrcc/xvn
- [ ] Deprecation notice visible in README
- [ ] Version shows as `1.7.0`
- [ ] README.DEPRECATION.md link works
- [ ] Package metadata correct

**Test functionality**:
- [ ] Install succeeds: `npm install -g @olvrcc/xvn`
- [ ] Binary works: `xvn --version` shows `1.7.0`
- [ ] Setup works: `xvn setup` (test in clean shell)
- [ ] Activation works: Test in directory with `.nvmrc`

---

### Task 0.12: Update GitHub Release Notes

**Edit the v1.7.0 release on GitHub**:

1. Go to: https://github.com/olvrcc/xvn/releases/tag/v1.7.0
2. Edit release notes to include:

```markdown
# v1.7.0 - Final Release (Deprecation Notice)

⚠️ **IMPORTANT**: This is the final release of `xvn` before the project is renamed to `anvs` (Automatic Node Version Switcher).

## Deprecation Notice

This package is being renamed to provide:
- Better package name: `anvs` (unnamespaced on npm)
- Clearer purpose: "Automatic Node Version Switcher"
- Improved discoverability

## For Existing Users

**Your current installation will continue to work!** However, future updates will only be published to the new `anvs` package.

### Migration Instructions

See [README.DEPRECATION.md](./README.DEPRECATION.md) for full details.

**Quick migration**:
```bash
xvn uninstall
npm install -g anvs
anvs setup
```

## Changes in v1.7.0

### Documentation
- Added deprecation notice to README.md
- Created README.DEPRECATION.md with migration guide
- Updated package metadata

## Links

- New package: https://www.npmjs.com/package/anvs (coming soon)
- Migration guide: [README.DEPRECATION.md](./README.DEPRECATION.md)
- Support: https://github.com/olvrcc/anvs/issues

---

*Thank you for using xvn! We look forward to seeing you on anvs.*
```

**Actions**:
- [ ] Edit GitHub release notes
- [ ] Add deprecation notice
- [ ] Include migration instructions
- [ ] Add links to new package (update when available)

---

## Verification Checklist

Before proceeding to Phase 1, verify ALL of the following:

- [ ] `@olvrcc/xvn@1.7.0` published to npm
- [ ] Package page shows deprecation notice prominently
- [ ] README.DEPRECATION.md visible on npm and GitHub
- [ ] Installation works: `npm install -g @olvrcc/xvn`
- [ ] Binary version shows `1.7.0`: `xvn --version`
- [ ] GitHub release v1.7.0 exists with updated notes
- [ ] CI/CD workflow passed successfully
- [ ] All platform binaries built and included in npm package
- [ ] Documentation accurate and links work
- [ ] Package will remain available (do not unpublish)

---

## Success Criteria

Phase 0 is complete when:

1. ✅ Version 1.7.0 is published to npm as `@olvrcc/xvn@1.7.0`
2. ✅ Deprecation notice is prominently displayed on package page
3. ✅ Migration guide is available and clear
4. ✅ Package installation and functionality verified
5. ✅ GitHub release notes updated
6. ✅ No breaking changes introduced (100% backward compatible)

---

## Next Steps

After completing Phase 0:

1. **Wait period** (optional): Consider waiting 1-2 weeks to allow users to discover the deprecation notice
2. **Monitor feedback**: Check for questions or issues about migration
3. **Update migration guide**: Address any common questions
4. **Proceed to Phase 1**: Begin actual rename work on a new branch

---

## Rollback Plan

If issues are discovered after publication:

1. **Do not unpublish** - npm discourages unpublishing
2. **Fix and republish**: Create v1.7.1 with fixes
3. **Update deprecation message**: Correct any misinformation
4. **Communicate changes**: Update GitHub release notes

---

## Notes

- This phase is **non-breaking** - all existing functionality remains unchanged
- Only documentation and metadata are updated
- Existing users can continue using v1.7.0 indefinitely
- This provides a graceful transition path to the new package name
- Phase 0 must be complete before any rename work begins
