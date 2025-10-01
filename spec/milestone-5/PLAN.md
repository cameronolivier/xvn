# Milestone 5: Testing & Polish - Implementation Plan

## Overview

Milestone 5 focuses on achieving comprehensive test coverage (>85%), establishing performance benchmarks, and completing documentation. This milestone ensures xvn is production-ready with robust testing, verified performance targets, and complete user/developer documentation.

**Goals:**
- Achieve >85% test coverage across all components
- Establish performance benchmarks and verify targets (<85ms P95 activation)
- Create comprehensive documentation (README, CONTRIBUTING, API docs)
- Ensure code quality with linting, formatting, and security audits

**Timeline:** Weeks 9-10
**Version:** v0.6.0-v0.9.0

## Prerequisites

Before starting, ensure:
- Milestones 1-4 are complete (all core functionality implemented)
- All existing tests are passing
- Development environment has:
  - `cargo-tarpaulin` installed: `cargo install cargo-tarpaulin`
  - `shellcheck` installed: `brew install shellcheck` (macOS) or `apt install shellcheck` (Linux)
  - `cargo-audit` installed: `cargo install cargo-audit`
  - `cargo-criterion` installed (comes with criterion dev-dependency)

## Implementation Tasks

### Task M5.1: Comprehensive unit test suite

**Objective:** Achieve >85% line coverage with comprehensive unit tests for all core modules.

**Implementation Steps:**

1. **Set up coverage tooling:**
   ```bash
   # Add to Cargo.toml [dev-dependencies] if not present:
   cargo install cargo-tarpaulin

   # Create script: scripts/coverage.sh
   #!/bin/bash
   cargo tarpaulin --out Html --out Lcov --output-dir coverage --exclude-files 'tests/*'
   ```

2. **Config module tests** (`tests/config_test.rs`):
   ```rust
   // Test cases to add:
   - test_config_with_empty_file() // Empty YAML should use defaults
   - test_config_with_invalid_yaml() // Should return ParseError
   - test_config_with_unknown_fields() // Should be ignored (serde default)
   - test_config_merge_precedence() // Project overrides user config
   - test_config_default_values() // Verify all defaults
   - test_config_plugin_priority_custom() // User-specified order
   - test_config_plugin_priority_default() // Default ["nvm", "fnm"]
   - test_config_with_comments() // YAML comments should be ignored
   - test_config_with_relative_paths() // Expand to absolute
   ```

3. **Version file discovery tests** (`tests/version_file_test.rs`):
   ```rust
   // Test cases to add:
   - test_find_version_file_current_dir() // .nvmrc in current dir
   - test_find_version_file_parent_dir() // Walk up tree
   - test_find_version_file_not_found() // Return None
   - test_find_version_file_multiple_types() // Prefer .nvmrc over .node-version
   - test_find_version_file_symlink() // Follow symlinks
   - test_find_version_file_permission_denied() // Skip unreadable dirs
   - test_parse_version_file_simple() // "18.20.0"
   - test_parse_version_file_with_v_prefix() // "v18.20.0" -> "18.20.0"
   - test_parse_version_file_with_whitespace() // Trim spaces/newlines
   - test_parse_version_file_with_comments() // Ignore # lines
   - test_parse_version_file_lts_alias() // "lts/hydrogen"
   - test_parse_version_file_node_prefix() // "node/18.20.0"
   - test_parse_version_file_empty() // Should error
   ```

4. **Plugin loading tests** (`tests/plugin_loading_test.rs`):
   ```rust
   // Test cases to add:
   - test_registry_loads_nvm_plugin()
   - test_registry_loads_fnm_plugin()
   - test_registry_respects_priority_order()
   - test_plugin_availability_nvm_not_installed()
   - test_plugin_availability_fnm_not_installed()
   - test_plugin_has_version_check()
   - test_plugin_activate_command_generation()
   - test_plugin_install_command_generation()
   - test_plugin_command_escaping() // Shell injection prevention
   ```

5. **Error handling tests** (`tests/error_handling_test.rs`):
   ```rust
   // Enhance existing tests/error_test.rs with:
   - test_error_display_user_friendly() // Check error messages
   - test_error_chain_preservation() // anyhow context
   - test_error_from_io_error()
   - test_error_from_yaml_parse_error()
   - test_error_no_version_file_found()
   - test_error_no_plugin_available()
   - test_error_version_not_installed()
   - test_error_config_not_found() // Should use defaults
   - test_error_permission_denied()
   - test_error_invalid_version_format()
   ```

6. **Activation orchestrator tests** (`tests/activation_test.rs`):
   ```rust
   // Test the full activation flow:
   - test_activation_with_installed_version()
   - test_activation_with_missing_version_prompts()
   - test_activation_with_no_available_plugins()
   - test_activation_with_plugin_priority_fallback()
   - test_activation_idempotency() // Same version twice
   - test_activation_fd3_output_format()
   - test_activation_error_handling()
   ```

