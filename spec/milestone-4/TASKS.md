# Milestone 4: Version Activation & Auto-Install - Tasks

**Timeline:** Weeks 7-8
**Version:** v0.4.0-v0.5.0
**Status:** Not Started

---

## Tasks

### M4.1: Implement `xvn activate` command
- [ ] Load configuration
- [ ] Parse version file
- [ ] Load plugin registry
- [ ] Find plugin with version installed
- [ ] Generate activation commands
- [ ] Write commands to FD:3
- [ ] Write user messages to stdout
- [ ] Handle errors gracefully

### M4.2: Implement auto-install logic
- [ ] Check auto_install config (prompt/always/never)
- [ ] Prompt user for confirmation
- [ ] Read stdin for user response
- [ ] Generate install + activate commands
- [ ] Handle install declined
- [ ] Show version mismatch message

### M4.3: Implement version mismatch detection
- [ ] Get current Node.js version (node --version)
- [ ] Compare to required version
- [ ] Format mismatch message

### M4.4: Implement idempotency check
- [ ] Shell hook tracks XVN_ACTIVE_FILE
- [ ] Skip activation if same file
- [ ] Update XVN_ACTIVE_FILE after activation

### M4.5: Unit tests for activation
- [ ] Version file parsing tests
- [ ] Plugin priority tests (first match wins)
- [ ] Auto-install prompt tests (mock stdin)
- [ ] Config override precedence tests
- [ ] Error message tests

### M4.6: Integration tests
- [ ] End-to-end activation with mock plugin
- [ ] Auto-install flow with mock stdin
- [ ] Multiple version files (nested directories)

---

## Success Criteria

- ✅ Activates installed versions correctly
- ✅ Prompts for missing versions
- ✅ Respects user choice (Y/n)
- ✅ Shows mismatch when declining install
- ✅ Handles all error cases gracefully

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
