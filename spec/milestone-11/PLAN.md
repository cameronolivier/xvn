# Milestone 11: Windows & PowerShell Support - Implementation Plan

## Overview

This milestone extends xvn to support Windows and PowerShell, enabling automatic Node.js version switching on Windows platforms with the same seamless experience as Unix systems. We'll implement a PowerShell hook script, cross-platform binary compilation, a JSON-based command protocol, and Windows-specific shell integration.

**Timeline:** 2-3 weeks
**Version:** v1.3.0
**Complexity:** High (new platform, different shell paradigm, cross-platform considerations)

**Key Goals:**
- Windows binary compilation (x64, ARM64)
- PowerShell hook script with directory change detection
- JSON command protocol (alternative to FD:3)
- Cross-platform path handling
- Windows version manager support (nvm-windows, fnm)
- Complete Windows documentation

## Prerequisites

**Required Tools:**
- Rust 1.70+ with Windows MSVC targets
- Access to Windows machine for testing (VM, WSL, or native)
- PowerShell 5.1+ or PowerShell Core 7+
- GitHub Actions (for CI/CD)

**Verification:**
```bash
rustup target list | grep windows    # Check available Windows targets
rustc --version                      # Should be >= 1.70
```

**Milestone Dependencies:**
- Milestones 1-8 completed (core functionality)
- Understanding of shell integration (Milestone 3)
- Plugin system knowledge (Milestone 2)

## Platform Differences: Unix vs Windows

| Aspect | Unix (bash/zsh) | Windows (PowerShell) |
|--------|----------------|---------------------|
| **Hook Mechanism** | `chpwd_functions`, `cd` wrapper | Prompt function override |
| **Execution Protocol** | File descriptor #3 | JSON output parsing |
| **Path Separator** | `/` | `\` (PowerShell handles both) |
| **Environment Export** | `export VAR=value` | `$env:VAR = "value"` |
| **Version Tracking** | `$XVN_ACTIVE_KEY` | `$global:XVN_ACTIVE_KEY` |
| **Line Endings** | LF (`\n`) | CRLF (`\r\n`) |
| **Profile File** | `~/.bashrc`, `~/.zshrc` | `$PROFILE` |
| **Binary Extension** | None | `.exe` |

## Implementation Tasks

---

### Task M11.1: Windows Binary Compilation Setup

**Objective:** Configure Rust toolchain and CI/CD to build Windows binaries for x64 and ARM64 architectures.

**Implementation Steps:**

1. **Add Windows MSVC targets to Rust toolchain:**
   ```bash
   rustup target add x86_64-pc-windows-msvc
   rustup target add aarch64-pc-windows-msvc
   ```

2. **Update `.cargo/config.toml`** (create if doesn't exist):
   ```toml
   # .cargo/config.toml
   [target.x86_64-pc-windows-msvc]
   linker = "link.exe"

   [target.aarch64-pc-windows-msvc]
   linker = "link.exe"

   [build]
   # Optimize for size on Windows
   [profile.release]
   opt-level = "z"      # Optimize for size
   lto = true           # Link-time optimization
   codegen-units = 1    # Better optimization
   strip = true         # Strip symbols
   ```

3. **Test local Windows builds** (if Windows dev environment available):
   ```bash
   # Build for Windows x64
   cargo build --release --target x86_64-pc-windows-msvc

   # Check binary size
   ls -lh target/x86_64-pc-windows-msvc/release/xvn.exe
   ```

4. **Update GitHub Actions workflow** (`.github/workflows/release.yml`):
   ```yaml
   jobs:
     build-windows:
       strategy:
         matrix:
           include:
             - os: windows-latest
               target: x86_64-pc-windows-msvc
               artifact: xvn-x86_64-pc-windows-msvc.exe
             - os: windows-latest
               target: aarch64-pc-windows-msvc
               artifact: xvn-aarch64-pc-windows-msvc.exe

       runs-on: ${{ matrix.os }}
       steps:
         - uses: actions/checkout@v4

         - name: Install Rust
           uses: dtolnay/rust-toolchain@stable
           with:
             targets: ${{ matrix.target }}

         - name: Build Windows binary
           run: cargo build --release --target ${{ matrix.target }}

         - name: Rename binary
           run: |
             cp target/${{ matrix.target }}/release/xvn.exe ${{ matrix.artifact }}

         - name: Upload artifact
           uses: actions/upload-artifact@v3
           with:
             name: ${{ matrix.artifact }}
             path: ${{ matrix.artifact }}
   ```

5. **Verify binary size and dependencies:**
   ```bash
   # Check dependencies (on Windows)
   dumpbin /dependents xvn.exe

   # Ensure no unexpected DLL dependencies
   # Should only depend on system DLLs
   ```

**Code Structure:**
- `.cargo/config.toml` - Rust build configuration
- `.github/workflows/release.yml` - CI/CD updates

**Key Considerations:**
- Windows binaries are larger than Unix (~20-30% increase)
- MSVC linker required for Windows builds (comes with Visual Studio or Build Tools)
- ARM64 Windows builds require ARM64 runners (may need cross-compilation)
- Target binary size: <6MB compressed

**Testing:**
```bash
# Verify compilation
cargo build --release --target x86_64-pc-windows-msvc

# Check binary works (on Windows)
.\target\x86_64-pc-windows-msvc\release\xvn.exe --version
```

**Dependencies:**
- None (foundation task)

**Enables:**
- M11.7 (npm package updates)
- M11.10 (release distribution)

---

### Task M11.2: Create PowerShell Hook Script (xvn.ps1)

**Objective:** Implement PowerShell hook script that detects directory changes and triggers version activation.

**Implementation Steps:**

1. **Create `shell/xvn.ps1` file:**
   ```powershell
   # xvn.ps1 - PowerShell integration for xvn
   # Automatically switches Node.js version when entering directories with version files

   # Store the last directory to detect changes
   $global:XVN_LAST_DIR = $PWD.Path
   $global:XVN_ACTIVE_KEY = $null

   # Store original prompt function
   if (-not (Test-Path function:global:__xvn_original_prompt)) {
       $global:__xvn_original_prompt = $function:prompt
   }

   # Find version file by walking up directory tree
   function Find-VersionFile {
       param([string]$StartPath)

       $current = Get-Item $StartPath
       $patterns = @('.nvmrc', '.node-version', 'package.json')

       while ($current) {
           foreach ($pattern in $patterns) {
               $file = Join-Path $current.FullName $pattern
               if (Test-Path $file) {
                   if ($env:XVN_DEBUG) {
                       Write-Host "xvn: Found version file: $file" -ForegroundColor Yellow
                   }
                   return $file
               }
           }

           $current = $current.Parent
       }

       return $null
   }

   # Compute hash of file content for idempotency
   function Get-ContentHash {
       param([string]$FilePath)

       if (-not (Test-Path $FilePath)) {
           return $null
       }

       $hash = Get-FileHash -Path $FilePath -Algorithm MD5
       return $hash.Hash
   }

   # Execute activation commands from xvn binary
   function Invoke-XvnActivation {
       param([string]$Directory)

       try {
           # Call xvn activate with current directory
           $output = & xvn activate $Directory 2>&1

           if ($env:XVN_DEBUG) {
               Write-Host "xvn: Raw output from binary:" -ForegroundColor Cyan
               Write-Host $output -ForegroundColor Gray
           }

           # Parse JSON command protocol
           if ($output -match '__XVN_COMMANDS_START__(.*)__XVN_COMMANDS_END__') {
               $jsonContent = $matches[1].Trim()
               $commands = ($jsonContent | ConvertFrom-Json).commands

               if ($env:XVN_DEBUG) {
                   Write-Host "xvn: Executing $($commands.Count) commands" -ForegroundColor Cyan
               }

               foreach ($cmd in $commands) {
                   if ($env:XVN_DEBUG) {
                       Write-Host "xvn: $cmd" -ForegroundColor Green
                   }
                   Invoke-Expression $cmd
               }
           }
           elseif ($output -match "error|failed") {
               # Only show errors if XVN_DEBUG is set
               if ($env:XVN_DEBUG) {
                   Write-Host "xvn error: $output" -ForegroundColor Red
               }
           }
       }
       catch {
           # Silently fail to avoid breaking the shell
           if ($env:XVN_DEBUG) {
               Write-Host "xvn exception: $_" -ForegroundColor Red
           }
       }
   }

   # Main hook function called on directory change
   function Invoke-XvnChpwd {
       $currentDir = $PWD.Path

       # Only trigger if directory changed
       if ($currentDir -ne $global:XVN_LAST_DIR) {
           $global:XVN_LAST_DIR = $currentDir

           # Find version file
           $versionFile = Find-VersionFile -StartPath $currentDir

           if ($versionFile) {
               # Compute activation key (file path + content hash)
               $contentHash = Get-ContentHash -FilePath $versionFile
               $activationKey = "$versionFile::$contentHash"

               # Only activate if key changed (idempotency)
               if ($activationKey -ne $global:XVN_ACTIVE_KEY) {
                   $global:XVN_ACTIVE_KEY = $activationKey

                   if ($env:XVN_DEBUG) {
                       Write-Host "xvn: Activating for $versionFile" -ForegroundColor Yellow
                   }

                   Invoke-XvnActivation -Directory $currentDir
               }
               elseif ($env:XVN_DEBUG) {
                   Write-Host "xvn: Already active for $versionFile" -ForegroundColor Gray
               }
           }
           else {
               # No version file found, clear active key
               $global:XVN_ACTIVE_KEY = $null

               if ($env:XVN_DEBUG) {
                   Write-Host "xvn: No version file found in $currentDir" -ForegroundColor Gray
               }
           }
       }
   }

   # Override prompt to trigger on directory change
   function global:prompt {
       Invoke-XvnChpwd
       & $global:__xvn_original_prompt
   }

   # Initial activation for current directory
   Invoke-XvnChpwd
   ```

2. **Add PowerShell script validation to CI:**
   ```yaml
   # .github/workflows/test.yml
   jobs:
     test-powershell:
       runs-on: windows-latest
       steps:
         - uses: actions/checkout@v4

         - name: Run PSScriptAnalyzer
           shell: pwsh
           run: |
             Install-Module -Name PSScriptAnalyzer -Force -Scope CurrentUser
             Invoke-ScriptAnalyzer -Path shell/xvn.ps1 -Recurse -Settings PSGallery
   ```

**Code Structure:**
- `shell/xvn.ps1` - PowerShell hook script
- Global functions: `Find-VersionFile`, `Invoke-XvnActivation`, `Invoke-XvnChpwd`
- Global variables: `$XVN_LAST_DIR`, `$XVN_ACTIVE_KEY`

**Key Considerations:**
- **Prompt override approach**: PowerShell doesn't have `chpwd`, so we override `prompt` function
- **Error handling**: Use try-catch to prevent shell breakage
- **Idempotency**: Track file path + content hash to avoid re-activation
- **Debug mode**: Respect `$env:XVN_DEBUG` for verbose logging
- **Performance**: Cache last directory to avoid unnecessary checks

**Testing:**
```powershell
# Test script syntax
powershell -NoProfile -Command "& { . .\shell\xvn.ps1 }"