7. **Run coverage and identify gaps:**
   ```bash
   ./scripts/coverage.sh
   # Open coverage/index.html and look for red/yellow areas
   # Add tests for uncovered branches
   ```

**Code Structure:**
- `tests/config_test.rs` - Config parsing and merging
- `tests/version_file_test.rs` - Version file discovery and parsing
- `tests/plugin_loading_test.rs` - Plugin registry and availability
- `tests/error_handling_test.rs` - Enhance existing error_test.rs
- `tests/activation_test.rs` - End-to-end activation logic
- `scripts/coverage.sh` - Coverage generation script

**Key Considerations:**
- Use `tempfile::TempDir` for file system tests (automatic cleanup)
- Use `MockPlugin` for testing activation without real version managers
- Test both success and failure paths for every function
- Focus on edge cases: empty files, missing files, malformed input
- Ensure error messages are user-friendly and actionable

**Testing:**
```bash
cargo test --lib            # Run unit tests
cargo test --test '*_test'  # Run integration tests
./scripts/coverage.sh       # Generate coverage report
# Target: >85% line coverage
```

**Dependencies:**
- None (can start immediately)

**Enables:**
- M5.2 (integration tests build on unit test patterns)
- M5.6 (code quality checks require passing tests)

---

### Task M5.2: Integration test suite

**Objective:** Test multi-component interactions and complex scenarios with realistic data.

**Implementation Steps:**

1. **End-to-end activation scenarios** (`tests/integration/activation_scenarios.rs`):
   ```rust
   // Create tests/integration/ directory structure:
   // tests/integration/
   //   mod.rs
   //   activation_scenarios.rs
   //   config_override.rs
   //   plugin_fallback.rs

   // Test cases:
   - test_e2e_simple_activation() {
       // Setup: Create temp dir with .nvmrc, mock nvm plugin
       // Execute: Run activation orchestrator
       // Verify: Check FD:3 output contains "nvm use 18.20.0"
   }

   - test_e2e_version_not_installed_prompt() {
       // Setup: .nvmrc with version not in mock plugin
       // Execute: Activation with user_prompt disabled
       // Verify: Output contains install command
   }

   - test_e2e_nested_directory_search() {
       // Setup: .nvmrc in parent dir, start in child dir
       // Execute: Find and activate
       // Verify: Correct version file found
   }

   - test_e2e_config_from_project_override() {
       // Setup: ~/.xvnrc with [nvm], .xvn.yaml with [fnm]
       // Execute: Activation in project
       // Verify: fnm takes precedence
   }
   ```

2. **Multi-plugin fallback logic** (`tests/integration/plugin_fallback.rs`):
   ```rust
   - test_fallback_first_unavailable() {
       // Setup: Priority [nvm, fnm], nvm unavailable
       // Execute: Activation
       // Verify: fnm is used
   }

   - test_fallback_first_no_version() {
       // Setup: Priority [nvm, fnm], nvm available but no version
       // Execute: Activation
       // Verify: fnm is used
   }

   - test_fallback_all_unavailable() {
       // Setup: All plugins unavailable
       // Execute: Activation
       // Verify: Error with helpful message
   }
   ```

3. **Config override precedence** (`tests/integration/config_override.rs`):
   ```rust
   - test_project_overrides_user_config() {
       // ~/.xvnrc: plugin_priority: [nvm]
       // .xvn.yaml: plugin_priority: [fnm]
       // Verify: fnm wins
   }

   - test_no_project_config_uses_user() {
       // Only ~/.xvnrc exists
       // Verify: User config values used
   }

   - test_partial_override() {
       // User: {plugin_priority: [nvm], auto_install: false}
       // Project: {auto_install: true}
       // Verify: Merged config {plugin_priority: [nvm], auto_install: true}
   }
   ```

4. **Auto-install flow variations** (`tests/integration/auto_install.rs`):
   ```rust
   - test_auto_install_prompt_yes() {
       // Mock user input "y"
       // Verify: Install command output
   }

   - test_auto_install_prompt_no() {
       // Mock user input "n"
       // Verify: Graceful exit
   }

   - test_auto_install_disabled_in_config() {
       // Config: auto_install: false
       // Verify: No prompt, just error message
   }
   ```

5. **Create test helpers** (`tests/common/mod.rs`):
   ```rust
   // Shared test utilities:
   pub fn create_test_environment() -> TestEnv {
       // Creates temp dir, mock config, mock plugins
   }

   pub fn mock_version_file(dir: &Path, content: &str) {
       // Creates .nvmrc with content
   }

   pub fn run_activation(env: &TestEnv, dir: &Path) -> Result<String> {
       // Runs activation and captures FD:3 output
   }
   ```

