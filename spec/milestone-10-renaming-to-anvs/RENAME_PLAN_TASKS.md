# XVN to ANVS Rename - Task Checklist

Track progress on renaming the project from **xvn** to **anvs**.

---

## Phase 0: Final XVN Release (Deprecation Notice)

**⚠️ COMPLETE THIS PHASE FIRST - Before any rename work begins**

### 0.1 Create Deprecation README
- [ ] Create `README.DEPRECATION.md` file
- [ ] Document project rename to `anvs`
- [ ] Explain why (better name, unnamespaced package)
- [ ] Add timeline for deprecation
- [ ] Include migration instructions
- [ ] Add link to new package (once published)
- [ ] Note that `xvn` will continue to work but won't receive updates

### 0.2 Update Current README
- [ ] Add prominent deprecation notice banner at top of `README.md`
- [ ] Keep all existing documentation intact
- [ ] Add migration section linking to new package
- [ ] Ensure notice is visible on npm package page

### 0.3 Version Bump and Release
- [ ] Bump version to `v1.7.0` in `package.json`
- [ ] Bump version to `v1.7.0` in `Cargo.toml`
- [ ] Update `Cargo.lock`: `cargo build --release`
- [ ] Update `CHANGELOG.md` with v1.7.0 entry
- [ ] Commit: `docs: add deprecation notice - project renamed to anvs`
- [ ] Create git tag: `git tag -a v1.7.0 -m "v1.7.0 - Deprecation notice"`
- [ ] Push commits: `git push`
- [ ] Push tag: `git push --tags`
- [ ] Wait for CI/CD to build binaries
- [ ] Download artifacts: `./scripts/download-artifacts.sh v1.7.0`
- [ ] Extract binaries: `./scripts/extract-binaries.sh v1.7.0`
- [ ] Create package: `npm pack`
- [ ] Publish to npm: `npm publish --otp=<code>`

### 0.4 Post-Publication Actions
- [ ] Verify `@olvrcc/xvn@1.7.0` is on npm: `npm view @olvrcc/xvn`
- [ ] Test installation: `npm install -g @olvrcc/xvn`
- [ ] Verify deprecation notice shows on npm package page
- [ ] Test that binary still works: `xvn --version`
- [ ] Confirm README displays properly on npm

**CHECKPOINT**: Do not proceed to Phase 1 until Phase 0 is complete and published!

---

## Phase 1: Core Configuration & Build Files

### 1.1 Package Configuration
- [ ] Update `package.json` - change name to `"anvs"`
- [ ] Update `package.json` - bump version to `"2.0.0"`
- [ ] Update `package.json` - update description
- [ ] Update `package.json` - change bin entry to `"anvs"`
- [ ] Update `package.json` - update repository URLs to `olvrcc/anvs`
- [ ] Update `package.json` - update homepage and bugs URLs

### 1.2 Rust Configuration
- [ ] Update `Cargo.toml` - change package name to `"anvs"`
- [ ] Update `Cargo.toml` - bump version to `"2.0.0"`
- [ ] Update `Cargo.toml` - update description to "Automatic Node Version Switcher"
- [ ] Update `Cargo.toml` - change [[bin]] name to `"anvs"`

### 1.3 Build & Lock Files
- [ ] Run `cargo build --release` to update Cargo.lock
- [ ] Verify build succeeds with new binary name

---

## Phase 2: Installation & Binary Files

### 2.1 JavaScript Installation Scripts
- [ ] Update `install.js` - rename `XVN_DIR` to `ANVS_DIR`
- [ ] Update `install.js` - change path `.xvn` to `.anvs`
- [ ] Update `install.js` - update binary name from `xvn` to `anvs`
- [ ] Update `install.js` - update all console messages
- [ ] Update `uninstall.js` - rename `XVN_DIR` to `ANVS_DIR`
- [ ] Update `uninstall.js` - rename `XVN_CONFIG` to `ANVS_CONFIG`
- [ ] Update `uninstall.js` - update paths `.xvn` → `.anvs`, `.xvnrc` → `.anvsrc`
- [ ] Update `uninstall.js` - update all console messages

### 2.2 Binary Wrapper
- [ ] Rename `bin/xvn` to `bin/anvs`
- [ ] Update `bin/anvs` - rename `XVN_BINARY` to `ANVS_BINARY`
- [ ] Update `bin/anvs` - change path to `$HOME/.anvs/bin/anvs`
- [ ] Update `bin/anvs` - update error messages

