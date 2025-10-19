# Phase 8: GitHub Workflows (CI/CD)

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes
**Depends On**: Phase 7 (scripts updated)

## Overview

Phase 8 updates all GitHub Actions workflows in the `.github/workflows/` directory. These workflows are critical infrastructure for continuous integration, automated builds, releases, and Homebrew formula updates. All references to `xvn` must be updated to `anvs` to ensure the CI/CD pipeline works correctly with the renamed project.

**Why Phase 8 is Critical:**
- GitHub Actions workflows build and publish release artifacts with platform-specific naming
- Artifact names (`xvn-*.tar.gz` → `anvs-*.tar.gz`) affect download scripts and npm packaging
- Workflow references to shell scripts must match the renamed files
- Homebrew update automation must target the new tap repository and formula
- CI/CD must work correctly before v2.0.0 can be released

**⚠️ IMPORTANT**: This phase must be completed BEFORE creating the v2.0.0 git tag, as pushing the tag will trigger the build workflow.

**📋 Workflow Scope:**
- 3 workflow files in `.github/workflows/`
- Binary and artifact naming updates
- Shell script path updates
- Homebrew tap repository references
- Commit message and description updates

---

## Implementation Tasks

### Task 8.1: Update .github/workflows/build.yml

**File**: `.github/workflows/build.yml`

**Changes Required**:

This is the most critical workflow - it builds release binaries for all platforms and creates GitHub releases.

1. **Update binary copy in Package step** (line 74):
   ```yaml
   # Before:
   cp target/${{ matrix.target }}/release/xvn package/

   # After:
   cp target/${{ matrix.target }}/release/anvs package/
   ```

2. **Update shell script path** (line 75):
   ```yaml
   # Before:
   cp shell/xvn.sh package/lib/

   # After:
   cp shell/anvs.sh package/lib/
   ```

3. **Update Windows binary reference** (line 78, in conditional):
   ```yaml
   # Before:
   tar czf xvn-${{ matrix.target }}.tar.gz xvn.exe lib/

   # After:
   tar czf anvs-${{ matrix.target }}.tar.gz anvs.exe lib/
   ```

4. **Update Unix binary reference** (line 80):
   ```yaml
   # Before:
   tar czf xvn-${{ matrix.target }}.tar.gz xvn lib/

   # After:
   tar czf anvs-${{ matrix.target }}.tar.gz anvs lib/
   ```

5. **Update tarball checksum generation** (line 82):
   ```yaml
   # Before:
   shasum -a 256 xvn-${{ matrix.target }}.tar.gz > xvn-${{ matrix.target }}.tar.gz.sha256

   # After:
   shasum -a 256 anvs-${{ matrix.target }}.tar.gz > anvs-${{ matrix.target }}.tar.gz.sha256
   ```

6. **Update artifact move** (line 83):
   ```yaml
   # Before:
   mv xvn-${{ matrix.target }}.tar.gz* ../target/${{ matrix.target }}/release/

   # After:
   mv anvs-${{ matrix.target }}.tar.gz* ../target/${{ matrix.target }}/release/
   ```

7. **Update artifact upload name** (line 88):
   ```yaml
   # Before:
   name: xvn-${{ matrix.target }}

   # After:
   name: anvs-${{ matrix.target }}
   ```

8. **Update artifact upload paths** (lines 90-91):
   ```yaml
   # Before:
   path: |
     target/${{ matrix.target }}/release/xvn-${{ matrix.target }}.tar.gz
     target/${{ matrix.target }}/release/xvn-${{ matrix.target }}.tar.gz.sha256

   # After:
   path: |
     target/${{ matrix.target }}/release/anvs-${{ matrix.target }}.tar.gz
     target/${{ matrix.target }}/release/anvs-${{ matrix.target }}.tar.gz.sha256
   ```

**Commands**:
```bash
# Review current state
grep -n "xvn" .github/workflows/build.yml

# After changes, verify no xvn references remain
grep -i "xvn" .github/workflows/build.yml  # Should return no results
grep "anvs" .github/workflows/build.yml | head -10
```

**Expected Result**: All artifact names, binary references, and shell script paths use "anvs"

**Actions**:
- [ ] Update binary copy command (line 74)
- [ ] Update shell script path (line 75)
- [ ] Update Windows tarball name (line 78)
- [ ] Update Unix tarball name (line 80)
- [ ] Update checksum generation (line 82)
- [ ] Update artifact move command (line 83)
- [ ] Update artifact upload name (line 88)
- [ ] Update artifact upload paths (lines 90-91)
- [ ] Verify YAML syntax: `yamllint .github/workflows/build.yml` (if available)
- [ ] Verify no "xvn" references remain

