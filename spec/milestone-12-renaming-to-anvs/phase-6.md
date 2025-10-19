# Phase 6: Documentation Files

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 2-3 hours

## Overview

Phase 6 updates all project documentation to reflect the rename from **xvn** to **anvs**. This is one of the most extensive phases, covering user-facing documentation, architecture docs, migration guides, and contributing guidelines. Clear, accurate documentation is critical for user adoption and developer onboarding.

**Why Phase 6 is Critical:**
- Documentation is the primary way users learn about and interact with the project
- Outdated references to `xvn` will cause confusion and support issues
- README.md is displayed on npm package page and GitHub repository
- Architecture and migration docs are essential for developers and advanced users
- Inconsistent naming across docs creates a poor user experience

**⚠️ IMPORTANT**: Phase 6 should begin after Phases 1-5 are complete (code changes done). This ensures documentation accurately reflects the codebase state.

**📋 Documentation Scope:**
- 6 main documentation files (README.md, CLAUDE.md, CHANGELOG.md, etc.)
- 4 docs/ directory files
- 3 other markdown files
- 2 supplementary documentation files
- **Note**: Historical spec/ documents (milestone-1 through milestone-11) should NOT be updated

---

## Implementation Tasks

### Task 6.1: Update README.md (Main User Documentation)

**File**: `README.md`

**This is the most important documentation file** - it appears on both npm and GitHub.

**Changes Required**:

1. **Replace deprecation banner** (lines ~1-7):
   ```markdown
   # Before (current deprecation notice):
   > **⚠️ DEPRECATION NOTICE**: This package has been renamed to [`anvs`]...

   # After (remove deprecation, update title):
   # ANVS - Automatic Node Version Switcher

   [![npm version](https://badge.fury.io/js/anvs.svg)](https://www.npmjs.com/package/anvs)
   [![GitHub](https://img.shields.io/github/license/olvrcc/anvs)](https://github.com/olvrcc/anvs)
   ```

2. **Update title and tagline** (line ~9):
   ```markdown
   # Before:
   # XVN - Extreme Version Switcher for Node.js

   # After:
   # ANVS - Automatic Node Version Switcher
   ```

3. **Update description paragraph** (lines ~11-15):
   ```markdown
   # Before:
   **xvn** is a fast, automatic Node.js version switcher...

   # After:
   **anvs** is a fast, automatic Node.js version switcher built in Rust. It automatically detects and activates the correct Node.js version when you `cd` into a project directory, making it seamless to work across projects with different Node.js requirements.
   ```

4. **Update badges** (lines ~17-20):
   - npm badge: `https://badge.fury.io/js/@olvrcc/xvn.svg` → `https://badge.fury.io/js/anvs.svg`
   - npm link: `https://www.npmjs.com/package/@olvrcc/xvn` → `https://www.npmjs.com/package/anvs`
   - GitHub license: `olvrcc/xvn` → `olvrcc/anvs`
   - GitHub repo: `olvrcc/xvn` → `olvrcc/anvs`

5. **Update Features section** (lines ~22-35):
   - Change all `xvn` references to `anvs`
   - Update paths: `~/.xvn/` → `~/.anvs/`, `~/.xvnrc` → `~/.anvsrc`, `.xvn.yaml` → `.anvs.yaml`

6. **Update Installation section** (lines ~37-60):
   ```markdown
   # Before:
   npm install -g @olvrcc/xvn

   # After:
   npm install -g anvs
   ```

   ```markdown
   # Before:
   brew install olvrcc/xvn/xvn

   # After:
   brew install olvrcc/anvs/anvs
   ```

7. **Update Quick Start section** (lines ~62-85):
   ```markdown
   # Before:
   xvn setup
   xvn activate
   xvn status

   # After:
   anvs setup
   anvs activate
   anvs status
   ```

8. **Update Configuration section** (lines ~87-120):
   - Update config file paths: `~/.xvnrc` → `~/.anvsrc`, `.xvn.yaml` → `.anvs.yaml`
   - Update example YAML with new filename
   - Update `xvn set` commands → `anvs set`

9. **Update Commands section** (lines ~122-180):
   - All command examples: `xvn` → `anvs`
   - All output examples referencing xvn

