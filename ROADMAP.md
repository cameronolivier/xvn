# xvn Roadmap

This document outlines the development roadmap for xvn, including completed milestones, current work, and future plans. We welcome community contributions!

## Current Version: v1.1.2

**Status:** Production-ready for macOS and Linux (bash/zsh)

---

## ✅ Completed Milestones

### Phase 1: MVP (v0.1.0 - v1.0.0)

- ✅ **M1:** Core Infrastructure - CLI, config, version file detection
- ✅ **M2:** Plugin System - Plugin trait, nvm/fnm plugins
- ✅ **M3:** Shell Integration - bash/zsh hooks, FD:3 protocol
- ✅ **M4:** Version Activation - Activation orchestration, auto-install
- ✅ **M5:** Testing & Polish - 89 passing tests, 57.93% coverage
- ✅ **M6:** Release Preparation - CI/CD, npm distribution
- ✅ **M7:** Interactive Setup Wizard - `xvn init` command
- ✅ **M8:** package.json Support - Semver range resolution

---

## 🚧 In Progress

### Phase 2: Enhanced Features (v1.1.0 - v1.5.0)

#### Milestone 9: Homebrew Distribution (v1.1.x)
**Status:** Planned
**Help Wanted:** macOS developers

- [ ] Create Homebrew formula for xvn
- [ ] Set up custom tap: `olvrcc/homebrew-xvn`
- [ ] Automated formula updates via GitHub Actions
- [ ] Support for macOS x64 and ARM64

**How to Contribute:**
- Experience with Homebrew formulae needed
- See [spec/milestone-9/](./spec/milestone-9/) for detailed plan

---

#### Milestone 11: Windows & PowerShell Support (v1.3.0)
**Status:** Foundation Complete - Testing Needed
**Help Wanted:** Windows developers with PowerShell experience

**✅ Completed:**
- Windows binary compilation setup (x86_64, ARM64)
- PowerShell hook script (`shell/xvn.ps1`)
- JSON command protocol implementation
- GitHub Actions Windows CI

**🔨 Remaining Work:**

