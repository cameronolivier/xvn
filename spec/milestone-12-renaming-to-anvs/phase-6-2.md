# Phase 6-2: Technical Documentation Updates

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 1-1.5 hours
**Depends On**: Phase 6-1 (main documentation) completed

## Overview

Phase 6-2 continues the documentation update work from Phase 6, focusing on **technical documentation** that developers and advanced users rely on. This phase updates architecture documentation, migration guides, Homebrew setup instructions, and test documentation.

**Why Phase 6-2 is Critical:**
- `docs/MIGRATION.md` needs comprehensive xvn→anvs migration guide for existing users
- `docs/ARCHITECTURE.md` contains technical details referenced by contributors
- `docs/HOMEBREW_SETUP.md` is essential for Homebrew tap maintenance
- `docs/TEST_REVIEW.md` guides contributors on testing practices

**⚠️ IMPORTANT**: This phase should begin after Phase 6-1 is complete (README.md, CLAUDE.md, CHANGELOG.md, CONTRIBUTING.md already updated).

**📋 Documentation Scope:**
- 1 critical migration guide (docs/MIGRATION.md)
- 3 technical documentation files
- All files must be updated before proceeding to Phase 7

---

## Implementation Tasks

### Task 6.2.1: Update docs/MIGRATION.md (Add xvn→anvs Section)

**File**: `docs/MIGRATION.md`

**⚠️ MOST IMPORTANT TASK**: This file needs a comprehensive xvn→anvs migration guide added at the top.

**Changes Required**:

1. **Add new migration section at the very top** (before existing v1.1.x→v1.2.0 section):

```markdown
# Migration Guide

## Migrating from xvn to anvs (v1.x → v2.0)

**IMPORTANT**: This is a breaking change. The project has been renamed from `xvn` to `anvs`.

### Why the Rename?

- **Better package name**: `anvs` is available unnamespaced on npm (vs `@olvrcc/xvn`)
- **Clearer purpose**: "Automatic Node Version Switcher" immediately communicates what the tool does
- **Improved discoverability**: Easier to find and remember
- **Tribute to avn**: Pays homage to the original project while being distinct

### What Changed

| Component | Old (xvn) | New (anvs) |
|-----------|-----------|------------|
| npm package | `@olvrcc/xvn` | `anvs` |
| Binary name | `xvn` | `anvs` |
| Install directory | `~/.xvn/` | `~/.anvs/` |
| User config | `~/.xvnrc` | `~/.anvsrc` |
| Project config | `.xvn.yaml` | `.anvs.yaml` |
| Shell script | `shell/xvn.sh` | `shell/anvs.sh` |
| Environment vars | `XVN_*` | `ANVS_*` |
| Repository | `github.com/olvrcc/xvn` | `github.com/olvrcc/anvs` |
| Homebrew tap | `olvrcc/xvn` | `olvrcc/anvs` |

### Migration Steps

#### Step 1: Backup Your Configuration

```bash
# Backup your xvn config (optional but recommended)
cp ~/.xvnrc ~/.xvnrc.backup

# Backup project-level config files if you use them
find . -name ".xvn.yaml" -exec cp {} {}.backup \;
```

#### Step 2: Uninstall Old xvn

```bash
# Option 1: Use xvn's uninstall command (recommended)
xvn uninstall

# Option 2: Manual uninstall
npm uninstall -g @olvrcc/xvn

# Remove shell integration from ~/.bashrc or ~/.zshrc
# Look for and remove lines like:
# [ -s "$HOME/.xvn/bin/xvn.sh" ] && . "$HOME/.xvn/bin/xvn.sh"

# Optionally remove old installation directory
rm -rf ~/.xvn
```

#### Step 3: Install New anvs

```bash
# Via npm (recommended)
npm install -g anvs

# Or via Homebrew
brew install olvrcc/anvs/anvs

# Run setup to configure shell integration
anvs setup
```

#### Step 4: Migrate Configuration

**Option A: Copy your old config** (if you had custom settings):
```bash
# Copy contents from backup to new config file
cp ~/.xvnrc.backup ~/.anvsrc