---

### Task 8.2: Update .github/workflows/test.yml

**File**: `.github/workflows/test.yml`

**Changes Required**:

This workflow runs tests, linting, and coverage. It has minimal references to the project name.

1. **Update PowerShell script path** (line 67):
   ```yaml
   # Before:
   $results = Invoke-ScriptAnalyzer -Path shell/xvn.ps1 -Recurse -Settings PSGallery

   # After:
   $results = Invoke-ScriptAnalyzer -Path shell/anvs.ps1 -Recurse -Settings PSGallery
   ```

**Note**: This is the ONLY change needed in test.yml. The rest of the workflow is language/framework agnostic.

**Commands**:
```bash
# Check for xvn references
grep -n "xvn" .github/workflows/test.yml

# After changes
grep -i "xvn" .github/workflows/test.yml  # Should return no results
```

**Expected Result**: PowerShell script path references `shell/anvs.ps1`

**Actions**:
- [ ] Update PowerShell script path (line 67)
- [ ] Verify YAML syntax
- [ ] Verify no "xvn" references remain

---

### Task 8.3: Update .github/workflows/update-homebrew.yml

**File**: `.github/workflows/update-homebrew.yml`

**This workflow automatically updates the Homebrew formula when a new release is published.**

**Changes Required**:

1. **Update HOMEBREW_TAP_REPO environment variable** (line 14):
   ```yaml
   # Before:
   HOMEBREW_TAP_REPO: olvrcc/homebrew-xvn

   # After:
   HOMEBREW_TAP_REPO: olvrcc/homebrew-anvs
   ```

2. **Update HOMEBREW_FORMULA_PATH environment variable** (line 15):
   ```yaml
   # Before:
   HOMEBREW_FORMULA_PATH: Formula/xvn.rb

   # After:
   HOMEBREW_FORMULA_PATH: Formula/anvs.rb
   ```

3. **Update checkout step comment** (line 23):
   ```yaml
   # Before:
   - name: Checkout xvn repository

   # After:
   - name: Checkout anvs repository
   ```

4. **Update macOS x64 download** (lines 41-42):
   ```yaml
   # Before:
   curl -L -o xvn-x86_64.tar.gz \
     "https://github.com/olvrcc/xvn/releases/download/${{ steps.version.outputs.version }}/xvn-x86_64-apple-darwin.tar.gz"

   # After:
   curl -L -o anvs-x86_64.tar.gz \
     "https://github.com/olvrcc/anvs/releases/download/${{ steps.version.outputs.version }}/anvs-x86_64-apple-darwin.tar.gz"
   ```

5. **Update macOS arm64 download** (lines 46-47):
   ```yaml
   # Before:
   curl -L -o xvn-aarch64.tar.gz \
     "https://github.com/olvrcc/xvn/releases/download/${{ steps.version.outputs.version }}/xvn-aarch64-apple-darwin.tar.gz"

   # After:
   curl -L -o anvs-aarch64.tar.gz \
     "https://github.com/olvrcc/anvs/releases/download/${{ steps.version.outputs.version }}/anvs-aarch64-apple-darwin.tar.gz"
   ```

6. **Update SHA256 checksum calculations** (lines 52-53):
   ```yaml
   # Before:
   SHA256_X64=$(shasum -a 256 xvn-x86_64.tar.gz | awk '{print $1}')
   SHA256_ARM64=$(shasum -a 256 xvn-aarch64.tar.gz | awk '{print $1}')

   # After:
   SHA256_X64=$(shasum -a 256 anvs-x86_64.tar.gz | awk '{print $1}')
   SHA256_ARM64=$(shasum -a 256 anvs-aarch64.tar.gz | awk '{print $1}')
   ```

7. **Update tap checkout step** (lines 59-64):
   ```yaml
   # Before:
   - name: Checkout homebrew-xvn tap
     uses: actions/checkout@v4
     with:
       repository: ${{ env.HOMEBREW_TAP_REPO }}
       token: ${{ secrets.HOMEBREW_TAP_TOKEN }}
       path: homebrew-xvn

   # After:
   - name: Checkout homebrew-anvs tap
     uses: actions/checkout@v4
     with:
       repository: ${{ env.HOMEBREW_TAP_REPO }}
       token: ${{ secrets.HOMEBREW_TAP_TOKEN }}
       path: homebrew-anvs
   ```

8. **Update formula update script working directory** (line 68):
   ```bash
   # Before:
   cd homebrew-xvn

   # After:
   cd homebrew-anvs
   ```

