# Renaming XVN to ANVS - Implementation Plan

## Introduction

This document outlines the comprehensive plan for renaming the **xvn** project to **anvs** (Automatic Node Version Switcher). This is a major rebrand that involves changing the package name, binary name, configuration paths, and all associated documentation and infrastructure.

### Why ANVS?

- **Better Name**: "Automatic Node Version Switcher" is more descriptive than "Extreme Version Switcher"
- **Unnamespaced npm Package**: `anvs` is available as an unnamespaced package (vs `@olvrcc/xvn`)
- **Clearer Purpose**: The name immediately communicates what the tool does
- **Tribute to avn**: The name pays homage to the original `avn` project while being distinct

### What We're Changing

**From:**
- Package: `@olvrcc/xvn`
- Binary: `xvn`
- Install path: `~/.xvn/`
- Config files: `~/.xvnrc`, `.xvn.yaml`
- Repository: `github.com/olvrcc/xvn`
- Homebrew tap: `olvrcc/homebrew-xvn`

**To:**
- Package: `anvs` (unnamespaced!)
- Binary: `anvs`
- Install path: `~/.anvs/`
- Config files: `~/.anvsrc`, `.anvs.yaml`
- Repository: `github.com/olvrcc/anvs`
- Homebrew tap: `olvrcc/homebrew-anvs`

### Scope of Changes

This is a **breaking change** that will require:
- 100+ file modifications across the codebase
- Repository rename on GitHub
- New npm package publication
- Homebrew tap repository rename and formula update
- Migration guide for existing users
- Version bump to `v2.0.0` (major version)

### Prerequisites Verified

✅ **npm package name `anvs` is available** (404 - not in registry)
✅ Current repository structure analyzed and documented
✅ All file references catalogued

---

## Implementation Plan

This plan is divided into logical phases that can be executed step-by-step.

### Phase 0: Final XVN Release (Deprecation Notice)

**Goal**: Publish a final version of `xvn` that notifies users of the upcoming rename and provides migration instructions.

#### 0.1 Create Deprecation README
- Create `README.DEPRECATION.md` with clear notice:
  - Project is being renamed to `anvs` (Automatic Node Version Switcher)
  - Why the rename (better name, unnamespaced npm package)
  - Timeline for deprecation
  - Migration instructions
  - Link to new `anvs` package once published
  - Note that `xvn` will continue to work but won't receive updates

#### 0.2 Update Current README
- Add prominent deprecation notice at the top of `README.md`
- Keep all existing documentation intact below the notice
- Add migration section linking to new package

#### 0.3 Version Bump and Release
- Bump version to `v1.7.0` (minor bump for documentation change)
- Update `CHANGELOG.md` with v1.7.0 entry noting deprecation
- Commit changes: `docs: add deprecation notice - project renamed to anvs`
- Create git tag `v1.7.0`
- Push to GitHub
- Wait for CI/CD to build binaries
- Publish to npm as `@olvrcc/xvn@1.7.0`
- This gives existing users one final update with clear migration path

#### 0.4 Post-Publication Actions
- Verify `@olvrcc/xvn@1.7.0` is available on npm
- Test that installation still works
- Verify README displays deprecation notice on npm package page
- Keep this version available indefinitely (do not unpublish)

**Important**: Complete this phase BEFORE starting the rename process. This ensures existing users get a clear migration message when they update.

---

### Phase 1: Core Configuration & Build Files

**Goal**: Update the fundamental package and build configuration files.

#### 1.1 Package Configuration
- Update `package.json`:
  - Change `name` from `"@olvrcc/xvn"` to `"anvs"`
  - Update `version` to `"2.0.0"`
  - Update `description` to reference "Automatic Node Version Switcher"
  - Update `bin` entry from `"xvn"` to `"anvs"`
  - Update repository URLs from `olvrcc/xvn` to `olvrcc/anvs`
  - Update homepage and bugs URLs