# Test with debug mode
$env:XVN_DEBUG = 1
. .\shell\xvn.ps1
cd C:\projects\my-app  # Should trigger activation

# Test idempotency
cd ..
cd C:\projects\my-app  # Should skip (same version file)
```

**Dependencies:**
- None (can be developed independently)

**Enables:**
- M11.3 (needs JSON protocol from binary)
- M11.5 (profile modification)
- M11.8 (testing)

---

### Task M11.3: Implement JSON Command Protocol

**Objective:** Add JSON-based command output protocol for Windows (alternative to FD:3).

**Implementation Steps:**

1. **Create `OutputProtocol` enum in `src/shell/mod.rs`:**
   ```rust
   use serde::{Serialize, Deserialize};

   /// Protocol for communicating activation commands to shell
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum OutputProtocol {
       /// Unix file descriptor #3 protocol (bash/zsh)
       Fd3,
       /// JSON protocol for Windows PowerShell
       Json,
   }

   impl OutputProtocol {
       /// Auto-detect protocol based on platform
       pub fn detect() -> Self {
           #[cfg(windows)]
           return OutputProtocol::Json;

           #[cfg(not(windows))]
           return OutputProtocol::Fd3;
       }

       /// Check if running in PowerShell
       pub fn from_env() -> Self {
           if cfg!(windows) || std::env::var("PSModulePath").is_ok() {
               OutputProtocol::Json
           } else {
               OutputProtocol::Fd3
           }
       }
   }

   /// JSON structure for command output
   #[derive(Serialize, Deserialize, Debug)]
   pub struct CommandOutput {
       pub commands: Vec<String>,
   }
   ```

2. **Create `JsonCommandWriter` in `src/shell/command_writer.rs`:**
   ```rust
   use super::{CommandWriter, OutputProtocol, CommandOutput};
   use anyhow::Result;
   use std::io::{self, Write};

   pub struct JsonCommandWriter {
       commands: Vec<String>,
   }

   impl JsonCommandWriter {
       pub fn new() -> Self {
           Self {
               commands: Vec::new(),
           }
       }

       /// Add environment variable export command (PowerShell syntax)
       pub fn export_env(&mut self, key: &str, value: &str) {
           // Escape PowerShell special characters
           let escaped_value = value
               .replace("`", "``")  // Backtick escape
               .replace("$", "`$")  // Dollar sign
               .replace("\"", "`\""); // Quote

           self.commands.push(format!(r#"$env:{} = "{}""#, key, escaped_value));
       }

       /// Add PATH prepend command (PowerShell syntax)
       pub fn prepend_path(&mut self, path: &str) {
           let escaped_path = path
               .replace("`", "``")
               .replace("$", "`$")
               .replace("\"", "`\"");

           self.commands.push(format!(
               r#"$env:PATH = "{};" + $env:PATH"#,
               escaped_path
           ));
       }

       /// Output JSON to stdout with markers
       pub fn write(self) -> Result<()> {
           if self.commands.is_empty() {
               return Ok(());
           }

           let output = CommandOutput {
               commands: self.commands,
           };

           let json = serde_json::to_string(&output)?;

           println!("__XVN_COMMANDS_START__");
           println!("{}", json);
           println!("__XVN_COMMANDS_END__");

           Ok(())
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_json_export_env() {
           let mut writer = JsonCommandWriter::new();
           writer.export_env("NODE_VERSION", "18.0.0");

           assert_eq!(
               writer.commands[0],
               r#"$env:NODE_VERSION = "18.0.0""#
           );
       }

       #[test]
       fn test_json_prepend_path() {
           let mut writer = JsonCommandWriter::new();
           writer.prepend_path(r"C:\nvm\v18.0.0");

           assert_eq!(
               writer.commands[0],
               r#"$env:PATH = "C:\nvm\v18.0.0;" + $env:PATH"#
           );
       }

       #[test]
       fn test_json_escaping() {
           let mut writer = JsonCommandWriter::new();
           writer.export_env("TEST", r#"value with "quotes" and $vars"#);

           // Should escape quotes and dollar signs
           assert!(writer.commands[0].contains(r#"`""#));
           assert!(writer.commands[0].contains(r#"`$"#));
       }
   }
   ```

3. **Update `activate` command to use protocol** in `src/cli.rs`:
   ```rust
   use crate::shell::{OutputProtocol, JsonCommandWriter};

   Commands::Activate { path, quiet } => {
       // ... existing activation logic ...

       // Detect output protocol
       let protocol = OutputProtocol::from_env();

       match protocol {
           OutputProtocol::Fd3 => {
               // Existing FD:3 implementation
               let mut writer = Fd3CommandWriter::new()?;
               writer.export_env("NODE_VERSION", &version);
               writer.prepend_path(&bin_path);
               writer.write()?;
           }
           OutputProtocol::Json => {
               // New JSON implementation for Windows
               let mut writer = JsonCommandWriter::new();
               writer.export_env("NODE_VERSION", &version);
               writer.prepend_path(&bin_path);
               writer.write()?;
           }
       }

       Ok(())
   }
   ```

4. **Add protocol override for testing** in `src/config/schema.rs`:
   ```rust
   #[derive(Debug, Clone, Deserialize, Serialize)]
   pub struct Config {
       // ... existing fields ...

       /// Override output protocol (for testing)
       #[serde(default)]
       pub output_protocol: Option<String>, // "fd3" or "json"
   }
   ```

**Code Structure:**
- `src/shell/mod.rs` - `OutputProtocol` enum
- `src/shell/command_writer.rs` - `JsonCommandWriter` implementation
- `src/cli.rs` - Protocol detection and selection

**Code Examples:**
```rust
// Auto-detect protocol
let protocol = OutputProtocol::detect();

// Or check environment
let protocol = OutputProtocol::from_env();

// Use JSON writer
let mut writer = JsonCommandWriter::new();
writer.export_env("NODE_VERSION", "18.0.0");
writer.prepend_path(r"C:\Users\username\.nvm\v18.0.0");
writer.write()?;

// Output:
// __XVN_COMMANDS_START__
// {"commands":["$env:NODE_VERSION = \"18.0.0\"","$env:PATH = \"C:\\Users\\username\\.nvm\\v18.0.0;\" + $env:PATH"]}
// __XVN_COMMANDS_END__
```

**Key Considerations:**
- **PowerShell escaping**: Escape backticks, dollar signs, and quotes
- **Markers**: Use `__XVN_COMMANDS_START__` / `__XVN_COMMANDS_END__` for reliable parsing
- **Protocol detection**: Check `PSModulePath` env var or use platform detection
- **Testing**: Both protocols must be tested in CI

**Testing:**
```rust
// Unit tests
#[test]
fn test_protocol_detection() {
    #[cfg(windows)]
    assert_eq!(OutputProtocol::detect(), OutputProtocol::Json);

    #[cfg(not(windows))]
    assert_eq!(OutputProtocol::detect(), OutputProtocol::Fd3);
}
```

**Dependencies:**
- Requires: M11.2 (PowerShell script needs to parse this)

**Enables:**
- M11.5 (complete activation flow)
- M11.8 (protocol testing)

---

### Task M11.4: Update Plugin System for Windows

**Objective:** Modify plugin system to detect and support Windows version managers (nvm-windows, fnm).

**Implementation Steps:**

1. **Update `nvm` plugin for Windows** in `src/plugins/nvm.rs`:
   ```rust
   use std::path::PathBuf;
   use std::env;

   #[derive(Debug)]
   pub struct NvmPlugin {
       nvm_dir: PathBuf,
       is_windows: bool,
   }

   impl NvmPlugin {
       pub fn new() -> Result<Self> {
           let is_windows = cfg!(windows);

           let nvm_dir = if is_windows {
               // Windows: Check APPDATA
               Self::detect_nvm_windows()?
           } else {
               // Unix: Use NVM_DIR or default
               Self::detect_nvm_unix()?
           };

           Ok(Self { nvm_dir, is_windows })
       }

       fn detect_nvm_windows() -> Result<PathBuf> {
           // Check for NVM_HOME environment variable first
           if let Ok(nvm_home) = env::var("NVM_HOME") {
               let path = PathBuf::from(nvm_home);
               if path.exists() {
                   return Ok(path);
               }
           }

           // Check default nvm-windows location
           if let Ok(appdata) = env::var("APPDATA") {
               let nvm_path = PathBuf::from(appdata).join("nvm");
               if nvm_path.exists() {
                   return Ok(nvm_path);
               }
           }

           Err(XvnError::VersionManagerNotFound("nvm-windows".to_string()))
       }

       fn detect_nvm_unix() -> Result<PathBuf> {
           // Existing Unix detection logic
           // ...
       }
   }

   impl VersionManagerPlugin for NvmPlugin {
       fn name(&self) -> &str {
           if self.is_windows {
               "nvm-windows"
           } else {
               "nvm"
           }
       }

       fn version_install_dir(&self, version: &str) -> Result<PathBuf> {
           if self.is_windows {
               // Windows: %APPDATA%\nvm\v18.0.0
               Ok(self.nvm_dir.join(format!("v{}", version)))
           } else {
               // Unix: $NVM_DIR/versions/node/v18.0.0
               Ok(self.nvm_dir.join("versions").join("node").join(format!("v{}", version)))
           }
       }

       fn activation_commands(&self, version: &str) -> Result<Vec<String>> {
           let install_dir = self.version_install_dir(version)?;

           if self.is_windows {
               // Windows: Add to PATH with proper separator
               let bin_dir = install_dir.to_string_lossy().to_string();
               Ok(vec![
                   format!("$env:PATH = \"{};\" + $env:PATH", bin_dir),
                   format!("$env:NODE_VERSION = \"{}\"", version),
               ])
           } else {
               // Unix: Existing logic
               let bin_dir = install_dir.join("bin").to_string_lossy().to_string();
               Ok(vec![
                   format!("export PATH=\"{}:$PATH\"", bin_dir),
                   format!("export NODE_VERSION=\"{}\"", version),
               ])
           }
       }

       fn installed_versions(&self) -> Result<Vec<String>> {
           if self.is_windows {
               // Windows: List subdirectories in nvm_dir
               let versions_dir = &self.nvm_dir;
               let entries = std::fs::read_dir(versions_dir)?;

               let mut versions = Vec::new();
               for entry in entries {
                   let entry = entry?;
                   if entry.file_type()?.is_dir() {
                       if let Some(name) = entry.file_name().to_str() {
                           if name.starts_with('v') {
                               versions.push(name[1..].to_string()); // Remove 'v' prefix
                           }
                       }
                   }
               }

               Ok(versions)
           } else {
               // Unix: Existing logic
               // ...
           }
       }
   }
   ```

2. **Update `fnm` plugin for Windows** in `src/plugins/fnm.rs`:
   ```rust
   impl FnmPlugin {
       pub fn new() -> Result<Self> {
           let fnm_dir = if cfg!(windows) {
               Self::detect_fnm_windows()?
           } else {
               Self::detect_fnm_unix()?
           };

           Ok(Self { fnm_dir })
       }

       fn detect_fnm_windows() -> Result<PathBuf> {
           // Check FNM_DIR first
           if let Ok(fnm_dir) = env::var("FNM_DIR") {
               let path = PathBuf::from(fnm_dir);
               if path.exists() {
                   return Ok(path);
               }
           }

           // Check LOCALAPPDATA
           if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
               let fnm_path = PathBuf::from(local_appdata).join("fnm");
               if fnm_path.exists() {
                   return Ok(fnm_path);
               }
           }

           Err(XvnError::VersionManagerNotFound("fnm".to_string()))
       }
   }
   ```

3. **Add Windows path utilities** in `src/utils/path.rs` (create if doesn't exist):
   ```rust
   use std::path::{Path, PathBuf};
   use std::env;

   /// Expand Windows environment variables (%VAR%)
   pub fn expand_env_vars(path: &str) -> String {
       if !cfg!(windows) {
           return path.to_string();
       }

       let mut result = path.to_string();

       // Match %VAR% patterns
       let re = regex::Regex::new(r"%([^%]+)%").unwrap();

       for cap in re.captures_iter(path) {
           let var_name = &cap[1];
           if let Ok(var_value) = env::var(var_name) {
               result = result.replace(&format!("%{}%", var_name), &var_value);
           }
       }

       result
   }

   /// Normalize path separators for Windows
   pub fn normalize_separators(path: &Path) -> PathBuf {
       if !cfg!(windows) {
           return path.to_path_buf();
       }

       // Convert forward slashes to backslashes on Windows
       let path_str = path.to_string_lossy();
       let normalized = path_str.replace('/', r"\");
       PathBuf::from(normalized)
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       #[cfg(windows)]
       fn test_expand_env_vars() {
           env::set_var("TESTVAR", "C:\\test");
           let expanded = expand_env_vars("%TESTVAR%\\subdir");
           assert_eq!(expanded, "C:\\test\\subdir");
       }

       #[test]
       #[cfg(windows)]
       fn test_normalize_separators() {
           let path = Path::new("C:/Users/test/file");
           let normalized = normalize_separators(path);
           assert_eq!(normalized.to_string_lossy(), r"C:\Users\test\file");
       }
   }
   ```

**Code Structure:**
- `src/plugins/nvm.rs` - Windows-aware nvm plugin
- `src/plugins/fnm.rs` - Windows-aware fnm plugin
- `src/utils/path.rs` - Path utilities (new file)

**Key Considerations:**
- **Environment variables**: Windows uses `%VAR%`, Unix uses `$VAR`
- **Install locations**: Different for nvm-windows (`%APPDATA%\nvm`)
- **Path separators**: Use `std::path::PathBuf` for cross-platform paths
- **Detection order**: Check env vars first, then default locations

**Testing:**
```rust
#[test]
#[cfg(windows)]
fn test_nvm_windows_detection() {
    let plugin = NvmPlugin::new().unwrap();
    assert_eq!(plugin.name(), "nvm-windows");
}

#[test]
#[cfg(windows)]
fn test_nvm_version_path() {
    let plugin = NvmPlugin::new().unwrap();
    let path = plugin.version_install_dir("18.0.0").unwrap();
    assert!(path.to_string_lossy().contains("v18.0.0"));
}
```

**Dependencies:**
- Requires: M11.1 (Windows compilation)

**Enables:**
- M11.6 (path handling)
- M11.8 (plugin testing on Windows)

---

### Task M11.5: Implement PowerShell Profile Modification

**Objective:** Update `setup` command to modify PowerShell profile and install hook script.

**Implementation Steps:**

1. **Add PowerShell profile detection** in `src/shell/profile.rs`:
   ```rust
   use std::path::PathBuf;
   use std::env;

   #[derive(Debug)]
   pub enum ShellProfile {
       Bash(PathBuf),      // ~/.bashrc
       Zsh(PathBuf),       // ~/.zshrc
       PowerShell(PathBuf), // $PROFILE
   }

   impl ShellProfile {
       /// Detect PowerShell profile path
       pub fn detect_powershell() -> Result<PathBuf> {
           // PowerShell $PROFILE variable location
           // For CurrentUserCurrentHost: ~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
           // or for PS 5.1: ~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1

           let home = dirs::home_dir()
               .ok_or_else(|| XvnError::Config("Cannot determine home directory".to_string()))?;

           // Try PowerShell Core 7+ first
           let ps7_profile = home
               .join("Documents")
               .join("PowerShell")
               .join("Microsoft.PowerShell_profile.ps1");

           // Fall back to PowerShell 5.1
           let ps5_profile = home
               .join("Documents")
               .join("WindowsPowerShell")
               .join("Microsoft.PowerShell_profile.ps1");

           // Use PS7 if it exists or if PS7 is installed, otherwise PS5
           if Self::is_powershell_7_installed() {
               Ok(ps7_profile)
           } else {
               Ok(ps5_profile)
           }
       }

       fn is_powershell_7_installed() -> bool {
           // Check if pwsh exists on PATH
           std::process::Command::new("pwsh")
               .arg("-Version")
               .output()
               .is_ok()
       }

       /// Get profile path for current shell
       pub fn detect() -> Result<Self> {
           if cfg!(windows) {
               let profile_path = Self::detect_powershell()?;
               Ok(ShellProfile::PowerShell(profile_path))
           } else {
               // Existing Unix shell detection
               // ...
           }
       }
   }
   ```

2. **Update setup command for Windows** in `src/shell/setup.rs`:
   ```rust
   use std::fs;
   use std::io::Write;

   pub fn setup_shell(force: bool) -> Result<()> {
       let profile = ShellProfile::detect()?;

       match profile {
           ShellProfile::PowerShell(profile_path) => {
               setup_powershell(profile_path, force)
           }
           ShellProfile::Bash(profile_path) => {
               setup_bash(profile_path, force)
           }
           ShellProfile::Zsh(profile_path) => {
               setup_zsh(profile_path, force)
           }
       }
   }

   fn setup_powershell(profile_path: PathBuf, force: bool) -> Result<()> {
       println!("Setting up xvn for PowerShell...");

       // 1. Copy xvn.ps1 to ~/.xvn/bin/
       let xvn_dir = dirs::home_dir()
           .ok_or_else(|| XvnError::Config("Cannot determine home directory".to_string()))?
           .join(".xvn")
           .join("bin");

       fs::create_dir_all(&xvn_dir)?;

       let script_dest = xvn_dir.join("xvn.ps1");

       // Copy from embedded script or bundled file
       let script_content = include_str!("../../shell/xvn.ps1");
       fs::write(&script_dest, script_content)?;

       println!("✓ Installed xvn.ps1 to {}", script_dest.display());

       // 2. Create profile directory if missing
       if let Some(parent) = profile_path.parent() {
           fs::create_dir_all(parent)?;
       }

       // 3. Check if profile exists, create if not
       if !profile_path.exists() {
           fs::write(&profile_path, "# PowerShell Profile\n")?;
           println!("✓ Created PowerShell profile at {}", profile_path.display());
       }

       // 4. Add sourcing line to profile (check idempotency)
       let profile_content = fs::read_to_string(&profile_path)?;
       let source_line = format!(
           ". \"$env:USERPROFILE\\.xvn\\bin\\xvn.ps1\""
       );

       if profile_content.contains(&source_line) {
           if force {
               println!("xvn already configured in profile (--force specified, skipping)");
           } else {
               println!("✓ xvn already configured in profile");
               return Ok(());
           }
       } else {
           let mut file = fs::OpenOptions::new()
               .append(true)
               .open(&profile_path)?;

           writeln!(file)?;
           writeln!(file, "# xvn - Automatic Node.js version switching")?;
           writeln!(file, "{}", source_line)?;

           println!("✓ Added xvn to PowerShell profile");
       }

       // 5. Create default ~/.xvnrc if missing
       let xvnrc_path = dirs::home_dir()
           .unwrap()
           .join(".xvnrc");

       if !xvnrc_path.exists() {
           let default_config = include_str!("../../templates/xvnrc.yaml");
           fs::write(&xvnrc_path, default_config)?;
           println!("✓ Created default configuration at {}", xvnrc_path.display());
       }

       // 6. Instructions
       println!("\n{}", "Setup complete!".green().bold());
       println!("\nNext steps:");
       println!("  1. Restart PowerShell or run: . $PROFILE");
       println!("  2. Navigate to a directory with a .nvmrc file");
       println!("  3. Node.js version will switch automatically");
       println!("\nNote: You may need to set execution policy:");
       println!("  Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser");

       Ok(())
   }
   ```

3. **Add execution policy check and helper**:
   ```rust
   fn check_execution_policy() -> Result<()> {
       let output = std::process::Command::new("powershell")
           .args(&["-Command", "Get-ExecutionPolicy -Scope CurrentUser"])
           .output()?;

       let policy = String::from_utf8_lossy(&output.stdout).trim().to_string();

       if policy == "Restricted" || policy == "Undefined" {
           println!("\n⚠️  Warning: PowerShell execution policy is '{}'", policy);
           println!("Scripts may be blocked. Run this command to allow:");
           println!("  Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser");
           println!();
       }

       Ok(())
   }
   ```

**Code Structure:**
- `src/shell/profile.rs` - Shell profile detection (updated)
- `src/shell/setup.rs` - Setup command (updated for Windows)
- `shell/xvn.ps1` - PowerShell hook script (embedded)

**Key Considerations:**
- **PowerShell versions**: Support both PS 5.1 and PS 7+
- **Profile locations**: Different for each PowerShell version
- **Execution policy**: Inform user if scripts are blocked
- **Idempotency**: Don't duplicate source lines
- **Directory creation**: Create profile directory if missing

**Testing:**
```powershell
# Test setup
xvn init --quick

# Verify profile modified
Get-Content $PROFILE | Select-String "xvn"

# Test idempotency
xvn init --quick
# Should not duplicate lines

# Test with force
xvn init --quick --force
```

**Dependencies:**
- Requires: M11.2 (xvn.ps1 exists)

**Enables:**
- M11.8 (end-to-end testing)
- M11.9 (documentation)

---

### Task M11.6: Cross-Platform Path Handling

**Objective:** Audit and update all path operations to work correctly on Windows.

**Implementation Steps:**

1. **Audit all path operations in codebase:**
   ```bash
   # Find potential issues
   rg "\.join\(\"/" src/
   rg "format!\(.*/" src/
   rg "\$HOME" src/
   ```

2. **Update config loader** in `src/config/loader.rs`:
   ```rust
   use std::path::PathBuf;
   use dirs;

   impl Config {
       pub fn load() -> Result<Self> {
           let mut config = Self::default();

           // 1. Load global config (~/.xvnrc)
           let global_config = Self::global_config_path()?;
           if global_config.exists() {
               let global = Self::from_file(&global_config)?;
               config.merge(global);
           }

           // 2. Load project config (.xvn.yaml)
           let project_config = Self::project_config_path()?;
           if project_config.exists() {
               let project = Self::from_file(&project_config)?;
               config.merge(project);
           }

           Ok(config)
       }

       fn global_config_path() -> Result<PathBuf> {
           let home = dirs::home_dir()
               .ok_or_else(|| XvnError::Config("Cannot determine home directory".to_string()))?;

           Ok(home.join(".xvnrc"))  // Works on both Unix and Windows
       }

       fn project_config_path() -> Result<PathBuf> {
           let current_dir = std::env::current_dir()?;
           Ok(current_dir.join(".xvn.yaml"))
       }

       /// Expand tilde and environment variables in path
       pub fn expand_path(path: &str) -> PathBuf {
           if path.starts_with('~') {
               if let Some(home) = dirs::home_dir() {
                   return home.join(&path[2..]);  // Skip "~/"
               }
           }

           // Expand environment variables
           #[cfg(windows)]
           {
               let expanded = crate::utils::path::expand_env_vars(path);
               return PathBuf::from(expanded);
           }

           #[cfg(not(windows))]
           {
               PathBuf::from(path)
           }
       }
   }
   ```

3. **Update version file finder** in `src/version_file/finder.rs`:
   ```rust
   use std::path::{Path, PathBuf};

   pub fn find_version_file(start_dir: &Path) -> Result<Option<PathBuf>> {
       let mut current = start_dir.to_path_buf();

       loop {
           // Check for version files
           for filename in &[".nvmrc", ".node-version", "package.json"] {
               let candidate = current.join(filename);  // Cross-platform join
               if candidate.exists() {
                   return Ok(Some(candidate));
               }
           }

           // Move up to parent directory
           if !current.pop() {
               // Reached root
               break;
           }
       }

       Ok(None)
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       use tempfile::TempDir;

       #[test]
       fn test_find_version_file_windows_paths() {
           let temp = TempDir::new().unwrap();
           let subdir = temp.path().join("projects").join("myapp");
           std::fs::create_dir_all(&subdir).unwrap();

           let nvmrc = temp.path().join(".nvmrc");
           std::fs::write(&nvmrc, "18.0.0").unwrap();

           let found = find_version_file(&subdir).unwrap();
           assert_eq!(found, Some(nvmrc));
       }
   }
   ```

4. **Create path utilities module** in `src/utils/mod.rs`:
   ```rust
   pub mod path;

   // Re-export commonly used functions
   pub use path::{expand_env_vars, normalize_separators};
   ```

5. **Add comprehensive path tests** in `tests/path_handling.rs`:
   ```rust
   #[cfg(test)]
   mod tests {
       use std::env;
       use std::path::PathBuf;

       #[test]
       #[cfg(windows)]
       fn test_windows_path_join() {
           let base = PathBuf::from("C:\\Users\\test");
           let joined = base.join(".xvn").join("config");
           assert_eq!(joined.to_string_lossy(), "C:\\Users\\test\\.xvn\\config");
       }

       #[test]
       #[cfg(windows)]
       fn test_home_dir_expansion() {
           let home = dirs::home_dir().unwrap();
           let expanded = xvn::config::Config::expand_path("~/.xvnrc");
           assert_eq!(expanded, home.join(".xvnrc"));
       }

       #[test]
       #[cfg(windows)]
       fn test_env_var_expansion() {
           env::set_var("TESTVAR", "C:\\test");
           let expanded = xvn::utils::path::expand_env_vars("%TESTVAR%\\file");
           assert_eq!(expanded, "C:\\test\\file");
       }
   }
   ```

**Code Structure:**
- `src/config/loader.rs` - Updated path handling
- `src/version_file/finder.rs` - Cross-platform file search
- `src/utils/path.rs` - Path utilities
- `tests/path_handling.rs` - Path handling tests

**Key Considerations:**
- **Use `PathBuf::join()`**: Never concatenate paths with strings
- **Use `dirs` crate**: For cross-platform home directory
- **Avoid hardcoded `/`**: Let PathBuf handle separators
- **Test on Windows**: Verify with actual Windows paths

**Testing:**
```bash
# Run path-specific tests
cargo test --test path_handling

# Run all tests on Windows
cargo test --target x86_64-pc-windows-msvc
```

**Dependencies:**
- Requires: M11.4 (plugin updates)

**Enables:**
- M11.8 (testing on Windows)

---

### Task M11.7: Update npm Package for Windows

**Objective:** Modify npm package to detect Windows, select correct binary, and install properly.

**Implementation Steps:**

1. **Update `install.js` to detect Windows:**
   ```javascript
   #!/usr/bin/env node

   const fs = require('fs');
   const path = require('path');
   const https = require('https');
   const { execSync } = require('child_process');

   // Detect platform and architecture
   function getPlatform() {
     const platform = process.platform;
     const arch = process.arch;

     const platformMap = {
       'win32': {
         'x64': 'x86_64-pc-windows-msvc',
         'arm64': 'aarch64-pc-windows-msvc',
       },
       'darwin': {
         'x64': 'x86_64-apple-darwin',
         'arm64': 'aarch64-apple-darwin',
       },
       'linux': {
         'x64': 'x86_64-unknown-linux-gnu',
         'arm64': 'aarch64-unknown-linux-gnu',
       },
     };

     if (!platformMap[platform]) {
       throw new Error(`Unsupported platform: ${platform}`);
     }

     if (!platformMap[platform][arch]) {
       throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
     }

     return platformMap[platform][arch];
   }

   // Get binary name (with .exe on Windows)
   function getBinaryName() {
     return process.platform === 'win32' ? 'xvn.exe' : 'xvn';
   }

   // Install binary
   async function install() {
     try {
       const target = getPlatform();
       const binaryName = getBinaryName();
       const version = require('./package.json').version;

       console.log(`Installing xvn ${version} for ${target}...`);

       // Check if we're in a published package (has native/ directory)
       const nativeDir = path.join(__dirname, 'native', target);
       const sourceBinary = path.join(nativeDir, binaryName);

       if (fs.existsSync(sourceBinary)) {
         // Published package - copy from native/
         console.log(`Found binary at ${sourceBinary}`);
         const destBinary = path.join(__dirname, binaryName);
         fs.copyFileSync(sourceBinary, destBinary);
         fs.chmodSync(destBinary, 0o755);
         console.log(`✓ Installed binary to ${destBinary}`);
       } else {
         // Development - download from GitHub releases
         console.log('Binary not found in package, downloading from GitHub...');
         await downloadBinary(version, target, binaryName);
       }

       console.log('\n✓ xvn installed successfully!');
       console.log('\nNext steps:');

       if (process.platform === 'win32') {
         console.log('  1. Run: xvn init');
         console.log('  2. Restart PowerShell');
         console.log('  3. cd into a project with .nvmrc');
         console.log('\nNote: You may need to set PowerShell execution policy:');
         console.log('  Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser');
       } else {
         console.log('  1. Run: xvn init');
         console.log('  2. Restart your shell');
         console.log('  3. cd into a project with .nvmrc');
       }

     } catch (error) {
       console.error('Installation failed:', error.message);
       process.exit(1);
     }
   }

   // Download binary from GitHub releases
   async function downloadBinary(version, target, binaryName) {
     const url = `https://github.com/cameronolivier/xvn/releases/download/v${version}/xvn-${target}${process.platform === 'win32' ? '.exe' : ''}`;
     const destPath = path.join(__dirname, binaryName);

     return new Promise((resolve, reject) => {
       const file = fs.createWriteStream(destPath);

       https.get(url, (response) => {
         if (response.statusCode === 302 || response.statusCode === 301) {
           // Follow redirect
           https.get(response.headers.location, (res) => {
             res.pipe(file);
             file.on('finish', () => {
               file.close();
               fs.chmodSync(destPath, 0o755);
               resolve();
             });
           }).on('error', reject);
         } else if (response.statusCode === 200) {
           response.pipe(file);
           file.on('finish', () => {
             file.close();
             fs.chmodSync(destPath, 0o755);
             resolve();
           });
         } else {
           reject(new Error(`Download failed: ${response.statusCode}`));
         }
       }).on('error', reject);
     });
   }

   // Run installation
   install();
   ```

2. **Create Windows wrapper script** `bin/xvn.cmd`:
   ```batch
   @echo off
   setlocal

   :: Find Node.js installation
   for %%i in (node.exe) do set NODE_PATH=%%~$PATH:i

   if "%NODE_PATH%"=="" (
       echo Error: Node.js not found in PATH
       exit /b 1
   )

   :: Get the directory of this script
   set SCRIPT_DIR=%~dp0

   :: Execute the binary with all arguments
   "%SCRIPT_DIR%\..\xvn.exe" %*
   ```

3. **Update `package.json`** to include Windows files:
   ```json
   {
     "name": "@olvrcc/xvn",
     "version": "1.3.0",
     "description": "Extreme Version Switcher for Node.js",
     "bin": {
       "xvn": "./bin/xvn"
     },
     "scripts": {
       "postinstall": "node install.js"
     },
     "files": [
       "install.js",
       "bin/xvn",
       "bin/xvn.cmd",
       "shell/xvn.sh",
       "shell/xvn.ps1",
       "native/"
     ],
     "os": [
       "darwin",
       "linux",
       "win32"
     ],
     "cpu": [
       "x64",
       "arm64"
     ],
     "engines": {
       "node": ">=14.0.0"
     }
   }
   ```

4. **Update `.npmignore`** to ensure binaries are included:
   ```
   # .npmignore
   target/
   .cargo/
   Cargo.lock
   .git/
   .github/
   tests/

   # Include native binaries
   !native/
   ```

5. **Test npm package structure:**
   ```bash
   # Pack the package
   npm pack

   # Extract and verify structure
   tar -tzf olvrcc-xvn-1.3.0.tgz | grep native
   # Should show:
   # package/native/x86_64-pc-windows-msvc/xvn.exe
   # package/native/aarch64-pc-windows-msvc/xvn.exe
   # etc.
   ```

**Code Structure:**
- `install.js` - Updated installation script
- `bin/xvn.cmd` - Windows wrapper (new)
- `package.json` - Updated with Windows support

**Key Considerations:**
- **Binary extension**: Add `.exe` for Windows
- **Platform detection**: Use `process.platform === 'win32'`
- **npm bin wrapper**: Create `.cmd` file for Windows
- **File permissions**: Use `fs.chmodSync()` (no-op on Windows)

**Testing:**
```bash
# Test on Windows
npm install -g @olvrcc/xvn

# Verify binary is executable
xvn --version

# Check global install location
npm list -g @olvrcc/xvn
```

**Dependencies:**
- Requires: M11.1 (Windows binaries exist)

**Enables:**
- M11.10 (release)

---

### Task M11.8: Windows-Specific Testing

**Objective:** Add comprehensive testing for Windows platform, including PowerShell script validation and integration tests.

**Implementation Steps:**

1. **Add PSScriptAnalyzer validation** in `.github/workflows/test.yml`:
   ```yaml
   name: Test

   on: [push, pull_request]

   jobs:
     test-powershell:
       runs-on: windows-latest
       steps:
         - uses: actions/checkout@v4

         - name: Install PSScriptAnalyzer
           shell: pwsh
           run: |
             Install-Module -Name PSScriptAnalyzer -Force -Scope CurrentUser

         - name: Lint PowerShell script
           shell: pwsh
           run: |
             $results = Invoke-ScriptAnalyzer -Path shell/xvn.ps1 -Recurse -Settings PSGallery
             if ($results) {
               $results | Format-Table -AutoSize
               exit 1
             }
             Write-Host "✓ PowerShell script passed linting"

         - name: Test PowerShell syntax
           shell: pwsh
           run: |
             $errors = $null
             $null = [System.Management.Automation.PSParser]::Tokenize(
               (Get-Content shell/xvn.ps1 -Raw),
               [ref]$errors
             )
             if ($errors) {
               $errors | Format-Table -AutoSize
               exit 1
             }
             Write-Host "✓ PowerShell script syntax valid"
   ```

2. **Create Windows integration tests** in `tests/windows_integration.rs`:
   ```rust
   #![cfg(windows)]

   use assert_cmd::Command;
   use predicates::prelude::*;
   use std::env;
   use std::fs;
   use std::path::PathBuf;
   use tempfile::TempDir;

   #[test]
   fn test_windows_binary_runs() {
       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("--version")
           .assert()
           .success()
           .stdout(predicate::str::contains("xvn"));
   }

   #[test]
   fn test_json_protocol_output() {
       let temp = TempDir::new().unwrap();
       let nvmrc = temp.path().join(".nvmrc");
       fs::write(&nvmrc, "18.0.0").unwrap();

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       cmd.arg("activate")
           .arg(temp.path())
           .env("PSModulePath", "C:\\dummy") // Force JSON protocol
           .assert()
           .success()
           .stdout(predicate::str::contains("__XVN_COMMANDS_START__"))
           .stdout(predicate::str::contains("__XVN_COMMANDS_END__"))
           .stdout(predicate::str::contains("$env:"));
   }

   #[test]
   fn test_powershell_command_format() {
       let temp = TempDir::new().unwrap();
       let nvmrc = temp.path().join(".nvmrc");
       fs::write(&nvmrc, "18.0.0").unwrap();

       let mut cmd = Command::cargo_bin("xvn").unwrap();
       let output = cmd
           .arg("activate")
           .arg(temp.path())
           .env("PSModulePath", "C:\\dummy")
           .output()
           .unwrap();

       let stdout = String::from_utf8_lossy(&output.stdout);

       // Parse JSON between markers
       let re = regex::Regex::new(r"__XVN_COMMANDS_START__(.*)__XVN_COMMANDS_END__").unwrap();
       let json = re.captures(&stdout).unwrap()[1].trim();

       let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
       let commands = parsed["commands"].as_array().unwrap();

       assert!(commands.len() > 0);
       assert!(commands[0].as_str().unwrap().starts_with("$env:"));
   }

   #[test]
   fn test_windows_path_handling() {
       let config = xvn::config::Config::default();

       // Test Windows path expansion
       env::set_var("TESTVAR", "C:\\test");
       let expanded = xvn::utils::path::expand_env_vars("%TESTVAR%\\file");
       assert_eq!(expanded, "C:\\test\\file");
   }

   #[test]
   fn test_nvm_windows_detection() {
       // This test requires nvm-windows to be installed
       if env::var("APPDATA").is_err() {
           return; // Skip if not on Windows
       }

       // Test nvm-windows plugin detection
       if let Ok(plugin) = xvn::plugins::NvmPlugin::new() {
           assert_eq!(plugin.name(), "nvm-windows");
       }
   }
   ```

3. **Add PowerShell end-to-end test script** in `tests/test-powershell.ps1`:
   ```powershell
   # test-powershell.ps1 - End-to-end PowerShell integration test

   param(
       [switch]$Verbose
   )

   $ErrorActionPreference = "Stop"

   Write-Host "Testing PowerShell integration..." -ForegroundColor Cyan

   # 1. Test xvn.ps1 syntax
   Write-Host "`n1. Testing xvn.ps1 syntax..." -ForegroundColor Yellow
   $errors = $null
   $null = [System.Management.Automation.PSParser]::Tokenize(
       (Get-Content shell/xvn.ps1 -Raw),
       [ref]$errors
   )
   if ($errors) {
       Write-Host "✗ Syntax errors found:" -ForegroundColor Red
       $errors | Format-Table -AutoSize
       exit 1
   }
   Write-Host "✓ Syntax valid" -ForegroundColor Green

   # 2. Test script loading
   Write-Host "`n2. Testing script loading..." -ForegroundColor Yellow
   try {
       . .\shell\xvn.ps1
       Write-Host "✓ Script loaded successfully" -ForegroundColor Green
   }
   catch {
       Write-Host "✗ Failed to load script: $_" -ForegroundColor Red
       exit 1
   }

   # 3. Test version file finder
   Write-Host "`n3. Testing version file finder..." -ForegroundColor Yellow
   $testDir = Join-Path $env:TEMP "xvn-test-$(Get-Random)"
   New-Item -ItemType Directory -Path $testDir | Out-Null
   Set-Content -Path (Join-Path $testDir ".nvmrc") -Value "18.0.0"

   $found = Find-VersionFile -StartPath $testDir
   if ($found -and (Test-Path $found)) {
       Write-Host "✓ Version file found: $found" -ForegroundColor Green
   }
   else {
       Write-Host "✗ Version file not found" -ForegroundColor Red
       exit 1
   }

   # 4. Test content hashing
   Write-Host "`n4. Testing content hashing..." -ForegroundColor Yellow
   $hash1 = Get-ContentHash -FilePath $found
   $hash2 = Get-ContentHash -FilePath $found
   if ($hash1 -eq $hash2) {
       Write-Host "✓ Content hash consistent: $hash1" -ForegroundColor Green
   }
   else {
       Write-Host "✗ Content hash inconsistent" -ForegroundColor Red
       exit 1
   }

   # 5. Test xvn binary (if available)
   Write-Host "`n5. Testing xvn binary..." -ForegroundColor Yellow
   $xvnPath = ".\target\x86_64-pc-windows-msvc\release\xvn.exe"
   if (Test-Path $xvnPath) {
       $version = & $xvnPath --version
       Write-Host "✓ Binary works: $version" -ForegroundColor Green

       # Test activation with JSON protocol
       $env:PSModulePath = "C:\dummy" # Force JSON protocol
       $output = & $xvnPath activate $testDir

       if ($output -match "__XVN_COMMANDS_START__") {
           Write-Host "✓ JSON protocol output detected" -ForegroundColor Green
       }
       else {
           Write-Host "✗ JSON protocol output not found" -ForegroundColor Red
           if ($Verbose) {
               Write-Host "Output: $output" -ForegroundColor Gray
           }
       }
   }
   else {
       Write-Host "⚠ Binary not found at $xvnPath (skipping)" -ForegroundColor Yellow
   }

   # Cleanup
   Remove-Item -Recurse -Force $testDir

   Write-Host "`n✓ All tests passed!" -ForegroundColor Green
   ```

4. **Add Windows to CI matrix** in `.github/workflows/ci.yml`:
   ```yaml
   jobs:
     test:
       strategy:
         matrix:
           include:
             - os: ubuntu-latest
               target: x86_64-unknown-linux-gnu
             - os: macos-latest
               target: x86_64-apple-darwin
             - os: windows-latest
               target: x86_64-pc-windows-msvc

       runs-on: ${{ matrix.os }}

       steps:
         - uses: actions/checkout@v4

         - name: Install Rust
           uses: dtolnay/rust-toolchain@stable
           with:
             targets: ${{ matrix.target }}

         - name: Run tests
           run: cargo test --target ${{ matrix.target }}

         - name: Run Windows integration tests
           if: matrix.os == 'windows-latest'
           shell: pwsh
           run: |
             cargo build --release --target ${{ matrix.target }}
             .\tests\test-powershell.ps1 -Verbose
   ```

5. **Create manual testing checklist** in `docs/WINDOWS_TESTING.md`:
   ```markdown
   # Windows Testing Checklist

   ## Environments to Test

   - [ ] Windows 10 x64
   - [ ] Windows 11 x64
   - [ ] Windows 11 ARM64 (if available)
   - [ ] PowerShell 5.1
   - [ ] PowerShell Core 7+
   - [ ] Windows Terminal
   - [ ] VS Code integrated terminal

   ## Test Scenarios

   ### Installation
   - [ ] `npm install -g @olvrcc/xvn` succeeds
   - [ ] Binary is in PATH
   - [ ] `xvn --version` works

   ### Setup
   - [ ] `xvn init --quick` creates profile
   - [ ] PowerShell profile modified correctly
   - [ ] `xvn.ps1` copied to `~/.xvn/bin/`
   - [ ] Default `~/.xvnrc` created

   ### Activation
   - [ ] Create `.nvmrc` with version
   - [ ] `cd` into directory triggers activation
   - [ ] `$env:NODE_VERSION` set correctly
   - [ ] `node --version` shows correct version
   - [ ] `cd` out clears activation

   ### Idempotency
   - [ ] `cd` into same directory doesn't re-activate
   - [ ] Changing `.nvmrc` content triggers re-activation
   - [ ] Multiple PowerShell windows work independently

   ### Error Handling
   - [ ] Invalid version in `.nvmrc` shows error
   - [ ] Missing version manager shows clear message
   - [ ] Shell doesn't break on errors

   ### Debug Mode
   - [ ] `$env:XVN_DEBUG=1` shows verbose output
   - [ ] Debug messages are helpful
   ```

**Code Structure:**
- `.github/workflows/test.yml` - PowerShell linting
- `tests/windows_integration.rs` - Rust integration tests
- `tests/test-powershell.ps1` - PowerShell E2E tests
- `docs/WINDOWS_TESTING.md` - Manual testing checklist

**Key Considerations:**
- **PSScriptAnalyzer**: Catches PowerShell best practice violations
- **Conditional tests**: Use `#![cfg(windows)]` for Windows-only tests
- **GitHub Actions**: Use `windows-latest` runner
- **Manual testing**: Some scenarios need real Windows hardware

**Testing:**
```bash
# Run Windows tests locally (on Windows)
cargo test --target x86_64-pc-windows-msvc

# Run PowerShell tests
pwsh .\tests\test-powershell.ps1 -Verbose

# CI will run automatically on push
```

**Dependencies:**
- Requires: M11.2 (PowerShell script)
- Requires: M11.3 (JSON protocol)
- Requires: M11.4 (plugins)
- Requires: M11.5 (setup)
- Requires: M11.6 (path handling)

**Enables:**
- M11.9 (documentation - test results inform docs)
- M11.10 (release - tests must pass)

---

### Task M11.9: Documentation Updates

**Objective:** Update all documentation to include Windows installation, setup, and troubleshooting.

**Implementation Steps:**

1. **Update README.md** with Windows section:
   ```markdown
   # xvn - Extreme Version Switcher

   Fast, automatic Node.js version switching for Unix and Windows.

   ## Installation

   ### macOS / Linux

   ```bash
   npm install -g @olvrcc/xvn
   xvn init
   # Restart your shell
   ```

   ### Windows (PowerShell)

   ```powershell
   npm install -g @olvrcc/xvn
   xvn init

   # Set execution policy if needed:
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

   # Restart PowerShell or run:
   . $PROFILE
   ```

   ## Platform Support

   | Platform | Shell | Status |
   |----------|-------|--------|
   | macOS | bash, zsh | ✅ Supported |
   | Linux | bash, zsh | ✅ Supported |
   | Windows | PowerShell 5.1+ | ✅ Supported |
   | Windows | PowerShell Core 7+ | ✅ Supported |
   | Windows | cmd.exe | ❌ Not supported |

   ## Version Managers

   | Manager | macOS | Linux | Windows |
   |---------|-------|-------|---------|
   | nvm | ✅ | ✅ | ✅ (nvm-windows) |
   | fnm | ✅ | ✅ | ✅ |
   | n | ✅ | ✅ | ❌ |
   | volta | ✅ | ✅ | ✅ |

   ## Quick Start

   1. Install a version manager (nvm, fnm, etc.)
   2. Install xvn: `npm install -g @olvrcc/xvn`
   3. Run setup: `xvn init`
   4. Restart your shell
   5. Create a `.nvmrc` file in your project
   6. `cd` into the directory - version switches automatically!
   ```

2. **Create Windows troubleshooting guide** in `docs/WINDOWS.md`:
   ```markdown
   # Windows & PowerShell Support

   ## Requirements

   - Windows 10 or later
   - PowerShell 5.1 or PowerShell Core 7+
   - A version manager (nvm-windows or fnm)
   - Node.js (for npm installation)

   ## Installation

   ### Step 1: Install a Version Manager

   **Option A: nvm-windows (Recommended)**
   ```powershell
   # Download installer from:
   # https://github.com/coreybutler/nvm-windows/releases

   # Verify installation
   nvm version
   ```

   **Option B: fnm**
   ```powershell
   # Using Cargo
   cargo install fnm

   # Or using Scoop
   scoop install fnm

   # Verify
   fnm --version
   ```

   ### Step 2: Install xvn

   ```powershell
   npm install -g @olvrcc/xvn
   ```

   ### Step 3: Configure PowerShell

   ```powershell
   xvn init

   # If you see execution policy errors:
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

   # Reload profile
   . $PROFILE
   ```

   ## Troubleshooting

   ### Issue: "Scripts are disabled on this system"

   **Cause:** PowerShell execution policy is too restrictive.

   **Solution:**
   ```powershell
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

   ### Issue: "xvn is not recognized"

   **Cause:** npm global bin directory not in PATH.

   **Solution:**
   ```powershell
   # Find npm global bin path
   npm config get prefix

   # Add to PATH (replace <npm-path> with output above)
   $env:PATH = "<npm-path>;$env:PATH"

   # Make permanent via System Properties > Environment Variables
   ```

   ### Issue: Version not switching

   **Cause:** Various reasons.

   **Debug:**
   ```powershell
   # Enable debug mode
   $env:XVN_DEBUG = 1

   # Navigate to project directory
   cd C:\projects\myapp

   # Check output for errors
   ```

   ### Issue: "nvm not found"

   **Cause:** nvm-windows not installed or not in PATH.

   **Solution:**
   ```powershell
   # Check nvm installation
   nvm version

   # Check environment variables
   $env:NVM_HOME
   $env:APPDATA

   # Verify nvm directory exists
   Test-Path "$env:APPDATA\nvm"
   ```

   ### Issue: Permission denied errors

   **Cause:** Antivirus or Windows Defender blocking script execution.

   **Solution:**
   - Add exception for `~\.xvn\` directory
   - Add exception for PowerShell script execution

   ## Architecture Differences

   ### Unix vs Windows

   | Feature | Unix | Windows |
   |---------|------|---------|
   | **Hook** | `chpwd_functions` | Prompt override |
   | **Protocol** | File descriptor #3 | JSON output |
   | **Env vars** | `export VAR=value` | `$env:VAR = "value"` |
   | **Paths** | `/` separators | `\` separators |
   | **Profile** | `~/.bashrc`, `~/.zshrc` | `$PROFILE` |

   ### PowerShell Integration

   xvn integrates with PowerShell by:

   1. **Overriding the prompt function** to detect directory changes
   2. **Parsing JSON output** from the xvn binary
   3. **Executing commands** via `Invoke-Expression`

   The hook script is located at: `~\.xvn\bin\xvn.ps1`

   ## Known Limitations

   - **cmd.exe not supported** - Use PowerShell
   - **WSL uses Unix scripts** - Not Windows integration
   - **Requires PowerShell 5.1+** - Windows 10+ only

   ## Performance

   Windows performance is similar to Unix:
   - Activation: <150ms (typical)
   - Idempotency check: <10ms
   - JSON parsing overhead: <5ms

   ## Examples

   ### Basic Usage
   ```powershell
   # Create a project
   mkdir C:\projects\myapp
   cd C:\projects\myapp

   # Create .nvmrc
   "18.0.0" | Out-File -Encoding ASCII .nvmrc

   # Install version (if needed)
   nvm install 18.0.0

   # cd triggers activation
   cd .

   # Verify
   node --version  # v18.0.0
   $env:NODE_VERSION  # 18.0.0
   ```

   ### Debug Mode
   ```powershell
   $env:XVN_DEBUG = 1
   cd C:\projects\myapp

   # Output:
   # xvn: Found version file: C:\projects\myapp\.nvmrc
   # xvn: Activating for C:\projects\myapp\.nvmrc
   # xvn: Raw output from binary: ...
   # xvn: Executing 2 commands
   # xvn: $env:NODE_VERSION = "18.0.0"
   # xvn: $env:PATH = "C:\Users\...\nvm\v18.0.0;" + $env:PATH
   ```
   ```

3. **Update ARCHITECTURE.md** with Windows section:
   ```markdown
   # Architecture

   ## Cross-Platform Design

   xvn supports both Unix (macOS, Linux) and Windows platforms with a shared core and platform-specific shell integration.

   ### Shared Components

   - **Rust core** - Version detection, plugin system, configuration (100% shared)
   - **Plugin trait** - `VersionManagerPlugin` interface (identical across platforms)
   - **Config system** - YAML parsing and merging (identical)

   ### Platform-Specific Components

   | Component | Unix | Windows |
   |-----------|------|---------|
   | **Shell Script** | `xvn.sh` (bash/zsh) | `xvn.ps1` (PowerShell) |
   | **Hook Mechanism** | `chpwd_functions`, `cd` wrapper | `prompt` function override |
   | **Command Protocol** | File descriptor #3 | JSON output parsing |
   | **Environment** | `export VAR=value` | `$env:VAR = "value"` |
   | **Profile** | `~/.bashrc`, `~/.zshrc` | `$PROFILE` |

   ### Command Protocols

   #### Unix: File Descriptor #3

   ```bash
   eval "$(xvn activate "$PWD" 3>&1 1>&2 2>&3 3>&-)"
   ```

   Commands written to FD:3 are captured by the shell.

   #### Windows: JSON Protocol

   ```powershell
   $output = xvn activate $PWD
   if ($output -match '__XVN_COMMANDS_START__(.*)__XVN_COMMANDS_END__') {
       $json = $matches[1] | ConvertFrom-Json
       foreach ($cmd in $json.commands) {
           Invoke-Expression $cmd
       }
   }
   ```

   JSON commands are parsed and executed by PowerShell.

   ### Why Different Protocols?

   - Windows doesn't support FD:3 in the same way
   - JSON is more robust for PowerShell parsing
   - Platform detection is automatic
   - Both protocols achieve the same result
   ```

4. **Add Windows examples to usage docs**:
   ```markdown
   ## Examples

   ### Windows PowerShell

   ```powershell
   # Setup
   npm install -g @olvrcc/xvn
   xvn init
   . $PROFILE

   # Create project
   mkdir C:\projects\myapp
   cd C:\projects\myapp
   "18.0.0" | Out-File -Encoding ASCII .nvmrc

   # Install Node version
   nvm install 18.0.0

   # Automatic activation on cd
   cd .  # Triggers activation
   node --version  # v18.0.0

   # Debug mode
   $env:XVN_DEBUG = 1
   cd ..
   cd myapp  # Shows debug output
   ```
   ```

**Code Structure:**
- `README.md` - Updated with Windows installation
- `docs/WINDOWS.md` - Windows-specific guide (new)
- `docs/ARCHITECTURE.md` - Cross-platform architecture
- `docs/TROUBLESHOOTING.md` - Windows troubleshooting section

**Key Considerations:**
- **Clear platform differences** - Explain why Windows is different
- **Troubleshooting first** - Most users will hit execution policy issues
- **Examples with paths** - Use realistic Windows paths (C:\, etc.)
- **nvm-windows vs nvm** - Clarify they're different tools

**Testing:**
```powershell
# Verify documentation accuracy
# Follow Windows installation steps exactly
# Ensure all examples work
```

**Dependencies:**
- Requires: M11.8 (testing validates documentation)

**Enables:**
- M11.10 (release - docs must be ready)

---

### Task M11.10: Release & Distribution

**Objective:** Build all platform binaries, test Windows package, and publish v1.3.0 with Windows support.

**Implementation Steps:**

1. **Update version to v1.3.0:**
   ```bash
   # Update Cargo.toml
   sed -i '' 's/^version = .*/version = "1.3.0"/' Cargo.toml

   # Update package.json
   npm version 1.3.0 --no-git-tag-version
   ```

2. **Update CHANGELOG.md:**
   ```markdown
   # Changelog

   ## [1.3.0] - 2025-10-XX

   ### Added
   - ✨ Windows and PowerShell support
   - ✨ JSON command protocol for PowerShell integration
   - ✨ PowerShell hook script (xvn.ps1)
   - ✨ Windows binary compilation (x64, ARM64)
   - ✨ nvm-windows plugin support
   - ✨ Cross-platform path handling utilities
   - 📖 Windows installation and troubleshooting documentation

   ### Changed
   - 🔧 Plugin system now detects Windows version managers
   - 🔧 Config loader handles Windows paths and environment variables
   - 🔧 Setup command creates PowerShell profile modifications

   ### Fixed
   - 🐛 Path handling now works correctly on Windows
   - 🐛 Environment variable expansion supports Windows syntax

   ### Platform Support
   - ✅ macOS (x64, ARM64)
   - ✅ Linux (x64, ARM64)
   - ✅ Windows (x64, ARM64) - **NEW**

   ### Breaking Changes
   - None

   ## [1.2.0] - 2025-09-XX
   ...
   ```

3. **Build all platform binaries:**
   ```bash
   # Linux
   cargo build --release --target x86_64-unknown-linux-gnu
   cargo build --release --target aarch64-unknown-linux-gnu

   # macOS
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target aarch64-apple-darwin

   # Windows (on Windows or via cross-compilation)
   cargo build --release --target x86_64-pc-windows-msvc
   cargo build --release --target aarch64-pc-windows-msvc

   # Copy to native/ directory for npm package
   mkdir -p native/{x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu,x86_64-apple-darwin,aarch64-apple-darwin,x86_64-pc-windows-msvc,aarch64-pc-windows-msvc}

   cp target/x86_64-unknown-linux-gnu/release/xvn native/x86_64-unknown-linux-gnu/
   cp target/aarch64-unknown-linux-gnu/release/xvn native/aarch64-unknown-linux-gnu/
   cp target/x86_64-apple-darwin/release/xvn native/x86_64-apple-darwin/
   cp target/aarch64-apple-darwin/release/xvn native/aarch64-apple-darwin/
   cp target/x86_64-pc-windows-msvc/release/xvn.exe native/x86_64-pc-windows-msvc/
   cp target/aarch64-pc-windows-msvc/release/xvn.exe native/aarch64-pc-windows-msvc/
   ```

4. **Test npm package on Windows:**
   ```powershell
   # Pack the package
   npm pack

   # Install locally
   npm install -g .\olvrcc-xvn-1.3.0.tgz

   # Test installation
   xvn --version  # Should show 1.3.0

   # Test setup
   xvn init --quick

   # Verify profile
   Get-Content $PROFILE | Select-String "xvn"

   # Test activation (with nvm-windows installed)
   mkdir C:\temp\test-xvn
   cd C:\temp\test-xvn
   "18.0.0" | Out-File -Encoding ASCII .nvmrc
   cd .  # Should activate

   # Verify
   $env:NODE_VERSION  # Should be 18.0.0
   node --version     # Should be v18.0.0

   # Cleanup
   npm uninstall -g @olvrcc/xvn
   Remove-Item -Recurse C:\temp\test-xvn
   ```

5. **Create GitHub release with artifacts:**
   ```bash
   # Tag the release
   git tag -a v1.3.0 -m "Release v1.3.0: Windows & PowerShell support"
   git push origin v1.3.0

   # GitHub Actions will build and upload artifacts
   # Or manually create release:
   gh release create v1.3.0 \
     --title "v1.3.0 - Windows & PowerShell Support" \
     --notes-file RELEASE_NOTES.md \
     target/x86_64-unknown-linux-gnu/release/xvn#xvn-x86_64-linux \
     target/aarch64-unknown-linux-gnu/release/xvn#xvn-aarch64-linux \
     target/x86_64-apple-darwin/release/xvn#xvn-x86_64-macos \
     target/aarch64-apple-darwin/release/xvn#xvn-aarch64-macos \
     target/x86_64-pc-windows-msvc/release/xvn.exe#xvn-x86_64-windows.exe \
     target/aarch64-pc-windows-msvc/release/xvn.exe#xvn-aarch64-windows.exe
   ```

6. **Publish to npm:**
   ```bash
   # Verify package contents
   npm pack --dry-run

   # Publish
   npm publish

   # Verify installation
   npm install -g @olvrcc/xvn@1.3.0
   xvn --version
   ```

7. **Create release notes** in `RELEASE_NOTES.md`:
   ```markdown
   # xvn v1.3.0 - Windows & PowerShell Support 🪟

   We're excited to announce Windows and PowerShell support for xvn! Automatic Node.js version switching now works seamlessly on Windows 10/11 with PowerShell 5.1 or PowerShell Core 7+.

   ## ✨ New Features

   ### Windows Platform Support
   - ✅ Windows 10 & 11 (x64, ARM64)
   - ✅ PowerShell 5.1 and PowerShell Core 7+
   - ✅ nvm-windows and fnm support
   - ✅ JSON-based command protocol
   - ✅ PowerShell profile integration

   ### Installation (Windows)

   ```powershell
   npm install -g @olvrcc/xvn
   xvn init
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   . $PROFILE
   ```

   ### What's New

   - **PowerShell Integration**: Automatic version switching via prompt override
   - **JSON Protocol**: Robust command execution for PowerShell
   - **Cross-Platform Paths**: Handles Windows and Unix paths correctly
   - **Windows Docs**: Comprehensive Windows installation and troubleshooting guide

   ## 📦 Binaries

   Download pre-compiled binaries for your platform:

   - **Linux**: `xvn-x86_64-linux`, `xvn-aarch64-linux`
   - **macOS**: `xvn-x86_64-macos`, `xvn-aarch64-macos`
   - **Windows**: `xvn-x86_64-windows.exe`, `xvn-aarch64-windows.exe`

   ## 📖 Documentation

   - [Windows Installation Guide](docs/WINDOWS.md)
   - [Troubleshooting](docs/WINDOWS.md#troubleshooting)
   - [Architecture](docs/ARCHITECTURE.md#cross-platform-design)

   ## 🙏 Acknowledgments

   Thanks to the nvm-windows team for making this integration possible!

   ## 🐛 Known Issues

   - cmd.exe is not supported (use PowerShell)
   - Windows Subsystem for Linux (WSL) should use Unix scripts

   ## 📝 Full Changelog

   See [CHANGELOG.md](CHANGELOG.md) for complete details.
   ```

8. **Announce release:**
   ```markdown
   # Social Media / Community Announcement

   🎉 xvn v1.3.0 is here with Windows & PowerShell support! 🪟

   Automatic Node.js version switching now works on Windows 10/11 with PowerShell.

   ✨ Features:
   - PowerShell integration
   - nvm-windows support
   - Cross-platform path handling
   - Comprehensive docs

   Install: npm install -g @olvrcc/xvn

   Docs: https://github.com/cameronolivier/xvn

   #nodejs #windows #powershell #devtools
   ```

**Code Structure:**
- `Cargo.toml` - Version 1.3.0
- `package.json` - Version 1.3.0
- `CHANGELOG.md` - Release notes
- `RELEASE_NOTES.md` - GitHub release description
- `native/` - All platform binaries

**Key Considerations:**
- **All platforms**: Build and test all binaries
- **npm package**: Must include all binaries in `native/`
- **GitHub release**: Upload individual binary artifacts
- **Testing**: Windows package must be tested before publish
- **Announcement**: Clear communication of new feature

**Testing:**
```bash
# Pre-release checklist
# □ All binaries compiled
# □ npm pack shows correct files
# □ Windows install tested
# □ Version activation tested
# □ Documentation reviewed
# □ CHANGELOG updated
# □ Release notes written
```

**Dependencies:**
- Requires: All M11.1-M11.9 complete

**Enables:**
- Full cross-platform xvn deployment
- Windows user adoption

---

## Integration Points

### How Tasks Work Together

```
┌─────────────────────────────────────────────────────────────┐
│                   Windows Support Flow                       │
└─────────────────────────────────────────────────────────────┘

M11.1 (Binary Setup)
   │
   ├──> Enables compilation for Windows
   │
   └──> M11.7 (npm Package)
           │
           └──> Distributes Windows binaries

M11.2 (PowerShell Script)
   │
   ├──> Detects directory changes (no chpwd on Windows)
   │
   └──> M11.3 (JSON Protocol)
           │
           ├──> Communicates with Rust binary
           │
           └──> M11.5 (Profile Modification)
                   │
                   └──> Installs and configures

M11.4 (Plugin Updates)
   │
   ├──> Detects Windows version managers
   │
   └──> M11.6 (Path Handling)
           │
           └──> Handles Windows paths correctly

M11.8 (Testing)
   │
   ├──> Validates all components
   │
   └──> M11.9 (Documentation)
           │
           └──> M11.10 (Release)
```

### Critical Dependencies

1. **PowerShell script (M11.2) depends on JSON protocol (M11.3)**
   - Script must parse JSON from binary
   - Protocol must be implemented before script can be tested

2. **Plugin system (M11.4) depends on path handling (M11.6)**
   - Plugins need path utilities for Windows
   - Path handling must support Windows conventions

3. **Testing (M11.8) depends on all implementation tasks**
   - Can't test until components are built
   - Integration tests verify everything works together

4. **Release (M11.10) depends on testing (M11.8) and docs (M11.9)**
   - Must pass all tests before release
   - Documentation must be complete

## Testing Strategy

### Unit Tests
- Path handling utilities (Windows-specific)
- JSON protocol serialization/deserialization
- PowerShell command escaping
- Plugin detection on Windows

### Integration Tests
- PowerShell script end-to-end
- JSON protocol communication
- Version activation flow
- Profile modification

### Manual Testing
- Windows 10 x64
- Windows 11 x64
- Windows 11 ARM64 (if available)
- PowerShell 5.1
- PowerShell Core 7+
- Windows Terminal
- VS Code terminal

### CI/CD Testing
- PSScriptAnalyzer for PowerShell
- Rust tests on `windows-latest`
- Cross-platform test matrix
- Binary size checks

## Success Criteria

### Functional Requirements
- ✅ Windows binary compiles for x64 and ARM64
- ✅ `xvn init` creates PowerShell profile modifications
- ✅ PowerShell hook activates on directory change
- ✅ Version switching works with nvm-windows and fnm
- ✅ Idempotency prevents unnecessary re-activation
- ✅ Error handling doesn't break shell

### Quality Requirements
- ✅ All tests pass on Windows platform
- ✅ PSScriptAnalyzer shows no issues
- ✅ Documentation covers Windows installation
- ✅ npm package installs correctly on Windows
- ✅ Performance meets targets (<150ms activation)

### Release Requirements
- ✅ v1.3.0 published to npm
- ✅ Windows binaries in GitHub release
- ✅ CHANGELOG updated with Windows features
- ✅ README includes Windows instructions
- ✅ Manual testing completed on real hardware

### User Experience Requirements
- ✅ Clear installation instructions for Windows
- ✅ Helpful error messages for common issues
- ✅ Execution policy guidance included
- ✅ Troubleshooting guide comprehensive
- ✅ Examples use Windows paths and syntax

## Performance Targets

### Windows-Specific
- Activation time: <150ms (P50), <200ms (P95)
- PowerShell parsing overhead: <10ms
- JSON serialization: <5ms
- Path normalization: <1ms

### Binary Size
- Windows x64: <6MB compressed
- Windows ARM64: <6MB compressed
- npm package total: <30MB (all platforms)

## Known Limitations

### Phase 1 Constraints
1. **PowerShell Only** - No cmd.exe support
2. **No WSL Integration** - WSL should use Unix scripts
3. **Requires PowerShell 5.1+** - Windows 10+ only
4. **nvm-windows differences** - Not identical to Unix nvm

### Future Enhancements
- cmd.exe support (low priority)
- Windows Terminal deep integration
- PowerShell Gallery distribution
- Windows-specific optimizations

## Risk Mitigation

### Technical Risks
- **Risk**: PowerShell execution policy blocks scripts
  - **Mitigation**: Clear documentation, setup warns user

- **Risk**: JSON parsing fails or is slow
  - **Mitigation**: Extensive testing, fallback to simpler format

- **Risk**: nvm-windows behaves differently than nvm
  - **Mitigation**: Test coverage for nvm-windows specifics

### Release Risks
- **Risk**: Windows binaries don't work on some systems
  - **Mitigation**: Test on multiple Windows versions, provide debug mode

- **Risk**: npm package installation fails on Windows
  - **Mitigation**: Test install.js thoroughly, add error handling

## Timeline

### Week 1: Foundation
- **Days 1-2**: M11.1 (Binary compilation)
- **Days 3-4**: M11.2 (PowerShell script)
- **Day 5**: M11.3 (JSON protocol)

### Week 2: Integration
- **Days 1-2**: M11.4 (Plugin updates)
- **Days 3-4**: M11.5 (Profile modification)
- **Day 5**: M11.6 (Path handling)

### Week 3: Quality & Release
- **Days 1-2**: M11.7 (npm package)
- **Days 3-4**: M11.8 (Testing)
- **Day 5**: M11.9 (Documentation)
- **Weekend**: M11.10 (Release)

**Total**: 2-3 weeks depending on testing availability

---

**Last Updated:** October 4, 2025
**Status:** Ready for implementation
**Next Steps:** Begin M11.1 (Windows binary compilation setup)