##### M11.4: Plugin System for Windows
- [ ] Update `nvm` plugin to detect nvm-windows
  - [ ] Check for `nvm` command on Windows
  - [ ] Detect version install location (`%APPDATA%\nvm\`)
  - [ ] Handle Windows-style paths in commands
- [ ] Update `fnm` plugin for Windows
  - [ ] Detect `fnm.exe` on PATH
  - [ ] Handle Windows install locations
- [ ] Add Windows-specific path utilities
  - [ ] Expand `%VAR%` environment variables
  - [ ] Handle UNC paths (`\\server\share`)

**Skills Needed:** Rust, Windows environment, nvm-windows experience

**Files to Modify:**
- `src/plugins/nvm.rs`
- `src/plugins/fnm.rs`
- New: `src/utils/windows_paths.rs` (suggested)

---

##### M11.5: PowerShell Profile Modification
- [ ] Add PowerShell profile detection
  - [ ] Detect `$PROFILE` path
  - [ ] Create profile directory if missing
  - [ ] Detect PowerShell version (5.1 vs 7+)
- [ ] Update `xvn init` command for Windows
  - [ ] Copy `xvn.ps1` to `~/.xvn/bin/`
  - [ ] Add source line to `$PROFILE`
  - [ ] Check idempotency (don't duplicate)
- [ ] Add Windows-specific setup instructions
  - [ ] Handle execution policy prompts
  - [ ] Test in different PowerShell hosts

**Skills Needed:** Rust, PowerShell, Windows development

**Files to Modify:**
- `src/setup/mod.rs`
- `src/setup/shell_detection.rs`
- `src/setup/profile_modification.rs`

---

##### M11.6: Cross-Platform Path Handling
- [ ] Audit all path operations in codebase
  - [ ] Use `PathBuf::join()` instead of string concat
  - [ ] Avoid hardcoded `/` separators
- [ ] Add Windows-specific path utilities
  - [ ] Expand `%VAR%` environment variables
  - [ ] Handle UNC paths
  - [ ] Normalize mixed separators
- [ ] Update config loader for Windows paths
  - [ ] Support `~` expansion on Windows
  - [ ] Support Windows environment variables

**Skills Needed:** Rust, cross-platform development

**Files to Audit:**
- `src/config/loader.rs`
- `src/version_file/finder.rs`
- `src/plugins/nvm.rs`
- `src/plugins/fnm.rs`

---

##### M11.7: npm Package for Windows
- [x] Update `install.js` to detect Windows
- [ ] Create `bin/xvn.cmd` wrapper script for Windows
- [ ] Test npm install on Windows
  - [ ] Verify binary extraction
  - [ ] Verify executable permissions
  - [ ] Test global install path

**Skills Needed:** Node.js, Windows batch scripting

**Files to Modify:**
- `install.js` (already updated)
- New: `bin/xvn.cmd`

---

##### M11.8: Windows Testing
- [ ] Add Windows integration tests
  - [ ] Test version activation with nvm-windows
  - [ ] Test directory change detection
  - [ ] Test idempotency
  - [ ] Test error handling
- [ ] Manual testing checklist
  - [ ] Windows 10 x64
  - [ ] Windows 11 ARM64
  - [ ] Windows Terminal
  - [ ] VS Code integrated terminal
  - [ ] PowerShell ISE

**Skills Needed:** Windows testing, PowerShell

**Testing Guide:** See [spec/milestone-11/PLAN.md](./spec/milestone-11/PLAN.md#testing-strategy)

---

##### M11.9: Windows Documentation
- [ ] Update README.md with Windows installation
- [ ] Create Windows troubleshooting guide
  - [ ] Execution policy issues
  - [ ] PATH configuration
  - [ ] nvm-windows vs Unix nvm differences
- [ ] Update ARCHITECTURE.md
  - [ ] Document PowerShell integration
  - [ ] Document JSON protocol

**Skills Needed:** Technical writing, Windows experience

---

## 🔮 Future Milestones

### Milestone 10: Daemon Mode (v1.2.0)
**Goal:** Achieve <10ms activation time

- [ ] Daemon process management
- [ ] IPC protocol (Unix socket)
- [ ] File system watcher
- [ ] Background version pre-loading

**Help Wanted:** Systems programming experience

---

### Milestone 12: Additional Version Managers (v1.4.0)

- [ ] Plugin: `n` (Node version manager)
- [ ] Plugin: `asdf` (multi-runtime version manager)
- [ ] Plugin: `volta` (fast JavaScript toolchain manager)

**Help Wanted:** Experience with these version managers

---

### Milestone 13: Performance Optimization (v1.5.0)

- [ ] Profile-guided optimization (PGO)
- [ ] Link-time optimization (LTO)
- [ ] Strip debug symbols
- [ ] Benchmark regression detection

**Help Wanted:** Performance optimization experience

---

## 🌟 Community Ideas

### Shell Plugin System (v2.0.0)
- [ ] Refactor bash/zsh as plugins
- [ ] Fish shell plugin
- [ ] Nushell plugin
- [ ] Elvish plugin

### Advanced Features (v2.1.0+)
- [ ] `xvn doctor` - Health check command
- [ ] `xvn which` - Show active version and why
- [ ] `xvn list` - List installed versions
- [ ] `xvn exec <cmd>` - Run command with specific version
- [ ] Shell completions (bash, zsh, fish)
- [ ] Self-update mechanism

### Integration & Extensions
- [ ] Remote version files (HTTP/HTTPS URLs)
- [ ] `.tool-versions` support (asdf compatibility)
- [ ] Git repository version pinning
- [ ] VSCode extension
- [ ] Vim/Neovim plugin
- [ ] Zsh theme integration (show version in prompt)
- [ ] Monorepo/workspace support

### Exotic Platforms
- [ ] FreeBSD support
- [ ] OpenBSD support
- [ ] RISC-V architecture
- [ ] Alpine Linux (musl)

---

## 🤝 How to Contribute

### Getting Started

1. **Check existing issues:** Look for issues tagged with `help-wanted` or `good-first-issue`
2. **Read the docs:** Familiarize yourself with [ARCHITECTURE.md](./docs/ARCHITECTURE.md) and [CONTRIBUTING.md](./CONTRIBUTING.md)
3. **Ask questions:** Open a discussion on GitHub if you need clarification

### Priority Areas

**High Priority:**
- ⭐ **Windows testing** - We need Windows developers to test and validate
- ⭐ **Homebrew formula** - macOS users would benefit from `brew install xvn`
- ⭐ **Additional version manager plugins** - Expand compatibility

**Medium Priority:**
- Performance optimizations
- Documentation improvements
- Cross-platform testing

**Low Priority (but welcomed!):**
- Additional shell support
- Advanced features
- Exotic platforms

### Windows Contributors Needed!

We have a solid foundation for Windows support but need help with:
- Testing on real Windows environments
- Debugging nvm-windows integration
- PowerShell profile setup testing
- Path handling edge cases

**Requirements:**
- Windows 10+ with PowerShell 5.1+
- Rust toolchain installed
- nvm-windows or fnm installed
- Familiarity with PowerShell

See [spec/milestone-11/TASKS.md](./spec/milestone-11/TASKS.md) for detailed breakdown.

---

## 📋 Development Guidelines

### Before Starting

1. Open an issue to discuss your proposal
2. Get consensus on approach
3. Follow the existing code style
4. Write tests for new features
5. Update documentation

### Code Quality

- Maintain >85% test coverage
- All clippy warnings must be addressed
- Run `cargo fmt` before committing
- Follow conventional commit messages

### Milestone Structure

Each milestone has:
- `spec/milestone-N/SPEC.md` - Architecture and design
- `spec/milestone-N/TASKS.md` - Task checklist
- `spec/milestone-N/PLAN.md` - Implementation guide

---

## 📞 Contact & Discussion

- **GitHub Issues:** Bug reports and feature requests
- **GitHub Discussions:** General questions and ideas
- **Pull Requests:** Code contributions

---

## 📜 License

xvn is MIT licensed. All contributions will be under the same license.

---

**Last Updated:** October 4, 2025
**Current Focus:** Milestone 9 (Homebrew) & Milestone 11 (Windows Testing)
