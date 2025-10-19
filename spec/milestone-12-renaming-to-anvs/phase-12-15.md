# Phase 12-15: Homebrew Integration

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes

## Overview

Phase 12-15 updates the Homebrew formula for `anvs` and verifies Homebrew installation. This provides an alternative installation method for macOS users.

**Why Phase 12-15 is Critical:**
- Provides Homebrew installation for macOS users
- Updates formula to point to new `anvs` release assets
- Calculates SHA256 checksums for all platforms
- Tests installation via Homebrew tap

**⚠️ CHECKPOINT**: Before starting this phase, ensure:
- Phase 12-14 is complete (npm package published)
- GitHub Release v2.0.0 exists with correct assets
- Homebrew tap repository accessible
- Ready to modify Homebrew formula

---

## Implementation Tasks

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

## Verification Checklist

Before proceeding to Phase 12-16, verify ALL of the following:

### Formula Update
- [ ] Homebrew workflow ran successfully (automatic or manual)
- [ ] Formula/anvs.rb updated to v2.0.0
- [ ] Download URLs point to correct release assets
- [ ] SHA256 checksums calculated and updated
- [ ] Formula committed to homebrew-anvs repository

### Formula Content
- [ ] Class name is `Anvs` (not `Xvn`)
- [ ] Version is `2.0.0`
- [ ] URLs use `anvs` assets (not `xvn`)
- [ ] All platform checksums present
- [ ] Binary name is `anvs` in install section
- [ ] Test uses `anvs` command
- [ ] Ruby syntax valid

### Installation Test
- [ ] Tap added successfully
- [ ] `brew install anvs` succeeds
- [ ] Binary installed to correct location
- [ ] Version shows as 2.0.0
- [ ] `anvs setup` works
- [ ] Shell integration configured
- [ ] Uninstall works (if tested)

---

## Success Criteria

Phase 12-15 is complete when:

1. ✅ Homebrew formula updated to v2.0.0
2. ✅ Formula points to correct `anvs` release assets
3. ✅ SHA256 checksums calculated for all platforms
4. ✅ Formula syntax valid and commits pushed
5. ✅ Homebrew installation works correctly
6. ✅ Installed binary functions properly
7. ✅ All verification checklist items passed

---

## Next Steps

After completing Phase 12-15:

1. **Proceed to Phase 12-16**: Final Release & Documentation
2. **Test on fresh macOS system**: Verify Homebrew installation elsewhere
3. **Update documentation**: Mention Homebrew installation option
4. **Monitor for issues**: Watch for Homebrew-related problems

**Proceed to Phase 12-16**: [Final Release & Documentation](./phase-12-16.md)

---

## Rollback Plan

If issues arise with Homebrew formula:

### Formula Issues
```bash
# Navigate to homebrew-anvs repo
cd /path/to/homebrew-anvs

# Reset to previous working version
git log --oneline  # Find previous commit
git reset --hard <previous-commit>

# Push reset
git push --force-with-lease

# Or create new fixed version
# Edit Formula/anvs.rb
git add Formula/anvs.rb
git commit -m "fix: correct formula for v2.0.0"
git push
```

### Installation Issues
```bash
# Uninstall broken version
brew uninstall anvs

# Force update
brew update

# Reinstall
brew install anvs
```

### Checksum Issues
```bash
# Recalculate checksums
./scripts/setup-homebrew-tap.sh

# Or manually calculate
shasum -a 256 /path/to/anvs-x86_64-apple-darwin.tar.gz
```

---

## Common Issues & Troubleshooting

### Issue: Automatic Homebrew update didn't run
**Solution**:
- Check workflow permissions: GitHub Actions need access to homebrew-anvs repo
- Verify workflow trigger: Release creation should trigger update
- Run manually: `./scripts/setup-homebrew-tap.sh`

### Issue: Formula has wrong URLs
**Solution**:
- Check release asset names: `gh release view v2.0.0 --json assets`
- Update URLs in Formula/anvs.rb manually
- Ensure assets use `anvs-` prefix, not `xvn-`

### Issue: SHA256 checksums don't match
**Solution**:
- Download assets and calculate checksums manually
- Use `shasum -a 256 file.tar.gz` on each platform
- Update formula with correct checksums
- Re-run setup script if needed

### Issue: brew install fails
**Solution**:
- Check formula syntax: `ruby -c Formula/anvs.rb`
- Verify URLs are accessible
- Check checksums match downloaded files
- Review Homebrew error logs

### Issue: Binary not found after install
**Solution**:
- Check install path: `brew --prefix anvs`
- Verify binary exists: `ls -la $(brew --prefix anvs)/bin/`
- Check PATH includes Homebrew bin directory
- Try `brew link anvs` if needed

---

## Notes

- **Homebrew is macOS-only** - no need to test on Linux
- **Formula updates are separate** from npm publication
- **Checksums are critical** - Homebrew verifies file integrity
- **Multiple platforms supported** - ARM64 and x86_64 for macOS
- **Testing is important** - Homebrew has strict requirements
- **Documentation should be updated** to include Homebrew option
- **Monitor for issues** - Homebrew users may report problems