9. **Update URL sed patterns** (lines 74-75):
   ```bash
   # Before:
   sed -i "s|download/v[0-9.]\+/xvn-aarch64-apple-darwin.tar.gz|download/${{ steps.version.outputs.version }}/xvn-aarch64-apple-darwin.tar.gz|" ${{ env.HOMEBREW_FORMULA_PATH }}
   sed -i "s|download/v[0-9.]\+/xvn-x86_64-apple-darwin.tar.gz|download/${{ steps.version.outputs.version }}/xvn-x86_64-apple-darwin.tar.gz|" ${{ env.HOMEBREW_FORMULA_PATH }}

   # After:
   sed -i "s|download/v[0-9.]\+/anvs-aarch64-apple-darwin.tar.gz|download/${{ steps.version.outputs.version }}/anvs-aarch64-apple-darwin.tar.gz|" ${{ env.HOMEBREW_FORMULA_PATH }}
   sed -i "s|download/v[0-9.]\+/anvs-x86_64-apple-darwin.tar.gz|download/${{ steps.version.outputs.version }}/anvs-x86_64-apple-darwin.tar.gz|" ${{ env.HOMEBREW_FORMULA_PATH }}
   ```

10. **Update commit working directory** (line 100):
    ```bash
    # Before:
    cd homebrew-xvn

    # After:
    cd homebrew-anvs
    ```

11. **Update commit message** (line 109):
    ```bash
    # Before:
    git commit -m "chore: update xvn to ${{ steps.version.outputs.version }}"

    # After:
    git commit -m "chore: update anvs to ${{ steps.version.outputs.version }}"
    ```

12. **Update summary output** (line 123):
    ```bash
    # Before:
    echo "  brew upgrade xvn"

    # After:
    echo "  brew upgrade anvs"
    ```

**Commands**:
```bash
# Review current state
grep -n "xvn" .github/workflows/update-homebrew.yml | head -20

# After changes
grep -i "xvn" .github/workflows/update-homebrew.yml  # Should return no results
grep "homebrew-anvs" .github/workflows/update-homebrew.yml | head -3
grep "anvs-" .github/workflows/update-homebrew.yml | head -5
```

**Expected Result**: All references use `anvs`, `homebrew-anvs`, and `olvrcc/anvs` repository

**Actions**:
- [ ] Update HOMEBREW_TAP_REPO env var (line 14)
- [ ] Update HOMEBREW_FORMULA_PATH env var (line 15)
- [ ] Update checkout step name (line 23)
- [ ] Update macOS x64 download URL and filename (lines 41-42)
- [ ] Update macOS arm64 download URL and filename (lines 46-47)
- [ ] Update SHA256 checksum filenames (lines 52-53)
- [ ] Update tap checkout step name and path (lines 59-64)
- [ ] Update first `cd homebrew-xvn` to `cd homebrew-anvs` (line 68)
- [ ] Update URL sed patterns (lines 74-75)
- [ ] Update second `cd homebrew-xvn` to `cd homebrew-anvs` (line 100)
- [ ] Update commit message (line 109)
- [ ] Update summary brew command (line 123)
- [ ] Verify YAML syntax
- [ ] Verify no "xvn" references remain

---

## Verification Checklist

Before proceeding to Phase 9, verify ALL of the following:

### Workflow File Updates
- [ ] .github/workflows/build.yml updated with all anvs references
- [ ] .github/workflows/test.yml updated (PowerShell script path)
- [ ] .github/workflows/update-homebrew.yml updated with all anvs references

### Content Verification
- [ ] No "xvn" in build.yml: `grep -i "xvn" .github/workflows/build.yml`
- [ ] No "xvn" in test.yml: `grep -i "xvn" .github/workflows/test.yml`
- [ ] No "xvn" in update-homebrew.yml: `grep -i "xvn" .github/workflows/update-homebrew.yml`
- [ ] Repository URLs use olvrcc/anvs: `grep -r "olvrcc/anvs" .github/workflows/`
- [ ] Artifact names use "anvs-" prefix: `grep "anvs-" .github/workflows/build.yml`
- [ ] Shell script path is shell/anvs.sh: `grep "shell/anvs" .github/workflows/`
- [ ] Homebrew tap is homebrew-anvs: `grep "homebrew-anvs" .github/workflows/update-homebrew.yml`
- [ ] Formula path is Formula/anvs.rb: `grep "Formula/anvs.rb" .github/workflows/update-homebrew.yml`

### YAML Syntax
- [ ] build.yml is valid YAML (test with yamllint or GitHub Actions validator)
- [ ] test.yml is valid YAML
- [ ] update-homebrew.yml is valid YAML
- [ ] No workflow syntax errors

