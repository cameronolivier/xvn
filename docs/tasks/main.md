# XVN Project Task Tracking - Main Branch

**Branch:** main
**Created:** October 1, 2025
**Status:** Planning Phase Complete

---

## Overview

This document tracks the implementation tasks for xvn (Extreme Version Switcher for Node.js), a high-performance Rust-based automatic Node.js version switcher.

---

## Phase 1: MVP (v0.1.0 - v0.9.0)

**Timeline:** 8-12 weeks
**Goal:** Feature-complete, production-ready core for bash/zsh + nvm/fnm

### Milestone 1: Core Infrastructure (Weeks 1-2)

- [ ] **M1.1** Set up Rust project structure with Cargo.toml
- [ ] **M1.2** Implement CLI framework with clap
  - [ ] `xvn setup` command skeleton
  - [ ] `xvn activate` command skeleton
  - [ ] `xvn --version` command
  - [ ] `xvn --help` command
  - [ ] Global `--verbose` flag
- [ ] **M1.3** Implement configuration system
  - [ ] Define Config struct with serde
  - [ ] Parse YAML from ~/.xvnrc
  - [ ] Parse YAML from .xvn.yaml (project config)
  - [ ] Merge configs with precedence (project > user > default)
  - [ ] Validate configuration (plugin names, version files)
- [ ] **M1.4** Implement version file detection
  - [ ] `find_version_file()` - walk up directory tree
  - [ ] Parse version file (first line, trim whitespace)
  - [ ] Stop at HOME directory
  - [ ] Handle errors (permissions, empty files)
- [ ] **M1.5** Set up error handling
  - [ ] Define XvnError enum with thiserror
  - [ ] Implement user-friendly error messages
  - [ ] Error context (file paths, versions)
- [ ] **M1.6** Set up logging
  - [ ] Configure env_logger
  - [ ] Add debug logging for key operations
- [ ] **M1.7** Unit tests for core infrastructure
  - [ ] Config parsing tests (valid, invalid, defaults)
  - [ ] Version file discovery tests
  - [ ] Error handling tests
  - [ ] Achieve >80% coverage

**Success Criteria:**
- ✅ `xvn --version` returns correct version
- ✅ Config file parsed correctly with defaults
- ✅ Version file found in parent directories
- ✅ Tests passing with >80% coverage

---

### Milestone 2: Plugin System (Weeks 3-4)

- [ ] **M2.1** Define plugin trait (VersionManagerPlugin)
  - [ ] name() method
  - [ ] version_files() method
  - [ ] is_available() method
  - [ ] has_version() method
  - [ ] activate_command() method
  - [ ] install_command() method
  - [ ] resolve_version() method
- [ ] **M2.2** Implement nvm plugin
  - [ ] Check nvm availability (~/.nvm directory)
  - [ ] Check version installed (nvm which)
  - [ ] Generate activate command (nvm use)
  - [ ] Generate install command (nvm install)
  - [ ] Resolve version (nvm version)
  - [ ] Shell escaping for version strings
- [ ] **M2.3** Implement fnm plugin
  - [ ] Check fnm availability (fnm --version)
  - [ ] Check version installed (fnm list)
  - [ ] Generate activate command (fnm use)
  - [ ] Generate install command (fnm install)
  - [ ] Parse fnm list output
- [ ] **M2.4** Implement plugin registry
  - [ ] Load built-in plugins (nvm, fnm)
  - [ ] Respect priority ordering from config
  - [ ] find_plugin() - first match wins
  - [ ] available_plugins() - filter by is_available()
  - [ ] Plugin caching
- [ ] **M2.5** Unit tests for plugin system
  - [ ] Mock plugin implementation
  - [ ] Plugin priority ordering tests
  - [ ] Shell escaping tests (command injection prevention)
  - [ ] Error handling (version not found)

**Success Criteria:**
- ✅ nvm plugin correctly detects nvm availability
- ✅ fnm plugin correctly detects fnm availability
- ✅ Plugins loaded in correct priority order
- ✅ Mock plugin can be tested in isolation

---

### Milestone 3: Shell Integration (Weeks 5-6)

- [ ] **M3.1** Create xvn.sh shell hook script
  - [ ] Version file search function (__xvn_find_file)
  - [ ] Activation function (__xvn_activate)
  - [ ] chpwd hook function (__xvn_chpwd)
  - [ ] Debug function (__xvn_debug)
  - [ ] File descriptor #3 protocol
  - [ ] Bash-specific integration (wrap cd/pushd/popd)
  - [ ] Zsh-specific integration (chpwd_functions)
