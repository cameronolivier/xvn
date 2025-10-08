# xvn Roadmap

This document outlines the development roadmap for xvn, including completed milestones, current work, and future plans. We welcome community contributions!

## Current Version: v1.2.0

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

### Phase 2: Stability & Features (v1.1.0 - v1.5.0)

- ✅ **M10: Version-Independent Installation (v1.2.0)** - Critical bootstrap fix to ensure `xvn` is always available by installing to `~/.xvn`.

---

## 🚧 In Progress

### Milestone 9: Homebrew Distribution (v1.3.0)
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

### Milestone 11: Windows & PowerShell Support (v1.4.0)
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
- [ ] Update `fnm` plugin for Windows

**Skills Needed:** Rust, Windows environment, nvm-windows experience

---

##### M11.5: PowerShell Profile Modification
- [ ] Add PowerShell profile detection
- [ ] Update `xvn setup` command for Windows

**Skills Needed:** Rust, PowerShell, Windows development

---

##### M11.6: Cross-Platform Path Handling
- [ ] Audit all path operations in codebase
- [ ] Add Windows-specific path utilities

**Skills Needed:** Rust, cross-platform development

---

##### M11.7: npm Package for Windows
- [ ] Create `bin/xvn.cmd` wrapper script for Windows
- [ ] Test npm install on Windows

**Skills Needed:** Node.js, Windows batch scripting

---

##### M11.8: Windows Testing
- [ ] Add Windows integration tests
- [ ] Manual testing checklist

**Skills Needed:** Windows testing, PowerShell

---

##### M11.9: Windows Documentation
- [ ] Update README.md with Windows installation
- [ ] Create Windows troubleshooting guide

**Skills Needed:** Technical writing, Windows experience

---

## 🔮 Future Milestones

### Daemon Mode (Post-v1.5.0)
**Goal:** Achieve <10ms activation time

- [ ] Daemon process management
- [ ] IPC protocol (Unix socket)
- [ ] File system watcher
- [ ] Background version pre-loading

**Help Wanted:** Systems programming experience

---

### Additional Version Managers (Post-v1.5.0)

- [ ] Plugin: `n` (Node version manager)
- [ ] Plugin: `asdf` (multi-runtime version manager)
- [ ] Plugin: `volta` (fast JavaScript toolchain manager)

**Help Wanted:** Experience with these version managers

---

### Performance Optimization (Post-v1.5.0)

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

---

## 🤝 How to Contribute

### Priority Areas

**High Priority:**
- ⭐ **Windows testing** - We need Windows developers to test and validate
- ⭐ **Homebrew formula** - macOS users would benefit from `brew install xvn`
- ⭐ **Additional version manager plugins** - Expand compatibility

See the relevant milestone specs in the `spec/` directory for details.

---

## 📋 Development Guidelines

- Open an issue to discuss your proposal
- Follow the existing code style
- Write tests for new features
- Update documentation
- Run `cargo fmt` and `cargo clippy` before committing

---

**Last Updated:** October 8, 2025
**Current Focus:** Milestone 9 (Homebrew) & Milestone 11 (Windows Testing)