# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**xvn** is a Rust-based automatic Node.js version switcher that activates on `cd`, designed to be 2-3x faster than its predecessor (avn).

**Key Characteristics:**
- Language: Rust (for performance, safety, zero runtime dependencies)
- Distribution: npm with pre-compiled binaries + Homebrew tap
- Platforms: Linux (x64/arm64), macOS (x64/arm64)
- Architecture: Modular plugin system for version managers (nvm, fnm)
- Version: 1.6.1 (MVP released, actively maintained)

## Project Status

**Current Phase:** Production (v1.6.1)
- MVP complete and published to npm (@olvrcc/xvn)
- Homebrew tap available (olvrcc/xvn)
- CI/CD with GitHub Actions for releases
- Active user base with ongoing enhancements

## Documentation Structure

- **[README.md](./README.md)** - User-facing documentation, installation, and usage
- **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - System design and architectural decisions
- **[docs/MIGRATION.md](./docs/MIGRATION.md)** - Migration guide for upgrading from older versions
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** - Contribution guidelines
- **[spec/](./spec/)** - Historical planning documents and milestone specifications (reference)

## Current Features (v1.6.1)

✅ **Implemented:**
- Core CLI with `activate`, `status`, `setup`, `uninstall`, `set` commands
- Configuration via `~/.xvnrc` and `./.xvn.yaml`
- Version file detection: `.nvmrc`, `.node-version`, `package.json`
- Plugin system with nvm and fnm support
- Shell integration (bash/zsh via chpwd hooks)
- FD:3 protocol for parent shell communication
- Auto-install prompts for missing versions
- Automatic return to default version when leaving projects
- Semver range support in `package.json` `engines.node`
- npm distribution with pre-compiled binaries
- Homebrew tap (olvrcc/xvn)
- CI/CD with GitHub Actions
- Comprehensive test suite (>85% coverage)

⏳ **Potential Future Enhancements:**
- Additional version managers (n, asdf, volta)
- Daemon mode for <10ms activation
- Windows/PowerShell support
- Additional shells (fish, nushell)
- Advanced commands (doctor, which, list, exec)

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


## Common Development Commands

```bash
# Development
cargo build                    # Build debug binary
cargo build --release          # Build optimized binary
cargo test                     # Run unit tests
cargo install --path .         # Install locally for testing

# Code Quality
cargo clippy -- -D warnings    # Rust linter (strict)
cargo fmt                      # Format code
npm run check                  # Run fmt, clippy, and tests together
npm run lint                   # Run clippy only

# Testing
cargo test                     # Run all tests
cargo test --test integration  # Run integration tests only
cargo test <test_name>         # Run specific test
./scripts/coverage.sh          # Generate code coverage report

# Local Installation for Development
npm run dev                    # Build and install locally (cargo install --path .)
npm run setup                  # Build, install locally, and run xvn setup

# Version Management
./scripts/bump-version.sh <major|minor|patch>  # Bump version in all files
./scripts/version.sh           # Display current version

# Release Process
npm run release:download       # Download release artifacts from GitHub Actions
npm run release:extract        # Extract binaries from archives
npm run release:pack           # Create npm package tarball
npm run release:verify         # Verify package contents
npm publish                    # Publish to npm (requires auth)

# Homebrew Tap
./scripts/setup-homebrew-tap.sh  # Create/update Homebrew formula

# User Commands
xvn setup                      # Configure shell integration
xvn activate [path]            # Manually activate version
xvn status                     # Show config and last activation time
xvn set <key>                  # Interactive config setting
xvn uninstall                  # Remove xvn completely
xvn --version                  # Check version
```

## Important Design Decisions

1. **Rust over Node.js** - 2-3x performance gain, memory safety, zero runtime dependencies
2. **npm distribution** - Leverages existing Node.js ecosystem, binary downloads via postinstall
3. **File descriptor #3 protocol** - Inherited from avn, allows child process to modify parent shell
4. **Plugin-first design** - Even built-in version managers are plugins
5. **Auto-install with prompts** - UX improvement over avn (prompts to install missing versions)
6. **Modular shell integration** - Future-ready for additional shell support

## Code Architecture

### Module Structure

