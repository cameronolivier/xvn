# Milestone 11: Windows & PowerShell Support - Tasks

**Timeline:** 2-3 weeks
**Version:** v1.3.0
**Status:** Not Started

---

## Tasks

### M11.1: Windows Binary Compilation Setup

- [x] Add Windows MSVC targets to Rust toolchain
  - [x] `x86_64-pc-windows-msvc`
  - [x] `aarch64-pc-windows-msvc`
- [x] Update `.cargo/config.toml` for Windows targets
- [ ] Test local Windows builds (if Windows dev environment available)
- [x] Add Windows to CI/CD matrix (GitHub Actions)
- [ ] Verify binary size and dependencies (requires CI run)

**Acceptance Criteria:**
- Windows binaries compile successfully
- Binaries are <6MB (compressed)
- CI builds Windows artifacts on every push

---

### M11.2: Create PowerShell Hook Script (xvn.ps1)

- [x] Create `shell/xvn.ps1` file
- [x] Implement version file finder function
  - [x] Walk up directory tree
  - [x] Support `.nvmrc`, `.node-version`, `package.json`
  - [x] Handle Windows path separators
- [x] Implement activation function
  - [x] Call `xvn activate` binary
  - [x] Parse command output (JSON protocol)
  - [x] Execute commands via `Invoke-Expression`
- [x] Implement directory change detection
  - [x] Override `prompt` function
  - [x] Track last directory with global variable
  - [x] Trigger activation on directory change
- [x] Implement idempotency check
  - [x] Track active version file + content hash
  - [x] Skip re-activation if unchanged
  - [x] Use `Get-FileHash` for content hashing