10. **Update Troubleshooting section** (lines ~182-220):
    - Update debug commands: `XVN_DEBUG=1` → `ANVS_DEBUG=1`
    - Update file paths in troubleshooting steps
    - Update shell integration verification commands

11. **Update Development section** (lines ~222-280):
    ```markdown
    # Before:
    cargo install --path .
    xvn setup
    xvn --version

    # After:
    cargo install --path .
    anvs setup
    anvs --version
    ```

12. **Update Contributing section** (lines ~282-300):
    - Repository URL: `github.com/olvrcc/xvn` → `github.com/olvrcc/anvs`
    - Issue tracker URL updates

13. **Update License section** (footer):
    - Repository references if present

14. **Remove Migration section** (if present from v1.7.0):
    - Delete the "Migration to ANVS" section that was added in the deprecation notice

**Commands**:
```bash
# Review current references
grep -n "xvn" README.md | wc -l
grep -n "@olvrcc/xvn" README.md
grep -n "\.xvn" README.md
grep -n "\.xvnrc" README.md
grep -n "XVN_" README.md

# After changes, verify all xvn references removed
grep -i "xvn" README.md  # Should return no results (or only historical context)
grep -i "anvs" README.md | head -10  # Should show new references
```

**Actions**:
- [ ] Update title to "ANVS - Automatic Node Version Switcher"
- [ ] Remove deprecation notice banner
- [ ] Update npm and GitHub badges with new URLs
- [ ] Update tagline and description
- [ ] Update all installation commands (npm and Homebrew)
- [ ] Update all CLI command examples (setup, activate, status, etc.)
- [ ] Update all config file paths (.xvnrc → .anvsrc, .xvn.yaml → .anvs.yaml)
- [ ] Update all directory paths (~/.xvn → ~/.anvs)
- [ ] Update all environment variables (XVN_* → ANVS_*)
- [ ] Update repository URLs (olvrcc/xvn → olvrcc/anvs)
- [ ] Update all code examples and output samples
- [ ] Remove migration section if present
- [ ] Verify no "xvn" references remain (except in historical context)
- [ ] Test all markdown formatting renders correctly

---

### Task 6.2: Update CLAUDE.md (Project Instructions for Claude Code)

**File**: `CLAUDE.md`

**Changes Required**:

1. **Update Project Overview section** (lines ~7-15):
   ```markdown
   # Before:
   **xvn** is a Rust-based automatic Node.js version switcher...

   # After:
   **anvs** is a Rust-based automatic Node.js version switcher that activates on `cd`, designed to be 2-3x faster than its predecessor (avn).
   ```

2. **Update Key Characteristics** (lines ~17-22):
   ```markdown
   # Before:
   - Distribution: npm with pre-compiled binaries + Homebrew tap

   # After:
   - Distribution: npm (@olvrcc/anvs was previously @olvrcc/xvn) with pre-compiled binaries + Homebrew tap
   ```

3. **Update Project Status section** (lines ~28-35):
   - Update version to v2.0.0
   - Update npm package name: `@olvrcc/xvn` → `anvs`
   - Update Homebrew tap: `olvrcc/xvn` → `olvrcc/anvs`

4. **Update Current Features section** (lines ~45-80):
   - Update config paths: `~/.xvnrc` → `~/.anvsrc`, `./.xvn.yaml` → `./.anvs.yaml`
   - Update all command examples

5. **Update Configuration Files section** (lines ~110-115):
   ```markdown
   # Before:
   - `~/.xvnrc` - User-level configuration (YAML)
   - `.xvn.yaml` - Project-level configuration overrides

   # After:
   - `~/.anvsrc` - User-level configuration (YAML)
   - `.anvs.yaml` - Project-level configuration overrides
   ```

6. **Update Common Development Commands section** (lines ~118-185):
   ```bash
   # Before:
   xvn setup
   xvn activate [path]
   xvn status
   xvn set <key>
   xvn uninstall
   xvn --version

   # After:
   anvs setup
   anvs activate [path]
   anvs status
   anvs set <key>
   anvs uninstall
   anvs --version
   ```

