# Milestone 11: Windows & PowerShell Support

**Timeline:** 2-3 weeks
**Status:** Planning
**Version:** v1.3.0
**Phase:** 2 - Enhanced Features

---

## Goal

Extend xvn to support Windows and PowerShell, enabling automatic Node.js version switching on Windows platforms with the same seamless experience as Unix systems.

---

## Deliverables

- [ ] Windows binary compilation (x64, arm64)
- [ ] PowerShell hook script (xvn.ps1)
- [ ] PowerShell profile modification logic
- [ ] Cross-platform path handling
- [ ] Windows-specific shell integration tests
- [ ] Updated documentation for Windows users

---

## Architecture Overview

### Cross-Platform Design Principles

1. **Shared Core Logic**
   - Rust core remains platform-agnostic
   - Plugin system works identically on Windows
   - Configuration system unchanged

2. **Platform-Specific Components**
   - Shell hooks: `xvn.sh` (Unix) vs `xvn.ps1` (Windows)
   - Profile paths: `~/.bashrc` vs `$PROFILE`
   - Path separators: `/` vs `\`
   - Line endings: LF vs CRLF

3. **Compilation Targets**
   - `x86_64-pc-windows-msvc` (Windows x64)
   - `aarch64-pc-windows-msvc` (Windows ARM64)

---

## PowerShell Hook Script (xvn.ps1)

### Requirements

- PowerShell 5.1+ (Windows 10+) and PowerShell Core 7+ support
- Directory change detection via prompt function
- Version file discovery (walk directory tree)
- Idempotency checks (avoid re-activation)
- Command execution from xvn binary output

### Key Differences from Bash/Zsh

| Feature | Unix (bash/zsh) | Windows (PowerShell) |
|---------|----------------|---------------------|
| **Hook Mechanism** | `chpwd_functions`, `cd` wrapper | Prompt function override |
| **Execution Protocol** | File descriptor #3 | Standard output parsing |
| **Path Separator** | `/` | `\` (but PowerShell handles both) |
| **Environment Export** | `export VAR=value` | `$env:VAR = "value"` |
| **Version Tracking** | `$XVN_ACTIVE_KEY` | `$global:XVN_ACTIVE_KEY` |

### PowerShell Prompt Integration

PowerShell doesn't have `chpwd` hooks, so we override the `prompt` function:

```powershell
# Store original prompt
$global:__xvn_original_prompt = $function:prompt

# Override prompt to trigger xvn on directory change
function global:prompt {
    __xvn_chpwd
    & $global:__xvn_original_prompt
}
```

---

## File Descriptor Protocol on Windows

### Challenge

Windows doesn't support file descriptor #3 in the same way as Unix.

### Solution: JSON Output Protocol

Instead of FD:3, use a special output format that can be parsed from stdout:

**Rust Output:**
```json
__XVN_COMMANDS_START__
{"commands": ["$env:PATH = 'C:\\nvm\\v18.0.0;' + $env:PATH", "$env:NODE_VERSION = '18.0.0'"]}
__XVN_COMMANDS_END__
```

**PowerShell Parsing:**
```powershell
$output = xvn activate $PWD
if ($output -match '__XVN_COMMANDS_START__(.*)__XVN_COMMANDS_END__') {
    $json = $matches[1] | ConvertFrom-Json
    foreach ($cmd in $json.commands) {
        Invoke-Expression $cmd
    }
}
```

### Alternative: FD:3 Emulation (Future)

For PowerShell 7+, we could use streams:
```powershell
$commands = xvn activate $PWD 3>&1 2>&1 | Where-Object { $_.GetType().Name -eq 'String' }
```

---

## Windows Version Manager Support

### Target Version Managers

1. **nvm-windows** (most common)
   - Install location: `%APPDATA%\nvm\`
   - Versions: `%APPDATA%\nvm\v{version}\`
   - Activation: `nvm use {version}`

2. **fnm** (Windows binary)
   - Install location: `%LOCALAPPDATA%\fnm\`
   - Activation: `fnm use {version}`

3. **volta** (Windows support)
   - Install location: `%LOCALAPPDATA%\Volta\`
   - Activation: Automatic via shims

### Plugin Modifications Required

- Update `nvm` plugin to detect `nvm-windows`
- Handle Windows path conventions
- Detect version manager executables on PATH

---

## Cross-Platform Path Handling

### Rust Standard Library Support

Use `std::path::PathBuf` and `std::env` which handle platform differences:

```rust
use std::path::PathBuf;
use std::env;