**Code Structure:**
- `tests/integration/mod.rs` - Integration test module
- `tests/integration/activation_scenarios.rs` - E2E activation tests
- `tests/integration/plugin_fallback.rs` - Plugin priority tests
- `tests/integration/config_override.rs` - Config merging tests
- `tests/integration/auto_install.rs` - Auto-install prompt tests
- `tests/common/mod.rs` - Shared test helpers

**Key Considerations:**
- Use `assert_cmd::Command` for CLI testing
- Mock plugins should simulate both available/unavailable states
- Test config merging exhaustively (this is error-prone)
- Use `predicates` crate for fuzzy output matching
- Ensure tests are reproducible (no external dependencies)

**Testing:**
```bash
cargo test --test integration  # Run all integration tests
cargo test integration::activation_scenarios::test_e2e_simple
```

**Dependencies:**
- Requires: M5.1 (unit tests establish testing patterns)

**Enables:**
- M5.3 (shell tests are similar integration-style tests)

---

### Task M5.3: Shell test suite

**Objective:** Test shell hook installation, directory change detection, and command evaluation in real bash/zsh environments.

**Implementation Steps:**

1. **Enhance existing shell integration test** (`tests/shell_integration.rs`):
   ```rust
   // Already has basic shellcheck test, add more:

   - test_hook_sources_correctly() {
       // Create temp profile, add source line
       // Verify: bash -c 'source profile && type __xvn_chpwd' succeeds
   }

   - test_hook_defines_chpwd_function() {
       // Source xvn.sh, check __xvn_chpwd exists
   }

   - test_hook_adds_to_chpwd_functions_array() {
       // Verify: PROMPT_COMMAND contains __xvn_chpwd (bash)
       // Verify: chpwd_functions contains __xvn_chpwd (zsh)
   }
   ```

2. **Create bash end-to-end tests** (`tests/shell/test_bash_e2e.sh`):
   ```bash
   #!/bin/bash
   set -euo pipefail

   # Test 1: Directory change triggers activation
   test_directory_change_activates() {
       # Create temp dir with .nvmrc
       # Source xvn.sh
       # cd into dir
       # Verify: activation occurred (check for command output)
   }

   # Test 2: Idempotency (cd to same dir twice)
   test_idempotency() {
       # cd into dir with .nvmrc
       # cd ../other_dir && cd back
       # Verify: Second cd doesn't re-activate
   }

   # Test 3: Version file change detection
   test_version_file_change() {
       # cd into dir with .nvmrc "18.20.0"
       # Change .nvmrc to "20.0.0"
       # cd .. && cd back
       # Verify: Re-activation with new version
   }

   # Test 4: Parent directory version file
   test_parent_directory_search() {
       # .nvmrc in /tmp/project/
       # cd into /tmp/project/src/
       # Verify: Parent .nvmrc is found
   }
   ```

3. **Create zsh end-to-end tests** (`tests/shell/test_zsh_e2e.sh`):
   ```bash
   #!/bin/zsh
   # Same tests as bash, but for zsh:
   # - Use chpwd_functions array instead of PROMPT_COMMAND
   # - Test zsh-specific hook syntax
   ```

4. **Command evaluation tests** (`tests/shell/test_fd3_protocol.sh`):
   ```bash
   # Test FD:3 protocol:

   test_fd3_command_evaluation() {
       # Mock xvn binary that writes to FD:3
       # Verify: Commands are evaluated in parent shell
       # Check: export commands modify shell environment
   }

   test_fd3_error_handling() {
       # Mock xvn binary that writes invalid command
       # Verify: Error is caught and logged
   }
   ```

5. **Idempotency tests** (enhance `tests/shell/test_xvn_sh.sh`):
   ```bash
   # Existing test file at tests/shell/test_xvn_sh.sh
   # Add tests:

   test_same_directory_no_reactivation() {
       # Set XVN_ACTIVE_VERSION_FILE=/tmp/test/.nvmrc
       # cd /tmp/test twice
       # Verify: Only one activation
   }

   test_different_version_file_triggers() {
       # cd to dir1 with .nvmrc
       # cd to dir2 with different .nvmrc
       # Verify: Activation happens for both
   }
   ```

6. **Profile modification tests** (`tests/shell/test_profile_modification.rs`):
   ```rust
   // Test the setup command's profile modification logic:

   - test_setup_adds_source_line() {
       // Create temp profile
       // Run setup
       // Verify: Profile contains "source ~/.xvn/bin/xvn.sh"
   }

   - test_setup_idempotent() {
       // Run setup twice
       // Verify: Only one source line added
   }

   - test_setup_preserves_existing_content() {
       // Profile with existing content
       // Run setup
       // Verify: Existing content unchanged
   }

   - test_setup_creates_profile_if_missing() {
       // No profile exists
       // Run setup
       // Verify: Profile created with source line
   }
   ```

