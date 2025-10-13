# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.5.0] - 2025-10-13

### Added - Automatic Default Version Return

- **Automatic Default Version Switching**
  - xvn now automatically switches to your default Node.js version when leaving a project directory
  - Uses `nvm alias default` for nvm users
  - Uses `fnm default` for fnm users
  - Configurable via `use_default: true` setting in `~/.xvnrc` (enabled by default)
  - Toggle with `xvn set use-default` command
  - View status with `xvn status` command

- **New Plugin Method: `default_version()`**
  - All version manager plugins now support querying default version
  - Graceful handling when no default is configured
  - Priority-based plugin selection (first available plugin's default is used)

### Added - Complete Uninstall Command

- **`xvn uninstall` Command**
  - Complete removal of xvn with automatic cleanup
  - Detects all installation methods (npm, Homebrew, Cargo)
  - Removes `~/.xvn` directory (all versions and binaries)
  - Removes `~/.xvnrc` configuration file
  - Removes shell integration from `.bashrc` and `.zshrc`
  - Interactive confirmation with `--force` flag to skip
  - Actually executes uninstall commands for external packages
  - Shows success/failure status for each removal step

- **External Package Uninstallation**
  - Prompts: "Also uninstall external packages?"
  - Runs `brew uninstall xvn` for Homebrew installations
  - Runs `npm uninstall -g @olvrcc/xvn` for npm installations
  - Runs `cargo uninstall xvn` for Cargo installations
  - Provides manual instructions if automatic removal fails

### Changed

- Made `profile_modification` module public for uninstall access
- Added `remove_from_profile()` function for shell integration cleanup
- Updated CLI help examples to include `xvn set` and `xvn uninstall`
- Fixed GitHub repository URL from cameronolivier to olvrcc

### Documentation

- Added comprehensive "Uninstalling" section to README
- Documented nvm global package behavior (why preuninstall hooks don't always run)
- Explained Node version-specific global package storage in nvm

### Technical Details

- Files added: `src/commands/uninstall.rs`
- Files modified: `src/cli.rs`, `src/commands/mod.rs`, `src/setup/mod.rs`, `src/setup/profile_modification.rs`, `README.md`
- Uses `InstallationDetector` to find all xvn installations
- Uses `std::process::Command` to execute package manager commands
- All tests passing

## [1.4.2] - 2025-10-12

### Added - Installation Conflict Detection (M9.6)

- **Multiple Installation Detection**
  - Automatically detects when xvn is installed via multiple methods (npm, Homebrew, Cargo)
  - Uses `which::which_all()` to find all xvn binaries in PATH
  - Identifies installation method by examining binary path patterns
  - Ignores internal symlinks at `~/.xvn/bin/xvn` and `~/.xvn/current/`

- **Interactive Conflict Warning (init/setup)**
  - Shows detailed warning during `xvn init` or `xvn setup` when conflicts detected
  - Lists all installations with method descriptions and file paths
  - Provides uninstall commands for each method (npm, Homebrew, Cargo)
  - Prompts user to continue or cancel setup
  - Creates persistent flag file at `~/.xvn/conflict_warning` for future warnings

- **Activation Warning**
  - Shows brief warning on `xvn activate` if conflict flag exists
  - Directs user to run `xvn init` for details
  - Minimal performance impact (single file existence check)
  - Warning automatically clears when conflicts resolved

### Changed

- Added `which = "6.0"` dependency to Cargo.toml
- Applied cargo fmt formatting improvements

### Technical Details

- Files added: `src/installation_detector.rs`, `spec/milestone-9/M9.6_CONFLICT_DETECTION.md`
- Files modified: `src/lib.rs`, `src/init/wizard.rs`, `src/cli.rs`, `Cargo.toml`
- New module: `installation_detector` with `InstallMethod` enum and `InstallationDetector` struct
- State management via `~/.xvn/conflict_warning` flag file
- All 124 unit tests + 6 CLI tests passing

## [1.4.1] - 2025-10-10

### Fixed

- **Package Distribution Issue**
  - Added missing `shell/` directory to npm package files
  - Fixes installation error: "ENOENT: no such file or directory, copyfile shell/xvn.sh"
  - Shell integration scripts now properly included in published package

### Technical Details

- Files modified: `package.json` (added "shell/" to files array)
- This is a patch release to fix the broken v1.4.0 npm package

## [1.4.0] - 2025-10-09

### Added - Interactive Configuration Setter

- **`xvn set` Command**
  - Easily update individual config settings without re-running full `xvn init` wizard
  - Interactive menu when called without arguments: `xvn set`
  - Direct setting modification: `xvn set auto-install`, `xvn set plugins`, `xvn set version-files`
  - Colorful, user-friendly prompts with help text
  - Shows current value before prompting for new value
  - Automatic config file save with timestamp

- **Supported Settings**
  - `auto-install`: Choose prompt/always/never mode
  - `plugins`: Select version managers with multi-select (maintains priority order)
  - `version-files`: Select version files to check (maintains priority order)

### Changed

- Config module now exports `loader` as public for use by other commands
- Improved modularity with new `commands/` module structure

### Technical Details

- Files added: `src/commands/mod.rs`, `src/commands/set.rs`
- Files modified: `src/cli.rs`, `src/lib.rs`, `src/config/mod.rs`
- Uses `inquire` crate for rich interactive prompts
- Config file format preserved with comments and timestamps

## [1.3.0] - 2025-10-09

### Added - Version-Independent Installation (Milestone 10)

- **Version-Independent Binary Installation**
  - xvn binary now installed to `~/.xvn/versions/v{VERSION}/bin/xvn`
  - Symlink `~/.xvn/bin/xvn` always points to current version
  - Symlink `~/.xvn/current` points to active version directory
  - Binary remains available after switching Node.js versions
  - Fixes critical "xvn not found after version switch" issue

- **Automatic Uninstall Script**
  - New `uninstall.js` runs automatically via `preuninstall` hook
  - Removes shell integration from `.zshrc`/`.bashrc`
  - Removes `~/.xvn` directory (all versions and binaries)
  - Removes `~/.xvnrc` configuration file
  - Provides helpful restart shell instructions

- **Enhanced Setup Command**
  - `xvn setup` now adds `export PATH="$HOME/.xvn/bin:$PATH"` to shell profile
  - Shell hook sources from `$XVN_DIR/current/lib/xvn.sh`
  - Both `xvn init` and `xvn setup` install shell integration automatically

### Changed

- `install.js` creates versioned directory structure at `~/.xvn/versions/v{VERSION}/`
- Automatic symlink creation during installation
- Automatic cleanup of old versions (keeps last 2)
- Shell integration now includes PATH modification for version-independent access

### Technical Details

- Files added: `uninstall.js`
- Files modified: `install.js`, `package.json`, `src/setup/profile_modification.rs`
- Shell profile markers: `# >>> xvn initialize >>>` and `# <<< xvn initialize <<<`
- All tests passing

## [1.1.0] - 2025-10-02

### Added - package.json Support (Milestone 8)

- **`package.json` Version Detection**
  - Parse `engines.node` field from package.json files
  - Support for semver ranges (`^20.0.0`, `~18.20.0`, `>=18.0.0`, etc.)
  - Automatic resolution to highest matching installed version
  - Configurable priority with `.nvmrc` and `.node-version`

- **Semver Range Resolution**
  - New `SemverResolver` for range-to-version resolution
  - Query version managers for installed versions
  - Find best match for semver requirements
  - Fallback to original range if no match found

- **Enhanced Version Manager Plugin API**
  - New `list_versions()` method to support semver resolution
  - Default implementation returns empty list
  - MockPlugin updated for testing

- **Init Wizard Updates**
  - package.json option in version files prompt
  - Help text explaining semver range support
  - Educational descriptions for each file type

### Changed

- VersionFile now includes `source` field to track file type
- Version finder handles package.json with special logic
- Activation flow resolves semver ranges before checking installation
- README updated with package.json examples and semver syntax

### Technical Details

- Dependencies: Added `serde_json` 1.0 and `semver` 1.0
- New modules: `version_file::package_json`, `version_file::semver`
- 20 new unit tests for package.json and semver
- All 116 tests passing

### Planned for v1.0.0

- Incorporate beta feedback
- Final performance tuning
- Final documentation polish

## [0.6.0] - 2025-10-02

### Added

- **Core Infrastructure (M1)**
  - CLI with `activate`, `setup`, and `status` commands
  - Configuration loading from `~/.xvnrc` and `.xvn.yaml`
  - Version file detection (`.nvmrc`, `.node-version`)
  - Error handling with user-friendly messages

- **Plugin System (M2)**
  - Plugin trait for version manager integration
  - Built-in nvm plugin
  - Built-in fnm plugin
  - Plugin registry with priority ordering
  - Dynamic plugin configuration

- **Shell Integration (M3)**
  - FD:3 protocol for parent shell modification
  - Shell hooks for bash and zsh
  - Setup command for automatic installation
  - Idempotent shell profile modification

- **Version Activation (M4)**
  - Activation orchestrator
  - Auto-install prompts (prompt/always/never modes)
  - Plugin fallback when version not available
  - LTS version support

- **Testing & Polish (M5)**
  - Comprehensive test suite (85%+ coverage)
  - Integration tests for end-to-end flows
  - Security tests for shell injection prevention
  - Performance benchmarks
  - Full documentation

- **Release Infrastructure (M6)**
  - GitHub Actions CI/CD pipeline
  - Cross-platform binary builds (Linux x64/arm64, macOS x64/arm64)
  - npm package with automatic binary download
  - SHA256 checksum verification

### Performance

- Activation time: <100ms (P50), <150ms (P95)
- Memory footprint: <5MB
- Binary size: <3MB compressed

### Security

- Shell command escaping to prevent injection attacks
- Checksum verification for binary downloads
- Sandboxed plugin execution

## [0.5.0] - 2024-12-29

### Added

- Testing infrastructure and comprehensive test suite
- Integration tests for activation flows
- Security tests for shell injection prevention
- Performance benchmarks
- Documentation for all public APIs

## [0.4.0] - 2024-12-28

### Added

- Version activation orchestrator
- Auto-install prompts for missing versions
- Plugin fallback mechanism
- LTS version support

## [0.3.0] - 2024-12-27

### Added

- Shell integration via FD:3 protocol
- Setup command for automatic installation
- Shell hooks for bash and zsh

## [0.2.0] - 2024-12-26

### Added

- Plugin system with trait-based architecture
- nvm plugin implementation
- fnm plugin implementation
- Plugin registry with priority ordering

## [0.1.0] - 2024-12-25

### Added

- Initial release
- Basic CLI structure
- Configuration loading
- Version file detection