---

## Phase 3: Shell Integration

### 3.1 Shell Script Files
- [ ] Rename `shell/xvn.sh` to `shell/anvs.sh`
- [ ] Update `shell/anvs.sh` - update file header comment
- [ ] Update `shell/anvs.sh` - rename `__xvn_debug` to `__anvs_debug`
- [ ] Update `shell/anvs.sh` - rename `__xvn_find_file` to `__anvs_find_file`
- [ ] Update `shell/anvs.sh` - rename `__xvn_activate` to `__anvs_activate`
- [ ] Update `shell/anvs.sh` - rename `__xvn_chpwd` to `__anvs_chpwd`
- [ ] Update `shell/anvs.sh` - rename `__xvn_original_cd` to `__anvs_original_cd`

### 3.2 Environment Variables
- [ ] Update shell script - rename `XVN_SHELL_LOADED` to `ANVS_SHELL_LOADED`
- [ ] Update shell script - rename `XVN_DEBUG` to `ANVS_DEBUG`
- [ ] Update shell script - rename `XVN_VERSION_FILES` to `ANVS_VERSION_FILES`
- [ ] Update shell script - rename `XVN_ACTIVE_KEY` to `ANVS_ACTIVE_KEY`

### 3.3 Shell Script Binary Calls
- [ ] Update shell script - change `xvn activate` to `anvs activate`
- [ ] Update shell script - update all debug messages
- [ ] Update shell script - update all comments

---

## Phase 4: Rust Source Code

### 4.1 Library Root
- [ ] Update `src/lib.rs` - documentation comment

### 4.2 Configuration Module
- [ ] Update `src/config/loader.rs` - `.xvnrc` → `.anvsrc`
- [ ] Update `src/config/loader.rs` - `.xvn.yaml` → `.anvs.yaml`
- [ ] Update `src/config/loader.rs` - all comments
- [ ] Update `src/config/schema.rs` - documentation
- [ ] Update `src/config/mod.rs` - module documentation

### 4.3 Setup Module
- [ ] Update `src/setup/installer.rs` - `.xvn` → `.anvs` paths
- [ ] Update `src/setup/profile_modification.rs` - shell script references
- [ ] Update `src/setup/shell_detection.rs` - shell script path
- [ ] Update `src/setup/mod.rs` - module documentation

### 4.4 Commands Module
- [ ] Update `src/commands/set.rs` - `.xvnrc` → `.anvsrc`
- [ ] Update `src/commands/uninstall.rs` - directory and config paths
- [ ] Update `src/commands/mod.rs` - documentation

### 4.5 Init/Wizard Module
- [ ] Update `src/init/wizard.rs` - config file paths
- [ ] Update `src/init/prompts.rs` - user messages
- [ ] Update `src/init/detection.rs` - binary name checks
- [ ] Update `src/init/validation.rs` - validation messages
- [ ] Update `src/init/mod.rs` - module documentation

### 4.6 Activation Module
- [ ] Update `src/activation/errors.rs` - error messages with paths
- [ ] Update `src/activation/orchestrator.rs` - documentation
- [ ] Update `src/activation/user_prompt.rs` - user prompts
- [ ] Update `src/activation/mod.rs` - module documentation

### 4.7 Shell Module
- [ ] Update `src/shell/fd3.rs` - comments
- [ ] Update `src/shell/json_writer.rs` - JSON fields if needed
- [ ] Update `src/shell/mod.rs` - module documentation

### 4.8 CLI Module
- [ ] Update `src/cli.rs` - CLI app name
- [ ] Update `src/cli.rs` - description and help text
- [ ] Update `src/cli.rs` - all command descriptions
- [ ] Update `src/cli.rs` - version string formatting

### 4.9 Plugins Module
- [ ] Update `src/plugins/` - documentation and messages

### 4.10 Version File Module
- [ ] Update `src/version_file/` - documentation and comments

---

## Phase 5: Test Files

### 5.1 Integration Tests
- [ ] Update `tests/integration.rs` - binary name in commands
- [ ] Update `tests/cli_test.rs` - CLI assertions
- [ ] Update `tests/config_test.rs` - config file paths
- [ ] Update `tests/installer_test.rs` - installation paths
- [ ] Update `tests/shell_integration.rs` - shell script tests
- [ ] Update `tests/plugin_test.rs` - plugin references
- [ ] Update `tests/plugin_loading_test.rs` - plugin loading
- [ ] Update `tests/error_test.rs` - error messages
- [ ] Update `tests/version_file_test.rs` - version file tests
- [ ] Update `tests/security_test.rs` - security tests