- [ ] **M3.2** Implement file descriptor #3 protocol in Rust
  - [ ] CommandWriter struct
  - [ ] Detect if FD:3 is open
  - [ ] Write commands to FD:3
  - [ ] Handle FD:3 unavailable gracefully
- [ ] **M3.3** Implement `xvn setup` command
  - [ ] Detect shell (bash, zsh)
  - [ ] Find shell profile files
  - [ ] Check if already installed (idempotency)
  - [ ] Copy xvn.sh to ~/.xvn/bin/
  - [ ] Modify shell profile (append source line)
  - [ ] Create default ~/.xvnrc if missing
  - [ ] Print setup instructions
- [ ] **M3.4** Shell integration tests
  - [ ] Validate xvn.sh syntax (shellcheck)
  - [ ] Test file descriptor protocol (mock FD:3)
  - [ ] Test setup idempotency (run twice)
  - [ ] Test profile detection (bash, zsh)
  - [ ] End-to-end shell test (bash script)

**Success Criteria:**
- ✅ `xvn setup` completes without errors
- ✅ Shell profile correctly modified
- ✅ Hook executes on `cd` command
- ✅ Commands from FD:3 evaluated in parent shell

---

### Milestone 4: Version Activation & Auto-Install (Weeks 7-8)

- [ ] **M4.1** Implement `xvn activate` command
  - [ ] Load configuration
  - [ ] Parse version file
  - [ ] Load plugin registry
  - [ ] Find plugin with version installed
  - [ ] Generate activation commands
  - [ ] Write commands to FD:3
  - [ ] Write user messages to stdout
  - [ ] Handle errors gracefully
- [ ] **M4.2** Implement auto-install logic
  - [ ] Check auto_install config (prompt/always/never)
  - [ ] Prompt user for confirmation
  - [ ] Read stdin for user response
  - [ ] Generate install + activate commands
  - [ ] Handle install declined
  - [ ] Show version mismatch message
- [ ] **M4.3** Implement version mismatch detection
  - [ ] Get current Node.js version (node --version)
  - [ ] Compare to required version
  - [ ] Format mismatch message
- [ ] **M4.4** Implement idempotency check
  - [ ] Shell hook tracks XVN_ACTIVE_FILE
  - [ ] Skip activation if same file
  - [ ] Update XVN_ACTIVE_FILE after activation
- [ ] **M4.5** Unit tests for activation
  - [ ] Version file parsing tests
  - [ ] Plugin priority tests (first match wins)
  - [ ] Auto-install prompt tests (mock stdin)
  - [ ] Config override precedence tests
  - [ ] Error message tests
- [ ] **M4.6** Integration tests
  - [ ] End-to-end activation with mock plugin
  - [ ] Auto-install flow with mock stdin
  - [ ] Multiple version files (nested directories)

**Success Criteria:**
- ✅ Activates installed versions correctly
- ✅ Prompts for missing versions
- ✅ Respects user choice (Y/n)
- ✅ Shows mismatch when declining install
- ✅ Handles all error cases gracefully

---

### Milestone 5: Testing & Polish (Weeks 9-10)

- [ ] **M5.1** Comprehensive unit test suite
  - [ ] Config parsing edge cases
  - [ ] Version file discovery edge cases
  - [ ] Plugin loading edge cases
  - [ ] Error handling for all error types
  - [ ] Achieve >85% coverage
- [ ] **M5.2** Integration test suite
  - [ ] End-to-end activation scenarios
  - [ ] Multi-plugin fallback logic
  - [ ] Config override precedence
  - [ ] Auto-install flow variations
- [ ] **M5.3** Shell test suite
  - [ ] Hook installation tests (bash, zsh)
  - [ ] Directory change detection tests
  - [ ] Command evaluation tests
  - [ ] Idempotency tests
  - [ ] Profile modification tests
- [ ] **M5.4** Performance benchmarking
  - [ ] File discovery benchmark (<5ms target)
  - [ ] Plugin matching benchmark (<20ms target)
  - [ ] Total activation benchmark (<85ms P95 target)
  - [ ] Set up criterion benchmarks
- [ ] **M5.5** Documentation
  - [ ] README.md (quick start, features, comparison)
  - [ ] CONTRIBUTING.md (development setup, guidelines)
  - [ ] API.md (plugin interface, config schema)
  - [ ] Inline code documentation (rustdoc)
- [ ] **M5.6** Code quality
  - [ ] Run clippy (Rust linter)
  - [ ] Format with rustfmt
  - [ ] Address all warnings
  - [ ] Security audit (cargo audit)

**Success Criteria:**
- ✅ All tests passing on CI
- ✅ Coverage >85% (measured by tarpaulin)
- ✅ Benchmarks meet targets (<85ms P95)
- ✅ Documentation complete and reviewed
- ✅ Zero known critical bugs

