# Milestone 1: Core Infrastructure - Tasks

**Timeline:** Weeks 1-2
**Version:** v0.1.0
**Status:** ✅ Complete

---

## Tasks

### M1.1: Set up Rust project structure with Cargo.toml
- [x] Initialize Rust project with `cargo init`
- [x] Configure Cargo.toml dependencies
- [x] Create module structure (cli, config, version_file, error)
- [x] Set up Git repository

### M1.2: Implement CLI framework with clap
- [x] `xvn setup` command skeleton
- [x] `xvn activate` command skeleton
- [x] `xvn --version` command
- [x] `xvn --help` command
- [x] Global `--verbose` flag

### M1.3: Implement configuration system
- [x] Define Config struct with serde
- [x] Parse YAML from ~/.xvnrc
- [x] Parse YAML from .xvn.yaml (project config)
- [x] Merge configs with precedence (project > user > default)
- [x] Validate configuration (plugin names, version files)

### M1.4: Implement version file detection
- [x] `find_version_file()` - walk up directory tree
- [x] Parse version file (first line, trim whitespace)
- [x] Stop at HOME directory
- [x] Handle errors (permissions, empty files)

### M1.5: Set up error handling
- [x] Define XvnError enum with thiserror
- [x] Implement user-friendly error messages
- [x] Error context (file paths, versions)

### M1.6: Set up logging
- [x] Configure env_logger
- [x] Add debug logging for key operations

### M1.7: Unit tests for core infrastructure
- [x] Config parsing tests (valid, invalid, defaults)
- [x] Version file discovery tests
- [x] Error handling tests
- [x] Achieve >80% coverage

---

## Success Criteria

- ✅ `xvn --version` returns correct version
- ✅ Config file parsed correctly with defaults
- ✅ Version file found in parent directories
- ✅ Tests passing with >80% coverage

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