#### 1.2 Rust Configuration
- Update `Cargo.toml`:
  - Change package `name` to `"anvs"`
  - Update `version` to `"2.0.0"`
  - Update `description` to "Automatic Node Version Switcher for Node.js"
  - Update `[[bin]]` name from `"xvn"` to `"anvs"`

#### 1.3 Build & Lock Files
- Run `cargo build --release` to update `Cargo.lock` with new package name
- Verify build succeeds with new binary name

### Phase 2: Installation & Binary Files

**Goal**: Update all installation scripts and binary wrapper.

#### 2.1 JavaScript Installation Scripts
- Update `install.js`:
  - Change `XVN_DIR` constant to `ANVS_DIR`
  - Update directory path from `'.xvn'` to `'.anvs'`
  - Update binary name references from `xvn` to `anvs`
  - Update all user-facing messages and console output
  - Update cleanup function references

- Update `uninstall.js`:
  - Change `XVN_DIR` constant to `ANVS_DIR`
  - Change `XVN_CONFIG` to `ANVS_CONFIG`
  - Update paths: `.xvn` → `.anvs`, `.xvnrc` → `.anvsrc`
  - Update all console output messages

#### 2.2 Binary Wrapper
- Rename `bin/xvn` → `bin/anvs`
- Update file contents:
  - Change `XVN_BINARY` to `ANVS_BINARY`
  - Update path from `$HOME/.xvn/bin/xvn` to `$HOME/.anvs/bin/anvs`
  - Update error messages to reference `anvs`

### Phase 3: Shell Integration

**Goal**: Update the shell hook script with new names and paths.

#### 3.1 Shell Script Files
- Rename `shell/xvn.sh` → `shell/anvs.sh`
- Update file header comment
- Rename all functions:
  - `__xvn_debug` → `__anvs_debug`
  - `__xvn_find_file` → `__anvs_find_file`
  - `__xvn_activate` → `__anvs_activate`
  - `__xvn_chpwd` → `__anvs_chpwd`
  - `__xvn_original_cd` → `__anvs_original_cd`

#### 3.2 Environment Variables
- Rename all environment variables:
  - `XVN_SHELL_LOADED` → `ANVS_SHELL_LOADED`
  - `XVN_DEBUG` → `ANVS_DEBUG`
  - `XVN_VERSION_FILES` → `ANVS_VERSION_FILES`
  - `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`

#### 3.3 Shell Script Binary Calls
- Update binary invocations from `xvn activate` to `anvs activate`
- Update all debug messages and logging
- Update comments and documentation

### Phase 4: Rust Source Code

**Goal**: Update all Rust source files with new binary, directory, and config names.

#### 4.1 Library Root
- Update `src/lib.rs` documentation comment

#### 4.2 Configuration Module (`src/config/`)
- Update `loader.rs`:
  - Config file paths: `.xvnrc` → `.anvsrc`, `.xvn.yaml` → `.anvs.yaml`
  - Update all comments and documentation
- Update `schema.rs`: Configuration struct documentation
- Update `mod.rs`: Module-level documentation

#### 4.3 Setup Module (`src/setup/`)
- Update `installer.rs`: Installation paths `.xvn` → `.anvs`
- Update `profile_modification.rs`: Shell integration references
- Update `shell_detection.rs`: Shell script path references
- Update `mod.rs`: Setup documentation

#### 4.4 Commands Module (`src/commands/`)
- Update `set.rs`: Config file path `.xvnrc` → `.anvsrc`
- Update `uninstall.rs`: Directory and config file references
- Update `mod.rs`: Command documentation

#### 4.5 Init/Wizard Module (`src/init/`)
- Update `wizard.rs`: Config file path references
- Update `prompts.rs`: User-facing messages
- Update `detection.rs`: Binary name checks
- Update `validation.rs`: Validation messages
- Update `mod.rs`: Module documentation

#### 4.6 Activation Module (`src/activation/`)
- Update `errors.rs`: Error messages mentioning config paths
- Update `orchestrator.rs`: Activation logic documentation
- Update `user_prompt.rs`: User-facing prompts
- Update `mod.rs`: Module documentation