// Get home directory (cross-platform)
let home = dirs::home_dir()?;

// Build paths (uses correct separator)
let config = home.join(".xvnrc");  // Unix: ~/.xvnrc, Windows: C:\Users\..\.xvnrc

// Expand environment variables
let appdata = env::var("APPDATA")?;  // Windows-specific
```

### Version File Search

Works identically on Windows - walk up directory tree looking for `.nvmrc`, `package.json`, etc.

---

## PowerShell Profile Modification

### Profile Locations

PowerShell has multiple profile files:

```powershell
$PROFILE                           # Current user, current host
$PROFILE.CurrentUserCurrentHost    # Same as above
$PROFILE.CurrentUserAllHosts       # Current user, all hosts
$PROFILE.AllUsersCurrentHost       # All users, current host
$PROFILE.AllUsersAllHosts          # All users, all hosts
```

**Target:** `$PROFILE.CurrentUserCurrentHost` (most common)

### Setup Command for Windows

```powershell
xvn init  # or xvn setup on Windows
```

**Actions:**
1. Create `~/.xvn/bin/xvn.ps1` if missing
2. Check if `$PROFILE` exists, create if not
3. Add sourcing line to `$PROFILE`:
   ```powershell
   # xvn - Automatic Node.js version switching
   . "$env:USERPROFILE\.xvn\bin\xvn.ps1"
   ```
4. Create default `~/.xvnrc` if missing

---

## Windows Binary Distribution

### Compilation

**GitHub Actions:**
```yaml
- name: Build Windows x64
  run: cargo build --release --target x86_64-pc-windows-msvc

- name: Build Windows ARM64
  run: cargo build --release --target aarch64-pc-windows-msvc
```

### npm Package Structure

```
native/
├── x86_64-pc-windows-msvc/
│   └── xvn.exe
├── aarch64-pc-windows-msvc/
│   └── xvn.exe
├── x86_64-apple-darwin/
│   └── xvn
└── ...
```

### install.js Detection

```javascript
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'win32' && arch === 'x64') {
    return 'x86_64-pc-windows-msvc';
  } else if (platform === 'win32' && arch === 'arm64') {
    return 'aarch64-pc-windows-msvc';
  }
  // ... other platforms
}
```

---

## Testing Strategy

### Unit Tests (Rust)

- Path handling on Windows
- Config file parsing with Windows line endings
- Environment variable expansion

### Integration Tests (PowerShell)

- `xvn.ps1` script validation (PSScriptAnalyzer)
- Profile modification idempotency
- Directory change detection
- Version activation

### Manual Testing

- Windows 10 (x64)
- Windows 11 (ARM64, if available)
- PowerShell 5.1 and PowerShell 7
- Windows Terminal
- VS Code integrated terminal

---

## Known Limitations

### Phase 1 Constraints

1. **PowerShell Only** - No cmd.exe support (use PowerShell)
2. **No Windows Subsystem for Linux (WSL)** - Use Unix scripts in WSL
3. **Requires PowerShell 5.1+** - Modern Windows 10+ only

### Future Enhancements

- cmd.exe support (low priority)
- Windows Terminal integration
- PowerShell Gallery distribution

---

## Dependencies

### Prerequisites

- Rust toolchain with Windows MSVC targets
- GitHub Actions Windows runners
- Access to Windows for testing

### Milestone Dependencies

- **Requires:** Milestone 1-8 complete (core functionality)
- **Enables:** Full cross-platform support for xvn

---

## Success Criteria

- ✅ Windows binary compiles for x64 and ARM64
- ✅ `xvn init` creates PowerShell profile modifications
- ✅ PowerShell hook activates on directory change
- ✅ Version switching works with nvm-windows and fnm
- ✅ All tests pass on Windows platform
- ✅ Documentation includes Windows setup instructions
- ✅ npm package installs correctly on Windows

---

## References

- [PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/)
- [nvm-windows](https://github.com/coreybutler/nvm-windows)
- [fnm Windows Support](https://github.com/Schniz/fnm)
- [Rust Windows MSVC Targets](https://doc.rust-lang.org/rustc/platform-support.html)
