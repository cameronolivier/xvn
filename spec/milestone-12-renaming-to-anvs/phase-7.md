# Phase 7: Build & Release Scripts

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes
**Depends On**: Phases 0-6 (all code and documentation) completed

## Overview

Phase 7 updates all build, release, and version management scripts in the `scripts/` directory. These scripts are critical infrastructure for the release process and must be updated to use the new `anvs` naming for artifacts, binaries, repository URLs, and Homebrew tap references.

**Why Phase 7 is Critical:**
- Release scripts orchestrate the entire v2.0.0 publication process
- Artifact naming affects GitHub Actions downloads and npm packaging
- Repository URLs must point to the renamed GitHub repository
- Homebrew tap scripts create and update the formula in the new tap
- Version management scripts are used throughout development and release

**⚠️ IMPORTANT**: This phase must be completed BEFORE attempting to publish v2.0.0, as these scripts are required for the release process.

**📋 Script Scope:**
- 5 shell scripts in `scripts/` directory
- 1 documentation file (`scripts/README.md` already updated in Phase 6-3)
- Repository URL updates
- Artifact and binary name changes
- Homebrew tap references

---

## Implementation Tasks

### Task 7.1: Update scripts/bump-version.sh

**File**: `scripts/bump-version.sh`

**Changes Required**:

This script bumps version numbers across the codebase. While it primarily works with version numbers rather than package names, it may contain comments or echo statements that reference the project name.

1. **Review the script** for any project name references:
   ```bash
   grep -n "xvn" scripts/bump-version.sh
   ```

2. **If found**, update comments and echo statements to use "anvs"

**Note**: This script primarily modifies version numbers in `Cargo.toml` and `package.json`, which were already updated in Phase 1. The script itself likely doesn't need changes unless it has hardcoded project name references.

**Commands**:
```bash
# Check for xvn references
grep -i "xvn" scripts/bump-version.sh

# Test the script (dry run if possible)
cat scripts/bump-version.sh | head -20
```

**Expected Result**: Likely no changes needed (script is version-agnostic)

**Actions**:
- [ ] Review script for project name references
- [ ] Update any "xvn" references to "anvs" if found
- [ ] Verify script is executable: `chmod +x scripts/bump-version.sh`
- [ ] Test script syntax: `bash -n scripts/bump-version.sh`

---

### Task 7.2: Update scripts/download-artifacts.sh

**File**: `scripts/download-artifacts.sh`

**This script downloads GitHub Actions build artifacts for a specific version.**

**Changes Required**:

1. **Update repository URLs** (lines ~25, ~30):
   ```bash
   # Before:
   echo "Check: https://github.com/cameronolivier/xvn/actions"
   echo "🔗 View at: https://github.com/cameronolivier/xvn/actions/runs/$RUN_ID"

   # After:
   echo "Check: https://github.com/olvrcc/anvs/actions"
   echo "🔗 View at: https://github.com/olvrcc/anvs/actions/runs/$RUN_ID"
   ```

2. **Update temp directory naming** (line ~33):
   ```bash
   # Before:
   TEMP_DIR="/tmp/xvn-${VERSION}-artifacts"

   # After:
   TEMP_DIR="/tmp/anvs-${VERSION}-artifacts"
   ```

3. **Update artifact pattern** if explicitly referenced:
   - Look for any `xvn-*` artifact name patterns
   - Change to `anvs-*`

**Commands**:
```bash
# Review current state
cat scripts/download-artifacts.sh

# After changes, verify
grep -i "xvn" scripts/download-artifacts.sh  # Should return no results
grep "anvs" scripts/download-artifacts.sh | head -5
```

**Actions**:
- [ ] Update repository URLs (cameronolivier → olvrcc, xvn → anvs)
- [ ] Update TEMP_DIR path (/tmp/xvn-* → /tmp/anvs-*)
- [ ] Update any artifact name patterns
- [ ] Verify script is executable
- [ ] Test script syntax: `bash -n scripts/download-artifacts.sh`
- [ ] Verify no "xvn" references remain