---

### Milestone 6: Release Preparation (Weeks 11-12)

- [ ] **M6.1** Set up CI/CD pipeline
  - [ ] GitHub Actions workflow (test.yml)
  - [ ] Test matrix (Ubuntu, macOS) × (stable, beta Rust)
  - [ ] Coverage reporting (Coveralls)
  - [ ] Automated builds on push/PR
- [ ] **M6.2** Set up binary builds
  - [ ] GitHub Actions workflow (build.yml)
  - [ ] Build matrix (Linux x64/arm64, macOS x64/arm64)
  - [ ] Cross-compilation setup
  - [ ] Upload artifacts to GitHub Releases
- [ ] **M6.3** Create npm package structure
  - [ ] package.json with metadata
  - [ ] install.js (postinstall script)
  - [ ] Binary download logic
  - [ ] Checksum verification
  - [ ] Platform detection
  - [ ] bin/ wrapper script
- [ ] **M6.4** Test installation flow
  - [ ] Test npm install locally
  - [ ] Test binary download
  - [ ] Test on fresh systems (Ubuntu, macOS)
  - [ ] Test with nvm and fnm
- [ ] **M6.5** Beta testing
  - [ ] Recruit 10-20 beta testers
  - [ ] Distribute pre-release builds
  - [ ] Collect feedback (setup, performance, errors)
  - [ ] Fix critical bugs
  - [ ] Iterate on UX issues
- [ ] **M6.6** Release preparation
  - [ ] Write CHANGELOG.md
  - [ ] Finalize README.md
  - [ ] Create GitHub Release (v0.7.0-beta)
  - [ ] Publish to npm (beta tag)
  - [ ] Test end-to-end installation
- [ ] **M6.7** v1.0.0 release
  - [ ] Address beta feedback
  - [ ] Final testing
  - [ ] Tag v1.0.0
  - [ ] Publish to npm (latest tag)
  - [ ] Announce on social media, Reddit, HN

**Success Criteria:**
- ✅ CI passing on all platforms
- ✅ Binaries successfully downloaded and installed
- ✅ Beta testers report successful installation
- ✅ Zero critical bugs from beta testing
- ✅ npm package installs without errors

---

## Phase 2: Enhanced Features (v1.0.0 - v1.5.0)

**Timeline:** 8-12 weeks post-MVP
**Status:** Not Started

### Milestone 7: package.json Support (v1.1.0)

- [ ] Parse `package.json` "engines.node" field
- [ ] Semver range resolution
- [ ] Prioritize `.nvmrc` / `.node-version` over `package.json`
- [ ] Test with complex semver ranges

### Milestone 8: Daemon Mode (v1.2.0)

- [ ] Daemon process management
- [ ] IPC protocol (Unix socket)
- [ ] File system watcher
- [ ] Shell hook modified for daemon mode
- [ ] Configuration for daemon enable/disable

### Milestone 9: Windows & PowerShell (v1.3.0)

- [ ] Windows binary
- [ ] PowerShell hook script
- [ ] Profile modification for PowerShell
- [ ] Test on Windows 10/11

### Milestone 10: Additional Version Managers (v1.4.0)

- [ ] Plugin: `n`
- [ ] Plugin: `asdf`
- [ ] Plugin: `volta`
- [ ] Plugin development guide

### Milestone 11: Performance Optimization (v1.5.0)

- [ ] Profile-guided optimization
- [ ] Link-time optimization
- [ ] Strip debug symbols
- [ ] Benchmark regression detection

---

## Phase 3: Advanced Capabilities (v2.0.0+)

**Status:** Future Roadmap

### Milestone 12: Shell Plugin System (v2.0.0)

- [ ] Shell plugin trait
- [ ] Refactor bash/zsh as plugins
- [ ] Fish shell plugin
- [ ] Nushell plugin

### Milestone 13: Advanced Features (v2.1.0+)

- [ ] `xvn doctor` - Health check
- [ ] `xvn which` - Show active version
- [ ] `xvn list` - List installed versions
- [ ] `xvn exec <cmd>` - Run command with specific version
- [ ] Shell completions
- [ ] Self-update mechanism

---

## Future Roadmap (Low Priority)

- [ ] Exotic platform support (FreeBSD, RISC-V)
- [ ] Remote version files (HTTP/HTTPS)
- [ ] Additional version files (.tool-versions)
- [ ] VSCode extension
- [ ] Vim/Neovim plugin
- [ ] Workspace support (monorepos)

---

## Current Status

**Phase:** Planning Complete
**Next Action:** Begin Milestone 1 (Core Infrastructure)

---

**Last Updated:** October 1, 2025