### 5.2 Shell Script Tests
- [ ] Rename `tests/shell/test_xvn_sh.sh` to `tests/shell/test_anvs_sh.sh`
- [ ] Update test assertions and expected output

---

## Phase 6: Documentation Files

### 6.1 Main Documentation
- [ ] Update `README.md` - title to "ANVS - Automatic Node Version Switcher"
- [ ] Update `README.md` - badges with new repo URLs
- [ ] Update `README.md` - tagline and description
- [ ] Update `README.md` - installation commands (`npm install -g anvs`)
- [ ] Update `README.md` - Homebrew installation (`brew install anvs`)
- [ ] Update `README.md` - all CLI examples
- [ ] Update `README.md` - configuration file references
- [ ] Update `README.md` - troubleshooting section
- [ ] Update `README.md` - development section

- [ ] Update `CLAUDE.md` - project overview
- [ ] Update `CLAUDE.md` - package and binary names
- [ ] Update `CLAUDE.md` - installation paths
- [ ] Update `CLAUDE.md` - all command examples
- [ ] Update `CLAUDE.md` - repository URLs

- [ ] Update `CHANGELOG.md` - add v2.0.0 entry for rename
- [ ] Update `CHANGELOG.md` - document breaking changes

- [ ] Update `CONTRIBUTING.md` - repository URLs
- [ ] Update `CONTRIBUTING.md` - binary name in examples

### 6.2 Docs Directory
- [ ] Update `docs/ARCHITECTURE.md` - binary and package references
- [ ] Update `docs/MIGRATION.md` - add xvn→anvs migration section
- [ ] Update `docs/HOMEBREW_SETUP.md` - tap name and formula
- [ ] Update `docs/TEST_REVIEW.md` - test command references

### 6.3 Other Files
- [ ] Update `ROADMAP.md` - package and binary references
- [ ] Update `WARP.md` - tool-specific references
- [ ] Update `scripts/README.md` - script examples

### 6.4 Homebrew Tap
- [ ] Update `homebrew-xvn/README.md` - tap name, binary, URLs

---

## Phase 7: Build & Release Scripts

- [ ] Update `scripts/bump-version.sh` - package name references
- [ ] Update `scripts/version.sh` - version detection
- [ ] Update `scripts/download-artifacts.sh` - artifact naming `xvn-*` → `anvs-*`
- [ ] Update `scripts/extract-binaries.sh` - binary and artifact naming
- [ ] Update `scripts/setup-homebrew-tap.sh` - tap repo and formula
- [ ] Update `scripts/coverage.sh` - binary name if referenced

---

## Phase 8: GitHub Workflows (CI/CD)

### 8.1 Build Workflow
- [ ] Update `.github/workflows/build.yml` - artifact names `xvn-*` → `anvs-*`
- [ ] Update `.github/workflows/build.yml` - binary name in build steps
- [ ] Update `.github/workflows/build.yml` - shell script path
- [ ] Update `.github/workflows/build.yml` - release asset names
- [ ] Update `.github/workflows/build.yml` - job descriptions

### 8.2 Test Workflow
- [ ] Update `.github/workflows/test.yml` - shell script path for linting
- [ ] Update `.github/workflows/test.yml` - test references

### 8.3 Homebrew Workflow
- [ ] Update `.github/workflows/update-homebrew.yml` - tap repo to `homebrew-anvs`
- [ ] Update `.github/workflows/update-homebrew.yml` - formula path to `Formula/anvs.rb`
- [ ] Update `.github/workflows/update-homebrew.yml` - download URLs
- [ ] Update `.github/workflows/update-homebrew.yml` - artifact names
- [ ] Update `.github/workflows/update-homebrew.yml` - commit messages

---

## Phase 9: GitHub Repository Changes

- [ ] Rename GitHub repository from `xvn` to `anvs`
- [ ] Update local git remote URL
- [ ] Verify push/pull works with new URL
- [ ] Update repository description on GitHub
- [ ] Update repository topics/tags
- [ ] Update repository homepage URL

---

## Phase 10: Homebrew Tap Changes