---

### Task 7.3: Update scripts/extract-binaries.sh

**File**: `scripts/extract-binaries.sh`

**This script extracts binaries from downloaded artifacts and copies them to `native/` directories.**

**Changes Required**:

1. **Update temp directory path** (line ~16):
   ```bash
   # Before:
   TEMP_DIR="/tmp/xvn-${VERSION}-artifacts"

   # After:
   TEMP_DIR="/tmp/anvs-${VERSION}-artifacts"
   ```

2. **Update artifact directory naming** (lines ~35-36):
   ```bash
   # Before:
   artifact_dir="$TEMP_DIR/xvn-$platform"
   tarball="$artifact_dir/xvn-$platform.tar.gz"

   # After:
   artifact_dir="$TEMP_DIR/anvs-$platform"
   tarball="$artifact_dir/anvs-$platform.tar.gz"
   ```

3. **Update binary name in copy operations** (lines ~52, 55, 56, 60):
   ```bash
   # Before:
   cp "$artifact_dir/xvn" "native/$platform/xvn"
   if [ -f "native/$platform/xvn" ]; then
       echo "  ✅ Copied to native/$platform/xvn"
       if [[ "$platform" == *"darwin"* ]]; then
           binary_version=$(./native/$platform/xvn --version 2>/dev/null || echo "unknown")

   # After:
   cp "$artifact_dir/anvs" "native/$platform/anvs"
   if [ -f "native/$platform/anvs" ]; then
       echo "  ✅ Copied to native/$platform/anvs"
       if [[ "$platform" == *"darwin"* ]]; then
           binary_version=$(./native/$platform/anvs --version 2>/dev/null || echo "unknown")
   ```

**Commands**:
```bash
# Review current state
grep -n "xvn" scripts/extract-binaries.sh

# After changes
grep -i "xvn" scripts/extract-binaries.sh  # Should return no results
```

**Actions**:
- [ ] Update TEMP_DIR path
- [ ] Update artifact_dir naming (xvn-$platform → anvs-$platform)
- [ ] Update tarball naming (xvn-*.tar.gz → anvs-*.tar.gz)
- [ ] Update binary copy operations (xvn → anvs)
- [ ] Update binary path checks
- [ ] Update version check command
- [ ] Verify script is executable
- [ ] Test script syntax: `bash -n scripts/extract-binaries.sh`
- [ ] Verify no "xvn" references remain

---

### Task 7.4: Update scripts/setup-homebrew-tap.sh

**File**: `scripts/setup-homebrew-tap.sh`

**This script creates and configures the Homebrew tap repository.**

**Changes Required**:

1. **Update tap repository name** throughout (lines ~49, 52-53, 59, 61, 64, 68-69, 71-72):
   ```bash
   # Before:
   echo "📦 Creating homebrew-xvn repository under $REPO_OWNER..."
   if gh repo view "$REPO_OWNER/homebrew-xvn" &> /dev/null; then
       echo "⚠️  Repository $REPO_OWNER/homebrew-xvn already exists"
       gh repo create "$REPO_OWNER/homebrew-xvn" \
           --description "Homebrew tap for xvn (Extreme Version Switcher for Node.js)" \
       echo "✅ Repository created: $REPO_OWNER/homebrew-xvn"
   if [ -d "homebrew-xvn" ]; then
       cd homebrew-xvn
       gh repo clone "$REPO_OWNER/homebrew-xvn"
       cd homebrew-xvn

   # After:
   echo "📦 Creating homebrew-anvs repository under $REPO_OWNER..."
   if gh repo view "$REPO_OWNER/homebrew-anvs" &> /dev/null; then
       echo "⚠️  Repository $REPO_OWNER/homebrew-anvs already exists"
       gh repo create "$REPO_OWNER/homebrew-anvs" \
           --description "Homebrew tap for anvs (Automatic Node Version Switcher)" \
       echo "✅ Repository created: $REPO_OWNER/homebrew-anvs"
   if [ -d "homebrew-anvs" ]; then
       cd homebrew-anvs
       gh repo clone "$REPO_OWNER/homebrew-anvs"
       cd homebrew-anvs
   ```