7. **Update Important Constraints & Conventions** (lines ~235-250):
   ```markdown
   # Before:
   - **Central installation:** xvn installs to `~/.xvn/bin`...

   # After:
   - **Central installation:** anvs installs to `~/.anvs/bin` to remain available across Node.js version changes
   - **Not a version manager:** anvs requires nvm/fnm to be installed; it's a switcher, not a manager
   ```

8. **Update all code examples and file paths** throughout document

**Commands**:
```bash
# Review current references
grep -n "xvn" CLAUDE.md | wc -l
grep -n "XVN" CLAUDE.md | wc -l

# After changes, verify
grep -i "xvn" CLAUDE.md  # Should return no results
grep -i "anvs" CLAUDE.md | head -10
```

**Actions**:
- [ ] Update project name and description in Overview
- [ ] Update package name references (npm and Homebrew)
- [ ] Update all command examples (setup, activate, status, etc.)
- [ ] Update all config file paths
- [ ] Update all directory paths
- [ ] Update all environment variables
- [ ] Update version to v2.0.0
- [ ] Update constraints section
- [ ] Verify all code blocks updated
- [ ] Verify no "xvn" or "XVN" references remain

---

### Task 6.3: Update CHANGELOG.md (Add v2.0.0 Entry)

**File**: `CHANGELOG.md`

**Changes Required**:

1. **Add new v2.0.0 entry at the top** (after the header):
   ```markdown
   ## [2.0.0] - 2025-10-XX

   ### ⚠️ BREAKING CHANGES

   **Project Renamed: xvn → anvs**

   This is a major breaking change. The project has been renamed from `xvn` to `anvs` (Automatic Node Version Switcher).

   **What Changed:**
   - Package name: `@olvrcc/xvn` → `anvs` (unnamespaced on npm)
   - Binary name: `xvn` → `anvs`
   - Install directory: `~/.xvn/` → `~/.anvs/`
   - Config files: `~/.xvnrc` → `~/.anvsrc`, `.xvn.yaml` → `.anvs.yaml`
   - Repository: `github.com/olvrcc/xvn` → `github.com/olvrcc/anvs`
   - Homebrew tap: `olvrcc/xvn` → `olvrcc/anvs`

   **Migration Required:**
   Existing users must manually migrate. See [docs/XVN_TO_ANVS_MIGRATION.md](./docs/XVN_TO_ANVS_MIGRATION.md) for step-by-step instructions.

   **Quick migration:**
   ```bash
   # Uninstall old version
   xvn uninstall
   npm uninstall -g @olvrcc/xvn

   # Install new version
   npm install -g anvs
   anvs setup

   # Migrate config (if you had custom settings)
   mv ~/.xvnrc ~/.anvsrc
   # In project directories: rename .xvn.yaml → .anvs.yaml
   ```

   **Why the rename:**
   - Better package name (unnamespaced on npm)
   - Clearer purpose ("Automatic Node Version Switcher")
   - Improved discoverability
   - Tribute to the original `avn` project

   ### Changed
   - Renamed all binaries, scripts, and configuration files
   - Updated all documentation and examples
   - Updated GitHub repository and Homebrew tap
   - Updated npm package to unnamespaced `anvs`

   ### Migration Guide
   - See [docs/XVN_TO_ANVS_MIGRATION.md](./docs/XVN_TO_ANVS_MIGRATION.md)
   - Old `@olvrcc/xvn@1.7.0` package will remain available but deprecated
   - No automatic migration path - manual migration required

   ---
   ```

2. **Keep all existing changelog entries** (v1.7.0, v1.6.2, etc.) - do not modify

3. **Update version links at bottom** (if present):
   ```markdown
   # Add new link:
   [2.0.0]: https://github.com/olvrcc/anvs/compare/v1.7.0...v2.0.0

   # Update other links if they reference olvrcc/xvn:
   [1.7.0]: https://github.com/olvrcc/anvs/compare/v1.6.2...v1.7.0
   ```

**Commands**:
```bash
# View current CHANGELOG structure
head -50 CHANGELOG.md

# After changes, verify new entry is at top
head -80 CHANGELOG.md
```

