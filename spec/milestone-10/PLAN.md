# Milestone 10: Version-Independent Installation (v1.2.0)

**Timeline:** 1-2 weeks
**Status:** Planning
**Priority:** CRITICAL - Fixes bootstrap problem

---

## Problem Statement

**Critical Issue**: When xvn is installed via `npm install -g @olvrcc/xvn`, it gets installed to the **current Node.js version's global packages directory**. When xvn switches to a different Node.js version, the xvn binary is no longer in PATH because it's not installed in that version's global packages.

**Example Scenario:**
```bash
# Using Node.js 20.x
$ npm install -g @olvrcc/xvn
$ which xvn
/Users/cam/.nvm/versions/node/v20.11.0/bin/xvn  # Installed in v20 globals

# xvn switches to Node.js 18.x when entering a project
$ cd my-project  # Has .nvmrc with "18.20.0"
$ which xvn
xvn not found  # ❌ xvn is not in Node.js 18's globals!

# Now stuck - can't switch back or to other versions
```

**Impact:**
- xvn becomes unusable after switching versions once
- Users must manually install xvn globally for EVERY Node.js version
- Defeats the purpose of automatic version switching
- Creates a poor user experience

## Goals

1. ✅ Install xvn binary in a version-independent location
2. ✅ Ensure xvn binary is always available in PATH regardless of active Node.js version
3. ✅ Maintain compatibility with npm installation workflow
4. ✅ Support both npm and Homebrew installation methods
5. ✅ Provide seamless upgrade path for existing users

## Solution Options Analysis

### Option 1: Version-Independent Binary Installation (RECOMMENDED)

Install the Rust binary to a system location that's always in PATH, independent of Node.js version.

**Approach:**
```
~/.xvn/
  bin/
    xvn           # Rust binary (version-independent)
  current -> versions/v1.2.0/
  versions/
    v1.2.0/
      lib/        # Shell scripts and helpers
```

**Implementation:**
- npm postinstall script copies binary to `~/.xvn/bin/xvn`
- Shell integration adds `~/.xvn/bin` to PATH (prepended)
- Binary location is stable across Node.js version switches
- npm can still manage updates (downloads new binary, replaces in ~/.xvn/bin)

**Pros:**
- ✅ Solves the bootstrap problem completely
- ✅ Works with npm installation (no additional tools needed)
- ✅ Binary always available regardless of Node.js version
- ✅ Consistent with how other version managers work (nvm, rbenv, pyenv)
- ✅ Can still distribute via npm

**Cons:**
- ⚠️ Requires modifying shell integration to add `~/.xvn/bin` to PATH
- ⚠️ Slightly more complex postinstall script

### Option 2: Homebrew-Only Installation

Distribute xvn exclusively via Homebrew (macOS/Linux).

**Pros:**
- ✅ Homebrew installs to system location (`/usr/local/bin` or `/opt/homebrew/bin`)
- ✅ Always in PATH
- ✅ Standard package management

**Cons:**
- ❌ Not available on Windows
- ❌ Requires users to have Homebrew installed
- ❌ More friction for Node.js developers (npm is guaranteed to be installed)
- ❌ Harder to discover (npm is the natural distribution channel)
- ❌ Still need npm for Windows support

### Option 3: Shim Script Approach

Create a lightweight shim script in each Node.js version that calls the actual binary.

**Approach:**
- Install actual binary to `~/.xvn/bin/xvn`
- npm postinstall creates shim in current Node.js version's bin
- Shim calls `~/.xvn/bin/xvn "$@"`

**Pros:**
- ✅ Can work with npm installation

**Cons:**
- ❌ Requires installing shim in EVERY Node.js version
- ❌ Users still need to run npm install in each version
- ❌ Doesn't fully solve the problem

### Option 4: PATH Manipulation in Shell Hook

Dynamically add Node.js version's global bin to PATH in shell hook.

**Cons:**
- ❌ Doesn't solve the problem - xvn still not installed in that version
- ❌ Can't run npm commands before xvn is in PATH
- ❌ Circular dependency issue

## Recommended Solution: Option 1

**Rationale:**
- Solves the core problem elegantly
- Maintains npm as primary distribution channel
- Works across all platforms (macOS, Linux, Windows)
- Consistent with version manager best practices
- Simple for users (one `npm install -g` command)

## Architecture

### Directory Structure

#### Unix (macOS/Linux)