2. **Update formula file references**:
   - Change `Formula/xvn.rb` → `Formula/anvs.rb`
   - Change formula class name if embedded in script

3. **Update description** to reference "Automatic Node Version Switcher"

4. **Update any binary or package name references** in:
   - README creation
   - Formula template
   - Test commands

**Commands**:
```bash
# Review current state
grep -n "xvn" scripts/setup-homebrew-tap.sh | head -20

# After changes
grep -i "xvn" scripts/setup-homebrew-tap.sh  # Should return no results
grep "homebrew-anvs" scripts/setup-homebrew-tap.sh | head -5
```

**Actions**:
- [ ] Update all "homebrew-xvn" → "homebrew-anvs" references
- [ ] Update tap description (Extreme → Automatic)
- [ ] Update Formula file path (xvn.rb → anvs.rb)
- [ ] Update formula class name if present
- [ ] Update binary name references
- [ ] Update test commands in formula
- [ ] Verify script is executable
- [ ] Test script syntax: `bash -n scripts/setup-homebrew-tap.sh`
- [ ] Verify no "xvn" references remain

---

### Task 7.5: Update scripts/coverage.sh

**File**: `scripts/coverage.sh`

**This script generates code coverage reports.**

**Changes Required**:

Coverage scripts typically don't reference the binary name, but verify to be certain.

**Commands**:
```bash
# Check for any xvn references
grep -i "xvn" scripts/coverage.sh

# If found, update them
# If not found, no changes needed
```

**Expected Result**: Likely no changes needed (script runs cargo tarpaulin which is project-agnostic)

**Actions**:
- [ ] Review script for project name references
- [ ] Update any "xvn" references to "anvs" if found
- [ ] Verify script is executable
- [ ] Test script syntax: `bash -n scripts/coverage.sh`

---

### Task 7.6: Update scripts/version.sh

**File**: `scripts/version.sh`

**This script displays the current version from Cargo.toml and package.json.**

**Changes Required**:

Version detection scripts typically work with version numbers rather than names, but check for any hardcoded references.

**Commands**:
```bash
# Check for xvn references
grep -i "xvn" scripts/version.sh

# Review the script structure
cat scripts/version.sh | head -30
```

**Expected Result**: Likely no changes needed (script is version-agnostic)

**Actions**:
- [ ] Review script for project name references
- [ ] Update any "xvn" references to "anvs" if found
- [ ] Update any echo statements with project name
- [ ] Verify script is executable
- [ ] Test script syntax: `bash -n scripts/version.sh`

---

## Verification Checklist

Before proceeding to Phase 8, verify ALL of the following:

### Script Updates
- [ ] scripts/bump-version.sh reviewed (likely no changes)
- [ ] scripts/download-artifacts.sh updated
- [ ] scripts/extract-binaries.sh updated
- [ ] scripts/setup-homebrew-tap.sh updated
- [ ] scripts/coverage.sh reviewed (likely no changes)
- [ ] scripts/version.sh reviewed (likely no changes)

### Content Verification
- [ ] No "xvn" in download-artifacts.sh: `grep -i "xvn" scripts/download-artifacts.sh`
- [ ] No "xvn" in extract-binaries.sh: `grep -i "xvn" scripts/extract-binaries.sh`
- [ ] No "xvn" in setup-homebrew-tap.sh: `grep -i "xvn" scripts/setup-homebrew-tap.sh`
- [ ] Repository URLs use olvrcc/anvs: `grep -r "olvrcc/anvs" scripts/`
- [ ] Artifact names use "anvs-" prefix
- [ ] Binary paths reference "anvs"
- [ ] Temp directories use /tmp/anvs-*

### Syntax & Permissions
- [ ] All scripts are executable: `ls -la scripts/*.sh`
- [ ] All scripts pass syntax check: `bash -n scripts/*.sh`
- [ ] No shell script errors reported

