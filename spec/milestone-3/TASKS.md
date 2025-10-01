# Milestone 3: Shell Integration - Tasks

**Timeline:** Weeks 5-6
**Version:** v0.3.0
**Status:** Not Started

---

## Tasks

### M3.1: Create xvn.sh shell hook script
- [x] Version file search function (__xvn_find_file)
- [x] Activation function (__xvn_activate)
- [x] chpwd hook function (__xvn_chpwd)
- [x] Debug function (__xvn_debug)
- [x] File descriptor #3 protocol
- [x] Bash-specific integration (wrap cd/pushd/popd)
- [x] Zsh-specific integration (chpwd_functions)

### M3.2: Implement file descriptor #3 protocol in Rust
- [x] CommandWriter struct
- [x] Detect if FD:3 is open
- [x] Write commands to FD:3
- [x] Handle FD:3 unavailable gracefully

### M3.3: Implement `xvn setup` command
- [ ] Detect shell (bash, zsh)
- [ ] Find shell profile files
- [ ] Check if already installed (idempotency)
- [ ] Copy xvn.sh to ~/.xvn/bin/
- [ ] Modify shell profile (append source line)
- [ ] Create default ~/.xvnrc if missing
- [ ] Print setup instructions

### M3.4: Shell integration tests
- [ ] Validate xvn.sh syntax (shellcheck)
- [ ] Test file descriptor protocol (mock FD:3)
- [ ] Test setup idempotency (run twice)
- [ ] Test profile detection (bash, zsh)
- [ ] End-to-end shell test (bash script)

---

## Success Criteria

- ✅ `xvn setup` completes without errors
- ✅ Shell profile correctly modified
- ✅ Hook executes on `cd` command
- ✅ Commands from FD:3 evaluated in parent shell

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