7. **Run shell tests:**
   ```bash
   # Add to Cargo.toml:
   [[test]]
   name = "shell_integration"
   path = "tests/shell_integration.rs"

   # Run tests:
   cargo test --test shell_integration
   bash tests/shell/test_bash_e2e.sh
   zsh tests/shell/test_zsh_e2e.sh
   ```

**Code Structure:**
- `tests/shell_integration.rs` - Rust-based shell tests (existing, enhance)
- `tests/shell/test_xvn_sh.sh` - Basic shell script tests (existing, enhance)
- `tests/shell/test_bash_e2e.sh` - Bash end-to-end tests (new)
- `tests/shell/test_zsh_e2e.sh` - Zsh end-to-end tests (new)
- `tests/shell/test_fd3_protocol.sh` - FD:3 protocol tests (new)
- `tests/shell/test_profile_modification.rs` - Setup command tests (new)

**Key Considerations:**
- Shell tests are slow (spawn subprocesses) - keep them focused
- Use temp directories for all file system operations
- Test both bash and zsh (they have different hook mechanisms)
- Verify idempotency carefully (this is a common bug source)
- FD:3 tests need to mock the xvn binary output

**Testing:**
```bash
# Run all shell tests:
cargo test --test shell_integration
bash tests/shell/test_bash_e2e.sh
zsh tests/shell/test_zsh_e2e.sh

# Validate shell script quality:
shellcheck shell/xvn.sh
shellcheck tests/shell/*.sh
```

**Dependencies:**
- Requires: M5.2 (integration test patterns)

**Enables:**
- M5.4 (benchmarking can start independently)

---

### Task M5.4: Performance benchmarking

**Objective:** Establish performance benchmarks and verify targets (<5ms file discovery, <20ms plugin matching, <85ms P95 activation).

**Implementation Steps:**

1. **Add criterion to dependencies** (`Cargo.toml`):
   ```toml
   [dev-dependencies]
   criterion = { version = "0.5", features = ["html_reports"] }

   [[bench]]
   name = "benchmarks"
   harness = false
   ```

2. **File discovery benchmark** (`benches/benchmarks.rs`):
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   use std::fs;
   use tempfile::TempDir;
   use xvn::version_file::find_version_file;

   fn bench_file_discovery(c: &mut Criterion) {
       // Setup: Create nested directory structure
       let temp = TempDir::new().unwrap();
       let deep = temp.path().join("a/b/c/d/e/f/g/h");
       fs::create_dir_all(&deep).unwrap();
       fs::write(temp.path().join(".nvmrc"), "18.20.0").unwrap();

       c.bench_function("file_discovery_deep_nested", |b| {
           b.iter(|| {
               find_version_file(black_box(&deep)).unwrap()
           });
       });

       // Target: <5ms
   }

   fn bench_file_discovery_current_dir(c: &mut Criterion) {
       // Setup: .nvmrc in current dir
       let temp = TempDir::new().unwrap();
       fs::write(temp.path().join(".nvmrc"), "18.20.0").unwrap();

       c.bench_function("file_discovery_current_dir", |b| {
           b.iter(|| {
               find_version_file(black_box(temp.path())).unwrap()
           });
       });

       // Target: <1ms (should be very fast)
   }
   ```

3. **Plugin matching benchmark** (`benches/benchmarks.rs`):
   ```rust
   fn bench_plugin_matching(c: &mut Criterion) {
       use xvn::plugins::{MockPlugin, PluginRegistry};

       // Setup: Registry with 3 plugins, version available in last
       let plugins = vec![
           MockPlugin::new("first"),
           MockPlugin::new("second"),
           MockPlugin::new("third").with_version("18.20.0"),
       ];

       c.bench_function("plugin_matching_worst_case", |b| {
           b.iter(|| {
               // Iterate plugins to find match
               plugins.iter()
                   .find(|p| p.has_version("18.20.0").unwrap_or(false))
           });
       });

       // Target: <20ms
   }
   ```

4. **Total activation benchmark** (`benches/benchmarks.rs`):
   ```rust
   fn bench_total_activation(c: &mut Criterion) {
       use xvn::activation::ActivationOrchestrator;

       // Setup: Full realistic scenario
       let temp = TempDir::new().unwrap();
       fs::write(temp.path().join(".nvmrc"), "18.20.0").unwrap();

       let mock_plugin = MockPlugin::new("nvm").with_version("18.20.0");
       let orchestrator = ActivationOrchestrator::new(
           vec![Arc::new(mock_plugin)],
           Config::default(),
       );

       c.bench_function("total_activation_e2e", |b| {
           b.iter(|| {
               orchestrator.activate(black_box(temp.path())).unwrap()
           });
       });

       // Target: <85ms P95 (criterion will report P50, P95, P99)
   }
   ```

5. **Config loading benchmark** (`benches/benchmarks.rs`):
   ```rust
   fn bench_config_loading(c: &mut Criterion) {
       use xvn::config::ConfigLoader;

       // Setup: Temp config files
       let temp = TempDir::new().unwrap();
       let user_config = temp.path().join(".xvnrc");
       fs::write(&user_config, "plugin_priority: [nvm, fnm]\n").unwrap();

       c.bench_function("config_load_and_merge", |b| {
           b.iter(|| {
               ConfigLoader::new().load(black_box(&user_config)).unwrap()
           });
       });

       // Target: <5ms
   }
   ```

6. **Create benchmark runner script** (`scripts/bench.sh`):
   ```bash
   #!/bin/bash
   set -e

   echo "Running benchmarks..."
   cargo bench --bench benchmarks

   echo ""
   echo "Performance targets:"
   echo "  File discovery: <5ms"
   echo "  Plugin matching: <20ms"
   echo "  Total activation: <85ms (P95)"
   echo ""
   echo "Results saved to: target/criterion/report/index.html"
   ```

7. **Set up performance regression detection** (`benches/regression_check.sh`):
   ```bash
   #!/bin/bash
   # Run benchmarks and compare to baseline

   cargo bench --bench benchmarks -- --save-baseline main

   # After changes:
   # cargo bench --bench benchmarks -- --baseline main
   # This will show % change from baseline
   ```

**Code Structure:**
- `benches/benchmarks.rs` - All criterion benchmarks
- `scripts/bench.sh` - Benchmark runner script
- `scripts/regression_check.sh` - Regression detection script

**Key Considerations:**
- Use `black_box()` to prevent compiler optimizations
- Run benchmarks on quiet system (no background processes)
- Benchmark with realistic data (real directory structures, configs)
- Set up regression detection to prevent performance degradation
- Document performance targets clearly

**Testing:**
```bash
./scripts/bench.sh
# Review HTML report: target/criterion/report/index.html

