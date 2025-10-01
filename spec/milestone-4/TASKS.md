# Milestone 4: Version Activation & Auto-Install - Tasks

**Timeline:** Weeks 7-8
**Version:** v0.4.0-v0.5.0
**Status:** Not Started

---

## Tasks

### M4.1: Implement `xvn activate` command
- [x] Load configuration
- [x] Parse version file
- [x] Load plugin registry
- [x] Find plugin with version installed
- [x] Generate activation commands
- [x] Write commands to FD:3
- [x] Write user messages to stdout
- [x] Handle errors gracefully

### M4.2: Implement auto-install logic
- [x] Check auto_install config (prompt/always/never)
- [x] Prompt user for confirmation
- [x] Read stdin for user response
- [x] Generate install + activate commands
- [x] Handle install declined
- [x] Show version mismatch message

### M4.3: Implement version mismatch detection
- [x] Get current Node.js version (node --version)
- [x] Compare to required version
- [x] Format mismatch message

### M4.4: Verify and test idempotency implementation
- [ ] Review existing XVN_ACTIVE_FILE logic in shell/xvn.sh
- [ ] Verify shell hook skips activation for same file (lines 54-57)
- [ ] Verify XVN_ACTIVE_FILE cleared when leaving directory (lines 96-99)
- [ ] Add integration test for idempotency behavior

### M4.5: Unit tests for activation
- [x] Activation performance test (<100ms target)
- [x] Version file parsing tests
- [x] Plugin priority tests (first match wins)
- [x] Auto-install prompt tests (mock stdin)
- [x] Config override precedence tests
- [x] Error message tests

### M4.6: Integration tests
- [ ] End-to-end activation with mock plugin
- [ ] Auto-install flow with mock stdin
- [ ] Multiple version files (nested directories)

### M4.7: Implement structured error handling
- [x] Create src/activation/errors.rs with ActivationError enum
- [x] Define error types (NoPluginsAvailable, InvalidVersionFile, ConfigError)
- [x] Add actionable hints for each error type
- [x] Update orchestrator to use structured errors
- [x] Update CLI to display error hints

### M4.8: Code quality checks
- [ ] Run cargo clippy and fix all warnings
- [ ] Run cargo fmt
- [ ] Verify all tests pass
- [ ] Check test coverage >85%

---

## Success Criteria

- ✅ Activates installed versions correctly
- ✅ Prompts for missing versions
- ✅ Respects user choice (Y/n)
- ✅ Shows mismatch when declining install
- ✅ Handles all error cases gracefully

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