- [ ] Rename `homebrew-xvn` repository to `homebrew-anvs`
- [ ] Update local tap clone remote URL
- [ ] Rename `Formula/xvn.rb` to `Formula/anvs.rb`
- [ ] Update formula - class name to `Anvs`
- [ ] Update formula - homepage URL
- [ ] Update formula - download URLs
- [ ] Update formula - binary name in install statements
- [ ] Update formula - test command
- [ ] Update tap README

---

## Phase 11: Migration Guide

- [ ] Create `docs/XVN_TO_ANVS_MIGRATION.md`
- [ ] Document step-by-step migration process
- [ ] Add backup instructions
- [ ] Add uninstall old xvn steps
- [ ] Add install new anvs steps
- [ ] Add config migration steps
- [ ] Add troubleshooting section
- [ ] Add FAQ section
- [ ] Optional: Create `scripts/migrate-xvn-to-anvs.sh` helper

---

## Phase 12: Build, Test, and Publish

### 12.1 Local Build and Test
- [ ] Run full test suite: `cargo test`
- [ ] Run linting: `cargo clippy -- -D warnings`
- [ ] Run formatting: `cargo fmt -- --check`
- [ ] Build release: `cargo build --release`
- [ ] Install locally: `cargo install --path .`
- [ ] Run `anvs setup`
- [ ] Test shell integration
- [ ] Test activation in test directory

### 12.2 Commit and Tag
- [ ] Stage all changes: `git add .`
- [ ] Commit: `feat: rename xvn to anvs - v2.0.0 breaking change`
- [ ] Create tag: `git tag -a v2.0.0 -m "v2.0.0 - Rename to anvs"`
- [ ] Push commits: `git push`
- [ ] Push tag: `git push --tags`

### 12.3 CI/CD Build
- [ ] Monitor GitHub Actions workflow
- [ ] Verify all platform builds succeed
- [ ] Wait for artifacts to be available

### 12.4 npm Publication
- [ ] Download artifacts: `./scripts/download-artifacts.sh v2.0.0`
- [ ] Extract binaries: `./scripts/extract-binaries.sh v2.0.0`
- [ ] Verify binaries: `ls -lh native/*/anvs`
- [ ] Create package: `npm pack`
- [ ] Verify contents: `npm run release:verify`
- [ ] Publish: `npm publish --otp=<code>`
- [ ] Verify: `npm view anvs`

### 12.5 Homebrew Tap Update
- [ ] Verify GitHub Actions updated Homebrew tap
- [ ] If not, run manually: `./scripts/setup-homebrew-tap.sh`
- [ ] Verify formula in tap repository
- [ ] Test: `brew install olvrcc/anvs/anvs`

### 12.6 GitHub Release
- [ ] Verify GitHub Release created by CI
- [ ] Edit release notes to highlight rename
- [ ] Add migration guide link
- [ ] Mark as major version

---

## Phase 13: Final Deprecation & Announcement

### 13.1 Deprecate Package on npm
- [ ] Run npm deprecate command with full message
- [ ] Verify deprecation warning shows when installing old package
- [ ] Confirm v1.7.0 README still shows deprecation notice

### 13.2 Repository Handling
- [ ] If renamed: Verify GitHub redirects work (olvrcc/xvn → olvrcc/anvs)
- [ ] If keeping separate: Add deprecation notice to old repo
- [ ] If keeping separate: Archive old repository
- [ ] Update repository description/topics

### 13.3 Announcements
- [ ] Create announcement issue in `anvs` repository
- [ ] Post to relevant communities/channels
- [ ] Update external references (blog posts, social media)
- [ ] Notify known users/contributors
- [ ] Update package registry metadata if possible

### 13.4 Monitor Migration
- [ ] Monitor issues for migration problems
- [ ] Help users with migration questions
- [ ] Document common issues in migration guide
- [ ] Plan to keep both packages for 3-6 months

---

## Post-Rename Verification

- [ ] `anvs --version` shows `2.0.0`
- [ ] `npm view anvs` shows published package
- [ ] `anvs setup` creates `~/.anvsrc`
- [ ] Shell integration triggers on `cd`
- [ ] Config loads from `~/.anvsrc` and `.anvs.yaml`
- [ ] Installation at `~/.anvs/bin/anvs`
- [ ] All CI/CD workflows pass
- [ ] Homebrew installation works
- [ ] Migration guide is complete
- [ ] Old package deprecated
- [ ] Documentation accurate

---

## Notes

- Mark tasks as complete with `[x]` as you finish them
- Test after each major phase before proceeding
- Keep backups of working versions
- Document any issues or deviations from plan
- Update this checklist if additional tasks are discovered
