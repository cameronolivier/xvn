# Milestone 3: Shell Integration

**Timeline:** Weeks 5-6  
**Status:** Planning  
**Version:** v0.3.0

---

## Plan

### Goal

Implement bash/zsh shell hooks, fd:3 protocol, and setup command for seamless shell integration.

### Deliverables

- [ ] Shell hook scripts (bash, zsh)
- [ ] File descriptor #3 protocol implementation
- [ ] chpwd_functions integration
- [ ] Shell profile modification logic
- [ ] `xvn setup` command implementation
- [ ] Idempotency checks

### Testing

- Shell script syntax validation (shellcheck)
- File descriptor protocol tests (mock fd:3)
- Setup idempotency tests
- Profile detection across different systems

### Success Criteria

- `xvn setup` completes without errors
- Shell profile correctly modified
- Hook executes on `cd` command
- Commands from fd:3 evaluated in parent shell

---

## Architecture

### Shell Hook (xvn.sh)

- Bash/zsh compatible script
- Directory change detection via chpwd_functions
- Version file search (walks up directory tree)
- Idempotency tracking with XVN_ACTIVE_FILE
- FD:3 protocol for command execution

### FD:3 Protocol Implementation

**Rust side:**
```rust
let mut fd3 = CommandWriter::new()?;
fd3.write_command("nvm use 18.20.0")?;
```

**Shell side:**
```bash
commands=$(xvn activate path 3>&1 1>&2)
eval "$commands"
```

### Setup Module

- Detect shell (bash/zsh)
- Find profile files (~/.bashrc, ~/.zshrc, ~/.bash_profile)
- Install xvn.sh hook (copy to ~/.xvn/bin/)
- Modify profile (idempotent, check existing)
- Create default ~/.xvnrc

See [ARCHITECTURE.md](../docs/ARCHITECTURE.md#shell-integration-architecture) for detailed shell integration design.

---