```
~/.xvn/
├── bin/
│   └── xvn              # Symlink to current version's binary
├── current -> versions/v1.2.0/
├── versions/
│   ├── v1.2.0/
│   │   ├── bin/
│   │   │   └── xvn      # Actual binary
│   │   └── lib/
│   │       └── xvn.sh   # Shell integration script
│   └── v1.1.2/          # Old version (for rollback)
└── config.yaml          # User configuration

# PATH structure
~/.xvn/bin:...:$PATH     # xvn bin always first in PATH
```

#### Windows (PowerShell)

```
~/.xvn/
├── bin/
│   ├── xvn.exe          # Copy of binary (NOT symlink - see rationale below)
│   └── xvn.ps1          # PowerShell integration script
├── current -> versions/v1.2.0/  # Directory junction
├── versions/
│   ├── v1.2.0/
│   │   ├── bin/
│   │   │   └── xvn.exe  # Actual binary
│   │   └── lib/
│   │       └── xvn.ps1  # Shell integration script
│   └── v1.1.2/          # Old version (for rollback)
└── config.yaml          # User configuration

# PATH structure
~/.xvn/bin;...;$env:PATH     # xvn bin always first in PATH
```

**Windows Symlink Rationale:**

**Problem:** Windows symlinks traditionally require administrator privileges or Developer Mode to be enabled (Windows 10 Creators Update+).

**Solution (Hybrid Approach):**
- **Unix (macOS/Linux)**: Use symlinks for the binary (`~/.xvn/bin/xvn -> versions/v1.2.0/bin/xvn`)
  - Works without special permissions
  - Efficient, no duplication

- **Windows**: Copy the binary file + use directory junction
  - **Binary**: `~/.xvn/bin/xvn.exe` is a **copy** of `versions/v1.2.0/bin/xvn.exe`
  - **Current directory**: `~/.xvn/current` is a **directory junction** (works without elevation)
  - Binary is small (~5MB), copying is acceptable
  - npm postinstall replaces the copied binary on upgrade

**Why not use symlinks on Windows?**
1. Requires administrator privileges (or Developer Mode)
2. Creates friction during installation
3. May fail in corporate/restricted environments
4. Copy approach is simpler and more reliable

**Why not copy on Unix?**
1. Symlinks work without any special setup
2. More efficient (no duplication)
3. Industry standard approach (rbenv, pyenv, etc.)

**Update mechanism:**
- Unix: Update symlink to point to new version
- Windows: Copy new binary over old one in `~/.xvn/bin/`

### Installation Flow

#### New Installation (npm)

**Unix (macOS/Linux):**
```bash
$ npm install -g @olvrcc/xvn

# Postinstall script (install.js):
1. Detect platform and architecture
2. Download/extract appropriate pre-compiled binary
3. Create ~/.xvn/ directory structure
4. Install binary to ~/.xvn/versions/v1.2.0/bin/xvn
5. Create symlink ~/.xvn/bin/xvn -> versions/v1.2.0/bin/xvn
6. Prompt user to run `xvn setup`

$ xvn setup

# Setup command:
1. Detect shell (bash/zsh)
2. Add ~/.xvn/bin to PATH in shell profile
3. Install shell hook (sources xvn.sh)
4. Verify installation
```

**Windows (PowerShell):**
```powershell
PS> npm install -g @olvrcc/xvn

# Postinstall script (install.js):
1. Detect platform and architecture
2. Download/extract appropriate pre-compiled binary
3. Create ~/.xvn/ directory structure
4. Install binary to ~/.xvn/versions/v1.2.0/bin/xvn.exe
5. Copy binary to ~/.xvn/bin/xvn.exe (NOT symlink)
6. Create directory junction ~/.xvn/current -> versions/v1.2.0/
7. Prompt user to run `xvn setup`

PS> xvn setup

# Setup command:
1. Detect shell (PowerShell)
2. Add ~/.xvn/bin to PATH in PowerShell profile
3. Install shell hook (sources xvn.ps1)
4. Verify installation
```

#### Upgrade (npm)

**Unix (macOS/Linux):**
```bash
$ npm update -g @olvrcc/xvn

# Postinstall script:
1. Install new binary to ~/.xvn/versions/v1.3.0/bin/xvn
2. Update symlink ~/.xvn/bin/xvn -> versions/v1.3.0/bin/xvn
3. Keep previous version for rollback (v1.2.0)
4. Clean up old versions (keep last 2)
```