# Check for regressions:
cargo bench --bench benchmarks -- --save-baseline main
# (make changes)
cargo bench --bench benchmarks -- --baseline main
```

**Dependencies:**
- None (can run in parallel with M5.1-5.3)

**Enables:**
- M5.6 (performance data for README)

---

### Task M5.5: Documentation

**Objective:** Create comprehensive user and developer documentation.

**Implementation Steps:**

1. **README.md** (user-facing documentation):
   ```markdown
   # xvn - Extreme Version Switcher for Node.js

   **Automatically switch Node.js versions based on project settings**

   🚀 **2-3x faster than avn** | 🦀 **Written in Rust** | 🔌 **Plugin-based**

   ## Quick Start

   ```bash
   npm install -g xvn
   xvn setup
   # Restart your shell
   cd your-project-with-nvmrc
   # Node version automatically activated!
   ```

   ## Features

   - ⚡ **Fast**: <100ms activation time
   - 🎯 **Automatic**: Detects `.nvmrc`, `.node-version` files
   - 🔌 **Flexible**: Supports nvm, fnm, n, and more
   - 🛡️ **Safe**: Never breaks your shell
   - 📦 **Easy**: Single npm install, works with existing tools

   ## Installation

   ### Requirements
   - A version manager installed (nvm, fnm, or n)
   - bash or zsh shell

   ### Install via npm
   ```bash
   npm install -g xvn
   xvn setup
   ```

   The `xvn setup` command adds a hook to your shell profile.

   ## Usage

   ### Automatic Version Switching
   Just `cd` into a directory with a version file:
   ```bash
   cd my-project  # Has .nvmrc with "18.20.0"
   # → Automatically switches to Node.js 18.20.0
   ```

   ### Manual Activation
   ```bash
   xvn activate /path/to/project
   ```

   ### Check Status
   ```bash
   xvn status
   ```

   ## Configuration

   Create `~/.xvnrc`:
   ```yaml
   plugin_priority:
     - nvm
     - fnm
   auto_install: true
   ```

   Project overrides: `.xvn.yaml` in your project root.

   ## Comparison to avn

   | Feature | xvn | avn |
   |---------|-----|-----|
   | Language | Rust | Node.js |
   | Speed | <100ms | ~250ms |
   | Memory | <5MB | ~50MB |
   | Auto-install prompts | ✅ | ❌ |
   | Plugin system | Built-in + dynamic | npm packages |

   ## Troubleshooting

   ### xvn not activating
   1. Check setup: `xvn setup --check`
   2. Verify version file: `cat .nvmrc`
   3. Check plugin availability: `xvn status`

   ### Performance issues
   Run benchmarks: `xvn benchmark`

   ## Contributing

   See [CONTRIBUTING.md](./CONTRIBUTING.md)

   ## License

   MIT
   ```

2. **CONTRIBUTING.md** (developer documentation):
   ```markdown
   # Contributing to xvn

   ## Development Setup

   ### Prerequisites
   - Rust 1.70+ (`rustup install stable`)
   - Node.js (for npm packaging)
   - shellcheck (`brew install shellcheck`)

   ### Build from source
   ```bash
   git clone https://github.com/your-username/xvn.git
   cd xvn
   cargo build
   ./target/debug/xvn --version
   ```

   ### Running tests
   ```bash
   cargo test                    # Unit + integration tests
   ./scripts/coverage.sh         # Coverage report
   cargo bench                   # Performance benchmarks
   shellcheck shell/xvn.sh       # Shell script validation
   ```

   ## Project Structure

   ```
   xvn/
   ├── src/
   │   ├── main.rs              # CLI entry point
   │   ├── config/              # Configuration loading
   │   ├── plugins/             # Plugin system
   │   ├── version_file/        # Version file discovery
   │   ├── activation/          # Activation orchestration
   │   └── shell/               # Shell integration (FD:3)
   ├── shell/
   │   └── xvn.sh               # Shell hook script
   ├── tests/                   # Integration tests
   └── benches/                 # Performance benchmarks
   ```

   ## Writing a Plugin

   Implement the `VersionManagerPlugin` trait:

   ```rust
   pub trait VersionManagerPlugin: Send + Sync {
       fn name(&self) -> &str;
       fn is_available(&self) -> Result<bool>;
       fn has_version(&self, version: &str) -> Result<bool>;
       fn activate_command(&self, version: &str) -> Result<String>;
       fn install_command(&self, version: &str) -> Result<String>;
   }
   ```

   See `src/plugins/nvm.rs` for a complete example.

   ## Code Style

   - Run `cargo fmt` before committing
   - Run `cargo clippy` and fix all warnings
   - Follow Rust naming conventions
   - Add tests for new features

   ## Performance Guidelines

   - Keep activation time <100ms
   - Avoid allocations in hot paths
   - Use `cargo bench` to verify performance

   ## Commit Guidelines

   Use conventional commits:
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation
   - `test:` Tests
   - `perf:` Performance improvement

   ## Pull Request Process

   1. Fork and create a feature branch
   2. Write tests for your changes
   3. Ensure all tests pass: `cargo test`
   4. Run `cargo fmt` and `cargo clippy`
   5. Submit PR with clear description

   ## Testing Philosophy

   - Unit tests: Test functions in isolation
   - Integration tests: Test component interactions
   - Shell tests: Test real bash/zsh behavior
   - Benchmarks: Verify performance targets

   ## Need Help?

   - Open an issue for bugs or questions
   - Check existing issues for known problems
   - Read the [Architecture doc](./docs/ARCHITECTURE.md)
   ```

3. **API.md** (API reference):
   ```markdown
   # xvn API Reference

   ## Plugin Interface

   ### VersionManagerPlugin Trait

   All version manager plugins must implement this trait:

   ```rust
   pub trait VersionManagerPlugin: Send + Sync {
       /// Unique plugin name (e.g., "nvm", "fnm")
       fn name(&self) -> &str;

       /// Check if this version manager is installed
       fn is_available(&self) -> Result<bool>;

       /// Check if a specific version is installed
       fn has_version(&self, version: &str) -> Result<bool>;

       /// Generate shell command to activate version
       fn activate_command(&self, version: &str) -> Result<String>;

       /// Generate shell command to install version
       fn install_command(&self, version: &str) -> Result<String>;
   }
   ```

   ## Configuration Schema

   ### User Config (~/.xvnrc)

   ```yaml
   plugin_priority:
     - nvm        # Try nvm first
     - fnm        # Fallback to fnm

   auto_install: true  # Prompt to install missing versions
   ```

   ### Project Config (.xvn.yaml)

   Same schema as user config. Project config overrides user config.

   ## CLI Commands

   ### xvn setup

   Install shell hooks.

   Options:
   - `--shell <bash|zsh>`: Specify shell (auto-detected by default)

   ### xvn activate <path>

   Manually activate Node version for directory.

   Returns: Shell commands on file descriptor #3.

   ### xvn status

   Show current configuration and plugin availability.

   ## File Descriptor #3 Protocol

   xvn writes shell commands to FD:3 for the parent shell to evaluate:

   ```bash
   export PATH="/path/to/node/bin:$PATH"
   export XVN_ACTIVE_VERSION="18.20.0"
   ```

   This allows a child process to modify the parent shell environment.
   ```

4. **Inline code documentation (rustdoc)**:

   Add comprehensive rustdoc comments to all public APIs:

   ```rust
   // src/plugins/trait_def.rs
   /// Trait for version manager plugins.
   ///
   /// Each plugin represents a Node.js version manager (nvm, fnm, n, etc.)
   /// and provides methods to check availability, query installed versions,
   /// and generate activation commands.
   ///
   /// # Example
   ///
   /// ```rust
   /// use xvn::plugins::{VersionManagerPlugin, NvmPlugin};
   ///
   /// let plugin = NvmPlugin::new();
   /// if plugin.is_available()? {
   ///     if plugin.has_version("18.20.0")? {
   ///         let cmd = plugin.activate_command("18.20.0")?;
   ///         println!("Run: {}", cmd);
   ///     }
   /// }
   /// ```
   pub trait VersionManagerPlugin: Send + Sync {
       /// Returns the unique name of this plugin (e.g., "nvm", "fnm").
       fn name(&self) -> &str;

       // ... etc for all methods
   }
   ```

   Add module-level docs:
   ```rust
   // src/lib.rs
   //! # xvn - Extreme Version Switcher for Node.js
   //!
   //! xvn automatically switches Node.js versions when you `cd` into
   //! a directory with a `.nvmrc` or `.node-version` file.
   //!
   //! ## Quick Example
   //!
   //! ```bash
   //! xvn setup      # Install shell hooks
   //! cd my-project  # Automatically activates Node version
   //! ```
   ```

5. **Generate rustdoc**:
   ```bash
   cargo doc --no-deps --open
   ```

**Code Structure:**
- `README.md` - User-facing documentation
- `CONTRIBUTING.md` - Developer guide
- `API.md` - API reference and schema
- Inline rustdoc comments in all `src/**/*.rs` files

**Key Considerations:**
- README should be scannable (use headers, lists, tables)
- Include troubleshooting section (common issues)
- API docs should have examples for every public function
- Keep CONTRIBUTING.md up-to-date with actual project structure
- Add badges (build status, coverage, version) to README

**Testing:**
```bash
# Validate documentation builds:
cargo doc --no-deps

