# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**xvn** is a Rust-based reimagining of automatic Node.js version switching, designed to be 2-3x faster than its predecessor (avn). The project is currently in the planning/design phase with no implementation yet.

**Key Characteristics:**
- Target language: Rust (for performance, safety, zero runtime dependencies)
- Distribution: npm with pre-compiled binaries
- Target platforms: Linux (x64/arm64), macOS (x64/arm64), future Windows support
- Architecture: Modular plugin system for version managers (nvm, fnm, n, etc.)

## Project Status

**Current Phase:** Planning/Design (Pre-v0.1.0)
- Comprehensive project specification complete (PROJECT_SPEC.md)
- Detailed architecture document complete (ARCHITECTURE.md)
- Phased project plan complete (PROJECT_PLAN.md)
- No code implementation exists yet

## Documentation Structure

The project documentation is organized hierarchically for easy navigation:

### High-Level Documents (in `docs/`)

1. **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - High-level system design and architectural principles
   - Architectural philosophy and key decisions
   - System architecture overview and component responsibilities
   - Core design principles (performance, modularity, error handling, testability)
   - Security, performance, and testing architecture
   - Technology stack and deployment strategy
   - **Milestone Overview section** with architectural focus for each milestone

### Project Specifications (in `spec/`)

1. **[spec/PROJECT_SPEC.md](./spec/PROJECT_SPEC.md)** - Original avn specification (reference)
   - Detailed specification based on the original avn project
   - Complete system architecture and component breakdown
   - Historical context and design rationale

2. **[spec/PROJECT_PLAN.md](./spec/PROJECT_PLAN.md)** - Project strategy and roadmap
   - Project vision, goals, and success criteria
   - Technical strategy and distribution approach
   - Phase overview (MVP, Enhanced, Advanced)
   - Risk analysis and mitigation strategies
   - Release strategy and success metrics
   - **Milestone Summary section** with goals and deliverables for each milestone

3. **[spec/PROGRESS.md](./spec/PROGRESS.md)** - High-level milestone progress tracking
   - Phase 1, 2, 3 milestone checkboxes
   - Links to detailed tasks for each milestone
   - Current status and next actions

### Milestone Specifications (in `spec/milestone-N/`)

Each milestone has a dedicated directory containing:

1. **[spec/milestone-1/](./spec/milestone-1/)** Core Infrastructure (Weeks 1-2)
   - [PLAN.md](./spec/milestone-1/PLAN.md) - Architecture and implementation plan
   - [TASKS.md](./spec/milestone-1/TASKS.md) - Detailed task checklist

2. **[spec/milestone-2/](./spec/milestone-2/)** Plugin System (Weeks 3-4)
   - [PLAN.md](./spec/milestone-2/PLAN.md) - Plugin trait, nvm/fnm implementation
   - [TASKS.md](./spec/milestone-2/TASKS.md) - Plugin development tasks

3. **[spec/milestone-3/](./spec/milestone-3/)** Shell Integration (Weeks 5-6)
   - [PLAN.md](./spec/milestone-3/PLAN.md) - Shell hooks, FD:3 protocol
   - [TASKS.md](./spec/milestone-3/TASKS.md) - Shell integration tasks

4. **[spec/milestone-4/](./spec/milestone-4/)** Version Activation (Weeks 7-8)
   - [PLAN.md](./spec/milestone-4/PLAN.md) - Activation orchestration
   - [TASKS.md](./spec/milestone-4/TASKS.md) - Activation and auto-install tasks

5. **[spec/milestone-5/](./spec/milestone-5/)** Testing & Polish (Weeks 9-10)
   - [PLAN.md](./spec/milestone-5/PLAN.md) - Test strategy and documentation
   - [TASKS.md](./spec/milestone-5/TASKS.md) - Testing and quality tasks

6. **[spec/milestone-6/](./spec/milestone-6/)** Release Preparation (Weeks 11-12)
   - [PLAN.md](./spec/milestone-6/PLAN.md) - CI/CD and distribution
   - [TASKS.md](./spec/milestone-6/TASKS.md) - Release and beta testing tasks

### Configuration Files

- **package.json** - npm package metadata (minimal, pre-implementation)

## Development Phases

### Phase 1: MVP (v0.1.0 - v1.0.0) - 8-12 weeks
1. **Milestone 1:** Core Infrastructure - CLI, config, version file detection
2. **Milestone 2:** Plugin System - Plugin trait, nvm/fnm plugins
3. **Milestone 3:** Shell Integration - bash/zsh hooks, setup command
4. **Milestone 4:** Version Activation - Activate command, auto-install prompts
5. **Milestone 5:** Testing & Polish - Comprehensive tests, benchmarks, docs
6. **Milestone 6:** Release Preparation - CI/CD, binary builds, npm packaging

