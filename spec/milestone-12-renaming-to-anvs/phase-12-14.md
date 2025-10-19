# Phase 12-14: npm Publication & Verification

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 30-60 minutes (including 2FA wait times)

## Overview

Phase 12-14 publishes the `anvs` package to npm and verifies the publication. This is the most critical phase as it makes the renamed package publicly available.

**Why Phase 12-14 is Critical:**
- Makes `anvs@2.0.0` publicly available on npm
- Cannot be easily undone (npm discourages unpublishing)
- Requires 2FA authentication
- Must verify package works correctly after publication

**⚠️ CHECKPOINT**: Before starting this phase, ensure:
- Phase 12-13 is 100% complete
- `anvs-2.0.0.tgz` package exists and verified
- All binaries correctly named `anvs`
- Ready for public release (this cannot be undone)

---

## Implementation Tasks

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

## Verification Checklist

Before proceeding to Phase 12-15, verify ALL of the following:

### Package Verification
- [ ] Package contents verified (no xvn references)
- [ ] package.json has correct name and version
- [ ] All required files included in package
- [ ] Binary wrapper uses ANVS_ variables
- [ ] Shell script uses ANVS_ variables

### Publication
- [ ] npm publish succeeded without errors
- [ ] Package published as `anvs` (unnamespaced)
- [ ] Version 2.0.0 visible in npm registry
- [ ] 2FA authentication successful

### Accessibility
- [ ] `npm view anvs` works correctly
- [ ] Package page loads on npmjs.com
- [ ] README displays correctly
- [ ] Installation command visible
- [ ] Repository link correct

### Installation Test
- [ ] `npm install -g anvs` succeeds
- [ ] `anvs` command available in PATH
- [ ] `anvs --version` shows 2.0.0
- [ ] `anvs setup` works correctly
- [ ] Shell integration configured
- [ ] Manual activation works

---

## Success Criteria

Phase 12-14 is complete when:

1. ✅ Package contents verified (no xvn references)
2. ✅ Successfully published to npm as `anvs@2.0.0`
3. ✅ Package accessible on npm registry
4. ✅ npm installation works correctly
5. ✅ Installed package functions properly
6. ✅ All verification checklist items passed
7. ✅ No publication errors or warnings

---

## Next Steps

After completing Phase 12-14:

1. **Proceed to Phase 12-15**: Homebrew Integration
2. **Monitor npm**: Watch for download counts and issues
3. **Test on fresh systems**: Verify installation works elsewhere
4. **Prepare for Homebrew**: Update formula for new release

**Proceed to Phase 12-15**: [Homebrew Integration](./phase-12-15.md)

---

## Rollback Plan

If critical issues are discovered after publication:

### Emergency Patch
```bash
# Fix the issue in codebase
# Bump to patch version (2.0.1)
./scripts/bump-version.sh patch

# Create new package
npm pack

# Publish patch version
npm publish --otp=<2FA_CODE>

# Deprecate broken version
npm deprecate anvs@2.0.0 "Critical issue found, please use 2.0.1"
```

### Package Name Issues
- If wrong package name published: Cannot unpublish, must publish correct name
- If wrong version published: Publish correct version, deprecate wrong one
- If broken package published: Publish patch version immediately

### DO NOT UNPUBLISH
- npm strongly discourages unpublishing
- Unpublishing breaks existing installations
- Can cause dependency resolution issues
- Use deprecation instead

---

## Common Issues & Troubleshooting

### Issue: npm publish fails with authentication error
**Solution**:
- Check npm login: `npm whoami`
- Re-login if needed: `npm login`
- Ensure 2FA code is fresh (<30 seconds)
- Check network connectivity

### Issue: Package name already taken
**Solution**:
- Check if name exists: `npm view anvs`
- If taken, choose different name or scoped package
- Update package.json and republish

### Issue: Package contains xvn references
**Solution**:
- Stop publication immediately
- Fix references in source code
- Rebuild and repackage
- Verify with `tar -tzf anvs-2.0.0.tgz`

### Issue: Installation fails after publish
**Solution**:
- Test installation in clean environment
- Check package.json dependencies
- Verify binary paths in package
- Publish patch version if needed

### Issue: 2FA not working
**Solution**:
- Use authenticator app for fresh codes
- Ensure system time is correct
- Try backup codes if available
- Contact npm support if locked out

---

## Notes

- **This is the point of no return** - npm publication cannot be undone
- **Double-check everything** before hitting publish
- **Have 2FA ready** - you'll need it during publication
- **Test thoroughly** after publication
- **Monitor for issues** - be ready to publish patches quickly
- **Document any issues** for future reference
- **Keep calm** - if issues arise, use the rollback plan