- [x] Add debug logging (controlled by `$env:XVN_DEBUG`)
- [x] Handle errors gracefully (don't break shell)

**Acceptance Criteria:**
- PowerShell script runs without errors
- Directory changes trigger version activation
- Idempotency prevents unnecessary re-activation
- Debug output available when `$env:XVN_DEBUG=1`

---

### M11.3: Implement JSON Command Protocol

- [x] Create `OutputProtocol` enum in Rust
  - [x] `Fd3` variant (Unix)
  - [x] `Json` variant (Windows)
- [x] Implement JSON output writer
  - [x] Serialize commands to JSON
  - [x] Wrap in `__XVN_COMMANDS_START__` / `__XVN_COMMANDS_END__` markers
  - [x] Write to stdout
- [x] Detect platform and choose protocol
  - [x] Unix: Use FD:3
  - [x] Windows: Use JSON
- [ ] Update `CommandWriter` to support both protocols (deferred - needs Windows testing)
- [ ] Add protocol selection to config (override for testing)

**Acceptance Criteria:**
- JSON output format matches spec
- Commands are properly escaped for PowerShell
- Protocol auto-detects based on platform
- Both protocols tested in CI

---

### M11.4: Update Plugin System for Windows

- [ ] Modify `nvm` plugin to detect `nvm-windows`
  - [ ] Check for `nvm` command on Windows
  - [ ] Detect version install location (`%APPDATA%\nvm\`)
  - [ ] Handle Windows-style paths in commands
- [ ] Update `fnm` plugin for Windows
  - [ ] Detect `fnm.exe` on PATH
  - [ ] Handle Windows install locations
- [ ] Add Windows-specific path handling utilities
  - [ ] Convert `/` to `\` where needed
  - [ ] Expand environment variables (`%APPDATA%`, etc.)
- [ ] Test version manager detection on Windows

**Acceptance Criteria:**
- `nvm-windows` detected and usable
- `fnm` works on Windows
- Version activation commands use correct paths

---

### M11.5: Implement PowerShell Profile Modification

- [ ] Add PowerShell profile detection
  - [ ] Detect `$PROFILE` path
  - [ ] Create profile directory if missing
  - [ ] Detect PowerShell version (5.1 vs 7+)
- [ ] Update `setup` command for Windows
  - [ ] Copy `xvn.ps1` to `~/.xvn/bin/`
  - [ ] Add source line to `$PROFILE`
  - [ ] Check idempotency (don't duplicate)
  - [ ] Create default `~/.xvnrc`
- [ ] Add Windows-specific setup instructions
  - [ ] Restart PowerShell after setup
  - [ ] Set execution policy if needed
  - [ ] Test in different PowerShell hosts

**Acceptance Criteria:**
- `xvn init` works on Windows
- Profile modified correctly
- Idempotent setup (safe to run multiple times)
- Clear user instructions

---

### M11.6: Cross-Platform Path Handling

- [ ] Audit all path operations in codebase
  - [ ] Use `PathBuf::join()` instead of string concat
  - [ ] Use `std::env::home_dir()` or `dirs::home_dir()`
  - [ ] Avoid hardcoded `/` separators
- [ ] Add Windows-specific path utilities
  - [ ] Expand `%VAR%` environment variables
  - [ ] Handle UNC paths (`\\server\share`)
  - [ ] Normalize mixed separators
- [ ] Update config loader for Windows paths
  - [ ] Support `~` expansion
  - [ ] Support Windows environment variables
- [ ] Add path handling tests
  - [ ] Unit tests for path utilities
  - [ ] Test config loading with Windows paths

**Acceptance Criteria:**
- All path operations work on Windows
- Config files work with Windows paths
- Version files found correctly

---

### M11.7: Update npm Package for Windows

- [ ] Update `install.js` to detect Windows
  - [ ] `process.platform === 'win32'`
  - [ ] Select correct binary (`.exe` extension)
  - [ ] Copy to `native/xvn.exe`
- [ ] Update `bin/xvn` wrapper for Windows
  - [ ] Create `bin/xvn.cmd` wrapper script
  - [ ] Route to correct binary
- [ ] Add Windows binaries to package files
  - [ ] `native/x86_64-pc-windows-msvc/xvn.exe`
  - [ ] `native/aarch64-pc-windows-msvc/xvn.exe`
- [ ] Test npm install on Windows
  - [ ] Verify binary extraction
  - [ ] Verify executable permissions
  - [ ] Test global install path

**Acceptance Criteria:**
- `npm install -g @olvrcc/xvn` works on Windows
- Binary is executable from PowerShell
- Correct binary selected for architecture

---

### M11.8: Windows-Specific Testing

- [ ] Add PowerShell script validation
  - [ ] PSScriptAnalyzer for `xvn.ps1`
  - [ ] Syntax checking in CI
- [ ] Create Windows integration tests
  - [ ] Test version activation
  - [ ] Test directory change detection
  - [ ] Test idempotency
  - [ ] Test error handling
- [ ] Add Windows to CI matrix
  - [ ] GitHub Actions `windows-latest` runner
  - [ ] Test on PowerShell 5.1 and 7
  - [ ] Cross-platform test suite
- [ ] Manual testing checklist
  - [ ] Windows 10 x64
  - [ ] Windows 11 ARM64 (if available)
  - [ ] Windows Terminal
  - [ ] VS Code terminal
  - [ ] PowerShell ISE

**Acceptance Criteria:**
- All tests pass on Windows
- CI runs Windows tests automatically
- Manual testing completed on real hardware

---

### M11.9: Documentation Updates

- [ ] Update README.md
  - [ ] Add Windows installation instructions
  - [ ] Add PowerShell setup steps
  - [ ] Note Windows-specific requirements
- [ ] Create Windows troubleshooting guide
  - [ ] Execution policy issues
  - [ ] PATH configuration
  - [ ] nvm-windows vs Unix nvm differences
- [ ] Update ARCHITECTURE.md
  - [ ] Document PowerShell integration
  - [ ] Document JSON protocol
  - [ ] Document cross-platform design
- [ ] Add Windows examples to docs
  - [ ] PowerShell commands
  - [ ] Windows paths in config

**Acceptance Criteria:**
- Windows users can install and setup xvn from README
- Troubleshooting covers common issues
- Architecture docs reflect Windows support

---

### M11.10: Release & Distribution

- [ ] Update version to v1.3.0
  - [ ] `Cargo.toml`
  - [ ] `package.json`
  - [ ] `CHANGELOG.md`
- [ ] Build all platform binaries
  - [ ] Linux x64, arm64
  - [ ] macOS x64, arm64
  - [ ] Windows x64, arm64
- [ ] Test npm package on Windows
  - [ ] Clean install
  - [ ] Version activation
  - [ ] Error scenarios
- [ ] Create GitHub release
  - [ ] Include all binaries
  - [ ] Windows-specific release notes
- [ ] Publish to npm
  - [ ] `npm publish`
  - [ ] Verify Windows binary downloads
- [ ] Announce Windows support
  - [ ] Release notes
  - [ ] Social media / community

**Acceptance Criteria:**
- v1.3.0 published to npm
- Windows binaries available
- Users can install on Windows

---

## Success Criteria

- ✅ Windows binary compiles for x64 and ARM64
- ✅ `xvn init` creates PowerShell profile modifications
- ✅ PowerShell hook activates on directory change
- ✅ Version switching works with nvm-windows and fnm
- ✅ All tests pass on Windows platform
- ✅ Documentation includes Windows setup instructions
- ✅ npm package installs correctly on Windows
- ✅ Manual testing completed on Windows 10 and 11

---

## Task Dependencies

```
M11.1 (Binary Setup)
  └─> M11.7 (npm Package)
      └─> M11.10 (Release)

M11.2 (PowerShell Script)
  └─> M11.3 (JSON Protocol)
      └─> M11.5 (Profile Modification)
          └─> M11.8 (Testing)

M11.4 (Plugin Updates)
  └─> M11.6 (Path Handling)
      └─> M11.8 (Testing)

M11.8 (Testing)
  └─> M11.9 (Documentation)
      └─> M11.10 (Release)
```

---

## Estimated Timeline

- **Week 1:** M11.1-M11.3 (Binary setup, PowerShell script, JSON protocol)
- **Week 2:** M11.4-M11.7 (Plugin updates, path handling, npm package)
- **Week 3:** M11.8-M11.10 (Testing, documentation, release)

**Total:** 2-3 weeks (depending on Windows testing availability)

---

**Last Updated:** October 4, 2025
