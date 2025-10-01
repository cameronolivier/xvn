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

## Milestone-Specific Plans

For detailed plans for each milestone, see:

- [Milestone 1: Core Infrastructure](./docs/milestone-1-core-infrastructure.md) (Weeks 1-2)
- [Milestone 2: Plugin System](./docs/milestone-2-plugin-system.md) (Weeks 3-4)
- [Milestone 3: Shell Integration](./docs/milestone-3-shell-integration.md) (Weeks 5-6)
- [Milestone 4: Version Activation & Auto-Install](./docs/milestone-4-version-activation.md) (Weeks 7-8)
- [Milestone 5: Testing & Polish](./docs/milestone-5-testing-polish.md) (Weeks 9-10)
- [Milestone 6: Release Preparation](./docs/milestone-6-release-preparation.md) (Weeks 11-12)

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