### Critical Paths Verified
- [ ] Binary path: `target/${{ matrix.target }}/release/anvs`
- [ ] Shell script: `shell/anvs.sh`
- [ ] Tarball pattern: `anvs-${{ matrix.target }}.tar.gz`
- [ ] Artifact name: `anvs-${{ matrix.target }}`
- [ ] Download URLs: `https://github.com/olvrcc/anvs/releases/download/.../anvs-*.tar.gz`

---

## Success Criteria

Phase 8 is complete when:

1. ✅ All 3 workflow files updated (build.yml, test.yml, update-homebrew.yml)
2. ✅ No "xvn" references in any workflow file
3. ✅ All repository URLs point to olvrcc/anvs
4. ✅ All artifact names use "anvs-" prefix
5. ✅ All binary references use "anvs"
6. ✅ Shell script path is shell/anvs.sh (or shell/anvs.ps1 for PowerShell)
7. ✅ Homebrew tap references use "homebrew-anvs"
8. ✅ Homebrew formula path is Formula/anvs.rb
9. ✅ All YAML files are syntactically valid
10. ✅ All verification commands pass
11. ✅ Changes committed with file list

---

## Next Steps

After completing Phase 8:

1. **DO NOT create v2.0.0 tag yet** - workflows are updated but other phases must complete first

2. **Commit the workflow changes**:
   ```bash
   git add .github/workflows/*.yml
   git commit -m "feat(ci): update GitHub workflows for anvs rename (Phase 8)

   Files changed:
   - .github/workflows/build.yml: Updated artifact names, binary paths, shell script paths
   - .github/workflows/test.yml: Updated PowerShell script path
   - .github/workflows/update-homebrew.yml: Updated tap repo, formula path, download URLs

   All workflows now reference 'anvs' instead of 'xvn'.
   Repository URLs updated to olvrcc/anvs.
   Homebrew tap updated to olvrcc/homebrew-anvs.
   Formula path updated to Formula/anvs.rb."
   ```

3. **Proceed to Phase 9**: GitHub Repository Changes - Rename the repository on GitHub and update settings

---

## Rollback Plan

If issues are discovered after pushing workflows:

1. **Identify specific workflow issue**: Which workflow has problems?
2. **Fix incrementally**: Update specific lines rather than reverting all
3. **Test locally if possible**: Some workflow changes can be tested with `act` (GitHub Actions local runner)
4. **Re-commit**: Apply fixes in separate commit for clarity

Example rollback for a single workflow:
```bash
# Revert specific workflow
git checkout HEAD~1 -- .github/workflows/build.yml

# Re-apply correct changes
# ... make fixes ...

# Commit the fix
git add .github/workflows/build.yml
git commit -m "fix(ci): correct artifact naming in build workflow"
```

---

## Notes

- **Most critical workflow**: `build.yml` - builds all release artifacts
- **Testing limitation**: Cannot fully test workflows until v2.0.0 tag is created
- **Repository dependency**: update-homebrew.yml depends on Phase 9 (repo rename) and Phase 10 (Homebrew tap)
- **Artifact naming**: This affects how download-artifacts.sh and extract-binaries.sh (Phase 7) will work
- **Chicken-and-egg**: Workflows reference new names, but they won't work until repository is renamed
- **Safe approach**: Update workflows first, then rename repository - GitHub Actions will use new workflows immediately
- **Homebrew tap**: The tap repository will need to be renamed in Phase 10
- **Testing strategy**: After Phase 9, create a test tag (e.g., v2.0.0-beta.1) to verify workflows before final release

**Estimated time breakdown**:
- Task 8.1 (build.yml): 20-25 minutes (most changes)
- Task 8.2 (test.yml): 5 minutes (minimal changes)
- Task 8.3 (update-homebrew.yml): 15-20 minutes (many changes)
- Verification and commit: 5-10 minutes
- **Total**: 45-60 minutes

**Critical dependencies chain**:
- Phase 7 (Scripts) → Phase 8 (Workflows) → Phase 9 (Repo rename) → Phase 10 (Homebrew) → v2.0.0 release

---

## Workflow Testing Plan (Optional but Recommended)

After Phase 9 (repo rename), consider testing workflows before v2.0.0:

1. **Create test tag**:
   ```bash
   git tag v2.0.0-beta.1
   git push origin v2.0.0-beta.1
   ```

2. **Monitor GitHub Actions**: https://github.com/olvrcc/anvs/actions

3. **Verify**:
   - Build workflow completes successfully
   - Artifacts are named correctly (anvs-*.tar.gz)
   - GitHub release is created
   - Homebrew workflow triggers (if release is published)

4. **If successful**:
   - Delete test tag and release
   - Proceed with final v2.0.0 release

5. **If failed**:
   - Fix workflow issues
   - Repeat testing with next beta tag

---
