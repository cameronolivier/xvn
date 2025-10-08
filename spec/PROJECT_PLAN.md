# XVN Project Plan

**Project:** xvn - Extreme Version Switcher for Node.js
**Version:** 1.0.0
**Date:** October 1, 2025
**Status:** Planning Phase

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Project Vision & Goals](#project-vision--goals)
3. [Success Criteria](#success-criteria)
4. [Technical Strategy](#technical-strategy)
5. [Phase Overview](#phase-overview)
6. [Risk Analysis & Mitigation](#risk-analysis--mitigation)
7. [Dependencies & Prerequisites](#dependencies--prerequisites)
8. [Release Strategy](#release-strategy)
9. [Success Metrics & KPIs](#success-metrics--kpis)

---

## Executive Summary

**xvn** is a complete reimagining of automatic Node.js version switching, built in Rust for maximum performance. Unlike avn (its spiritual predecessor), xvn prioritizes speed, modern architecture, and extensibility while maintaining simplicity of use.

**Key Differentiators:**
- **2-3x faster** than avn (~50-85ms vs 150-200ms)
- **Rust-native** for speed, safety, and zero runtime dependencies
- **Auto-installation** of missing Node.js versions with user confirmation
- **Modular plugin architecture** for version managers and future shell support
- **Modern configuration** with YAML/JSON and project-level overrides
- **npm distribution** with pre-compiled binaries for common platforms

**Target Users:**
- Developers working on multiple Node.js projects with different version requirements
- Teams enforcing Node.js version consistency across environments
- Anyone frustrated by manual `nvm use` commands

---

## Project Vision & Goals

### Vision Statement

**"Make Node.js version switching so fast and seamless that developers never think about it."**

### Primary Goals

1. **Performance First**
   - Sub-100ms activation time on modern hardware
   - Imperceptible delay when changing directories
   - Optimized file I/O and minimal syscalls

2. **Reliability**
   - No broken workflows or edge case failures
   - Comprehensive error handling with helpful messages
   - Graceful degradation when version managers unavailable

3. **Simplicity**
   - One command setup: `npm install -g xvn && xvn setup`
   - Works out-of-box with nvm (most common version manager)
   - Sensible defaults, minimal configuration required

4. **Extensibility**
   - Plugin system for version managers (nvm, fnm, n, etc.)
   - Future-ready architecture for shell plugins
   - Community can extend without core changes

5. **Modern Engineering**
   - Type-safe Rust implementation
   - Comprehensive test coverage
   - Clear documentation and architectural decisions

### Non-Goals (Explicitly Out of Scope)

- ❌ **Not a version manager replacement** - Requires nvm/fnm/n to be installed
- ❌ **Not a package manager** - Doesn't manage npm/yarn/pnpm
- ❌ **Not a project scaffolding tool** - Purely version switching
- ❌ **Not backward compatible with avn** - Clean slate implementation

---

## Success Criteria

### Quantitative Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **Activation Time** | <100ms (P50), <150ms (P95) | Benchmarking script across 1000 invocations |
| **Memory Footprint** | <5MB resident memory | `ps` / Activity Monitor during operation |
| **Binary Size** | <5MB (compressed) | Release artifact size |
| **Test Coverage** | >85% line coverage | `cargo tarpaulin` |
| **Setup Time** | <30 seconds | Timed end-to-end installation |
| **Platform Support** | Linux (x64/arm64), macOS (x64/arm64) | CI build matrix |

### Qualitative Metrics

- ✅ User reports "feels instant" when changing directories
- ✅ Zero reported data loss or broken shell sessions
- ✅ Documentation clear enough for non-Rust developers to contribute plugins
- ✅ Installation process has zero manual steps beyond `npm install -g xvn && xvn setup`
- ✅ Error messages provide actionable next steps

### MVP Success Criteria (Must-Have for v1.0)

1. ✅ Successfully switches Node.js versions using nvm on bash/zsh
2. ✅ Detects `.nvmrc` and `.node-version` files
3. ✅ Prompts to install missing versions
4. ✅ Configuration via `~/.xvnrc` works correctly
5. ✅ No regressions in existing shell workflows (cd, pushd, popd)
6. ✅ Works on macOS and Linux (common platforms)
7. ✅ Published to npm with working binaries
8. ✅ Comprehensive README and setup guide

---

## Technical Strategy

### Language & Runtime

**Primary Language:** Rust (stable channel)
- **Rationale:** 2-3x performance over Node.js, memory safety, no runtime dependencies
- **Trade-offs:** Steeper learning curve, smaller contributor pool
- **Mitigation:** Excellent documentation, clear plugin API, Rust adoption growing

### Architecture Principles

1. **Modular Core**
   - Clean separation: shell integration, core logic, plugins, version managers
   - Each module has clear responsibilities and interfaces
   - Easy to test in isolation

2. **Plugin-First Design**
   - Even built-in version managers (nvm, fnm) are plugins
   - Core provides plugin loading, caching, and orchestration
   - Plugins stateless and side-effect free (return commands to execute)

3. **Zero-Copy Where Possible**
   - Minimize string allocations in hot path
   - Use references and slices over owned strings
   - Profile-guided optimization for critical sections

4. **Fail-Fast with Context**
   - Errors include actionable context (file paths, version strings)
   - Graceful degradation (no version file = silent)
   - Never crash the shell

5. **Future-Proof Extensibility**
   - Shell integration abstracted (future shell plugins)
   - Version manager interface generic
   - Configuration system extensible without breaking changes

### Distribution Strategy

**npm as Primary Distribution Channel**

- Pre-compiled binaries for common platforms
- Postinstall script downloads correct binary from GitHub Releases
- Checksum verification before extraction
- Source compilation fallback for unsupported platforms

**Binary Hosting:** GitHub Releases
- CI builds binaries for all platforms
- Automated release process
- Version tagging and changelog generation

---

## Phase Overview

### Phase 1: MVP (v0.1.0 - v1.0.0)

**Timeline:** 8-12 weeks
**Goal:** Feature-complete, production-ready core for bash/zsh + nvm/fnm

**Milestones:**
1. **Core Infrastructure** (Weeks 1-2) - CLI, config, version file detection
2. **Plugin System** (Weeks 3-4) - Plugin trait, nvm/fnm plugins, loading
3. **Shell Integration** (Weeks 5-6) - bash/zsh hooks, setup command, fd:3 protocol
4. **Version Activation** (Weeks 7-8) - Activate command, auto-install prompts
5. **Testing & Polish** (Weeks 9-10) - Comprehensive tests, benchmarks, docs
6. **Release Preparation** (Weeks 11-12) - CI/CD, binary builds, npm packaging

**Deliverables:**
- v1.0.0 public release with core functionality
- Comprehensive documentation
- Working CI/CD pipeline
- npm package with pre-built binaries

### Phase 2: Enhanced Features (v1.1.0 - v1.5.0)

**Timeline:** 8-12 weeks post-MVP
**Goal:** package.json support, daemon mode, Windows/PowerShell

**Key Features:**
- package.json "engines.node" support
- Daemon mode for <10ms activation
- Windows/PowerShell support
- Additional version managers (n, asdf, volta)
- Performance optimization (PGO, LTO)

### Phase 3: Advanced Capabilities (v2.0.0+)

**Timeline:** TBD (post v1.5.0)
**Goal:** Advanced features, shell plugins, ecosystem

**Key Features:**
- Shell plugin system (fish, nushell)
- Advanced commands (doctor, which, list, exec)
- Community-driven extensions

---

## Windows Support Strategy

**Status:** Secondary priority - Focus on "bullet proof" macOS and Linux first

### Current Windows Implementation Status

xvn has comprehensive Windows support implemented in Milestone 11 (v1.3.0), including:

- ✅ Windows binary compilation (x64, ARM64)
- ✅ PowerShell hook script (xvn.ps1)
- ✅ JSON command protocol for PowerShell
- ✅ nvm-windows and fnm plugin support
- ✅ Cross-platform path handling
- ✅ PowerShell profile modification
- ✅ Complete Windows documentation

**Reference:** See [Milestone 11: Windows & PowerShell Support](./milestone-11/PLAN.md) for full implementation details.

### Platform Development Priorities

1. **Primary Focus: macOS** (Weeks 1-10)
   - Core platform for development and testing
   - Target: "bullet proof" reliability and performance
   - All features must work flawlessly on macOS first
   - Performance targets must be met on macOS

2. **Secondary Focus: Linux** (Weeks 8-12)
   - Test on Ubuntu, Debian, Fedora, Arch
   - Verify binary compatibility across distributions
   - CI/CD testing on multiple Linux versions
   - Performance validation on Linux

3. **Tertiary Focus: Windows** (Post-MVP, Phase 2)
   - Windows implementation exists but is **lower priority for refinement**
   - Focus on macOS/Linux stability before Windows optimizations
   - Windows features documented but may have rough edges
   - Community contributions welcome for Windows improvements

### Windows-Specific Features & Issues

#### Implemented Features (Milestone 11)

| Feature | Status | Notes |
|---------|--------|-------|
| Windows binary compilation | ✅ Complete | x64 and ARM64 targets |
| PowerShell integration | ✅ Complete | Prompt override approach |
| JSON command protocol | ✅ Complete | Alternative to FD:3 |
| nvm-windows support | ✅ Complete | Plugin detects %APPDATA%\nvm |
| fnm support | ✅ Complete | Works on Windows |
| Path handling | ✅ Complete | Windows path separators |
| Profile modification | ✅ Complete | Modifies $PROFILE |
| Documentation | ✅ Complete | Windows-specific guide |

#### Known Windows Limitations

| Limitation | Impact | Workaround | Priority |
|------------|--------|------------|----------|
| **cmd.exe not supported** | Medium | Use PowerShell | Low - PowerShell is standard |
| **Execution policy required** | High | User must set `RemoteSigned` | Documentation covers this |
| **Symlinks require elevation** | Medium | Milestone 10 uses copy approach | Addressed in M10 |
| **Slower than Unix** | Low | ~150-200ms vs ~100ms | Acceptable for v1.x |
| **WSL uses Unix scripts** | Low | WSL should use bash/zsh | Not a bug, by design |

#### Windows Installation Approach (Milestone 10 Consideration)

**Problem:** Symlinks on Windows traditionally require administrator privileges or Developer Mode.

**Solution (Hybrid Approach):**

```
Unix (macOS/Linux):
  ~/.xvn/bin/xvn -> versions/v1.2.0/bin/xvn  (symlink)

Windows:
  ~/.xvn/bin/xvn.exe                         (copy of binary)
  ~/.xvn/current -> versions/v1.2.0/         (directory junction)
```

**Rationale:**
- **Symlinks**: Work without elevation on macOS/Linux
- **Copy + Junction**: Works without elevation on Windows
- **Binary size**: ~5MB, copying is acceptable
- **Update strategy**: npm postinstall replaces copied binary

**Reference:** See [Milestone 10: Version-Independent Installation](./milestone-10/PLAN.md) section on Windows considerations.

#### Outstanding Windows Tasks (Backlog)

These tasks are **documented but deprioritized** until macOS/Linux are stable:

1. **Performance Optimization**
   - Profile Windows-specific bottlenecks
   - Optimize PowerShell parsing
   - Target: <100ms activation (currently ~150ms)
   - Priority: **Low** (Phase 2+)

2. **Windows Terminal Deep Integration**
   - Tab title updates with Node version
   - Custom color scheme integration
   - Priority: **Low** (Phase 3+)

3. **PowerShell Gallery Distribution**
   - Alternative to npm installation
   - `Install-Module xvn`
   - Priority: **Low** (Phase 2+)

4. **cmd.exe Support**
   - Batch script wrapper
   - Environment variable protocol
   - Priority: **Very Low** (unlikely)

5. **Windows-Specific Plugins**
   - Volta plugin for Windows
   - asdf-windows support
   - Priority: **Low** (Phase 2+)

6. **Enhanced Error Messages**
   - Windows-specific troubleshooting
   - Detect antivirus interference
   - Priority: **Medium** (Phase 2)

### Windows Testing Strategy

**Current Coverage:**
- ✅ Unit tests with `#[cfg(windows)]`
- ✅ CI testing on `windows-latest`
- ✅ PSScriptAnalyzer validation
- ✅ Integration tests for JSON protocol
- ✅ Manual testing checklist (docs/WINDOWS_TESTING.md)

**Deferred Testing:**
- ⏳ Real hardware testing (multiple Windows versions)
- ⏳ Performance benchmarking on Windows
- ⏳ Long-term stability testing
- ⏳ Antivirus compatibility testing

### Windows Documentation

**Completed:**
- ✅ Windows installation guide (README.md)
- ✅ Windows-specific documentation (docs/WINDOWS.md)
- ✅ PowerShell troubleshooting guide
- ✅ Execution policy guidance
- ✅ Architecture documentation for Windows

**Maintained:**
- Windows docs will be kept up-to-date
- Community can contribute Windows improvements
- Issues will be tracked but may not be prioritized

### Development Workflow

**Phase 1 (MVP):** macOS-first development
```bash
# Develop on macOS
cargo build
cargo test

# Verify Linux compatibility
cargo test --target x86_64-unknown-linux-gnu

# Windows builds in CI (don't block on Windows issues)
```

**Phase 2 (Enhanced):** Linux validation
```bash
# After macOS is stable, focus on Linux
# Test on multiple distributions
# Fix Linux-specific issues

# Windows improvements welcome but not required
```

**Phase 3 (Advanced):** Windows refinement
```bash
# Once macOS/Linux are "bullet proof"
# Dedicate time to Windows optimizations
# Community-driven Windows features
```

### Community Contributions

**Windows improvements are welcome!**

Areas where community can help:
- Performance optimizations for Windows
- Better Windows Terminal integration
- Additional Windows version manager plugins
- Windows-specific bug fixes
- Enhanced troubleshooting documentation

**Contribution Guidelines:**
- Windows PRs must not break macOS/Linux
- All platforms must pass CI before merge
- Windows-specific code should be clearly marked with `#[cfg(windows)]`
- Document Windows-specific behavior

### Success Metrics

**macOS (Primary):**
- ✅ Activation time: <70ms (P50), <100ms (P95)
- ✅ Zero reported crashes or shell breakage
- ✅ 95%+ user satisfaction

**Linux (Secondary):**
- ✅ Activation time: <80ms (P50), <120ms (P95)
- ✅ Works on Ubuntu, Debian, Fedora, Arch
- ✅ 90%+ user satisfaction

**Windows (Tertiary):**
- ⚠️ Activation time: <150ms (P50), <200ms (P95) *(acceptable)*
- ⚠️ Works on Windows 10/11 with PowerShell
- ⚠️ 80%+ user satisfaction *(lower bar initially)*

### Milestone Breakdown

| Milestone | macOS | Linux | Windows |
|-----------|-------|-------|---------|
| M1-M5 | 🎯 Primary | ✓ Supported | ⏸️ Deferred |
| M6 | 🎯 Primary | ✓ Supported | ⏸️ Deferred |
| M7 | 🎯 Primary | ✓ Supported | ⏸️ Deferred |
| M10 | 🎯 Primary | 🎯 Secondary | ⚠️ Hybrid approach |
| M11 | ⏸️ Stable | ⏸️ Stable | 🎯 Implementation |
| Post-M11 | 🎯 Refinement | 🎯 Refinement | ⏸️ Community-driven |

**Legend:**
- 🎯 Active development focus
- ✓ Supported and tested
- ⚠️ Special considerations
- ⏸️ Lower priority
- 🎯 Secondary: Important but not primary

### Summary

**Windows support is complete but secondary to macOS/Linux excellence.**

The goal is to have a "bullet proof" working version of xvn for macOS first, with Linux as a close secondary. Windows implementation exists and works, but refinements and optimizations will come after macOS and Linux are rock-solid.

This approach ensures:
1. **Quality over breadth** - Perfect one platform before spreading resources
2. **Clear priorities** - macOS developers (primary audience) get best experience
3. **Community engagement** - Windows users can contribute improvements
4. **Realistic expectations** - Windows users know it's supported but may have rough edges

---

## Risk Analysis & Mitigation

### Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Rust learning curve slows development** | Medium | High | - Start with MVP features<br>- Pair programming<br>- Code reviews<br>- Leverage existing crates |
| **Binary distribution complexity** | Low | High | - Use proven tools (GitHub Releases)<br>- Test on multiple platforms early<br>- Provide source compilation fallback |
| **Shell compatibility issues** | Medium | Medium | - Test on multiple shell versions<br>- Comprehensive shell test suite<br>- Clear error messages for unsupported shells |
| **Version manager API changes** | Low | Medium | - Abstract plugin interface<br>- Version-specific plugin logic<br>- Monitor upstream changes |
| **Performance doesn't meet targets** | Low | High | - Benchmark early and often<br>- Profile-guided optimization<br>- Daemon mode as fallback |

### Non-Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Low adoption (users prefer avn/volta)** | Medium | High | - Clear value proposition (speed)<br>- Migration guide<br>- Community engagement<br>- Show benchmarks |
| **Maintenance burden** | Medium | Medium | - Modular architecture<br>- Comprehensive tests<br>- Good documentation<br>- Encourage community contributions |
| **Breaking changes in dependencies** | Low | Medium | - Pin dependency versions<br>- Automated dependency updates (Dependabot)<br>- Test before upgrading |
| **Security vulnerabilities** | Low | High | - Regular security audits (`cargo audit`)<br>- Minimize dependencies<br>- Quick patch releases |

---

## Dependencies & Prerequisites

### Development Dependencies

**Required:**
- Rust 1.70+ (stable channel)
- Cargo (comes with Rust)
- Git

**Optional (for full development):**
- shellcheck (shell script linting)
- tarpaulin (code coverage)
- cargo-watch (auto-rebuild)
- cargo-audit (security audits)

### Runtime Dependencies (User)

**Required:**
- bash 4.0+ or zsh 5.0+
- One of: nvm, fnm, n (version manager)
- Node.js (any version, for npm installation)

**Optional:**
- Rust toolchain (for source compilation on unsupported platforms)

### CI/CD Dependencies

- GitHub Actions (free for open source)
- Coveralls (code coverage reporting)
- GitHub Releases (binary hosting)
- npm registry (package distribution)

---

## Release Strategy

### Versioning Scheme

**Semantic Versioning (semver):**
- **Major (X.0.0):** Breaking changes (config format, CLI interface)
- **Minor (x.Y.0):** New features (new plugins, new commands)
- **Patch (x.y.Z):** Bug fixes, performance improvements

**Pre-release Tags:**
- **v0.x.x:** Pre-1.0 development (breaking changes allowed)
- **v1.0.0-alpha.x:** Feature-complete, needs testing
- **v1.0.0-beta.x:** Stable, user testing phase
- **v1.0.0-rc.x:** Release candidate, final testing

### Release Cadence

**Development Phase (v0.x.x):**
- Release every 2 weeks (milestone completion)
- Fast iteration, breaking changes acceptable

**Stable Phase (v1.x.x+):**
- Minor releases: Every 4-8 weeks
- Patch releases: As needed (critical bugs)
- Major releases: Every 6-12 months

### Communication Channels

**Pre-Launch:**
- GitHub repository (primary)
- Personal blog / social media
- Dev communities (Reddit /r/rust, /r/node, HN)

**Post-Launch:**
- GitHub Releases (changelog)
- Twitter / Mastodon
- Reddit announcements
- Hacker News "Show HN"
- Node Weekly newsletter

---

## Success Metrics & KPIs

### Adoption Metrics

| Metric | 3 Months | 6 Months | 12 Months |
|--------|----------|----------|-----------|
| **npm downloads/week** | 100 | 500 | 2000 |
| **GitHub stars** | 50 | 200 | 500 |
| **Active users (telemetry)** | 50 | 250 | 1000 |

### Quality Metrics

| Metric | Target |
|--------|--------|
| **Test coverage** | >85% |
| **Open issues** | <20 |
| **Issue response time** | <48 hours |
| **Critical bug resolution** | <7 days |
| **Documentation completeness** | 100% public APIs documented |

### Performance Metrics

| Metric | v1.0.0 | v2.0.0 (daemon) |
|--------|--------|-----------------|
| **Activation time (P50)** | <70ms | <8ms |
| **Activation time (P95)** | <100ms | <15ms |
| **Memory usage** | <5MB | <10MB |
| **Binary size** | <5MB | <5MB |

---

## Milestone Summary

### Milestone 1: Core Infrastructure (Weeks 1-2, v0.1.0)

**Goal:** Establish foundational Rust project with CLI, configuration, and version file detection.

**Key Deliverables:**
- Rust project structure with Cargo.toml configured
- CLI framework with clap (setup, activate, status commands)
- Configuration system (YAML parsing, precedence: project > user > defaults)
- Version file detection (directory traversal, parse .nvmrc/.node-version)
- Basic error handling with thiserror

**Success Metrics:** CLI parses correctly, config loaded from multiple sources, version files discovered in parent directories, >80% test coverage

### Milestone 2: Plugin System (Weeks 3-4, v0.2.0)

**Goal:** Implement extensible plugin architecture with built-in nvm/fnm plugins.

**Key Deliverables:**
- VersionManagerPlugin trait definition
- Built-in nvm plugin (check availability, has_version, activate/install commands)
- Built-in fnm plugin (similar to nvm)
- Plugin registry with priority ordering and caching
- Mock plugin for testing

**Success Metrics:** Plugins correctly detect version managers, return shell commands, handle errors, priority ordering works

### Milestone 3: Shell Integration (Weeks 5-6, v0.3.0)

**Goal:** Integrate with bash/zsh shells using hooks and fd:3 protocol.

**Key Deliverables:**
- xvn.sh shell script (bash/zsh compatible, chpwd_functions)
- File descriptor #3 protocol (Rust writes commands, shell evaluates)
- Setup command (detect shell, modify profile, install hooks)
- Idempotency checks (XVN_ACTIVE_FILE tracking)

**Success Metrics:** Setup completes without errors, shell hook triggers on cd, commands executed in parent shell environment

### Milestone 4: Version Activation & Auto-Install (Weeks 7-8, v0.4.0-v0.5.0)

**Goal:** Complete activation flow with auto-install prompts and user confirmations.

**Key Deliverables:**
- Activate command orchestration (load config → read version → match plugin)
- Auto-install prompt UI (read stdin, show helpful messages)
- Install command generation and execution
- Three modes: prompt (default), always, never
- Comprehensive error messages with actionable guidance

**Success Metrics:** Versions activate correctly, missing versions prompt for install, user choice respected, all error cases handled

### Milestone 5: Testing & Polish (Weeks 9-10, v0.6.0-v0.9.0)

**Goal:** Achieve comprehensive test coverage, benchmark performance, complete documentation.

**Key Deliverables:**
- Unit test suite (>85% coverage with tarpaulin)
- Integration tests with mock plugins and config scenarios
- Shell integration tests (bash/zsh)
- Performance benchmarks (criterion: <85ms P95 target)
- Documentation (README, CONTRIBUTING, API rustdoc)

**Success Metrics:** All tests pass on CI, benchmarks meet targets, documentation complete, zero critical bugs

### Milestone 6: Release Preparation (Weeks 11-12, v0.7.0-v1.0.0)

**Goal:** Establish CI/CD, build binaries for all platforms, package for npm, prepare for public release.

**Key Deliverables:**
- GitHub Actions CI/CD pipeline (test matrix, binary builds)
- Pre-compiled binaries for Linux/macOS (x64/arm64)
- npm package with postinstall binary download
- Release automation (tag → build → GitHub Release → npm publish)
- Beta testing with 10-20 users
- Migration guide from avn

**Success Metrics:** CI passes on all platforms, binaries install correctly, beta testers succeed, npm package works, v1.0.0 released

---

### Extra Milestones (Phase 1 Enhancements)

These milestones extend Phase 1 with quality-of-life improvements and enhanced user experience.

### Milestone 7: Interactive Setup Wizard (v0.8.0)

**Goal:** Transform the basic setup command into an interactive wizard that guides users through configuration and provides educational context.

**Core Deliverables:**
- Rename `xvn setup` to `xvn init` (maintain `setup` as alias for compatibility)
- Interactive configuration wizard:
  - Shell selection (auto-detect with confirmation)
  - Plugin priority ordering (nvm, fnm, n, etc.)
  - Auto-install mode preference (prompt, always, never)
  - Version file preferences (.nvmrc, .node-version, custom)
- Educational prompts explaining each configuration option
- Configuration preview before saving
- Final summary showing:
  - Generated config file location (`~/.xvnrc`)
  - Shell profile modified (`.bashrc`, `.zshrc`)
  - Next steps and helpful commands
- Option to skip wizard with `--quick` flag (uses defaults)
- Ability to re-run wizard to modify existing configuration

**User Experience Flow:**
```
$ xvn init

xvn - automatic node version switching
Welcome! Let's set up xvn for your environment.

[1/5] Shell Detection
  ✓ Detected: zsh
  Use zsh? [Y/n]:

[2/5] Version Managers
  Which version managers do you have installed?
  [✓] nvm (detected at ~/.nvm)
  [ ] fnm
  [ ] n

  Priority order: nvm
  Press enter to continue...

[3/5] Auto-Install Behavior
  When a required Node.js version isn't installed:
  1. Prompt me each time (recommended)
  2. Always install automatically
  3. Never install, just show error

  Choose [1-3]: 1

[4/5] Version Files
  Which files should xvn check for version info?
  [✓] .nvmrc (standard)
  [✓] .node-version (alternative)
  [ ] .tool-versions (asdf)

  Priority order: .nvmrc, .node-version
  Press enter to continue...

[5/5] Review Configuration
  Shell:        zsh
  Profile:      /Users/you/.zshrc
  Plugins:      nvm
  Auto-install: prompt
  Version files: .nvmrc, .node-version
  Config file:  /Users/you/.xvnrc

  Looks good? [Y/n]:

xvn: ✓ Configuration saved!
xvn: ✓ Shell integration installed!

xvn: To start using xvn:
  1. Restart your shell, or run:
       source /Users/you/.zshrc
  2. Navigate to a project with a .nvmrc file
  3. xvn will automatically activate the correct Node.js version

xvn: Your config file is at: /Users/you/.xvnrc
xvn: Run 'xvn init' again to modify your configuration.
```

**Technical Implementation:**
- Use `dialoguer` or `inquire` crate for interactive prompts
- Detect installed version managers by checking common paths
- Validate configuration before saving
- Preserve existing config comments when re-running wizard
- Graceful fallback to non-interactive mode if stdin isn't a TTY

**Success Metrics:**
- Users complete wizard without confusion
- Generated config matches user intent
- Wizard completable in <2 minutes
- Config file location clearly communicated

---

## Milestone-Specific Plans

For detailed plans for each milestone, see:

- [Milestone 1: Core Infrastructure](./milestone-1/PLAN.md) (Weeks 1-2)
- [Milestone 2: Plugin System](./milestone-2/PLAN.md) (Weeks 3-4)
- [Milestone 3: Shell Integration](./milestone-3/PLAN.md) (Weeks 5-6)
- [Milestone 4: Version Activation & Auto-Install](./milestone-4/PLAN.md) (Weeks 7-8)
- [Milestone 5: Testing & Polish](./milestone-5/PLAN.md) (Weeks 9-10)
- [Milestone 6: Release Preparation](./milestone-6/PLAN.md) (Weeks 11-12)
- [Milestone 7: Interactive Setup Wizard](./milestone-7/PLAN.md) (Extra - Phase 1 Enhancement)

---

## Conclusion

This project plan provides a comprehensive roadmap for building **xvn** - a fast, modern, Rust-based Node.js version switcher. The phased approach ensures:

1. **Rapid MVP delivery** (8-12 weeks) with core functionality
2. **Iterative enhancement** based on user feedback
3. **Long-term sustainability** with modular architecture
4. **Clear success criteria** at each milestone

**Key Success Factors:**
- ✅ Performance focus from day one
- ✅ Comprehensive testing at every phase
- ✅ User-centric design (auto-install, helpful errors)
- ✅ Extensible architecture (plugins, future-proof)
- ✅ Clear documentation and communication

**Next Steps:**
1. Review and approve this plan
2. Set up development environment
3. Begin Phase 1, Milestone 1: Core Infrastructure
4. Establish CI/CD pipeline early
5. Recruit beta testers before v0.7.0

---

**Document Version:** 1.0
**Last Updated:** October 1, 2025
**Status:** Approved for implementation
