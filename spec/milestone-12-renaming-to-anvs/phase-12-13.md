# Phase 12-13: Fix Native Binaries & Prepare Package

**Status**: Completed
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes

## Overview

Phase 12-13 fixes the critical binary naming issue and prepares the npm package. The current `native/` directory contains binaries named `xvn` instead of `anvs`, which blocks npm package creation.

**Why Phase 12-13 is Critical:**
- Fixes binary naming issue (xvn → anvs)
- Downloads correctly named artifacts from GitHub release
- Creates the npm package tarball for publication
- Prerequisite for all subsequent publication phases

**⚠️ CHECKPOINT**: Before starting this phase, ensure:
- GitHub Release v2.0.0 exists with correct asset names
- Current working tree is clean
- All previous phases (0-12) are complete

---

## Implementation Tasks

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

## Verification Checklist

Before proceeding to Phase 12-14, verify ALL of the following:

### Binary Fixes
- [ ] Old `xvn` binaries removed from `native/` directory
- [ ] New `anvs` binaries present in `native/` directory
- [ ] All 4 platform binaries present
- [ ] Binaries are executable
- [ ] Binary names are `anvs` (not `xvn`)

### Version Verification
- [ ] All binaries report version `2.0.0`
- [ ] Binary help text shows `anvs` commands
- [ ] No `xvn` references in binary output

### Package Creation
- [ ] `anvs-2.0.0.tgz` created successfully
- [ ] Package size reasonable (<20MB)
- [ ] All required files included in package
- [ ] No `xvn` references in package contents

### Script Verification
- [ ] `download-artifacts.sh` script works correctly
- [ ] `extract-binaries.sh` script works correctly
- [ ] Scripts handle `anvs` naming correctly

---

## Success Criteria

Phase 12-13 is complete when:

1. ✅ All `native/` binaries are named `anvs` (not `xvn`)
2. ✅ All 4 platform binaries present and executable
3. ✅ All binaries report version `2.0.0`
4. ✅ `anvs-2.0.0.tgz` package created successfully
5. ✅ Package contains all required files
6. ✅ No `xvn` references in package contents
7. ✅ All verification checklist items passed

---

## Next Steps

After completing Phase 12-13:

1. **Proceed to Phase 12-14**: npm Publication & Verification
2. **Critical**: Do not proceed until binaries are correctly named
3. **Backup**: Keep `anvs-2.0.0.tgz` safe for publication
4. **Verification**: Double-check package contents before publishing

**Proceed to Phase 12-14**: [npm Publication & Verification](./phase-12-14.md)

---

## Rollback Plan

If issues arise during this phase:

### Binary Extraction Issues
```bash
# Remove corrupted binaries
rm -rf native/*

# Restore from backup if available
# Or re-download and re-extract
./scripts/download-artifacts.sh v2.0.0
./scripts/extract-binaries.sh v2.0.0
```

### Package Creation Issues
```bash
# Remove corrupted package
rm -f anvs-2.0.0.tgz

# Fix native/ directory issues first
# Then recreate package
npm pack
```

### Script Issues
- Check script permissions: `chmod +x scripts/*.sh`
- Verify GitHub CLI authentication: `gh auth status`
- Check release exists: `gh release view v2.0.0`

---

## Common Issues & Troubleshooting

### Issue: Binaries still named xvn after extraction
**Solution**:
- Check if GitHub release assets have correct names
- Verify download script downloads correct artifacts
- Check extraction script renames binaries correctly

### Issue: npm pack fails
**Solution**:
- Verify `package.json` has correct name and version
- Check that all required files exist
- Ensure `native/` directory contains `anvs` binaries

### Issue: Package contains xvn references
**Solution**:
- Search for remaining xvn references: `grep -r "xvn" . --exclude-dir=.git`
- Fix any remaining references in source files
- Rebuild and re-extract binaries if needed

### Issue: Binary execution fails
**Solution**:
- Check binary permissions: `chmod +x native/*/anvs`
- Verify binary format: `file native/*/anvs`
- Test on compatible platform only

---

## Notes

- **This phase fixes the critical blocker** for npm publication
- **Binary naming is essential** - cannot publish package with wrong binary names
- **Package creation is local** - no external changes yet
- **Verification is critical** - ensure package is correct before publishing
- **Keep the package tarball** - needed for next phase