# Or manually edit
nano ~/.anvsrc
```

**Option B: Start fresh** (if you used defaults):
```bash
# anvs will create a default config on first run
# Customize with:
anvs set config.default_version
anvs set plugins.priority
```

**Project-level config files**:
```bash
# Rename .xvn.yaml files to .anvs.yaml in your projects
# In each project directory:
mv .xvn.yaml .anvs.yaml

# Or use find to batch rename:
find ~/projects -name ".xvn.yaml" -execdir mv {} .anvs.yaml \;
```

#### Step 5: Reload Shell

```bash
# Reload your shell configuration
source ~/.bashrc  # or source ~/.zshrc

# Verify anvs is loaded
which anvs
# Should show: /Users/yourusername/.anvs/bin/anvs
```

#### Step 6: Verify Installation

```bash
# Check version
anvs --version
# Should show: anvs 2.0.0

# Check status
anvs status

# Test activation in a project with .nvmrc
cd your-project
# Should auto-activate the Node.js version
```

### Troubleshooting Migration

#### Shell integration not working

```bash
# Check if shell script is sourced
grep -i "anvs" ~/.bashrc ~/.zshrc

# Should see line like:
# [ -s "$HOME/.anvs/bin/anvs.sh" ] && . "$HOME/.anvs/bin/anvs.sh"

# If missing, run setup again
anvs setup
```

#### Old xvn references remain in shell

```bash
# Check for and remove old xvn references
grep -i "xvn" ~/.bashrc ~/.zshrc

# Manually remove any lines containing .xvn
# Then reload shell
source ~/.bashrc  # or source ~/.zshrc
```

#### Config not loading

```bash
# Verify config file exists
ls -la ~/.anvsrc

# Check config syntax
cat ~/.anvsrc

# Test with explicit config path
ANVS_DEBUG=1 anvs status
```

#### Both xvn and anvs installed

```bash
# Check what's in your PATH
which xvn
which anvs

# If both present, make sure anvs takes precedence
# Or fully uninstall xvn:
npm uninstall -g @olvrcc/xvn
rm -rf ~/.xvn
```

### Rollback (If Needed)

If you need to go back to xvn:

```bash
# Uninstall anvs
anvs uninstall
npm uninstall -g anvs

# Reinstall old xvn
npm install -g @olvrcc/xvn@1.7.0
xvn setup

# Restore backup config
cp ~/.xvnrc.backup ~/.xvnrc
```

### FAQ

**Q: Will my old xvn installation stop working?**
A: No, existing installations continue to work. However, xvn won't receive updates.

**Q: Can I keep both xvn and anvs installed?**
A: Not recommended. They use similar shell hooks and may conflict. Choose one.

**Q: Do I need to update my project's .nvmrc files?**
A: No, .nvmrc files remain unchanged. anvs reads the same version files as xvn.

**Q: What about my nvm/fnm installation?**
A: No changes needed. anvs works with your existing version manager.

**Q: Is there an automatic migration script?**
A: Not currently. Manual migration ensures you understand the changes.

**Q: Where can I get help?**
A: Open an issue at https://github.com/olvrcc/anvs/issues

---

## Upgrading to xvn v1.2+ (Historical)

*Note: The section below is for historical reference only. For current migrations, see the xvn→anvs section above.*
```