#### 4.7 Shell Module (`src/shell/`)
- Update `fd3.rs`: File descriptor protocol comments
- Update `json_writer.rs`: JSON output fields if applicable
- Update `mod.rs`: Module documentation

#### 4.8 CLI Module (`src/cli.rs`)
- Update CLI application name, description, and help text
- Update all command descriptions and examples
- Update version string formatting
- Update uninstall command description

#### 4.9 Plugins Module (`src/plugins/`)
- Update plugin documentation if it references binary name
- Update any error messages or logging

#### 4.10 Version File Module (`src/version_file/`)
- Update documentation and comments

### Phase 5: Test Files

**Goal**: Update all test files with new binary and package names.

#### 5.1 Integration Tests
- Update `tests/integration.rs`: Binary name in test commands
- Update `tests/cli_test.rs`: CLI assertions
- Update `tests/config_test.rs`: Config file path tests
- Update `tests/installer_test.rs`: Installation path tests
- Update `tests/shell_integration.rs`: Shell script tests
- Update all other test files as needed

#### 5.2 Shell Script Tests
- Rename `tests/shell/test_xvn_sh.sh` → `tests/shell/test_anvs_sh.sh`
- Update test assertions and expected output

### Phase 6: Documentation Files

**Goal**: Update all markdown documentation.

#### 6.1 Main Documentation
- **`README.md`** - Complete rewrite:
  - New title: "ANVS - Automatic Node Version Switcher"
  - Update badges with new repo URLs
  - Update tagline and description
  - Update installation commands (`npm install -g anvs`, `brew install anvs`)
  - Update all CLI examples (`anvs setup`, `anvs activate`, etc.)
  - Update configuration file references
  - Update troubleshooting section
  - Update development section

- **`CLAUDE.md`** - Update references:
  - Project overview section
  - Package name and binary name
  - Installation paths and config files
  - All command examples
  - Repository URLs

- **`CHANGELOG.md`** - Add v2.0.0 entry:
  - Document the rename as a breaking change
  - List what changed
  - Include migration guide link

- **`CONTRIBUTING.md`** - Update:
  - Repository URLs
  - Binary name in examples
  - Development commands

#### 6.2 Docs Directory (`docs/`)
- Update `docs/ARCHITECTURE.md`: Binary and package name references
- Update `docs/MIGRATION.md`: Add section for xvn→anvs migration
- Update `docs/HOMEBREW_SETUP.md`: Tap name and formula references
- Update `docs/TEST_REVIEW.md`: Test command references
- Update any other docs

#### 6.3 Spec Directory (`spec/`)
- **Do NOT update** historical planning documents (milestone-1 through milestone-11)
- These are historical reference and should remain as-is
- Update `spec/PROGRESS.md` only if tracking current work
- Update `spec/BACKLOG.md` if it contains future references

#### 6.4 Other Markdown Files
- Update `ROADMAP.md`: Package and binary name references
- Update `WARP.md`: Any tool-specific references

#### 6.5 Scripts Documentation
- Update `scripts/README.md`: Script usage examples with new binary name

#### 6.6 Homebrew Tap Documentation
- Update `homebrew-xvn/README.md`: Tap name, binary name, repo URLs

### Phase 7: Build & Release Scripts

**Goal**: Update all build, release, and version management scripts.

#### 7.1 Script Updates
- Update `scripts/bump-version.sh`: Package name references
- Update `scripts/version.sh`: Version detection logic if needed
- Update `scripts/download-artifacts.sh`: Artifact naming (`xvn-*` → `anvs-*`)
- Update `scripts/extract-binaries.sh`: Binary and artifact naming
- Update `scripts/setup-homebrew-tap.sh`: Tap repo and formula references
- Update `scripts/coverage.sh`: Binary name if referenced

### Phase 8: GitHub Workflows (CI/CD)

**Goal**: Update all GitHub Actions workflows.