### Documentation
- [ ] scripts/README.md already updated in Phase 6-3
- [ ] All command examples in README use v2.0.0 and anvs

---

## Success Criteria

Phase 7 is complete when:

1. ✅ All 3 core scripts updated (download-artifacts.sh, extract-binaries.sh, setup-homebrew-tap.sh)
2. ✅ All 3 utility scripts reviewed (bump-version.sh, coverage.sh, version.sh)
3. ✅ No "xvn" references in any scripts (except comments about migration if applicable)
4. ✅ All repository URLs point to olvrcc/anvs
5. ✅ All artifact names use "anvs-" prefix
6. ✅ All binary references use "anvs"
7. ✅ All Homebrew tap references use "homebrew-anvs"
8. ✅ All verification commands pass
9. ✅ All scripts are executable and syntactically valid
10. ✅ Changes committed with file list

---

## Next Steps

After completing Phase 7:

1. **Test the updated scripts** (if possible without triggering actual release):
   ```bash
   # Syntax check all scripts
   for script in scripts/*.sh; do
       echo "Checking $script..."
       bash -n "$script"
   done
   ```

2. **Commit the script changes**:
   ```bash
   git add scripts/*.sh
   git commit -m "feat(scripts): update build and release scripts for anvs rename (Phase 7)

   Files changed:
   - scripts/download-artifacts.sh: Updated repo URLs and artifact naming
   - scripts/extract-binaries.sh: Updated binary and artifact naming
   - scripts/setup-homebrew-tap.sh: Updated Homebrew tap repository name
   - scripts/bump-version.sh: Reviewed (no changes needed)
   - scripts/coverage.sh: Reviewed (no changes needed)
   - scripts/version.sh: Reviewed (no changes needed)

   All scripts now reference 'anvs' instead of 'xvn'.
   Repository URLs updated to olvrcc/anvs.
   Homebrew tap updated to olvrcc/homebrew-anvs."
   ```

3. **Proceed to Phase 8**: GitHub Workflows (CI/CD) - Update `.github/workflows/` files

---

## Rollback Plan

If issues are discovered:

1. **Identify specific script issues**: Which script has problems?
2. **Fix incrementally**: Update specific lines rather than reverting all
3. **Test fixes**: Run `bash -n script.sh` to verify syntax
4. **Re-commit**: Apply fixes in separate commit for clarity

Example rollback for a single script:
```bash
# Revert specific script
git checkout HEAD~1 -- scripts/download-artifacts.sh

# Re-apply correct changes
# ... make fixes ...

# Commit the fix
git add scripts/download-artifacts.sh
git commit -m "fix(scripts): correct repository URL in download-artifacts.sh"
```

---

## Notes

- **Most critical scripts**: `download-artifacts.sh`, `extract-binaries.sh`, `setup-homebrew-tap.sh`
- **Likely unchanged scripts**: `bump-version.sh`, `coverage.sh`, `version.sh` (version-agnostic)
- **Repository URL migration**: cameronolivier/xvn → olvrcc/anvs (note organization change!)
- **Artifact naming**: This affects how GitHub Actions artifacts are named in Phase 8
- **Homebrew tap**: The tap repository will need to be renamed on GitHub in Phase 10
- **Testing limitations**: Cannot fully test these scripts until Phase 8 (GitHub Actions) is complete
- **Dependency chain**: Phase 7 → Phase 8 (Workflows) → Phase 9 (Repo rename) → Phase 10 (Homebrew)

**Estimated time breakdown**:
- Task 7.1 (bump-version.sh): 5 minutes (review only)
- Task 7.2 (download-artifacts.sh): 10 minutes
- Task 7.3 (extract-binaries.sh): 15 minutes (most changes)
- Task 7.4 (setup-homebrew-tap.sh): 15 minutes (complex script)
- Task 7.5 (coverage.sh): 5 minutes (review only)
- Task 7.6 (version.sh): 5 minutes (review only)
- Verification and commit: 5-10 minutes
- **Total**: 45-60 minutes

---
