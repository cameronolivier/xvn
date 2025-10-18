# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**anvs** is a high-performance Rust-based automatic Node.js version switcher designed to be 2-3x faster than existing solutions like avn. It automatically switches your Node.js version when you `cd` into a directory with a `.nvmrc` or `.node-version` file.

**Current Status:** Production (v2.0.0) - The project is production-ready with full functionality for macOS and Linux.

## Build and Development Commands

### Essential Development Commands

```bash
# Build and test
make build              # Build debug binary
make release           # Build optimized release binary
make test              # Run all tests
make coverage          # Generate code coverage report

# Development workflow
make dev               # Build and install for local development
make setup             # Install and run setup (local development)
make check             # Run all checks (fmt, clippy, test)

# Code quality
make lint              # Run clippy linter
make fmt               # Format code
make fmt-check         # Check code formatting

# Installation testing
make install           # Install anvs to ~/.cargo/bin
make uninstall         # Remove anvs from ~/.cargo/bin

# Release management
make version-patch     # Bump patch version (0.6.1 -> 0.6.2)
make version-minor     # Bump minor version (0.6.1 -> 0.7.0)

# NPM packaging
make npm-pack          # Create npm package tarball
```

### Cargo Commands

```bash
# Direct Cargo usage
cargo build --release          # Optimized build
cargo test                    # Run unit tests
cargo test --all-features     # Run all tests including integration
cargo bench                   # Run performance benchmarks
cargo clippy -- -D warnings   # Strict linting
cargo fmt                     # Code formatting
```

### Testing Specific Components

```bash
# Run specific test files
cargo test cli_test           # CLI command tests
cargo test config_test        # Configuration tests
cargo test plugin_test        # Plugin system tests
cargo test integration       # Integration tests

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Architecture Overview

### Core Components and Responsibilities

1. **Shell Integration (`shell/anvs.sh`)** - Bash/zsh hooks that detect directory changes and trigger version switching
2. **CLI Module (`src/cli.rs`)** - Command-line interface using clap for setup, activate, and status commands
3. **Config System (`src/config.rs`)** - YAML-based configuration loading from `~/.anvsrc` and `.anvs.yaml`
4. **Plugin System (`src/plugins/`)** - Modular version manager support (nvm, fnm) via trait-based plugins
5. **Activation Orchestrator (`src/activation.rs`)** - Coordinates version detection, plugin matching, and command generation
6. **Version File Detection (`src/version_file.rs`)** - Searches directory tree for `.nvmrc` and `.node-version` files

### Plugin Architecture

The plugin system uses Rust traits to abstract version managers:

```rust
pub trait VersionManagerPlugin {
    fn name(&self) -> &str;
    fn is_available(&self) -> Result<bool>;
    fn has_version(&self, version: &str) -> Result<bool>;
    fn activate_command(&self, version: &str) -> Result<String>;
    fn install_command(&self, version: &str) -> Result<String>;
}
```

Built-in plugins (nvm, fnm) are compiled into the binary for performance. Future versions will support dynamic plugin loading.

### File Descriptor #3 Protocol

anvs uses a sophisticated IPC mechanism where the child process (anvs binary) writes shell commands to file descriptor 3, which the parent shell captures and evaluates. This allows anvs to modify the parent shell environment without requiring `eval` or `source` commands.

## Performance Targets and Benchmarks

- **Target Activation Time:** <100ms (P95) for version switching without installation
- **Memory Usage:** <5MB resident
- **Binary Size:** <5MB compressed

Performance testing is integrated into the test suite with benchmarks using the `criterion` crate.

## Configuration Management

### Configuration Hierarchy (highest precedence first)
1. Project-level: `.anvs.yaml` in project root
2. User-level: `~/.anvsrc`
3. Built-in defaults

### Configuration Options
- `plugins`: Priority-ordered list of version managers (`nvm`, `fnm`)
- `auto_install`: Behavior for missing versions (`prompt`, `always`, `never`)
- `silent`: Suppress output (boolean)
- `version_files`: Priority-ordered version file names

## Error Handling Philosophy

The project follows a strict error handling philosophy:
- **Silent errors:** Expected conditions (no version file found)
- **Warnings:** Show to user but not fatal (version not installed)
- **Fatal errors:** Show detailed message with actionable hints, exit cleanly

All errors implement the `AnvsError` trait with helpful context and hints for resolution.

## Testing Strategy

### Test Coverage Requirements
- Unit tests: >85% line coverage
- Integration tests with mock plugins
- Shell integration tests (bash, zsh)
- Performance benchmarks with regression detection

### Test Organization
- `tests/` - Integration and end-to-end tests
- `src/` - Unit tests alongside implementation
- Comprehensive CLI testing using `assert_cmd` crate

## Important Implementation Notes

1. **Security**: All shell arguments are escaped using the `shell-escape` crate to prevent command injection
2. **Distribution**: The project uses npm for distribution with pre-compiled Rust binaries downloaded during postinstall
3. **Shell Compatibility**: Currently supports bash and zsh, with planned support for fish and nushell
4. **Platform Support**: macOS and Linux (x64/arm64), Windows support planned

## Development Rules from CLAUDE.md

- Always use conventional commits when committing code
- Follow the architectural principles: Speed First, Modular & Extensible, Fail-Safe Operation, UNIX Philosophy
- Plugin system must remain extensible - even built-in plugins follow the same trait interface
- Never break the user's shell environment - graceful degradation on all errors
- Maintain performance targets - profile hot paths and optimize for <100ms activation time
- All configuration changes must be backward compatible
- Test coverage must remain above 85% for core modules

## Project Documentation Structure

- `docs/ARCHITECTURE.md` - High-level system design and architectural decisions
- `spec/` - Detailed project specifications and milestone plans
- `CONTRIBUTING.md` - Guidelines for contributors
- `README.md` - User-facing documentation and installation instructions

## Build Artifacts and Distribution

The project produces:
- Rust binary compiled for multiple platforms (Linux/macOS x64/arm64)
- npm package with platform-specific binary downloads
- Shell integration scripts for bash/zsh

Binary distribution is handled via GitHub Releases with npm postinstall downloading the correct binary for the user's platform.