#### 8.1 Build Workflow (`.github/workflows/build.yml`)
- Update artifact names: `xvn-*` → `anvs-*`
- Update binary name in build steps
- Update shell script path: `shell/xvn.sh` → `shell/anvs.sh`
- Update release asset names
- Update any job or step descriptions

#### 8.2 Test Workflow (`.github/workflows/test.yml`)
- Update shell script path for linting
- Update any test references

#### 8.3 Homebrew Update Workflow (`.github/workflows/update-homebrew.yml`)
- Update `HOMEBREW_TAP_REPO`: `olvrcc/homebrew-xvn` → `olvrcc/homebrew-anvs`
- Update `HOMEBREW_FORMULA_PATH`: `Formula/xvn.rb` → `Formula/anvs.rb`
- Update download URLs: `xvn-*.tar.gz` → `anvs-*.tar.gz`
- Update artifact names in workflow
- Update commit messages

### Phase 9: GitHub Repository Changes

**Goal**: Rename the main repository on GitHub.

#### 9.1 Repository Rename
- Navigate to GitHub repository settings
- Rename repository from `xvn` to `anvs`
- GitHub will automatically set up redirects from old URL

#### 9.2 Update Local Git Remote
- Update local git remote URL to new repository name
- Verify push/pull still works

#### 9.3 Repository Settings
- Update repository description
- Update repository topics/tags
- Update repository homepage URL (if set)

### Phase 10: Homebrew Tap Changes

**Goal**: Update or recreate the Homebrew tap.

#### 10.1 Tap Repository
- Rename `homebrew-xvn` repository → `homebrew-anvs` on GitHub
- Update local clone remote URL

#### 10.2 Homebrew Formula
- Rename `Formula/xvn.rb` → `Formula/anvs.rb` in tap repository
- Update formula contents:
  - Class name: `class Anvs < Formula`
  - Homepage URL
  - Download URLs (will be updated by CI once main repo renamed)
  - Binary name in `bin.install` statements
  - Shell script installation if referenced
  - Test command: `xvn --version` → `anvs --version`

#### 10.3 Tap Documentation
- Update tap README with new name and installation instructions
- Update any other tap-specific documentation

### Phase 11: Migration Guide

**Goal**: Create comprehensive migration documentation for existing users.

#### 11.1 Create Migration Document
- Create `docs/XVN_TO_ANVS_MIGRATION.md`
- Document step-by-step migration:
  1. Backup existing config (`~/.xvnrc`)
  2. Uninstall old xvn: `xvn uninstall` or manual removal
  3. Remove shell integration from `.bashrc`/`.zshrc`
  4. Install new anvs: `npm install -g anvs` or `brew install anvs`
  5. Run `anvs setup`
  6. Migrate config file manually or with helper script
  7. Test activation
- Include troubleshooting section
- Add FAQs

#### 11.2 Optional: Create Migration Script
- Create `scripts/migrate-xvn-to-anvs.sh` helper script
- Automate config file migration
- Verify installation paths
- Provide clear output and error messages

### Phase 12: Build, Test, and Publish

**Goal**: Build the renamed project and publish to npm and Homebrew.

#### 12.1 Local Build and Test
- Run full test suite: `cargo test`
- Run linting: `cargo clippy -- -D warnings`
- Run formatting check: `cargo fmt -- --check`
- Build release binary: `cargo build --release`
- Test binary locally: `cargo install --path .`
- Run `anvs setup` and verify shell integration
- Test activation in a test directory

#### 12.2 Commit and Tag
- Stage all changes: `git add .`
- Commit with message: `feat: rename xvn to anvs - v2.0.0 breaking change`
- Create annotated git tag: `git tag -a v2.0.0 -m "v2.0.0 - Rename to anvs"`
- Push commits: `git push`
- Push tag: `git push --tags`

#### 12.3 CI/CD Build
- Wait for GitHub Actions to build all platform binaries
- Monitor workflow progress
- Verify all builds succeed
- Download artifacts when complete