**Actions**:
- [ ] Add v2.0.0 entry at top of CHANGELOG
- [ ] Document breaking changes clearly
- [ ] Include migration instructions
- [ ] Explain why the rename happened
- [ ] List all major changes
- [ ] Link to migration guide
- [ ] Update version comparison links if present
- [ ] Verify markdown formatting is correct

---

### Task 6.4: Update CONTRIBUTING.md

**File**: `CONTRIBUTING.md`

**Changes Required**:

1. **Update repository URLs** (throughout document):
   ```markdown
   # Before:
   https://github.com/olvrcc/xvn

   # After:
   https://github.com/olvrcc/anvs
   ```

2. **Update issue tracker references**:
   ```markdown
   # Before:
   https://github.com/olvrcc/xvn/issues

   # After:
   https://github.com/olvrcc/anvs/issues
   ```

3. **Update binary name in development examples** (lines ~30-60):
   ```markdown
   # Before:
   cargo build
   cargo install --path .
   xvn --version

   # After:
   cargo build
   cargo install --path .
   anvs --version
   ```

4. **Update package name references**:
   ```markdown
   # Before:
   The project is published as `@olvrcc/xvn` on npm.

   # After:
   The project is published as `anvs` on npm.
   ```

5. **Update PR and commit guidelines** (if they mention specific commands):
   - Update any example commit messages mentioning xvn
   - Update any PR templates that reference the old name

**Commands**:
```bash
# Review current references
grep -n "xvn" CONTRIBUTING.md
grep -n "olvrcc/xvn" CONTRIBUTING.md

# After changes
grep -i "xvn" CONTRIBUTING.md  # Should return no results
```

**Actions**:
- [ ] Update all repository URLs (github.com/olvrcc/xvn → github.com/olvrcc/anvs)
- [ ] Update issue tracker links
- [ ] Update PR template if it exists
- [ ] Update binary name in examples
- [ ] Update package name references
- [ ] Verify all links are valid
- [ ] Verify no "xvn" references remain

---

### Task 6.5: Update docs/ARCHITECTURE.md

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

2. **Update binary name references** throughout:
   - `xvn` → `anvs` in all examples
   - `xvn activate` → `anvs activate`
   - `xvn setup` → `anvs setup`

3. **Update file paths** in architecture diagrams and examples:
   - `~/.xvn/` → `~/.anvs/`
   - `~/.xvnrc` → `~/.anvsrc`
   - `.xvn.yaml` → `.anvs.yaml`
   - `shell/xvn.sh` → `shell/anvs.sh`
   - `bin/xvn` → `bin/anvs`

4. **Update environment variables** in examples:
   - `XVN_DEBUG` → `ANVS_DEBUG`
   - `XVN_SHELL_LOADED` → `ANVS_SHELL_LOADED`
   - Any other `XVN_*` variables

5. **Update code examples and diagrams**:
   - Shell function names: `__xvn_*` → `__anvs_*`
   - Configuration constants in Rust code examples
   - Any ASCII diagrams or flowcharts

6. **Update package references**:
   ```markdown
   # Before:
   The package is distributed as `@olvrcc/xvn`...

   # After:
   The package is distributed as `anvs`...
   ```

**Commands**:
```bash
# Review current state
grep -n "xvn" docs/ARCHITECTURE.md | wc -l
grep -n "XVN" docs/ARCHITECTURE.md | wc -l

# After changes
grep -i "xvn" docs/ARCHITECTURE.md  # Should return no results
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

### Task 6.6: Update docs/MIGRATION.md (Add xvn→anvs Section)

**File**: `docs/MIGRATION.md`

**Changes Required**:

1. **Add new section at the top** (this is the most important migration guide):
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
   ```

2. **Keep existing migration guides** for other version upgrades (if any)

**Commands**:
```bash
# View current structure
head -30 docs/MIGRATION.md

# After changes
head -100 docs/MIGRATION.md
```

**Actions**:
- [ ] Add new "xvn to anvs" migration section at top
- [ ] Include comprehensive step-by-step instructions
- [ ] Document all changes in table format
- [ ] Provide backup instructions
- [ ] Include troubleshooting section
- [ ] Add FAQ section
- [ ] Include rollback instructions
- [ ] Verify all commands are accurate
- [ ] Test migration steps are complete

---

### Task 6.7: Update docs/HOMEBREW_SETUP.md

