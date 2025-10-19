# Phase 10: Homebrew Tap Changes

**Status**: Completed
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes

## Overview

Phase 10 handles updating the Homebrew tap infrastructure to reflect the rename from `xvn` to `anvs`. This includes renaming the tap repository, updating the formula file, and ensuring the Homebrew installation method works correctly with the new package name.

**Why Phase 10 is Essential:**
- Provides macOS users with a native package manager installation method
- Ensures consistency between npm package name and Homebrew formula name
- Maintains professional presence in the Homebrew ecosystem
- Updates all tap infrastructure to match the new project identity
- Required before the v2.0.0 release to provide Homebrew installation option

**⚠️ IMPORTANT NOTES:**
- This phase should be completed AFTER Phase 9 (GitHub repository rename)
- The tap repository rename is independent but follows the same pattern
- GitHub provides automatic redirects for the old tap repository URL
- Formula must be updated to reference new artifact names from CI/CD
- Test the formula locally before pushing to ensure it works

**Prerequisites:**
- [ ] Phase 9 completed (main repository renamed to `anvs`)
- [ ] Main repository CI/CD workflows updated (Phase 8)
- [ ] Access to `homebrew-xvn` repository

---

## Implementation Tasks

### Task 10.1: Prepare for Tap Repository Rename

**Pre-rename checklist**:

Before renaming the tap repository, ensure:
- [ ] You have cloned the `homebrew-xvn` repository locally
- [ ] Working directory is clean
- [ ] All pending changes committed
- [ ] Current formula is working (if published)

**Commands**:
```bash
# If you don't have the tap repository cloned:
cd ~/dev/tooling
git clone https://github.com/olvrcc/homebrew-xvn.git
cd homebrew-xvn

# If already cloned, navigate to it:
cd ~/path/to/homebrew-xvn

# Verify clean working directory
git status

# Verify current remote
git remote get-url origin

# Check current files
ls -la Formula/
```

**Expected output**:
```
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean
```

**Expected files**:
```
Formula/
  xvn.rb
README.md
```

**Actions**:
- [ ] Clone or navigate to `homebrew-xvn` repository
- [ ] Verify working directory is clean
- [ ] Note current remote URL
- [ ] Check that `Formula/xvn.rb` exists
- [ ] Backup current formula: `cp Formula/xvn.rb Formula/xvn.rb.backup`

---

### Task 10.2: Rename Homebrew Tap Repository on GitHub

**Navigate to tap repository settings on GitHub**:

1. **Open repository settings**:
   - Go to: https://github.com/olvrcc/homebrew-xvn
   - Click **Settings** tab
   - Scroll down to **Danger Zone** section

2. **Rename the repository**:
   - Click **Rename** button
   - **Old name**: `homebrew-xvn`
   - **New name**: `homebrew-anvs`
   - Read the warning about redirects
   - Type the new name to confirm: `homebrew-anvs`
   - Click **I understand, rename repository**

3. **GitHub's automatic actions**:
   - Redirects created: `github.com/olvrcc/homebrew-xvn` → `github.com/olvrcc/homebrew-anvs`
   - All commits and history preserved
   - Tap URL updated automatically

**Expected result**:
- Repository accessible at: https://github.com/olvrcc/homebrew-anvs
- Old URL redirects: https://github.com/olvrcc/homebrew-xvn → https://github.com/olvrcc/homebrew-anvs

**Homebrew tap naming convention**:
- Repository: `homebrew-anvs`
- Tap name: `olvrcc/anvs` (Homebrew automatically strips the `homebrew-` prefix)
- Installation: `brew install olvrcc/anvs/anvs`

**Actions**:
- [ ] Navigate to tap repository settings on GitHub
- [ ] Click rename in Danger Zone
- [ ] Enter new name: `homebrew-anvs`
- [ ] Confirm rename
- [ ] Verify redirect works (visit old URL)
- [ ] Verify new URL loads correctly
- [ ] Note new tap name: `olvrcc/anvs`