#### 12.4 npm Publication
- Download artifacts: `./scripts/download-artifacts.sh v2.0.0`
- Extract binaries: `./scripts/extract-binaries.sh v2.0.0`
- Verify binaries: `ls -lh native/*/anvs`
- Create package: `npm pack`
- Verify package contents: `npm run release:verify`
- Publish to npm: `npm publish --otp=<code>`
- Verify publication: `npm view anvs`

#### 12.5 Homebrew Tap Update
- GitHub Actions should automatically update Homebrew tap
- If not, run manually: `./scripts/setup-homebrew-tap.sh`
- Verify formula in tap repository
- Test Homebrew installation: `brew install olvrcc/anvs/anvs`

#### 12.6 GitHub Release
- Verify GitHub Release was created by CI
- Edit release notes to highlight the rename
- Add migration guide link to release notes
- Mark as a major version release

### Phase 13: Final Deprecation & Announcement

**Goal**: Fully deprecate the old package and communicate the change.

#### 13.1 Deprecate Old Package on npm
- Run: `npm deprecate @olvrcc/xvn "⚠️ RENAMED: This package is now 'anvs'. Install with: npm install -g anvs | See: https://github.com/olvrcc/anvs"`
- This shows a warning when users try to install the old package
- The v1.7.0 release already has deprecation in README

#### 13.2 Update Old Repository
- If repository was renamed (recommended):
  - GitHub automatically redirects `olvrcc/xvn` → `olvrcc/anvs`
  - Old links continue to work
- If keeping separate repository:
  - Add prominent deprecation notice
  - Link to new repository
  - Archive the old repository

#### 13.3 Announcements
- Create announcement issue in `anvs` repository
- Post to discussions/community channels
- Update any external references (blog posts, social media, etc.)
- Notify any known users or contributors
- Update package registry metadata if possible

#### 13.4 Monitor Migration
- Monitor issues for migration problems
- Help users with migration questions
- Document common issues in migration guide
- Consider keeping both packages available for 3-6 months before final deprecation

---

## Post-Rename Verification Checklist

After completing all phases, verify:

- [ ] `anvs --version` shows `2.0.0`
- [ ] `npm view anvs` shows the published package
- [ ] `anvs setup` creates `~/.anvsrc` and modifies shell profile
- [ ] Shell integration triggers on `cd` to project directory
- [ ] Config files load from `~/.anvsrc` and `.anvs.yaml`
- [ ] Installation path is `~/.anvs/bin/anvs`
- [ ] All CI/CD workflows pass
- [ ] Homebrew formula installs successfully
- [ ] Migration guide is clear and complete
- [ ] Old package is deprecated on npm
- [ ] Documentation is accurate and complete

---

## Rollback Plan

If critical issues are discovered:

1. Do not deprecate old `@olvrcc/xvn` package
2. Keep both packages available temporarily
3. Fix issues in `anvs` package
4. Republish with patch version
5. Complete deprecation only when stable

---

## Timeline Estimate

- **Phase 0** (Final xvn release): 30 minutes + CI/CD time
- **Phase 1-5** (Code changes): 2-3 hours
- **Phase 6** (Documentation): 1-2 hours
- **Phase 7-8** (Scripts & CI): 1 hour
- **Phase 9-10** (GitHub/Homebrew): 30 minutes
- **Phase 11** (Migration guide): 1 hour
- **Phase 12** (Build & publish): 1 hour
- **Phase 13** (Final deprecation): 30 minutes

**Total estimated time**: 8-11 hours of focused work

**Recommended Schedule**:
1. **Day 1**: Complete Phase 0 (final xvn release with deprecation notice)
2. **Day 2-3**: Complete Phases 1-8 (all code changes)
3. **Day 4**: Complete Phases 9-13 (infrastructure changes and publication)

---

## Notes

- This is a breaking change requiring major version bump to `v2.0.0`
- Existing users must manually migrate (no automatic upgrade path)
- Old `xvn` installations will continue to work but won't receive updates
- Consider maintaining `@olvrcc/xvn` for a deprecation period before archiving
- Test thoroughly in a clean environment before publishing
- The rename is irreversible once published to npm