### Phase 2: Enhanced Features (v1.1.0 - v1.5.0)
- package.json "engines.node" support
- Daemon mode for <10ms activation
- Windows/PowerShell support
- Additional version managers (n, asdf, volta)
- Performance optimization (PGO, LTO)

### Phase 3: Advanced Capabilities (v2.0.0+)
- Shell plugin system (fish, nushell)
- Advanced features (doctor, which, list, exec commands)

## Core Architecture Concepts

### Plugin System
- **VersionManagerPlugin trait** - Interface for version manager plugins
- Built-in plugins compiled into binary (nvm, fnm)
- Dynamic plugin loading from `~/.xvn/plugins/`
- Plugin priority ordering via configuration

### Shell Integration
- File descriptor #3 protocol for parent shell environment modification
- chpwd_functions hook integration (bash/zsh)
- Version file discovery (walk up directory tree)
- Idempotency checks to prevent re-activation

### Performance Targets
- Activation time: <100ms (P50), <150ms (P95) for v1.0
- Activation time: <8ms (P50), <15ms (P95) for v2.0 (daemon mode)
- Binary size: <5MB compressed
- Memory footprint: <5MB resident

## Configuration Files

- `~/.xvnrc` - User-level configuration (YAML)
- `.xvn.yaml` - Project-level configuration overrides
- `.nvmrc` / `.node-version` - Node.js version specification files

## Testing Strategy

### Test Coverage Requirements
- Unit tests: >85% line coverage
- Integration tests with mock plugins
- Shell integration tests (bash, zsh)
- Performance benchmarks with regression detection

### Test Tools (Planned)
- `cargo test` - Rust unit tests
- `cargo tarpaulin` - Code coverage
- `cargo bench` - Performance benchmarks
- shellcheck - Shell script validation

## Common Development Commands

**Note:** Project is in planning phase. These commands will be relevant once implementation begins:

```bash
# Development
cargo build                    # Build debug binary
cargo build --release          # Build optimized binary
cargo test                     # Run unit tests
cargo test --all-features      # Run all tests
cargo bench                    # Run benchmarks

# Code Quality
cargo clippy                   # Rust linter
cargo fmt                      # Format code
cargo tarpaulin --out Lcov     # Generate coverage report

# Installation (once implemented)
npm install -g xvn            # Install from npm
xvn setup                     # Configure shell integration
xvn --version                 # Check version
xvn activate <path>           # Manually activate version

# Development Tools
cargo watch -x test           # Auto-run tests on changes
cargo audit                   # Security vulnerability check
```

## Important Design Decisions

1. **Rust over Node.js** - 2-3x performance gain, memory safety, zero runtime dependencies
2. **npm distribution** - Leverages existing Node.js ecosystem, binary downloads via postinstall
3. **File descriptor #3 protocol** - Inherited from avn, allows child process to modify parent shell
4. **Plugin-first design** - Even built-in version managers are plugins
5. **Auto-install with prompts** - UX improvement over avn (prompts to install missing versions)
6. **Modular shell integration** - Future-ready for additional shell support

## File Structure (Planned)

```
xvn/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── config.rs            # Configuration parsing
│   ├── plugins/             # Plugin system
│   │   ├── mod.rs           # Plugin trait and loader
│   │   ├── nvm.rs           # nvm plugin
│   │   └── fnm.rs           # fnm plugin
│   ├── shell/               # Shell integration
│   │   ├── mod.rs           # Shell abstraction
│   │   └── hooks.rs         # chpwd hooks
│   └── version.rs           # Version file detection
├── tests/                   # Integration tests
├── shell/
│   └── xvn.sh               # Shell hook script
├── Cargo.toml               # Rust project manifest
├── package.json             # npm package manifest
└── install.js               # npm postinstall binary download
```

## Key Constraints

- **No backward compatibility with avn** - Clean slate implementation
- **Not a version manager replacement** - Requires nvm/fnm/n to be installed
- **Unix-only initially** - Windows support in Phase 2
- **Shell-specific** - bash/zsh only in Phase 1

## Task Tracking

Tasks are tracked in `/docs/tasks/<branch-name>.md` using the conventional commit format. Update task lists as work progresses with checkmarks for completed items.