# Check for broken links (if using mdbook):
mdbook test

# Proofread with a spell checker
```

**Dependencies:**
- Requires: M5.4 (performance numbers for README)

**Enables:**
- M5.6 (documentation review as part of quality check)

---

### Task M5.6: Code quality

**Objective:** Ensure code quality with linting, formatting, and security audits.

**Implementation Steps:**

1. **Run clippy (Rust linter)**:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

   Common issues to fix:
   - Unused variables: Prefix with `_` or remove
   - Unnecessary clones: Use references instead
   - Complex boolean expressions: Extract to named variables
   - Missing error handling: Add `.context()` or `?`
   - Inefficient string operations: Use `&str` instead of `String`

2. **Format with rustfmt**:
   ```bash
   cargo fmt --all
   ```

   Add to `rustfmt.toml`:
   ```toml
   edition = "2021"
   max_width = 100
   use_small_heuristics = "Max"
   imports_granularity = "Crate"
   ```

3. **Address all warnings**:
   ```bash
   cargo build --all-targets 2>&1 | grep warning
   ```

   Common warnings:
   - Unused imports
   - Unused variables
   - Dead code (private functions never called)
   - Non-idiomatic code patterns

   Fix or explicitly allow with `#[allow(dead_code)]` if intentional.

4. **Security audit (cargo-audit)**:
   ```bash
   cargo install cargo-audit
   cargo audit
   ```

   Review findings:
   - Update dependencies with known vulnerabilities
   - Check for yanked crates
   - Review RUSTSEC advisories

   Add to CI pipeline:
   ```yaml
   # .github/workflows/security.yml
   - name: Security audit
     run: cargo audit
   ```