2. **Keep the existing v1.1.x→v1.2.0 section** below the new section (it's historical reference)

**Commands**:
```bash
# View current file structure
head -30 docs/MIGRATION.md

# After changes, verify new section is at top
head -150 docs/MIGRATION.md
```

**Actions**:
- [ ] Add new "xvn to anvs" migration section at top
- [ ] Include comprehensive step-by-step instructions
- [ ] Document all changes in table format
- [ ] Provide backup instructions
- [ ] Include troubleshooting section (4 common issues)
- [ ] Add FAQ section (6 questions)
- [ ] Include rollback instructions
- [ ] Mark existing v1.2+ section as "Historical"
- [ ] Verify all commands are accurate
- [ ] Test migration steps are complete

---

### Task 6.2.2: Update docs/ARCHITECTURE.md

**File**: `docs/ARCHITECTURE.md`

**Changes Required**:

1. **Update document title and introduction**:
   ```markdown
   # Before:
   # XVN Architecture

   This document describes the architecture of **xvn**...

   # After:
   # ANVS Architecture

   This document describes the architecture of **anvs** (Automatic Node Version Switcher)...
   ```

2. **Update all binary name references** throughout:
   - `xvn` → `anvs`
   - `xvn activate` → `anvs activate`
   - `xvn setup` → `anvs setup`

3. **Update all file paths**:
   - `~/.xvn/` → `~/.anvs/`
   - `~/.xvnrc` → `~/.anvsrc`
   - `.xvn.yaml` → `.anvs.yaml`
   - `shell/xvn.sh` → `shell/anvs.sh`
   - `bin/xvn` → `bin/anvs`

4. **Update environment variables**:
   - `XVN_DEBUG` → `ANVS_DEBUG`
   - `XVN_SHELL_LOADED` → `ANVS_SHELL_LOADED`
   - All `XVN_*` → `ANVS_*`

5. **Update code examples**:
   - Shell function names: `__xvn_*` → `__anvs_*`
   - Configuration constants in examples
   - Package name: `@olvrcc/xvn` → `anvs`

**Commands**:
```bash
# Review current state
grep -n "xvn" docs/ARCHITECTURE.md | wc -l
grep -n "XVN" docs/ARCHITECTURE.md | wc -l

# After changes, verify
grep -i "xvn" docs/ARCHITECTURE.md  # Should return no results
grep -i "anvs" docs/ARCHITECTURE.md | head -10
```

**Actions**:
- [ ] Update document title and introduction
- [ ] Update all binary name references
- [ ] Update all file and directory paths
- [ ] Update all environment variables
- [ ] Update code examples (shell scripts, Rust code)
- [ ] Update function names in examples
- [ ] Update package name references
- [ ] Update architecture diagrams if present
- [ ] Verify all technical accuracy maintained
- [ ] Verify no "xvn" or "XVN" references remain

---

### Task 6.2.3: Update docs/HOMEBREW_SETUP.md

**File**: `docs/HOMEBREW_SETUP.md`

**Changes Required**:

1. **Update title**:
   ```markdown
   # Before:
   # Homebrew Setup for XVN

   # After:
   # Homebrew Setup for ANVS
   ```

2. **Update tap references**:
   ```markdown
   # Before:
   brew tap olvrcc/xvn
   brew install olvrcc/xvn/xvn

   # After:
   brew tap olvrcc/anvs
   brew install olvrcc/anvs/anvs
   ```

3. **Update formula name**:
   - `Formula/xvn.rb` → `Formula/anvs.rb`

4. **Update formula class example**:
   ```ruby
   # Before:
   class Xvn < Formula
     desc "Extreme Version Switcher for Node.js"
     homepage "https://github.com/olvrcc/xvn"

   # After:
   class Anvs < Formula
     desc "Automatic Node Version Switcher"
     homepage "https://github.com/olvrcc/anvs"
   ```

5. **Update repository URLs**:
   - `homebrew-xvn` → `homebrew-anvs`
   - `olvrcc/xvn` → `olvrcc/anvs`

**Commands**:
```bash
# Review current state
grep -n "xvn" docs/HOMEBREW_SETUP.md

# After changes
grep -i "xvn" docs/HOMEBREW_SETUP.md  # Should return no results
```

**Actions**:
- [ ] Update document title
- [ ] Update tap name (olvrcc/xvn → olvrcc/anvs)
- [ ] Update formula name (xvn.rb → anvs.rb)
- [ ] Update formula class name (Xvn → Anvs)
- [ ] Update repository URLs
- [ ] Update all installation examples
- [ ] Update all code examples
- [ ] Verify no "xvn" references remain

---

### Task 6.2.4: Update docs/TEST_REVIEW.md

**File**: `docs/TEST_REVIEW.md`

**Changes Required**:

1. **Update test command references**:
   ```bash
   # Before:
   ./target/debug/xvn --version

   # After:
   ./target/debug/anvs --version
   ```

2. **Update test file paths**:
   - `tests/shell/test_xvn_sh.sh` → `tests/shell/test_anvs_sh.sh`

3. **Update test output examples**:
   ```
   # Before:
   xvn 1.6.2

   # After:
   anvs 2.0.0
   ```

4. **Update environment variables**:
   - `XVN_DEBUG` → `ANVS_DEBUG`

**Commands**:
```bash
# Review current state
grep -n "xvn" docs/TEST_REVIEW.md

# After changes
grep -i "xvn" docs/TEST_REVIEW.md  # Should return no results
```

**Actions**:
- [ ] Update all test command examples
- [ ] Update binary name in test output
- [ ] Update test file paths
- [ ] Update environment variables
- [ ] Verify all examples accurate
- [ ] Verify no "xvn" references remain

---

## Verification Checklist

Before proceeding to Phase 6-3, verify ALL of the following:

### Critical Files
- [ ] docs/MIGRATION.md has comprehensive xvn→anvs section at top
- [ ] docs/MIGRATION.md includes all 6 migration steps
- [ ] docs/MIGRATION.md includes troubleshooting (4 sections)
- [ ] docs/MIGRATION.md includes FAQ (6 questions)
- [ ] docs/ARCHITECTURE.md title updated
- [ ] docs/ARCHITECTURE.md all paths updated
- [ ] docs/HOMEBREW_SETUP.md tap name updated
- [ ] docs/TEST_REVIEW.md test commands updated

### Verification Commands
- [ ] No "xvn" in ARCHITECTURE: `grep -i "xvn" docs/ARCHITECTURE.md`
- [ ] No "XVN_" in ARCHITECTURE: `grep "XVN_" docs/ARCHITECTURE.md`
- [ ] No ".xvnrc" refs: `grep -r "\.xvnrc" docs/*.md`
- [ ] No ".xvn" directory refs: `grep -r "\.xvn[^s]" docs/*.md`
- [ ] All repo URLs updated: `grep -r "olvrcc/xvn" docs/*.md`

### Quality Checks
- [ ] All markdown renders correctly
- [ ] Migration guide is clear and comprehensive
- [ ] All technical details accurate
- [ ] Code examples use correct syntax

---

## Success Criteria

Phase 6-2 is complete when:

1. ✅ docs/MIGRATION.md has comprehensive xvn→anvs migration guide
2. ✅ docs/ARCHITECTURE.md fully updated with anvs references
3. ✅ docs/HOMEBREW_SETUP.md updated for new tap
4. ✅ docs/TEST_REVIEW.md test commands updated
5. ✅ No incorrect "xvn" references in any docs/ files
6. ✅ All verification commands pass
7. ✅ Migration guide tested for accuracy

---

## Next Steps

After completing Phase 6-2:

1. **Commit technical documentation changes**:
   ```bash
   git add docs/*.md
   git commit -m "docs: update technical documentation for anvs rename (Phase 6-2)

   Files changed:
   - docs/MIGRATION.md: Added comprehensive xvn→anvs migration guide
   - docs/ARCHITECTURE.md: Updated all technical references
   - docs/HOMEBREW_SETUP.md: Updated Homebrew tap instructions
   - docs/TEST_REVIEW.md: Updated test command examples"
   ```

2. **Proceed to Phase 6-3**: Supplementary documentation updates (ROADMAP, WARP, AGENTS, scripts/README, spec verification)

---

## Rollback Plan

If issues are discovered:

1. **Identify specific issues**: What documentation is incorrect?
2. **Fix incrementally**: Update specific sections rather than reverting all
3. **Test fixes**: Verify markdown renders correctly
4. **Re-commit**: Apply fixes in separate commit for clarity

---

## Notes

- **Most important**: docs/MIGRATION.md needs full xvn→anvs guide (Task 6.2.1)
- This is high-impact documentation that users will reference during migration
- ARCHITECTURE.md is referenced by contributors - must be accurate
- Test all migration steps if possible before committing
- Migration guide should be clear enough for non-technical users
- Consider adding diagrams to MIGRATION.md if helpful

**Estimated time breakdown**:
- docs/MIGRATION.md (comprehensive guide): 30-40 minutes
- docs/ARCHITECTURE.md: 15-20 minutes
- docs/HOMEBREW_SETUP.md: 10 minutes
- docs/TEST_REVIEW.md: 5-10 minutes
- **Total**: 1-1.5 hours

---