---

### Task 10.3: Update Local Tap Repository Remote

**Update your local tap repository clone's remote URL**:

```bash
# Navigate to tap repository
cd ~/path/to/homebrew-xvn  # Still has old directory name locally

# Check current remote URL
git remote get-url origin

# Should show: https://github.com/olvrcc/homebrew-xvn.git
# or: git@github.com:olvrcc/homebrew-xvn.git

# Update to new URL (HTTPS)
git remote set-url origin https://github.com/olvrcc/homebrew-anvs.git

# Or update to new URL (SSH)
git remote set-url origin git@github.com:olvrcc/homebrew-anvs.git

# Verify new URL
git remote get-url origin

# Test connection
git fetch origin

# Verify branch tracking
git branch -vv
```

**Expected output after `git remote get-url origin`**:
```
https://github.com/olvrcc/homebrew-anvs.git
```

or

```
git@github.com:olvrcc/homebrew-anvs.git
```

**Optional: Rename local directory**:
```bash
# Go up one directory
cd ..

# Rename local directory to match new repository name
mv homebrew-xvn homebrew-anvs

# Navigate back into it
cd homebrew-anvs

# Verify everything still works
git status
```

**Actions**:
- [ ] Check current remote URL
- [ ] Update remote URL to new repository name
- [ ] Verify new URL is set correctly
- [ ] Test `git fetch` succeeds
- [ ] Optionally rename local directory
- [ ] Verify `git status` works

---

### Task 10.4: Rename Formula File

**Rename the formula file from `xvn.rb` to `anvs.rb`**:

Homebrew formula files must match the formula name. Since we're renaming to `anvs`, the file must be `anvs.rb`.

**Commands**:
```bash
# Ensure you're in the tap repository
cd ~/path/to/homebrew-anvs

# Rename the formula file using git mv (preserves history)
git mv Formula/xvn.rb Formula/anvs.rb

# Verify rename
ls -la Formula/

# Check git status
git status
```

**Expected output**:
```
On branch main
Your branch is up to date with 'origin/main'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        renamed:    Formula/xvn.rb -> Formula/anvs.rb
```

**Actions**:
- [ ] Use `git mv` to rename `Formula/xvn.rb` to `Formula/anvs.rb`
- [ ] Verify new file exists: `ls Formula/anvs.rb`
- [ ] Verify old file gone: `ls Formula/xvn.rb` should fail
- [ ] Check git status shows rename

---

### Task 10.5: Update Formula Class Name

**File**: `Formula/anvs.rb`

Homebrew formulas must have a class name matching the formula name.

**Current class declaration** (line 1):
```ruby
class Xvn < Formula
```

**New class declaration**:
```ruby
class Anvs < Formula
```

**Note**: Ruby class names must be capitalized. `xvn` → `Xvn`, `anvs` → `Anvs`.

**Changes Required**:
- Line 1: Change `class Xvn < Formula` to `class Anvs < Formula`

**Actions**:
- [ ] Open `Formula/anvs.rb` in editor
- [ ] Change class name from `Xvn` to `Anvs` (line 1)
- [ ] Save file
- [ ] Verify syntax: `ruby -c Formula/anvs.rb`

---

### Task 10.6: Update Formula Metadata

**File**: `Formula/anvs.rb`

Update the formula's description, homepage, and metadata fields.

**Current metadata** (approximate lines 2-5):
```ruby
desc "Automatic Node Version Switcher - faster avn alternative in Rust"
homepage "https://github.com/olvrcc/xvn"
url "https://github.com/olvrcc/xvn/releases/download/v1.6.2/xvn-x86_64-apple-darwin.tar.gz"
sha256 "..."
```