5. **Dependency review**:
   ```bash
   cargo tree
   ```

   Check:
   - No duplicate versions of same crate
   - All dependencies are necessary
   - License compatibility (all MIT/Apache-2.0 compatible)

6. **Create pre-commit hook** (`.git/hooks/pre-commit`):
   ```bash
   #!/bin/bash
   set -e

   echo "Running pre-commit checks..."

   # Format check
   cargo fmt --all -- --check

   # Clippy
   cargo clippy --all-targets --all-features -- -D warnings

   # Tests
   cargo test --all

   echo "✅ Pre-commit checks passed"
   ```

7. **CI configuration** (`.github/workflows/ci.yml`):
   ```yaml
   name: CI

   on: [push, pull_request]

   jobs:
     test:
       strategy:
         matrix:
           os: [ubuntu-latest, macos-latest]
           rust: [stable, beta]
       runs-on: ${{ matrix.os }}
       steps:
         - uses: actions/checkout@v3
         - uses: actions-rs/toolchain@v1
           with:
             toolchain: ${{ matrix.rust }}

         - name: Format check
           run: cargo fmt -- --check

         - name: Clippy
           run: cargo clippy -- -D warnings

         - name: Tests
           run: cargo test --all

         - name: Security audit
           run: |
             cargo install cargo-audit
             cargo audit
   ```