**Windows (PowerShell):**
```powershell
PS> npm update -g @olvrcc/xvn

# Postinstall script:
1. Install new binary to ~/.xvn/versions/v1.3.0/bin/xvn.exe
2. Copy new binary to ~/.xvn/bin/xvn.exe (overwrite old copy)
3. Update junction ~/.xvn/current -> versions/v1.3.0/
4. Keep previous version for rollback (v1.2.0)
5. Clean up old versions (keep last 2)
```

### Shell Integration Changes

#### Current (.zshrc / .bashrc)

```bash
# xvn shell integration
export XVN_DIR="$HOME/.xvn"
[ -s "$XVN_DIR/lib/xvn.sh" ] && . "$XVN_DIR/lib/xvn.sh"
```

#### Updated (.zshrc / .bashrc)

```bash
# xvn shell integration
export XVN_DIR="$HOME/.xvn"
export PATH="$XVN_DIR/bin:$PATH"  # Add xvn bin to PATH first
[ -s "$XVN_DIR/current/lib/xvn.sh" ] && . "$XVN_DIR/current/lib/xvn.sh"
```

### npm Postinstall Script Changes

**Current:** `install.js` downloads binary to `bin/xvn` in npm package

**Updated:** `install.js` installs binary to `~/.xvn/`

```javascript
// install.js (simplified)
const os = require('os');
const path = require('path');
const fs = require('fs');

const XVN_DIR = path.join(os.homedir(), '.xvn');
const VERSION = require('./package.json').version;
const VERSION_DIR = path.join(XVN_DIR, 'versions', `v${VERSION}`);

async function install() {
  // 1. Create directory structure
  fs.mkdirSync(path.join(VERSION_DIR, 'bin'), { recursive: true });
  fs.mkdirSync(path.join(VERSION_DIR, 'lib'), { recursive: true });
  fs.mkdirSync(path.join(XVN_DIR, 'bin'), { recursive: true });

  // 2. Download and install binary
  const binaryPath = await downloadBinary(VERSION_DIR);

  // 3. Copy shell integration scripts
  fs.copyFileSync('./shell/xvn.sh', path.join(VERSION_DIR, 'lib/xvn.sh'));

  // 4. Create/update symlinks
  const binSymlink = path.join(XVN_DIR, 'bin', 'xvn');
  if (fs.existsSync(binSymlink)) {
    fs.unlinkSync(binSymlink);
  }
  fs.symlinkSync(path.join(VERSION_DIR, 'bin', 'xvn'), binSymlink);

  const currentSymlink = path.join(XVN_DIR, 'current');
  if (fs.existsSync(currentSymlink)) {
    fs.unlinkSync(currentSymlink);
  }
  fs.symlinkSync(VERSION_DIR, currentSymlink);

  // 5. Prompt user to run setup
  console.log('✓ xvn installed to ~/.xvn/');
  console.log('');
  console.log('Please run: xvn setup');
  console.log('Or manually add to your shell profile:');
  console.log(`  export PATH="$HOME/.xvn/bin:$PATH"`);
}
```

### Setup Command Changes

**Current:** `xvn setup` adds hook to shell profile

**Updated:** `xvn setup` adds BOTH PATH and hook

```rust
// src/commands/setup.rs
pub fn run_setup() -> Result<()> {
    let shell = detect_shell()?;
    let profile_path = get_profile_path(&shell)?;

    // Check if already set up
    if is_already_setup(&profile_path)? {
        println!("xvn is already set up in {}", profile_path.display());
        return Ok(());
    }

    // Add both PATH and shell hook
    let setup_lines = r#"
# xvn shell integration
export XVN_DIR="$HOME/.xvn"
export PATH="$XVN_DIR/bin:$PATH"
[ -s "$XVN_DIR/current/lib/xvn.sh" ] && . "$XVN_DIR/current/lib/xvn.sh"
"#;

    append_to_profile(&profile_path, setup_lines)?;

    println!("✓ xvn setup complete!");
    println!("  Shell: {}", shell.name());
    println!("  Profile: {}", profile_path.display());
    println!();
    println!("Restart your shell or run:");
    println!("  source {}", profile_path.display());

    Ok(())
}
```

## Implementation Tasks

### Phase 1: Core Infrastructure (Week 1)

1. **Update install.js**
   - Implement ~/.xvn directory structure creation
   - Install binary to versioned location
   - Create symlinks (bin/xvn and current)
   - Handle upgrade scenario (detect existing installation)
   - Clean up old versions (keep last 2)