**New metadata**:
```ruby
desc "Automatic Node Version Switcher - fast, Rust-based version switching"
homepage "https://github.com/olvrcc/anvs"
url "https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-x86_64-apple-darwin.tar.gz"
sha256 "TO_BE_UPDATED_AFTER_RELEASE"
```

**Important**: The `url` and `sha256` will need to be updated AFTER the v2.0.0 release is published with artifacts.

**Changes Required**:
- `desc`: Update description (optional, can keep similar)
- `homepage`: Change `olvrcc/xvn` to `olvrcc/anvs`
- `url`: Change repository name and artifact name:
  - `olvrcc/xvn` → `olvrcc/anvs`
  - `xvn-x86_64-apple-darwin.tar.gz` → `anvs-x86_64-apple-darwin.tar.gz`
  - Version will be `v2.0.0` (update after release)
- `sha256`: Will be updated after v2.0.0 release (use placeholder for now)

**Actions**:
- [ ] Update `desc` to reference "anvs" if needed
- [ ] Update `homepage` to `https://github.com/olvrcc/anvs`
- [ ] Update `url` repository name: `olvrcc/xvn` → `olvrcc/anvs`
- [ ] Update `url` artifact name: `xvn-*.tar.gz` → `anvs-*.tar.gz`
- [ ] Add comment: `# TODO: Update sha256 after v2.0.0 release`
- [ ] Update version in `url` to `v2.0.0` (when ready)

---

### Task 10.7: Update Formula Binary Installation

**File**: `Formula/anvs.rb`

Update the binary name in the installation block.

**Current installation block** (approximate lines 10-15):
```ruby
def install
  bin.install "xvn"
end
```

or possibly:

```ruby
def install
  bin.install "bin/xvn"
end
```

**New installation block**:
```ruby
def install
  bin.install "anvs"
end
```

or:

```ruby
def install
  bin.install "bin/anvs"
end
```

**Changes Required**:
- Change binary name from `xvn` to `anvs` in `bin.install` statement
- If there are multiple install statements, update all references

**Actions**:
- [ ] Find the `def install` section
- [ ] Change `bin.install "xvn"` to `bin.install "anvs"`
- [ ] Or change `bin.install "bin/xvn"` to `bin.install "bin/anvs"`
- [ ] Check for any other install statements (shell scripts, etc.)
- [ ] Save file

---

### Task 10.8: Update Formula Test Block

**File**: `Formula/anvs.rb`

Update the test command to use the new binary name.

**Current test block** (approximate lines 17-20):
```ruby
test do
  assert_match version.to_s, shell_output("#{bin}/xvn --version")
end
```

**New test block**:
```ruby
test do
  assert_match version.to_s, shell_output("#{bin}/anvs --version")
end
```

**Changes Required**:
- Change `#{bin}/xvn` to `#{bin}/anvs` in test command
- Update any other test assertions if present

**Actions**:
- [ ] Find the `test do` section
- [ ] Change `#{bin}/xvn` to `#{bin}/anvs`
- [ ] Verify test logic still makes sense
- [ ] Save file

---

### Task 10.9: Update Formula License (If Present)

**File**: `Formula/anvs.rb`

Some formulas include a `license` field. If present, verify it's still accurate.

**Current** (if present):
```ruby
license "MIT"
```

**Actions**:
- [ ] Check if `license` field exists
- [ ] If exists, verify license is still correct (should be MIT)
- [ ] If not present, optionally add: `license "MIT"`

---

### Task 10.10: Review Complete Formula File

**File**: `Formula/anvs.rb`

Review the complete formula to ensure all references updated.

**Complete example formula** (after all updates):
```ruby
class Anvs < Formula
  desc "Automatic Node Version Switcher - fast, Rust-based version switching"
  homepage "https://github.com/olvrcc/anvs"
  url "https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-x86_64-apple-darwin.tar.gz"
  sha256 "TO_BE_UPDATED_AFTER_RELEASE"
  license "MIT"

  def install
    bin.install "anvs"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/anvs --version")
  end
end
```

