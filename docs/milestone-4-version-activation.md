# Milestone 4: Version Activation & Auto-Install

**Timeline:** Weeks 7-8  
**Status:** Planning  
**Version:** v0.4.0-v0.5.0

---

## Plan

### Goal

Implement complete version activation flow with auto-install prompts and user confirmations.

### Deliverables

- [ ] `xvn activate` command implementation
- [ ] Version resolution (read file, parse, trim)
- [ ] Plugin matching algorithm
- [ ] Auto-install prompt UI
- [ ] User confirmation handling (stdin)
- [ ] Install command execution
- [ ] Error messages with actionable guidance

### Testing

- Version file parsing (various formats)
- Plugin priority ordering
- Auto-install prompt (mock stdin)
- Config override precedence
- All error cases

### Success Criteria

- Activates installed versions correctly
- Prompts for missing versions
- Respects user choice (Y/n)
- Shows mismatch when declining install
- Handles all error cases gracefully

---

## Architecture

### Activation Flow

1. Load config (user + project merge)
2. Read version file
3. Load plugins
4. Try each plugin in priority order
5. If no plugin has version:
   - Check auto_install config
   - Prompt user (if mode = "prompt")
   - Generate install + activate commands
6. Write commands to fd:3
7. Shell executes

### Auto-Install Modes

- **prompt** (default): Ask user before installing
- **always**: Install without prompt
- **never**: Show error, don't install

### Error Handling

- NoVersionFile: Silent (expected)
- VersionNotInstalled: Prompt or error based on config
- InstallDeclined: Show version mismatch
- NoPluginAvailable: Helpful error with install instructions

See [ARCHITECTURE.md](../ARCHITECTURE.md#component-architecture) for activation module details.

---

