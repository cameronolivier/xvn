# Milestone 1: Core Infrastructure - Tasks

**Timeline:** Weeks 1-2
**Version:** v0.1.0
**Status:** Not Started

---

## Tasks

### M1.1: Set up Rust project structure with Cargo.toml
- [x] Initialize Rust project with `cargo init`
- [x] Configure Cargo.toml dependencies
- [x] Create module structure (cli, config, version_file, error)
- [x] Set up Git repository

### M1.2: Implement CLI framework with clap
- [ ] `xvn setup` command skeleton
- [ ] `xvn activate` command skeleton
- [ ] `xvn --version` command
- [ ] `xvn --help` command
- [ ] Global `--verbose` flag

### M1.3: Implement configuration system
- [ ] Define Config struct with serde
- [ ] Parse YAML from ~/.xvnrc
- [ ] Parse YAML from .xvn.yaml (project config)
- [ ] Merge configs with precedence (project > user > default)
- [ ] Validate configuration (plugin names, version files)

### M1.4: Implement version file detection
- [ ] `find_version_file()` - walk up directory tree
- [ ] Parse version file (first line, trim whitespace)
- [ ] Stop at HOME directory
- [ ] Handle errors (permissions, empty files)

### M1.5: Set up error handling
- [ ] Define XvnError enum with thiserror
- [ ] Implement user-friendly error messages
- [ ] Error context (file paths, versions)

### M1.6: Set up logging
- [ ] Configure env_logger
- [ ] Add debug logging for key operations

### M1.7: Unit tests for core infrastructure
- [ ] Config parsing tests (valid, invalid, defaults)
- [ ] Version file discovery tests
- [ ] Error handling tests
- [ ] Achieve >80% coverage

---

## Success Criteria

- ✅ `xvn --version` returns correct version
- ✅ Config file parsed correctly with defaults
- ✅ Version file found in parent directories
- ✅ Tests passing with >80% coverage

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