**Search for any remaining old references**:
```bash
# In the tap repository, search for "xvn"
grep -n "xvn" Formula/anvs.rb

# Should find NO matches (exit code 1 is expected)
```

**Expected output**:
```
(no output - no matches found)
```

**Actions**:
- [ ] Search formula for "xvn" references: `grep -i "xvn" Formula/anvs.rb`
- [ ] Verify NO matches found (all updated to "anvs")
- [ ] Verify class name is `Anvs`
- [ ] Verify homepage is `github.com/olvrcc/anvs`
- [ ] Verify binary name is `anvs`
- [ ] Verify test uses `anvs`
- [ ] Validate Ruby syntax: `ruby -c Formula/anvs.rb`

---

### Task 10.11: Update Tap README

**File**: `README.md`

Update the tap repository's README to reflect the new name and usage.

**Current README** might contain:
```markdown
# Homebrew XVN

Homebrew tap for [xvn](https://github.com/olvrcc/xvn) - Automatic Node Version Switcher.

## Installation

```bash
brew install olvrcc/xvn/xvn
```

## Usage

After installation, run:
```bash
xvn setup
```
```

**New README**:
```markdown
# Homebrew ANVS

Homebrew tap for [anvs](https://github.com/olvrcc/anvs) - Automatic Node Version Switcher.

## Installation

Install the latest version:
```bash
brew install olvrcc/anvs/anvs
```

Or add the tap first:
```bash
brew tap olvrcc/anvs
brew install anvs
```

## Usage

After installation, run:
```bash
anvs setup
```

This will configure shell integration for automatic Node.js version switching.

## Links

- Main repository: https://github.com/olvrcc/anvs
- npm package: https://www.npmjs.com/package/anvs
- Documentation: https://github.com/olvrcc/anvs#readme

## Migrating from XVN

If you previously installed `xvn` via Homebrew:

```bash
brew uninstall xvn
brew untap olvrcc/xvn  # Optional
brew install olvrcc/anvs/anvs
anvs setup
```

See the [migration guide](https://github.com/olvrcc/anvs/blob/main/docs/XVN_TO_ANVS_MIGRATION.md) for details.
```

**Changes Required**:
- Title: "Homebrew XVN" → "Homebrew ANVS"
- All repository URLs: `olvrcc/xvn` → `olvrcc/anvs`
- All tap references: `olvrcc/xvn` → `olvrcc/anvs`
- All binary references: `xvn` → `anvs`
- Add migration section for existing users

**Actions**:
- [ ] Open `README.md`
- [ ] Update title to "Homebrew ANVS"
- [ ] Update repository links
- [ ] Update installation commands
- [ ] Update tap name
- [ ] Update binary name in usage examples
- [ ] Add migration section
- [ ] Save file

---

### Task 10.12: Check for Other Documentation Files

**Search for any other files that might need updating**:

```bash
# List all files in tap repository
find . -type f -not -path './.git/*' | sort

# Search all files for "xvn" references
grep -r "xvn" . --exclude-dir=.git
```