**File**: `docs/HOMEBREW_SETUP.md`

**Changes Required**:

1. **Update title and introduction**:
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

3. **Update formula name references**:
   ```markdown
   # Before:
   Formula file: `Formula/xvn.rb`

   # After:
   Formula file: `Formula/anvs.rb`
   ```

4. **Update repository URLs**:
   - Tap repository: `homebrew-xvn` → `homebrew-anvs`
   - Formula repo: `olvrcc/xvn` → `olvrcc/anvs`
   - GitHub URLs throughout

5. **Update binary and package references** in all examples

6. **Update formula code examples**:
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

### Task 6.8: Update docs/TEST_REVIEW.md

**File**: `docs/TEST_REVIEW.md`

**Changes Required**:

1. **Update test command references**:
   ```bash
   # Before:
   cargo test
   ./target/debug/xvn --version

   # After:
   cargo test
   ./target/debug/anvs --version
   ```

2. **Update test file paths** if mentioned:
   - `tests/shell/test_xvn_sh.sh` → `tests/shell/test_anvs_sh.sh`

3. **Update any test output examples** that show xvn:
   ```
   # Before:
   running 23 tests
   xvn 1.6.2

   # After:
   running 23 tests
   anvs 2.0.0
   ```

4. **Update environment variable references**:
   - `XVN_DEBUG` → `ANVS_DEBUG`
   - Any other `XVN_*` variables in test examples

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

### Task 6.9: Update ROADMAP.md

**File**: `ROADMAP.md`

**Changes Required**:

1. **Update title and introduction**:
   ```markdown
   # Before:
   # XVN Roadmap

   # After:
   # ANVS Roadmap
   ```

2. **Update package name references**:
   ```markdown
   # Before:
   The `xvn` project aims to...

   # After:
   The `anvs` project aims to...
   ```

3. **Add v2.0.0 milestone** at the top:
   ```markdown
   ## v2.0.0 - Project Rename (Completed)

   - [x] Rename project from xvn to anvs
   - [x] Update npm package to unnamespaced `anvs`
   - [x] Update Homebrew tap
   - [x] Comprehensive migration documentation
   - [x] Deprecate old `@olvrcc/xvn` package
   ```

4. **Update all future version references** to use `anvs`:
   - Update command examples
   - Update feature descriptions
   - Update any technical references

**Commands**:
```bash
# Review current state
grep -n "xvn" ROADMAP.md

# After changes
grep -i "xvn" ROADMAP.md  # Should only show historical context
```

**Actions**:
- [ ] Update document title
- [ ] Add v2.0.0 rename milestone
- [ ] Update all package name references
- [ ] Update all future feature descriptions
- [ ] Update all command examples
- [ ] Verify chronological accuracy
- [ ] Verify no incorrect "xvn" references

---

### Task 6.10: Update WARP.md

**File**: `WARP.md`

**This file contains Warp terminal-specific configuration and examples.**

**Changes Required**:

1. **Update command examples**:
   ```bash
   # Before:
   xvn setup
   xvn activate

   # After:
   anvs setup
   anvs activate
   ```

2. **Update binary and package references**:
   ```markdown
   # Before:
   Install xvn: npm install -g @olvrcc/xvn

   # After:
   Install anvs: npm install -g anvs
   ```

3. **Update any Warp-specific workflow examples**:
   - Update snippets that reference xvn
   - Update keyboard shortcuts or aliases if they mention xvn

4. **Update file paths** if present:
   - `~/.xvn/` → `~/.anvs/`
   - Configuration files

**Commands**:
```bash
# Review current state
grep -n "xvn" WARP.md

# After changes
grep -i "xvn" WARP.md  # Should return no results
```

**Actions**:
- [ ] Update all command examples
- [ ] Update package installation instructions
- [ ] Update file paths
- [ ] Update workflow examples
- [ ] Verify no "xvn" references remain

---

### Task 6.11: Update AGENTS.md

**File**: `AGENTS.md`

**This file documents AI agent workflows.**

**Changes Required**:

1. **Update agent examples** that reference xvn:
   ```markdown
   # Before:
   Agent: "Installing xvn..."

   # After:
   Agent: "Installing anvs..."
   ```

