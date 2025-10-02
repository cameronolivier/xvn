# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for v1.0.0

- Incorporate beta feedback
- Final performance tuning
- Final documentation polish

## [0.6.0] - 2025-01-02

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