**Common files to check**:
- `README.md` (updated in Task 10.11)
- `LICENSE` (usually doesn't need changes)
- `.github/workflows/*` (if tap has CI/CD)
- Any other `.md` files

**Actions**:
- [ ] List all files in repository
- [ ] Search for "xvn" references
- [ ] Update any found references (except in git history)
- [ ] Verify LICENSE file exists and is correct

---

### Task 10.13: Validate Formula Syntax

**Before committing, validate the formula**:

```bash
# Check Ruby syntax
ruby -c Formula/anvs.rb

# Expected output:
# Syntax OK

# If you have Homebrew installed, audit the formula
brew audit Formula/anvs.rb

# Or from within Homebrew:
brew audit --formula anvs

# Note: Some warnings are expected until the formula is published
# with correct sha256 and version
```

**Expected validations**:
- [ ] Ruby syntax is valid
- [ ] No obvious errors in formula structure
- [ ] Class name matches file name

**Actions**:
- [ ] Run `ruby -c Formula/anvs.rb`
- [ ] Verify "Syntax OK" output
- [ ] Optionally run `brew audit` (warnings expected)
- [ ] Fix any syntax errors found

---

### Task 10.14: Commit Tap Repository Changes

**Commit all changes to the tap repository**:

```bash
# Ensure you're in tap repository
cd ~/path/to/homebrew-anvs

# Review all changes
git status

# Should see:
# - renamed: Formula/xvn.rb -> Formula/anvs.rb
# - modified: README.md
# - possibly other modified files

# Stage all changes
git add .

# Commit with descriptive message
git commit -m "feat: rename xvn to anvs - update formula and tap

Changes:
- Rename Formula/xvn.rb to Formula/anvs.rb
- Update class name: Xvn -> Anvs
- Update homepage: olvrcc/xvn -> olvrcc/anvs
- Update binary name: xvn -> anvs
- Update test command
- Update README with new tap name and migration guide
- Update artifact URLs (sha256 to be updated after v2.0.0 release)

BREAKING CHANGE: Formula renamed from xvn to anvs"

# View commit
git log --oneline -1

# Push to GitHub
git push origin main
```

**Actions**:
- [ ] Stage all changes: `git add .`
- [ ] Review changes: `git status`, `git diff --cached`
- [ ] Commit with descriptive message
- [ ] Include file list in commit message
- [ ] Push to remote: `git push origin main`
- [ ] Verify push succeeds

---

### Task 10.15: Update Repository Metadata on GitHub

**Update tap repository description and topics**:

1. **Go to tap repository homepage**: https://github.com/olvrcc/homebrew-anvs

2. **Click the gear icon** (⚙️) next to **About** section

3. **Update description**:
   - **New**: "Homebrew tap for anvs - Automatic Node Version Switcher for Node.js"

4. **Update topics/tags**:
   - Add: `homebrew-tap`, `anvs`, `node-version-switcher`, `homebrew-formula`
   - Keep: `nodejs`, `rust`, `nvm`, `fnm`, `homebrew`
   - Remove: `xvn` (if present)

**Actions**:
- [ ] Click gear icon in About section
- [ ] Update description to reference "anvs"
- [ ] Add/update topics
- [ ] Remove old "xvn" topic
- [ ] Save changes

---

### Task 10.16: Test Formula Installation (Post-Release)

**⚠️ NOTE**: This task can only be completed AFTER v2.0.0 is released with artifacts.

**After v2.0.0 release, update sha256 and test**:

```bash
# Download the artifact from GitHub release
curl -L -o anvs-x86_64-apple-darwin.tar.gz \
  https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-x86_64-apple-darwin.tar.gz

# Calculate SHA256
shasum -a 256 anvs-x86_64-apple-darwin.tar.gz

# Copy the hash and update Formula/anvs.rb

# Clean up
rm anvs-x86_64-apple-darwin.tar.gz

# Commit sha256 update
git add Formula/anvs.rb
git commit -m "chore(formula): update sha256 for v2.0.0"
git push origin main
```

**Test installation**:
```bash
# Uninstall any existing installation
brew uninstall anvs 2>/dev/null || true

# Uninstall old xvn if present
brew uninstall xvn 2>/dev/null || true

# Untap old tap if present
brew untap olvrcc/xvn 2>/dev/null || true

# Tap the new repository
brew tap olvrcc/anvs

# Install from tap
brew install olvrcc/anvs/anvs

# Or install directly
brew install olvrcc/anvs/anvs

# Verify installation
which anvs

# Check version
anvs --version

# Should show: anvs 2.0.0

# Test functionality
anvs setup
```

**Expected results**:
- [ ] Formula taps successfully
- [ ] Installation completes without errors
- [ ] Binary available at `/usr/local/bin/anvs` or `/opt/homebrew/bin/anvs`
- [ ] `anvs --version` shows `2.0.0`
- [ ] `anvs setup` works

**Actions (after v2.0.0 release)**:
- [ ] Download v2.0.0 artifact
- [ ] Calculate sha256
- [ ] Update `Formula/anvs.rb` with correct sha256
- [ ] Commit and push sha256 update
- [ ] Test tap installation
- [ ] Test binary execution
- [ ] Test version command
- [ ] Verify formula works end-to-end

---

## Verification Checklist

Before proceeding to Phase 11, verify ALL of the following:

- [ ] Tap repository renamed: `homebrew-xvn` → `homebrew-anvs`
- [ ] Local tap remote URL updated
- [ ] Formula file renamed: `Formula/xvn.rb` → `Formula/anvs.rb`
- [ ] Formula class name updated: `Xvn` → `Anvs`
- [ ] Formula homepage updated to `github.com/olvrcc/anvs`
- [ ] Formula artifact URLs updated (xvn-* → anvs-*)
- [ ] Formula binary name updated: `xvn` → `anvs`
- [ ] Formula test command updated
- [ ] Tap README updated with new name and instructions
- [ ] All changes committed and pushed
- [ ] Repository description/topics updated
- [ ] No "xvn" references remain in tap repository: `grep -r "xvn" .`
- [ ] Ruby syntax valid: `ruby -c Formula/anvs.rb`
- [ ] (Post-release) SHA256 updated and tested

---

## Success Criteria

Phase 10 is complete when:

1. ✅ Homebrew tap repository renamed from `homebrew-xvn` to `homebrew-anvs`
2. ✅ Formula renamed from `xvn.rb` to `anvs.rb`
3. ✅ Formula class name updated to `Anvs`
4. ✅ All formula metadata updated (homepage, URLs, binary name)
5. ✅ Tap README updated with migration guide
6. ✅ All changes committed and pushed to GitHub
7. ✅ Repository metadata updated
8. ✅ (Post-release) Formula tested and working with Homebrew

---

## Next Steps

After completing Phase 10:

1. **Phase 11**: Create comprehensive migration guide
   - Document step-by-step migration for xvn users
   - Create `docs/XVN_TO_ANVS_MIGRATION.md`
   - Optionally create migration helper script

2. **Phase 12**: Build, test, and publish v2.0.0
   - Local build and testing
   - Commit and tag v2.0.0
   - CI/CD build
   - npm publication
   - Return to Phase 10, Task 10.16 to update sha256 and test Homebrew formula

3. **Note**: The Homebrew formula won't be fully functional until v2.0.0 is published with artifacts. The sha256 update (Task 10.16) will be done after Phase 12.

---

## Rollback Plan

If issues are discovered after tap rename:

1. **Rename tap repository back** (if necessary):
   - Go to `homebrew-anvs` settings on GitHub
   - Rename back to `homebrew-xvn`
   - Update local remote: `git remote set-url origin https://github.com/olvrcc/homebrew-xvn.git`

2. **Rename formula file back**:
   ```bash
   git mv Formula/anvs.rb Formula/xvn.rb
   # Revert changes in formula file
   git checkout HEAD~1 -- Formula/xvn.rb
   git commit -m "revert: rollback to xvn formula"
   git push origin main
   ```

3. **Fix issues** and re-attempt when ready

**Note**: Rollback should only be done if critical issues are discovered. After v2.0.0 is published to npm, rolling back becomes much more complex.

---

## Notes

- Tap repository renaming follows the same pattern as main repository
- GitHub's automatic redirects mean old tap URL will continue to work
- Formula testing requires v2.0.0 release artifacts to be published first
- The tap can be updated incrementally - rename first, test after release
- Homebrew automatically handles the `homebrew-` prefix in tap names
- Installation: `brew install olvrcc/anvs/anvs` (tap/formula)
- After tapping: `brew install anvs` (shorter form)

---

## Common Issues and Solutions

### Issue: Formula audit fails

**Solution**:
```bash
# Run audit to see specific issues
brew audit --formula anvs

# Common issues:
# - SHA256 mismatch (update after release)
# - URL 404 (wait for release to be published)
# - Class name mismatch (ensure class Anvs matches filename)

# Fix issues and re-audit
ruby -c Formula/anvs.rb
brew audit --formula anvs
```

### Issue: Installation fails with "formula not found"

**Solution**:
```bash
# Update Homebrew
brew update

# Untap and retap
brew untap olvrcc/anvs
brew tap olvrcc/anvs

# Try installation again
brew install olvrcc/anvs/anvs

# Or use full path
brew install https://raw.githubusercontent.com/olvrcc/homebrew-anvs/main/Formula/anvs.rb
```

### Issue: Binary not found after installation

**Solution**:
```bash
# Check where Homebrew installed it
brew list anvs

# Check if binary is in PATH
echo $PATH | grep -o "/usr/local/bin\|/opt/homebrew/bin"

# Link manually if needed
brew link anvs

# Or check installation location
which anvs
```

### Issue: SHA256 mismatch

**Solution**:
```bash
# Recalculate SHA256 from actual release artifact
curl -L -O https://github.com/olvrcc/anvs/releases/download/v2.0.0/anvs-x86_64-apple-darwin.tar.gz
shasum -a 256 anvs-x86_64-apple-darwin.tar.gz

# Update Formula/anvs.rb with correct hash
# Commit and push update
git add Formula/anvs.rb
git commit -m "fix(formula): correct sha256 for v2.0.0"
git push origin main
```

### Issue: Old xvn formula still showing

**Solution**:
```bash
# Untap old tap
brew untap olvrcc/xvn

# Update Homebrew
brew update

# Tap new tap
brew tap olvrcc/anvs

# Search for formula
brew search anvs
```

---

## Homebrew Tap Naming Convention Reference

**Repository naming**:
- Repository must be named: `homebrew-<tap_name>`
- Example: `homebrew-anvs`

**Tap naming**:
- Homebrew strips the `homebrew-` prefix
- Tap becomes: `<user>/<tap_name>`
- Example: `olvrcc/anvs`

**Formula naming**:
- Formula file: `Formula/<formula_name>.rb`
- Example: `Formula/anvs.rb`
- Class name must match (capitalized): `class Anvs < Formula`

**Installation commands**:
- Full: `brew install <user>/<tap_name>/<formula_name>`
- Example: `brew install olvrcc/anvs/anvs`
- After tapping: `brew tap olvrcc/anvs` then `brew install anvs`

---

## Timeline

**Estimated time for Phase 10**: 30-45 minutes (+ testing time after release)

**Breakdown**:
- Task 10.1 (Prepare): 3-5 minutes
- Task 10.2 (Rename tap repo on GitHub): 2-3 minutes
- Task 10.3 (Update local remote): 2-3 minutes
- Task 10.4 (Rename formula file): 1-2 minutes
- Task 10.5 (Update class name): 1-2 minutes
- Task 10.6 (Update metadata): 3-5 minutes
- Task 10.7 (Update binary): 1-2 minutes
- Task 10.8 (Update test): 1-2 minutes
- Task 10.9 (Update license): 1 minute
- Task 10.10 (Review formula): 3-5 minutes
- Task 10.11 (Update README): 5-10 minutes
- Task 10.12 (Check other files): 2-3 minutes
- Task 10.13 (Validate syntax): 2-3 minutes
- Task 10.14 (Commit changes): 3-5 minutes
- Task 10.15 (Update metadata): 2-3 minutes
- Task 10.16 (Test - post-release): 10-15 minutes

**Total**: 30-45 minutes for initial work, + 10-15 minutes after v2.0.0 release for SHA256 update and testing