2. **Update setup command**
   - Add PATH export to shell profile
   - Update shell hook source path (use current/)
   - Add verification that ~/.xvn/bin/xvn exists
   - Handle migration from old installation

3. **Update shell integration script (xvn.sh)**
   - No changes needed (uses $XVN_DIR from environment)

4. **Add migration support**
   - Detect old-style installation (binary in npm global)
   - Offer to migrate to new structure
   - Backup old configuration

### Phase 2: Testing & Validation (Week 1)

5. **Test installation flow**
   - Fresh install on clean system
   - Upgrade from v1.1.x to v1.2.0
   - Test across different shells (bash, zsh)
   - Test version switching behavior

6. **Test binary availability**
   - Install xvn while on Node 20
   - Switch to Node 18 (should still have xvn)
   - Switch to Node 16 (should still have xvn)
   - Verify `which xvn` always points to ~/.xvn/bin/xvn

7. **Test upgrade path**
   - Simulate upgrading from v1.1.2 to v1.2.0
   - Verify old version is kept
   - Verify symlinks are updated correctly

### Phase 3: Documentation (Week 2)

8. **Update README.md**
   - Document new installation location
   - Explain PATH modification
   - Add troubleshooting section

9. **Update installation guide**
   - Fresh installation steps
   - Upgrade instructions
   - Migration from old version
   - Uninstallation procedure

10. **Create migration guide**
    - For users upgrading from v1.1.x
    - Manual migration steps if needed
    - Rollback procedure

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_version_dir_creation() {
    // Test that install creates correct directory structure
}

#[test]
fn test_symlink_creation() {
    // Test symlink creation and updates
}

#[test]
fn test_version_cleanup() {
    // Test old version cleanup (keep last 2)
}
```

### Integration Tests

```bash
# Test: Fresh installation
npm install -g @olvrcc/xvn
test -f ~/.xvn/bin/xvn
test -L ~/.xvn/current
xvn --version

# Test: Version switching
nvm use 20
xvn activate ~/project-a  # Node 18
which xvn  # Should be ~/.xvn/bin/xvn
xvn --version  # Should work

cd ~/project-b  # Node 16
which xvn  # Should still be ~/.xvn/bin/xvn
xvn --version  # Should work

# Test: Upgrade
npm install -g @olvrcc/xvn@1.3.0
test -f ~/.xvn/versions/v1.3.0/bin/xvn
test -f ~/.xvn/versions/v1.2.0/bin/xvn  # Old version kept
```

## Rollback Strategy

If issues are discovered:

1. **Keep old installation method**
   - Detect if user has old-style installation
   - Continue supporting it (don't force migration)

2. **Version rollback**
   - Users can manually symlink to previous version:
     ```bash
     ln -sf ~/.xvn/versions/v1.1.2/bin/xvn ~/.xvn/bin/xvn
     ```

3. **npm rollback**
   ```bash
   npm install -g @olvrcc/xvn@1.1.2
   ```

## Success Metrics

- ✅ xvn remains available after switching Node.js versions
- ✅ No manual installation needed in each Node.js version
- ✅ Upgrade path works smoothly for existing users
- ✅ Installation time < 10 seconds
- ✅ Zero user-reported "xvn not found" issues after version switches

## Future Enhancements (Post-v1.2.0)

- **Homebrew distribution** (Milestone 9)
  - Install to `/usr/local/bin` or `/opt/homebrew/bin`
  - Even simpler PATH management

- **Self-update command**
  - `xvn update` to check for new versions
  - Update without going through npm

- **Version rollback command**
  - `xvn rollback` to switch to previous version
  - Useful if new version has issues

## Dependencies

- **None** - This milestone is independent
- Can be implemented immediately (v1.2.0)
- Should be completed BEFORE Homebrew distribution (Milestone 9)

## Breaking Changes

⚠️ **PATH Modification Required**

Users will need to:
1. Run `xvn setup` again (or manually update shell profile)
2. Add `export PATH="$HOME/.xvn/bin:$PATH"` to shell profile
3. Restart shell

**Migration Strategy:**
- Detect old installation in postinstall
- Show migration instructions
- Offer automatic migration if old binary detected

## Related Issues

- Fixes: "xvn not found after version switch" (#1)
- Enables: Homebrew distribution (Milestone 9)
- Improves: User experience and reliability