8. **Coverage tracking**:
   ```bash
   # Generate coverage
   cargo tarpaulin --out Lcov --output-dir coverage

   # Upload to Coveralls (in CI):
   # - uses: coverallsapp/github-action@v2
   #   with:
   #     github-token: ${{ secrets.GITHUB_TOKEN }}
   #     path-to-lcov: ./coverage/lcov.info
   ```

**Code Structure:**
- `rustfmt.toml` - Formatting configuration
- `.git/hooks/pre-commit` - Pre-commit quality checks
- `.github/workflows/ci.yml` - CI pipeline
- `scripts/quality-check.sh` - Run all quality checks locally

**Key Considerations:**
- Fix all clippy warnings (use `-D warnings` to fail on warnings)
- Format code consistently (rustfmt is non-negotiable)
- Keep dependencies minimal and audited
- Set up CI to enforce quality checks
- Make quality checks fast (<5min) to encourage frequent running

**Testing:**
```bash
# Run all quality checks:
cargo fmt --check
cargo clippy -- -D warnings
cargo audit
cargo test --all

# Or use script:
./scripts/quality-check.sh
```

**Dependencies:**
- Requires: M5.1, M5.2, M5.3 (all tests must pass)
- Requires: M5.5 (documentation reviewed for quality)

**Enables:**
- M6 (release preparation requires passing quality checks)

---

## Integration Points

### How Tasks Work Together

1. **M5.1 → M5.2**: Unit tests establish testing patterns and mock infrastructure that integration tests build upon.

2. **M5.2 → M5.3**: Integration tests demonstrate multi-component testing that shell tests extend to real environments.

3. **M5.4 (parallel)**: Benchmarks can run independently but inform documentation (M5.5) about actual performance.

4. **M5.5 depends on M5.4**: README needs performance numbers from benchmarks.

5. **M5.6 depends on M5.1-5.3**: Code quality checks require all tests to pass first.

6. **All tasks enable M6**: Milestone 6 (Release Preparation) requires complete testing, documentation, and quality assurance.

## Testing Strategy

### Overall Approach

1. **Fast Feedback Loop**: Unit tests run in <5s for quick iteration
2. **Comprehensive Coverage**: Integration tests catch multi-component issues
3. **Real-World Validation**: Shell tests verify actual bash/zsh behavior
4. **Performance Monitoring**: Benchmarks prevent regressions

### Coverage Goals

- **Unit tests**: >85% line coverage (measured by tarpaulin)
- **Integration tests**: All critical user flows (activation, fallback, config)
- **Shell tests**: Both bash and zsh on Linux and macOS
- **Benchmarks**: All performance-critical paths

### CI Pipeline

```yaml
matrix:
  os: [ubuntu-latest, macos-latest]
  rust: [stable, beta]

steps:
  1. cargo fmt --check
  2. cargo clippy -- -D warnings
  3. cargo test --all
  4. cargo tarpaulin (coverage)
  5. cargo audit (security)
  6. cargo bench (performance check)
```

## Success Criteria

**Milestone 5 is complete when:**

✅ **Testing**
- [ ] All tests passing on CI (Linux + macOS)
- [ ] Coverage >85% (verified by tarpaulin)
- [ ] Shell tests pass for both bash and zsh
- [ ] Zero flaky tests (run 10x without failures)

✅ **Performance**
- [ ] Benchmarks meet targets:
  - File discovery: <5ms
  - Plugin matching: <20ms
  - Total activation: <85ms (P95)
- [ ] No performance regressions from baseline

✅ **Documentation**
- [ ] README.md complete with examples
- [ ] CONTRIBUTING.md with development guide
- [ ] API.md with full schema and interface docs
- [ ] All public APIs have rustdoc comments
- [ ] `cargo doc` builds without warnings

✅ **Code Quality**
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] No security vulnerabilities (`cargo audit`)
- [ ] No unused dependencies
- [ ] Pre-commit hooks installed

✅ **Deliverables**
- [ ] Coverage report published (HTML + Lcov)
- [ ] Benchmark results published (criterion HTML)
- [ ] CI pipeline configured and green
- [ ] All documentation reviewed and proofread

**Ready for Milestone 6**: Release Preparation 🚀