2. **Update any command examples**:
   - `xvn` → `anvs` in all examples

3. **Update package references**:
   - `@olvrcc/xvn` → `anvs`

**Commands**:
```bash
# Review current state
grep -n "xvn" AGENTS.md

# After changes
grep -i "xvn" AGENTS.md  # Should return no results
```

**Actions**:
- [ ] Update all agent workflow examples
- [ ] Update command references
- [ ] Update package name
- [ ] Verify no "xvn" references remain

---

### Task 6.12: Update scripts/README.md

**File**: `scripts/README.md`

**Changes Required**:

1. **Update script descriptions**:
   ```markdown
   # Before:
   Build and publish xvn to npm

   # After:
   Build and publish anvs to npm
   ```

2. **Update command examples**:
   ```bash
   # Before:
   ./scripts/download-artifacts.sh v1.6.2
   # Downloads: xvn-x86_64-apple-darwin.tar.gz

   # After:
   ./scripts/download-artifacts.sh v2.0.0
   # Downloads: anvs-x86_64-apple-darwin.tar.gz
   ```

3. **Update artifact name references**:
   - `xvn-*.tar.gz` → `anvs-*.tar.gz`

4. **Update binary name references**:
   - `native/*/xvn` → `native/*/anvs`

**Commands**:
```bash
# Review current state
grep -n "xvn" scripts/README.md

# After changes
grep -i "xvn" scripts/README.md  # Should return no results
```

**Actions**:
- [ ] Update all script descriptions
- [ ] Update command examples
- [ ] Update artifact name patterns
- [ ] Update binary paths
- [ ] Verify all examples accurate
- [ ] Verify no "xvn" references remain

---

### Task 6.13: Update homebrew-xvn/README.md

**File**: `homebrew-xvn/README.md`

**⚠️ NOTE**: This file is in a separate repository (`homebrew-xvn`). You may need to update it there, or if it's a submodule/included here, update it now.

**Changes Required**:

1. **Update repository name** (if this is a local copy):
   - Repository should be renamed to `homebrew-anvs`
   - This file should be in that new repository

2. **Update tap installation**:
   ```bash
   # Before:
   brew tap olvrcc/xvn
   brew install xvn

   # After:
   brew tap olvrcc/anvs
   brew install anvs
   ```

3. **Update formula reference**:
   ```markdown
   # Before:
   Formula: `Formula/xvn.rb`

   # After:
   Formula: `Formula/anvs.rb`
   ```

4. **Update all binary and package references**

**Commands**:
```bash
# If this directory exists locally:
ls -la homebrew-xvn/

# Review current state
grep -n "xvn" homebrew-xvn/README.md 2>/dev/null

# After changes
grep -i "xvn" homebrew-xvn/README.md  # Should return no results
```

**Actions**:
- [ ] Verify if homebrew-xvn/ exists in this repo
- [ ] Update tap installation instructions
- [ ] Update formula name references
- [ ] Update binary name
- [ ] Update repository URLs
- [ ] Verify no "xvn" references remain
- [ ] Note: Full Homebrew tap update is in Phase 10

---

### Task 6.14: Verify Spec Directory (No Changes Required)

**Directory**: `spec/`

**⚠️ IMPORTANT**: Do NOT update historical planning documents!

**Files to SKIP** (keep as-is for historical reference):
- `spec/milestone-1/` through `spec/milestone-11/` - Historical, do not modify
- Any `PLAN.md`, `SPEC.md`, `TASKS.md` in old milestones
- `spec/PROJECT_PLAN.md` - Historical project planning
- `spec/PROJECT_SPEC.md` - Historical specifications

**Files you MAY update** (if they reference current work):
- `spec/BACKLOG.md` - Update if it mentions xvn for future features
- `spec/PROGRESS.md` - Update if tracking current milestone progress

**Actions**:
- [ ] Review `spec/BACKLOG.md` - update only future references to xvn
- [ ] Review `spec/PROGRESS.md` - update if needed for current progress tracking
- [ ] Verify historical milestones (1-11) remain unchanged
- [ ] Verify milestone-12 planning documents remain unchanged (they document the rename plan itself)

