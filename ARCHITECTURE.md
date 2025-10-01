# XVN Architecture Documentation

**Project:** xvn - Extreme Version Switcher for Node.js
**Version:** 1.0.0
**Date:** October 1, 2025
**Status:** Design Phase

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [System Architecture Overview](#system-architecture-overview)
3. [Core Design Principles](#core-design-principles)
4. [Component Architecture](#component-architecture)
5. [Data Flow & Execution Model](#data-flow--execution-model)
6. [Security Architecture](#security-architecture)
7. [Performance Architecture](#performance-architecture)
8. [Testing Architecture](#testing-architecture)
9. [Future Extensibility](#future-extensibility)
10. [Technology Stack](#technology-stack)

---

## Executive Summary

**xvn** is a high-performance, Rust-based automatic Node.js version switcher designed to be 2-3x faster than existing solutions while maintaining simplicity and reliability.

### Architectural Philosophy

**1. Speed First, Always**
- Every design decision optimized for sub-100ms activation time
- Zero-copy operations where possible
- Minimal allocations in hot paths
- Compiled binary with no runtime overhead

**2. Modular & Extensible**
- Plugin-based version manager support (nvm, fnm, n, etc.)
- Future-proof for shell plugins (bash, zsh, fish, etc.)
- Clear interface boundaries between components
- Community can extend without modifying core

**3. Fail-Safe Operation**
- Never break the user's shell
- Graceful degradation on errors
- Clear, actionable error messages
- Silent when nothing needs to happen

**4. UNIX Philosophy**
- Do one thing well (version switching)
- Compose with existing tools (nvm, fnm)
- Use standard protocols (file descriptors)
- Text-based configuration

### Key Architectural Decisions

| Decision | Rationale | Trade-offs |
|----------|-----------|------------|
| **Rust implementation** | 2-3x faster than Node.js, memory safe | Steeper learning curve, smaller contributor pool |
| **Direct execution (no daemon in MVP)** | Simpler, more reliable, still fast enough | Daemon mode deferred to Phase 2 for <10ms switching |
| **Plugin system with built-in plugins** | Extensible, community-driven | Built-in nvm/fnm compiled into binary for speed |
| **File descriptor #3 protocol** | Standard UNIX mechanism, clean separation | Requires shell cooperation, not cross-platform (yet) |
| **YAML configuration** | Human-readable, standard format | Slightly slower parsing than binary (negligible) |
| **npm distribution with pre-built binaries** | Easy installation, wide reach | Binary size (~5MB per platform) |

---

## System Architecture Overview

### High-Level Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                              USER SHELL                                   │
│                         (bash, zsh, fish, etc.)                          │
│  - User's interactive environment                                        │
│  - Current Node.js version controlled by version manager                 │
└────────────────────────┬─────────────────────────────────────────────────┘
                         │ sources on startup
                         ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                       ~/.xvn/bin/xvn.sh                                   │
│  ROLE: Shell Integration Layer                                           │
│  - Hook into shell directory change events                               │
│  - Search for version files up directory tree                            │
│  - Track active version file (prevent redundant activations)             │
│  - Invoke xvn binary when version file changes                           │
│  - Evaluate commands returned via file descriptor #3                     │
└────────────────────────┬─────────────────────────────────────────────────┘
                         │ spawns when version file found/changed
                         ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                         xvn (Rust binary)                                 │
│  ROLE: Core Orchestration & Logic                                        │
│  MODULES:                                                                 │
│  - CLI: Parse arguments, dispatch to handlers                            │
│  - Config: Load/merge user and project configuration                     │
│  - Activation: Read version file, match plugin, generate commands        │
│  - Plugin Registry: Discover, load, cache plugins                        │
│  - Setup: Install shell hooks, create config                             │
└────────────────────────┬─────────────────────────────────────────────────┘
                         │ loads and invokes
                         ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                    PLUGIN SYSTEM                                          │
│  Built-in: nvm, fnm (compiled into binary)                               │
│  Future: Dynamic loading from ~/.xvn/plugins/                            │
│  Interface: VersionManagerPlugin trait                                   │
└────────────────────────┬─────────────────────────────────────────────────┘
                         │ returns shell commands
                         ▼
┌──────────────────────────────────────────────────────────────────────────┐
│  FILE DESCRIPTOR #3 PROTOCOL                                              │
│  - xvn writes shell commands to fd:3                                     │
│  - Shell captures and evaluates commands                                 │
│  - Allows child process to modify parent shell environment               │
└────────────────────────┬─────────────────────────────────────────────────┘
                         │ shell executes commands
                         ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                    VERSION MANAGERS                                       │
│  External tools: nvm, fnm, n (not part of xvn)                          │
│  xvn orchestrates, doesn't replace them                                  │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibility Breakdown

| Component | Responsibility | Complexity |
|-----------|---------------|------------|
| **Shell Hook (xvn.sh)** | Directory change detection, version file search | Low |
| **xvn Core (Rust)** | Orchestration, configuration, activation logic | Medium |
| **Plugin System** | Version manager abstraction, command generation | Low-Medium |
| **Version Managers** | Actual Node.js installation/switching | High (external) |

---

## Core Design Principles

### 1. Performance-Driven Architecture

**Goal:** Sub-100ms activation time (P95)

**Strategies:**
- Minimize system calls (cache plugin availability)
- Optimize hot paths (file I/O, string parsing)
- Lazy initialization (load plugins only when needed)
- Efficient data structures (stack allocations, borrowed slices)
- Profile-guided optimization (PGO) in release builds

### 2. Modular Architecture

**Principle:** Each module has a single responsibility and clear interface.

**Module Dependency Graph:**
```
┌─────────┐
│   CLI   │  (Entry point)
└────┬────┘
     │
     ├─────────────┬─────────────┬─────────────┐
     ▼             ▼             ▼             ▼
┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
│ Setup   │  │ Activate│  │ Version │  │  Help   │
└─────────┘  └────┬────┘  └─────────┘  └─────────┘
                  │
     ┌────────────┼────────────┬────────────┐
     ▼            ▼            ▼            ▼
┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
│ Config  │  │ Plugins │  │ Version │  │ Auto-   │
│         │  │ Registry│  │  File   │  │ Install │
└─────────┘  └─────────┘  └─────────┘  └─────────┘
```

### 3. Error Handling Philosophy

**Principle:** Errors should be informative, actionable, and never crash the shell.

**Error Hierarchy:**
- **Silent errors:** Expected conditions (no version file)
- **Warnings:** Show to user but not fatal (version not installed)
- **Fatal errors:** Show detailed message, exit cleanly (config error)

**Implementation:** Use Rust's `Result<T, E>` everywhere, no panics in production code.

### 4. Testability

**Principle:** Every component should be testable in isolation.

**Strategies:**
- Dependency injection for external dependencies
- Trait-based abstractions for file system, process spawning
- Clear unit/integration/shell test boundaries

---

## Component Architecture

### Core Components

**1. CLI Module**
- Purpose: Parse command-line arguments, dispatch to handlers
- Framework: clap v4 (declarative, generates help)
- Commands: setup, activate, status, doctor

**2. Configuration Module**
- Purpose: Load, parse, merge configuration
- Format: YAML (user: ~/.xvnrc, project: .xvn.yaml)
- Precedence: Environment vars > Project config > User config > Defaults

**3. Version File Module**
- Purpose: Discover and parse version files in directory hierarchy
- Search: Walk up from cwd to HOME, stop at first match
- Parse: Read first line, trim whitespace

**4. Plugin System**
- Purpose: Load, manage, invoke version manager plugins
- Interface: `VersionManagerPlugin` trait
- Built-in: nvm, fnm (compiled into binary)
- Future: Dynamic loading from ~/.xvn/plugins/

**5. Activation Module**
- Purpose: Orchestrate version activation flow
- Responsibilities:
  - Load configuration
  - Parse version file
  - Find matching plugin
  - Handle auto-install prompts
  - Generate shell commands

**6. Setup Module**
- Purpose: Install shell hooks, create configuration files
- Responsibilities:
  - Detect shell (bash/zsh)
  - Modify shell profile (idempotent)
  - Install xvn.sh hook script
  - Create default ~/.xvnrc

---

## Data Flow & Execution Model

### Typical Activation Flow

```
1. User: cd ~/project
2. Shell hook triggered (__xvn_chpwd)
3. Find version file: ~/project/.nvmrc
4. Compare to XVN_ACTIVE_FILE (different? continue)
5. Execute: xvn activate ~/project/.nvmrc
6. xvn binary:
   - Load config (user + project)
   - Read version file: "18.20.0"
   - Load plugins: [nvm, fnm]
   - Try nvm: version not installed
   - Check auto_install config: "prompt"
   - Prompt user: "Install 18.20.0 using nvm? [Y/n]"
   - User: Y
   - Generate commands:
     * "nvm install 18.20.0"
     * "nvm use 18.20.0"
   - Write to fd:3, messages to stdout
7. Shell captures fd:3 commands
8. Shell evaluates: nvm install && nvm use
9. nvm downloads, installs, activates
10. Update XVN_ACTIVE_FILE
11. Success! Node.js 18.20.0 active

Total time: ~50-85ms (without install)
```

---

## Security Architecture

### Threat Model

**Assumptions:**
- User trusts xvn binary
- User trusts version managers (nvm, fnm)
- Attacker may control version files in cloned repos

**Assets to Protect:**
- User's shell environment
- File system integrity
- Credentials in environment

### Security Measures

**1. Command Injection Prevention**
- Always escape shell arguments using `shell_escape` crate
- Example: `.nvmrc` with `"18; rm -rf ~"` → `"nvm use '18; rm -rf ~'"` (safe)

**2. Path Traversal Prevention**
- Canonicalize paths, check they're under HOME or cwd
- Reject version files outside allowed directories

**3. Configuration Validation**
- Plugin names must be alphanumeric + hyphens
- Version file names must not contain path separators

**4. Principle of Least Privilege**
- Run with user permissions only (no sudo)
- Only modify user's shell environment
- Only write to ~/.xvn/ (user-owned)

---

## Performance Architecture

### Performance Budget

**Target:** <100ms (P95) for activation without install

| Phase | Budget | Strategy |
|-------|--------|----------|
| Shell hook | <5ms | Pure bash, no external commands |
| File search | <5ms | Optimized Rust fs, early exit |
| Binary spawn | <10ms | Compiled binary, no runtime |
| Config load | <5ms | Lazy loading, cache |
| Plugin load | <5ms | Built-in plugins, no dynamic load in MVP |
| Plugin match | <20ms | Shell out to version manager |
| Command gen | <5ms | String formatting |
| FD:3 write | <1ms | Direct write, no buffer |
| Shell eval | <10ms | Shell builtin |
| Version manager | <40ms | nvm/fnm execution (external) |
| **Total** | **<85ms** | **Meets target!** |

### Optimization Techniques

1. **Zero-Copy String Operations** - Use borrowed slices instead of owned Strings
2. **Lazy Configuration Loading** - Only load config once, cache with `once_cell`
3. **Plugin Availability Caching** - Don't re-check if nvm available every time
4. **Compile-Time Optimization** - LTO, PGO, strip symbols in release builds

---

## Testing Architecture

### Test Strategy

**1. Unit Tests (Fast, Isolated)**
- Test individual functions in isolation
- Mock external dependencies
- Coverage target: >85%

**2. Integration Tests (Medium Speed, Multi-Component)**
- Test component interactions with mock plugins
- Test configuration precedence, version resolution

**3. Shell Integration Tests (Slow, End-to-End)**
- Test actual shell hook execution (bash, zsh)
- Test directory change detection, command evaluation

**4. Property-Based Tests (Randomized Inputs)**
- Test invariants (version parsing always trims, config merge associative)

### CI/CD Pipeline

- Matrix: [ubuntu-latest, macos-latest] × [stable, beta]
- Run unit + integration + shell tests
- Generate coverage report (tarpaulin → Coveralls)
- Build binaries for all platforms

---

## Future Extensibility

### Planned Extension Points

**1. Shell Plugin System (Phase 3)**
- Abstract bash/zsh as plugins
- Add fish, nushell support
- Community can add new shells

**2. Dynamic Plugin Loading (Phase 2)**
- Load plugins from ~/.xvn/plugins/*.so
- IPC-based for security (separate processes)

**3. Daemon Mode (Phase 2)**
- Long-running background process
- Unix socket IPC
- File system watcher
- Target: <10ms activation

**4. Custom Version Resolvers**
- Pluggable resolution strategies
- NvmAliasResolver, SemverResolver, DockerTagResolver

**5. Event Hooks**
- pre_activate, post_activate, on_install
- Run custom commands on events

---

## Technology Stack

### Core Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "1"
anyhow = "1"
dirs = "5"
shell-escape = "0.1"
once_cell = "1"
```

### Rust Toolchain

- **Version:** 1.70+ (stable)
- **Edition:** 2021
- **Targets:** x86_64/aarch64 (Linux, macOS, future Windows)

### Build Configuration

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

---

## Deployment & Distribution

### npm Package Structure

```
xvn/
├── package.json
├── install.js           # Postinstall: download binary
├── bin/xvn              # Wrapper script
├── native/              # Pre-compiled binaries
│   ├── xvn-linux-x64
│   ├── xvn-linux-arm64
│   ├── xvn-darwin-x64
│   └── xvn-darwin-arm64
└── shell/xvn.sh         # Shell integration
```

### Installation Flow

1. User: `npm install -g xvn`
2. Postinstall downloads correct binary from GitHub Releases
3. Verifies checksum, extracts to native/
4. npm creates symlink: `/usr/local/bin/xvn`
5. User: `xvn setup` (installs hooks, creates config)
6. User restarts terminal
7. xvn active!

---

## File Descriptor #3 Protocol

**The Innovation:** Allows child process (xvn) to modify parent shell environment.

**How it works:**
1. Shell opens FD:3 before spawning xvn
2. xvn writes shell commands to FD:3
3. Shell captures FD:3 output: `commands=$(xvn activate ... 3>&1 1>&2)`
4. Shell evaluates: `eval "$commands"`

**Benefits:**
- Clean separation: fd:3 = commands, stdout = messages, stderr = errors
- Standard UNIX mechanism, no hacks
- Works with any shell that supports file descriptors

---

## Milestone Overview

### Milestone 1: Core Infrastructure (Weeks 1-2)

**Architectural Focus:** Foundation & Configuration

- **Modules:** CLI parsing (clap), Config system (YAML), Version file detection
- **Key Decisions:** Synchronous I/O, serde-based config, directory traversal in Rust
- **Architecture:** Modular design with clear separation: cli → config/version_file
- **Success:** Rust project compiles, CLI works, config merges correctly, version files discovered

### Milestone 2: Plugin System (Weeks 3-4)

**Architectural Focus:** Extensibility & Abstraction

- **Modules:** VersionManagerPlugin trait, Built-in plugins (nvm/fnm), Plugin registry
- **Key Decisions:** Trait-based plugins compiled into binary (not dynamic in MVP)
- **Architecture:** Registry pattern for plugin discovery, priority ordering, availability caching
- **Success:** Plugins detect version managers, return commands, handle errors gracefully

### Milestone 3: Shell Integration (Weeks 5-6)

**Architectural Focus:** Shell Hooks & IPC

- **Modules:** xvn.sh shell script, FD:3 protocol, Setup command, Profile modification
- **Key Decisions:** chpwd_functions for directory change hooks, FD:3 for command passing
- **Architecture:** Shell (xvn.sh) ↔ Rust binary (fd:3) ↔ Version manager (eval)
- **Success:** Shell hook triggers on cd, commands executed in parent shell, setup idempotent

### Milestone 4: Version Activation & Auto-Install (Weeks 7-8)

**Architectural Focus:** Orchestration & User Experience

- **Modules:** Activation orchestrator, Auto-install prompts, Error formatting
- **Key Decisions:** Prompt/always/never modes, stdin for user input, helpful error messages
- **Architecture:** Activation flow: config → version file → plugins → auto-install → commands
- **Success:** Versions activate, missing versions prompt for install, all error cases handled

### Milestone 5: Testing & Polish (Weeks 9-10)

**Architectural Focus:** Quality & Documentation

- **Modules:** Unit tests, Integration tests, Shell tests, Benchmarks, Documentation
- **Key Decisions:** >85% coverage target, criterion for benchmarks, comprehensive docs
- **Architecture:** Test pyramid: unit (fast) → integration (medium) → shell (slow)
- **Success:** All tests pass, benchmarks meet targets, docs complete, no critical bugs

### Milestone 6: Release Preparation (Weeks 11-12)

**Architectural Focus:** Distribution & Deployment

- **Modules:** CI/CD pipeline, Binary builds, npm packaging, Release automation
- **Key Decisions:** GitHub Actions for CI, GitHub Releases for binaries, npm for distribution
- **Architecture:** CI builds → GitHub Release → npm postinstall → user system
- **Success:** Binaries build on all platforms, npm package installs, beta testers succeed

---

## Milestone-Specific Details

For detailed implementation specifications for each milestone, see:

- [Milestone 1: Core Infrastructure](./docs/milestone-1-core-infrastructure.md) (Weeks 1-2)
- [Milestone 2: Plugin System](./docs/milestone-2-plugin-system.md) (Weeks 3-4)
- [Milestone 3: Shell Integration](./docs/milestone-3-shell-integration.md) (Weeks 5-6)
- [Milestone 4: Version Activation & Auto-Install](./docs/milestone-4-version-activation.md) (Weeks 7-8)
- [Milestone 5: Testing & Polish](./docs/milestone-5-testing-polish.md) (Weeks 9-10)
- [Milestone 6: Release Preparation](./docs/milestone-6-release-preparation.md) (Weeks 11-12)

---

**END OF ARCHITECTURE.md**

This document provides the high-level architectural foundation for xvn. Implementation details for specific features are documented in milestone-specific files.