```
src/
├── main.rs                  # CLI entry point (minimal)
├── lib.rs                   # Library exports
├── cli.rs                   # Command-line interface using clap
├── error.rs                 # Error types and XvnError
├── output.rs                # Terminal output formatting
│
├── activation/              # Version activation orchestration
│   ├── mod.rs               # Main activation logic
│   ├── orchestrator.rs      # Coordinates plugins and shell
│   ├── errors.rs            # Activation-specific errors
│   └── user_prompt.rs       # Interactive prompts for auto-install
│
├── commands/                # CLI subcommands
│   ├── mod.rs               # Command dispatching
│   ├── set.rs               # Interactive config setting
│   └── uninstall.rs         # Clean removal
│
├── config/                  # Configuration management
│   ├── mod.rs               # Config API
│   ├── schema.rs            # Config struct and defaults
│   └── loader.rs            # YAML parsing and merging
│
├── init/                    # Setup wizard
│   ├── mod.rs               # Setup command
│   ├── wizard.rs            # Interactive setup flow
│   ├── detection.rs         # Detect shell, version managers
│   ├── validation.rs        # Validate installation
│   └── prompts.rs           # User prompts
│
├── plugins/                 # Version manager plugins
│   ├── mod.rs               # Plugin exports
│   ├── trait_def.rs         # VersionManagerPlugin trait
│   ├── registry.rs          # Plugin loading and priority
│   ├── nvm.rs               # nvm implementation
│   ├── fnm.rs               # fnm implementation
│   └── mock.rs              # Test mock plugin
│
├── setup/                   # Shell profile modification
│   ├── mod.rs               # Setup logic
│   ├── installer.rs         # Binary installation
│   ├── shell_detection.rs   # Detect shell type
│   └── profile_modification.rs  # Modify .bashrc/.zshrc
│
├── shell/                   # Shell communication
│   ├── mod.rs               # Shell abstraction
│   ├── fd3.rs               # File descriptor 3 protocol
│   └── json_writer.rs       # JSON output for --json flag
│
└── version_file/            # Version file detection
    ├── mod.rs               # Main finder logic
    ├── finder.rs            # Walk directory tree
    ├── package_json.rs      # Parse package.json engines
    └── semver.rs            # Semver range resolution

tests/                       # Integration tests
├── integration.rs           # End-to-end tests
├── config_test.rs           # Config loading tests
├── plugin_test.rs           # Plugin system tests
├── version_file_test.rs     # Version file detection tests
├── shell_integration.rs     # Shell hook tests
└── security_test.rs         # Security validations

shell/
└── xvn.sh                   # Shell hook script (bash/zsh)

scripts/                     # Release and dev scripts
├── bump-version.sh          # Bump version across files
├── version.sh               # Display current version
├── download-artifacts.sh    # Download CI build artifacts
├── extract-binaries.sh      # Extract release binaries
└── setup-homebrew-tap.sh    # Create Homebrew formula
```

### Key Components

**Activation Flow:**
1. `cli.rs` parses command (`activate`, `status`, etc.)
2. `activation/orchestrator.rs` coordinates the activation:
   - `version_file/finder.rs` walks up directory tree to find `.nvmrc`, etc.
   - `version_file/package_json.rs` parses `engines.node` if needed
   - `plugins/registry.rs` loads plugins in priority order
   - Each plugin attempts to resolve the version
   - If missing, `activation/user_prompt.rs` prompts to install
3. `shell/fd3.rs` writes activation command to file descriptor 3
4. Parent shell executes the command

**Shell Integration:**
- `shell/xvn.sh` hooks into `chpwd` (bash/zsh)
- Triggers `xvn activate` on directory change
- Uses FD:3 protocol to modify parent shell environment
- Handles idempotency (doesn't re-activate same version)

**Plugin System:**
- `plugins/trait_def.rs` defines `VersionManagerPlugin` trait
- Built-in plugins: `nvm.rs`, `fnm.rs` (compiled into binary)
- Registry loads plugins in priority order from config
- Each plugin checks if the version manager is installed
- Plugins generate shell commands for activation

**Configuration:**
- `config/loader.rs` reads `~/.xvnrc` and `./.xvn.yaml`
- Project config overrides global config
- YAML format with validation
- Interactive editing via `xvn set` command

## Important Constraints & Conventions

- **Central installation:** xvn installs to `~/.xvn/bin` to remain available across Node.js version changes
- **Not a version manager:** xvn requires nvm/fnm to be installed; it's a switcher, not a manager
- **Platform support:** Linux and macOS only (x64/arm64); Windows not supported
- **Shell support:** bash and zsh only (via chpwd hooks)
- **File descriptor 3 protocol:** Child process communicates shell commands to parent via FD:3
- **Conventional commits:** Always use conventional commit format (feat:, fix:, chore:, etc.)
- **No `any` types:** Never use `any` in TypeScript; always provide correct types (comment if impossible)
- **Commit after changes:** Always commit after every change with file list in message
- **Add before commit:** Always `git add` all files before committing

## Release Process

1. **Version bump:** Use `./scripts/bump-version.sh <major|minor|patch>` to update version in all files
2. **Git tag:** Create annotated git tag matching version (e.g., `v1.6.1`)
3. **Push tag:** `git push --tags` triggers GitHub Actions CI/CD
4. **CI builds:** GitHub Actions builds binaries for all platforms
5. **Download artifacts:** `npm run release:download` to get binaries
6. **Extract binaries:** `npm run release:extract` to prepare for packaging
7. **npm publish:** `npm publish` to release to npm registry
8. **Homebrew:** `./scripts/setup-homebrew-tap.sh` to update Homebrew formula

## Testing Strategy

### Unit Tests
- Located in `tests/` directory
- Run with `cargo test`
- Use `tempfile` for filesystem tests
- Mock plugins available in `src/plugins/mock.rs`

### Integration Tests
- `tests/integration.rs` for end-to-end scenarios
- Test shell hook integration
- Validate version file detection across directory trees

### Test Coverage
- Target: >85% line coverage
- Generate reports: `./scripts/coverage.sh`
- Uses `cargo tarpaulin`

## Dependencies

**Core:**
- `clap` - CLI argument parsing with derive macros
- `serde` + `serde_yaml` - Config serialization
- `anyhow` + `thiserror` - Error handling
- `semver` - Semantic version parsing
- `dirs` - Cross-platform directory locations

**User Interaction:**
- `inquire` - Interactive prompts
- `owo-colors` - Terminal colors
- `log` + `env_logger` - Debugging

**Dev:**
- `tempfile` - Temporary files in tests
- `assert_cmd` - CLI testing
- `predicates` - Test assertions