**Commands**:
```bash
# Verify historical docs are untouched
git status spec/milestone-{1..11}/

# Review backlog for current references
grep -n "xvn" spec/BACKLOG.md

# If backlog has future references, update them
# If progress tracking mentions xvn, update to anvs
```

---

## Verification Checklist

Before proceeding to Phase 7, verify ALL of the following:

### Main Documentation
- [ ] README.md updated with new title, badges, commands, paths
- [ ] README.md has no deprecation banner (that was for v1.7.0)
- [ ] CLAUDE.md project overview updated
- [ ] CLAUDE.md all commands and paths updated
- [ ] CHANGELOG.md has v2.0.0 entry with breaking changes documented
- [ ] CONTRIBUTING.md repository URLs updated

### Docs Directory
- [ ] docs/ARCHITECTURE.md binary and paths updated
- [ ] docs/MIGRATION.md has comprehensive xvn→anvs guide
- [ ] docs/HOMEBREW_SETUP.md tap and formula updated
- [ ] docs/TEST_REVIEW.md test commands updated

### Other Documentation
- [ ] ROADMAP.md updated with v2.0.0 milestone
- [ ] WARP.md commands and examples updated
- [ ] AGENTS.md examples updated
- [ ] scripts/README.md script documentation updated

### Verification Commands
- [ ] No "xvn" in README.md: `grep -i "xvn" README.md`
- [ ] No "XVN_" in CLAUDE.md: `grep "XVN_" CLAUDE.md`
- [ ] No ".xvnrc" references: `grep -r "\.xvnrc" *.md docs/*.md`
- [ ] No ".xvn" directory references: `grep -r "\.xvn[^s]" *.md docs/*.md`
- [ ] All repository URLs updated: `grep -r "olvrcc/xvn" *.md docs/*.md`

### Quality Checks
- [ ] All markdown files render correctly (no broken formatting)
- [ ] All links are valid (no 404s)
- [ ] All code examples use correct syntax
- [ ] Migration guide is clear and comprehensive
- [ ] Historical documentation (spec/milestone-1 through 11) unchanged

---

## Success Criteria

Phase 6 is complete when:

1. ✅ All user-facing documentation updated (README, CONTRIBUTING)
2. ✅ All technical documentation updated (ARCHITECTURE, MIGRATION)
3. ✅ CHANGELOG.md has v2.0.0 entry with breaking changes
4. ✅ Migration guide comprehensive and tested
5. ✅ All repository URLs point to new location
6. ✅ All commands and examples use `anvs` instead of `xvn`
7. ✅ All file paths updated (.xvnrc → .anvsrc, etc.)
8. ✅ All environment variables updated (XVN_* → ANVS_*)
9. ✅ No incorrect "xvn" references remain (verified with grep)
10. ✅ Historical documentation preserved unchanged

---

## Next Steps

After completing Phase 6:

1. **Review all changes**: Do a final pass to ensure consistency
2. **Test markdown rendering**: View README.md on GitHub preview
3. **Commit documentation changes**:
   ```bash
   git add *.md docs/*.md scripts/README.md
   git commit -m "docs: update all documentation for anvs rename"
   ```
4. **Proceed to Phase 7**: Build & Release Scripts

---

## Rollback Plan

If documentation issues are discovered:

1. **Identify specific issues**: What documentation is incorrect?
2. **Fix incrementally**: Update specific files rather than reverting all
3. **Test fixes**: Verify markdown renders correctly
4. **Re-commit**: Apply fixes in separate commit for clarity

**Note**: Documentation changes are low-risk and easily reversible.

---

## Notes

- Phase 6 is the longest phase due to volume of documentation
- Take breaks to avoid fatigue and maintain accuracy
- Use grep/search to verify completeness
- Test markdown rendering as you go
- Migration guide is critical - ensure it's clear and complete
- README.md is most important (shown on npm and GitHub)
- Keep commits organized by documentation type (main docs, technical docs, etc.)
- Consider splitting into multiple commits if changes are extensive

**Estimated time breakdown**:
- Main docs (README, CLAUDE, CHANGELOG): 45 minutes
- Technical docs (ARCHITECTURE, MIGRATION): 45 minutes
- Other docs and verification: 30-60 minutes
- **Total**: 2-3 hours

---
