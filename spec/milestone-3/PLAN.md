# Milestone 3: Shell Integration - Implementation Plan

**Timeline:** Weeks 5-6
**Status:** Planning
**Version:** v0.3.0

---

## Overview

This milestone implements the critical shell integration layer that enables xvn to automatically activate Node.js versions when changing directories. The shell integration consists of three main components:

1. **xvn.sh** - A shell script that hooks into directory change events
2. **FD:3 Protocol** - Inter-process communication mechanism for the Rust binary to send commands to the parent shell
3. **Setup Command** - Installation tool that configures the user's shell environment

By the end of this milestone, users will be able to run `xvn setup`, have their shell automatically configured, and experience seamless version switching when navigating between projects.

### Goals

- Shell hook script (xvn.sh) that detects directory changes and triggers version activation
- File descriptor #3 protocol for parent shell environment modification
- Idempotency checks to prevent redundant activations
- `xvn setup` command that safely modifies shell profiles
- Support for both bash and zsh
- Comprehensive testing including shell script validation

### Approach

- **Shell-agnostic design**: Core functionality works in both bash and zsh with shell-specific adapters
- **Safety first**: Never break the user's shell; graceful degradation on errors
- **Idempotency**: Multiple setups don't cause issues; activations only happen when needed
- **Performance**: Minimize overhead in directory change hooks (<100ms activation time)
- **Testability**: Shell scripts validated with shellcheck; Rust code unit tested with mocked FD:3

---

## Prerequisites

- Milestone 1 (Core Infrastructure) must be complete
- Milestone 2 (Plugin System) must be complete
- shellcheck installed for shell script validation: `brew install shellcheck` (macOS) or `apt-get install shellcheck` (Linux)
- Basic understanding of bash/zsh hooks and file descriptors

---

## FD:3 Protocol Deep Dive

The file descriptor #3 protocol is the core inter-process communication mechanism that allows xvn (the child process) to modify the parent shell's environment. This section explains how it works and why it's necessary.

### The Problem

When a child process changes environment variables (e.g., `export PATH=...`), those changes only affect the child process and its descendants. The parent shell is not affected. This is a fundamental Unix process isolation principle.

**Example:**
```bash
$ echo $NODE_VERSION
# (empty)

$ bash -c 'export NODE_VERSION=18.20.0; echo $NODE_VERSION'
18.20.0

$ echo $NODE_VERSION
# (still empty - parent shell not affected)
```

### The Solution

We use file descriptor #3 as a communication channel:

1. **Shell opens FD:3** and runs: `xvn activate /path 3>&1 1>&2 2>&3`
2. **xvn detects FD:3** is open and writes shell commands to it
3. **Shell captures FD:3** output: `commands=$(xvn activate ... 3>&1 1>&2 2>&3)`
4. **Shell evaluates commands**: `eval "$commands"`
5. **Commands run in parent shell's context**, modifying its environment

### Redirection Magic: `3>&1 1>&2 2>&3`

This complex redirection achieves a three-way swap:
- `3>&1` - FD:3 output goes to current stdout (captured by `$()`)
- `1>&2` - Regular stdout goes to stderr (visible to user, not captured)
- `2>&3` - stderr goes to original stdout (visible to user, not captured)

**Net effect**: Only FD:3 is captured in `$commands`; stdout and stderr remain visible.

### Implementation Details

- **Shell side**: See `__xvn_activate()` in xvn.sh (Task M3.1)
- **Rust side**: See `CommandWriter` in src/shell/fd3.rs (Task M3.2)
- **Security**: Commands must be carefully constructed to prevent injection attacks
- **Testing**: Use `dup2()` to create test FD:3 in unit tests

### Alternative Approaches (Not Used)

- **Source file**: Would require creating temp files and `source` command
- **Shell wrapper**: Would require wrapping the entire shell, complex setup
- **Environment file**: Limited to env vars, can't run arbitrary commands

---

## Implementation Tasks

### Task M3.1: Create xvn.sh shell hook script

**Objective:** Implement the shell script that integrates with bash/zsh, detects directory changes, and triggers version activation.

**Implementation Steps:**

1. **Create the shell directory and base script file:**
   ```bash
   mkdir -p shell
   touch shell/xvn.sh
   chmod +x shell/xvn.sh
   ```

2. **Implement the shell script header with error handling:**
   ```bash
   #!/usr/bin/env bash
   # xvn.sh - Shell integration for xvn (Extreme Version Switcher)
   # This script hooks into directory change events and activates Node.js versions

   # Error handling
   # NOTE: Do NOT use 'set -e' in sourced scripts as it will terminate the user's shell
   # on any error. Instead, handle errors explicitly in functions.
   set -u  # Treat unset variables as errors

   # Prevent multiple initialization
   if [[ -n "${XVN_SHELL_LOADED:-}" ]]; then
       return 0
   fi
   export XVN_SHELL_LOADED=1
   ```

3. **Implement `__xvn_debug` function for optional debug output:**
   ```bash
   # Debug logging (enabled via XVN_DEBUG=1)
   __xvn_debug() {
       if [[ "${XVN_DEBUG:-0}" == "1" ]]; then
           echo "[xvn] $*" >&2
       fi
   }
   ```

4. **Implement `__xvn_find_file` function to search up the directory tree:**
   ```bash
   # Find version file by walking up directory tree
   # Usage: __xvn_find_file <start_path>
   # Returns: Full path to version file, or empty string if not found
   __xvn_find_file() {
       local current_dir="${1:-$PWD}"
       local search_files="${XVN_VERSION_FILES:-.nvmrc .node-version}"

       __xvn_debug "Searching for version files: $search_files"

       while [[ "$current_dir" != "/" ]]; do
           for filename in $search_files; do
               local filepath="$current_dir/$filename"
               if [[ -f "$filepath" ]]; then
                   __xvn_debug "Found version file: $filepath"
                   echo "$filepath"
                   return 0
               fi
           done
           current_dir="$(dirname "$current_dir")"
       done

       __xvn_debug "No version file found"
       return 1
   }
   ```

5. **Implement `__xvn_activate` function to trigger activation:**
   ```bash
   # Activate version for a given path
   # Usage: __xvn_activate <version_file_path>
   __xvn_activate() {
       local version_file="$1"

       # Check if already activated for this file
       if [[ "${XVN_ACTIVE_FILE:-}" == "$version_file" ]]; then
           __xvn_debug "Already activated for $version_file, skipping"
           return 0
       fi

       __xvn_debug "Activating version from $version_file"

       # Call xvn binary with FD:3 protocol
       # FD:3 Protocol redirection explanation:
       # We want to capture only FD:3 output in $commands, while keeping stdout/stderr visible.
       #
       # Redirection breakdown:
       #   3>&1  - FD:3 output goes to current stdout (captured by $())
       #   1>&2  - Regular stdout goes to stderr (visible to user)
       #   2>&3  - stderr goes to FD:3's original target (which becomes stdout via 3>&1)
       #
       # Net effect: Only FD:3 is captured; stdout and stderr are swapped but both visible.
       local commands
       commands=$(xvn activate "$(dirname "$version_file")" 3>&1 1>&2 2>&3) || {
           # Activation failed, but don't break the shell
           __xvn_debug "Activation failed (exit code $?)"
           return 1
       }

       if [[ -n "$commands" ]]; then
           __xvn_debug "Evaluating commands: $commands"
           eval "$commands"
           export XVN_ACTIVE_FILE="$version_file"
       else
           __xvn_debug "No commands returned"
       fi
   }
   ```

6. **Implement `__xvn_chpwd` function (main directory change hook):**
   ```bash
   # Main hook function called on directory change
   __xvn_chpwd() {
       __xvn_debug "Directory changed to: $PWD"

       local version_file
       if version_file=$(__xvn_find_file "$PWD"); then
           __xvn_activate "$version_file"
       else
           # No version file found, clear active file
           if [[ -n "${XVN_ACTIVE_FILE:-}" ]]; then
               __xvn_debug "Left project directory, clearing active file"
               unset XVN_ACTIVE_FILE
           fi
       fi
   }
   ```

7. **Implement bash-specific integration:**
   ```bash
   # Bash-specific integration
   if [[ -n "${BASH_VERSION:-}" ]]; then
       __xvn_debug "Detected bash shell"

       # Bash doesn't have native chpwd support, so we wrap cd, pushd, popd
       if ! declare -f __xvn_original_cd > /dev/null; then
           # Only wrap once - store original builtin as function
           __xvn_original_cd() { builtin cd "$@"; }

           cd() {
               __xvn_original_cd "$@" || return $?
               __xvn_chpwd
           }

           # Also wrap pushd and popd if they exist
           if declare -f pushd > /dev/null 2>&1 || command -v pushd > /dev/null 2>&1; then
               __xvn_original_pushd() { builtin pushd "$@"; }
               pushd() {
                   __xvn_original_pushd "$@" || return $?
                   __xvn_chpwd
               }
           fi

           if declare -f popd > /dev/null 2>&1 || command -v popd > /dev/null 2>&1; then
               __xvn_original_popd() { builtin popd "$@"; }
               popd() {
                   __xvn_original_popd "$@" || return $?
                   __xvn_chpwd
               }
           fi
       fi

       # Trigger on shell startup
       __xvn_chpwd
   fi
   ```

8. **Implement zsh-specific integration:**
   ```bash
   # Zsh-specific integration
   if [[ -n "${ZSH_VERSION:-}" ]]; then
       __xvn_debug "Detected zsh shell"

       # Zsh has native chpwd_functions support
       if [[ -z "${chpwd_functions[(r)__xvn_chpwd]}" ]]; then
           chpwd_functions+=(__xvn_chpwd)
       fi

       # Trigger on shell startup
       __xvn_chpwd
   fi
   ```

**Code Structure:**
- File: `shell/xvn.sh`
  - Shell hook script with all functions
  - Bash and zsh compatibility
  - Debug mode support
  - FD:3 protocol integration

**Key Considerations:**
- **Idempotency**: Check `XVN_SHELL_LOADED` to prevent multiple initialization
- **Error handling**: Use `set -eo pipefail` but handle errors in `__xvn_activate` to prevent shell breakage
- **Performance**: Cache `XVN_ACTIVE_FILE` to avoid redundant activations
- **Shell detection**: Use `$BASH_VERSION` and `$ZSH_VERSION` to detect shell type
- **FD:3 protocol**: The magic `3>&1 1>&2 2>&3` redirects FD:3 to stdout and swaps stdout/stderr

**Testing:**
- Validate syntax with shellcheck: `shellcheck shell/xvn.sh`
- Test in bash: `bash -c 'source shell/xvn.sh; __xvn_find_file /some/path'`
- Test in zsh: `zsh -c 'source shell/xvn.sh; __xvn_find_file /some/path'`

**Dependencies:**
- None (first task)

**Enables:**
- M3.2 (FD:3 protocol in Rust depends on shell script calling `xvn activate`)
- M3.3 (Setup command needs xvn.sh to copy)

---

### Task M3.2: Implement file descriptor #3 protocol in Rust

**Objective:** Implement the Rust side of the FD:3 protocol, allowing the `xvn activate` command to write shell commands to file descriptor 3, which the parent shell can then execute.

**Implementation Steps:**

1. **Create the shell module structure:**
   ```bash
   mkdir -p src/shell
   touch src/shell/mod.rs
   touch src/shell/fd3.rs
   ```

2. **Add shell module to `src/lib.rs`:**
   ```rust
   // In src/lib.rs
   pub mod shell;
   pub use shell::CommandWriter;
   ```

3. **Implement the `CommandWriter` struct in `src/shell/fd3.rs`:**
   ```rust
   use std::io::{self, Write};
   use std::os::unix::io::RawFd;
   use log::{debug, trace};

   /// Writer for shell commands via file descriptor #3
   ///
   /// This struct handles the file descriptor #3 protocol, which allows
   /// a child process (xvn) to send commands to the parent shell for execution.
   ///
   /// # Protocol
   /// - The shell opens FD:3 when invoking xvn: `xvn activate <path> 3>&1 1>&2 2>&3`
   /// - xvn writes commands to FD:3
   /// - The shell captures FD:3 output and evaluates it: `eval "$commands"`
   ///
   /// # Safety
   /// Commands must be properly escaped to prevent command injection.
   /// FD:3 is owned by the parent shell, so we use raw writes without taking ownership.
   pub struct CommandWriter {
       fd: Option<RawFd>,
   }

   impl CommandWriter {
       /// Creates a new CommandWriter
       ///
       /// Attempts to open file descriptor 3. If FD:3 is not available,
       /// returns a CommandWriter that silently discards writes.
       ///
       /// # Returns
       /// Always returns Ok - a non-functional writer if FD:3 unavailable
       pub fn new() -> io::Result<Self> {
           const FD3: RawFd = 3;

           // Check if FD:3 is open by attempting to get file status
           let fd = if Self::is_fd_open(FD3) {
               debug!("File descriptor 3 is available");
               Some(FD3)
           } else {
               debug!("File descriptor 3 is not available, commands will be discarded");
               None
           };

           Ok(Self { fd })
       }

       /// Checks if a file descriptor is open
       ///
       /// Uses fcntl to check if the FD is valid without modifying it.
       fn is_fd_open(fd: RawFd) -> bool {
           // Try to get file status flags
           // If fcntl succeeds, the FD is open
           unsafe {
               libc::fcntl(fd, libc::F_GETFD) != -1
           }
       }

       /// Writes a shell command to FD:3
       ///
       /// The command will be executed by the parent shell. Multiple commands
       /// can be written; they will be concatenated with newlines.
       ///
       /// # Arguments
       /// * `command` - The shell command to write (must not contain newlines)
       ///
       /// # Security
       /// The caller must ensure the command is properly escaped to prevent
       /// command injection vulnerabilities.
       ///
       /// # Returns
       /// - Ok(()) if write successful or FD:3 not available
       /// - Err(io::Error) if write failed
       pub fn write_command(&mut self, command: &str) -> io::Result<()> {
           trace!("Writing command to FD:3: {}", command);

           if let Some(fd) = self.fd {
               let data = format!("{}\n", command);
               unsafe {
                   let bytes_written = libc::write(
                       fd,
                       data.as_ptr() as *const libc::c_void,
                       data.len(),
                   );
                   if bytes_written == -1 {
                       return Err(io::Error::last_os_error());
                   }
               }
           } else {
               trace!("FD:3 not available, command discarded");
           }

           Ok(())
       }

       /// Checks if FD:3 is available
       ///
       /// # Returns
       /// true if commands will be written to FD:3, false if they'll be discarded
       pub fn is_available(&self) -> bool {
           self.fd.is_some()
       }
   }

   impl Default for CommandWriter {
       fn default() -> Self {
           Self::new().expect("CommandWriter::new should never fail")
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       use std::os::unix::io::IntoRawFd;
       use tempfile::NamedTempFile;

       #[test]
       fn test_command_writer_without_fd3() {
           // FD:3 not open in test environment
           let writer = CommandWriter::new().unwrap();
           assert!(!writer.is_available());
       }

       #[test]
       fn test_command_writer_with_fd3() {
           // Create a temporary file and assign it to FD:3
           let temp = NamedTempFile::new().unwrap();
           let fd = temp.as_file().as_raw_fd();

           // Duplicate to FD:3
           unsafe {
               libc::dup2(fd, 3);
           }

           let mut writer = CommandWriter::new().unwrap();
           assert!(writer.is_available());

           // Write a command
           writer.write_command("echo test").unwrap();

           // Clean up
           unsafe {
               libc::close(3);
           }
       }

       #[test]
       fn test_write_command_without_fd3() {
           let mut writer = CommandWriter::new().unwrap();
           // Should not panic even if FD:3 not available
           assert!(writer.write_command("echo test").is_ok());
       }
   }
   ```

4. **Create the module export in `src/shell/mod.rs`:**
   ```rust
   mod fd3;

   pub use fd3::CommandWriter;
   ```

5. **Integrate CommandWriter into the activate command in `src/cli.rs`:**
   ```rust
   // In src/cli.rs, in the activate command handler
   use crate::shell::CommandWriter;
   use log::{debug, info};

   fn handle_activate(path: Option<String>) -> anyhow::Result<()> {
       let path = path.as_deref().unwrap_or(".");
       debug!("Activating version for path: {}", path);

       // Open FD:3 for writing commands
       let mut fd3 = CommandWriter::new()?;

       if !fd3.is_available() {
           debug!("FD:3 not available, commands will not be written");
       }

       // TODO: Actual activation logic will be implemented in Milestone 4
       // For now, we'll write a dummy command to test the protocol

       // Example: Write a command to FD:3
       let test_command = "export XVN_TEST=activated";
       fd3.write_command(test_command)?;
       info!("Activation complete");

       Ok(())
   }
   ```

6. **Add libc dependency to `Cargo.toml`:**
   ```toml
   [dependencies]
   # ... existing dependencies ...
   libc = "0.2"
   ```

7. **Build and test:**
   ```bash
   cargo build
   cargo test shell::fd3
   ```

**Code Structure:**
- File: `src/shell/mod.rs`
  - Module exports
- File: `src/shell/fd3.rs`
  - CommandWriter struct
  - FD:3 detection and writing logic
  - Unit tests
- File: `src/cli.rs` (modified)
  - Integration in activate command

**Key Considerations:**
- **Safety**: Using `from_raw_fd` is unsafe; must verify FD is open first with `fcntl`
- **Graceful degradation**: If FD:3 not available, commands are silently discarded (enables testing without shell integration)
- **Command security**: The CommandWriter does NOT escape commands; that's the caller's responsibility
- **Flushing**: Must flush after each write to ensure commands are available to shell immediately
- **Error handling**: Return io::Result for write operations but never panic

**Testing:**
- Unit tests with mocked FD:3 (duplicate temp file to FD:3)
- Test `is_available()` returns false when FD:3 not open
- Test `write_command()` succeeds even without FD:3
- Integration test: Run `xvn activate` from shell with `3>&1` and verify output

**Dependencies:**
- Requires: M3.1 (shell script must call `xvn activate` with FD:3 protocol)

**Enables:**
- M3.3 (setup command can now explain how activation works)
- M3.4 (integration tests can test FD:3 protocol)

---

### Task M3.3: Implement `xvn setup` command

**Objective:** Implement the `xvn setup` command that configures the user's shell environment by copying xvn.sh, modifying shell profiles, and creating default configuration.

**Implementation Steps:**

1. **Create the setup module structure:**
   ```bash
   mkdir -p src/setup
   touch src/setup/mod.rs
   touch src/setup/shell_detection.rs
   touch src/setup/profile_modification.rs
   touch src/setup/installer.rs
   ```

2. **Add setup module to `src/lib.rs`:**
   ```rust
   // In src/lib.rs
   pub mod setup;
   ```

3. **Implement shell detection in `src/setup/shell_detection.rs`:**
   ```rust
   use std::env;
   use std::path::{Path, PathBuf};
   use anyhow::{Result, Context};
   use log::debug;

   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum Shell {
       Bash,
       Zsh,
   }

   impl Shell {
       /// Detects the current shell from environment variables
       pub fn detect() -> Result<Self> {
           // Try $SHELL first
           if let Ok(shell_path) = env::var("SHELL") {
               debug!("Detected shell from $SHELL: {}", shell_path);
               return Self::from_path(&shell_path);
           }

           // Fallback to parent process (not implemented in MVP)
           anyhow::bail!("Could not detect shell. Please set $SHELL environment variable.");
       }

       /// Determines shell from path
       fn from_path(path: &str) -> Result<Self> {
           let shell_name = Path::new(path)
               .file_name()
               .and_then(|s| s.to_str())
               .context("Invalid shell path")?;

           match shell_name {
               "bash" => Ok(Self::Bash),
               "zsh" => Ok(Self::Zsh),
               _ => anyhow::bail!("Unsupported shell: {}. xvn currently supports bash and zsh.", shell_name),
           }
       }

       /// Returns the name of this shell
       pub fn name(&self) -> &str {
           match self {
               Self::Bash => "bash",
               Self::Zsh => "zsh",
           }
       }

       /// Returns the profile files for this shell in priority order
       ///
       /// The first existing file will be used, or the first in the list
       /// if none exist.
       pub fn profile_files(&self, home: &Path) -> Vec<PathBuf> {
           match self {
               Self::Bash => vec![
                   home.join(".bashrc"),
                   home.join(".bash_profile"),
                   home.join(".profile"),
               ],
               Self::Zsh => vec![
                   home.join(".zshrc"),
                   home.join(".zprofile"),
               ],
           }
       }
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_from_path() {
           assert_eq!(Shell::from_path("/bin/bash").unwrap(), Shell::Bash);
           assert_eq!(Shell::from_path("/usr/local/bin/zsh").unwrap(), Shell::Zsh);
           assert!(Shell::from_path("/bin/fish").is_err());
       }

       #[test]
       fn test_shell_name() {
           assert_eq!(Shell::Bash.name(), "bash");
           assert_eq!(Shell::Zsh.name(), "zsh");
       }
   }
   ```

4. **Implement profile modification in `src/setup/profile_modification.rs`:**
   ```rust
   use std::fs;
   use std::path::{Path, PathBuf};
   use anyhow::{Context, Result};
   use log::{debug, info};

   const XVN_MARKER_START: &str = "# >>> xvn initialize >>>";
   const XVN_MARKER_END: &str = "# <<< xvn initialize <<<";

   /// Checks if a profile file already contains xvn initialization
   pub fn is_already_installed(profile: &Path) -> Result<bool> {
       if !profile.exists() {
           return Ok(false);
       }

       let content = fs::read_to_string(profile)
           .with_context(|| format!("Failed to read profile: {}", profile.display()))?;

       Ok(content.contains(XVN_MARKER_START))
   }

   /// Adds xvn initialization to a profile file
   ///
   /// The initialization block is wrapped in markers for idempotency checks.
   ///
   /// # Arguments
   /// * `profile` - Path to the profile file
   /// * `xvn_sh_path` - Path to the xvn.sh script to source
   pub fn add_to_profile(profile: &Path, xvn_sh_path: &Path) -> Result<()> {
       debug!("Adding xvn to profile: {}", profile.display());

       // Read existing content (or empty if file doesn't exist)
       let mut content = if profile.exists() {
           fs::read_to_string(profile)
               .with_context(|| format!("Failed to read profile: {}", profile.display()))?
       } else {
           String::new()
       };

       // Check if already installed
       if content.contains(XVN_MARKER_START) {
           info!("xvn already installed in {}", profile.display());
           return Ok(());
       }

       // Ensure content ends with newline
       if !content.is_empty() && !content.ends_with('\n') {
           content.push('\n');
       }

       // Add initialization block
       content.push('\n');
       content.push_str(XVN_MARKER_START);
       content.push('\n');
       content.push_str(&format!("# xvn (Extreme Version Switcher) - Automatic Node.js version switching\n"));
       content.push_str(&format!("if [ -s \"{}\" ]; then\n", xvn_sh_path.display()));
       content.push_str(&format!("    source \"{}\"\n", xvn_sh_path.display()));
       content.push_str("fi\n");
       content.push_str(XVN_MARKER_END);
       content.push('\n');

       // Write back
       fs::write(profile, content)
           .with_context(|| format!("Failed to write profile: {}", profile.display()))?;

       info!("Added xvn initialization to {}", profile.display());
       Ok(())
   }

   /// Removes xvn initialization from a profile file
   ///
   /// Used for uninstallation (not in MVP, but defined for completeness)
   pub fn remove_from_profile(profile: &Path) -> Result<()> {
       if !profile.exists() {
           return Ok(());
       }

       let content = fs::read_to_string(profile)
           .with_context(|| format!("Failed to read profile: {}", profile.display()))?;

       if !content.contains(XVN_MARKER_START) {
           return Ok(());
       }

       // Remove everything between markers (including markers)
       let start_idx = content.find(XVN_MARKER_START).unwrap();
       let end_idx = content.find(XVN_MARKER_END)
           .context("Found start marker but not end marker")?;

       // Find the end of the end marker line
       let end_line_end = content[end_idx..].find('\n').unwrap_or(content.len() - end_idx) + end_idx + 1;

       // Also remove preceding newline if present
       let actual_start = if start_idx > 0 && content.as_bytes()[start_idx - 1] == b'\n' {
           start_idx - 1
       } else {
           start_idx
       };

       let new_content = format!("{}{}", &content[..actual_start], &content[end_line_end..]);

       fs::write(profile, new_content)
           .with_context(|| format!("Failed to write profile: {}", profile.display()))?;

       info!("Removed xvn initialization from {}", profile.display());
       Ok(())
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       use tempfile::NamedTempFile;
       use std::io::Write;

       #[test]
       fn test_is_already_installed() {
           let mut temp = NamedTempFile::new().unwrap();
           writeln!(temp, "some content").unwrap();
           writeln!(temp, "{}", XVN_MARKER_START).unwrap();
           writeln!(temp, "source ~/.xvn/bin/xvn.sh").unwrap();
           writeln!(temp, "{}", XVN_MARKER_END).unwrap();
           temp.flush().unwrap();

           assert!(is_already_installed(temp.path()).unwrap());
       }

       #[test]
       fn test_add_to_profile() {
           let temp = NamedTempFile::new().unwrap();
           let xvn_sh = PathBuf::from("/home/user/.xvn/bin/xvn.sh");

           add_to_profile(temp.path(), &xvn_sh).unwrap();

           let content = fs::read_to_string(temp.path()).unwrap();
           assert!(content.contains(XVN_MARKER_START));
           assert!(content.contains(XVN_MARKER_END));
           assert!(content.contains("/home/user/.xvn/bin/xvn.sh"));
       }

       #[test]
       fn test_add_to_profile_idempotent() {
           let temp = NamedTempFile::new().unwrap();
           let xvn_sh = PathBuf::from("/home/user/.xvn/bin/xvn.sh");

           add_to_profile(temp.path(), &xvn_sh).unwrap();
           let content1 = fs::read_to_string(temp.path()).unwrap();

           // Add again
           add_to_profile(temp.path(), &xvn_sh).unwrap();
           let content2 = fs::read_to_string(temp.path()).unwrap();

           // Should be identical
           assert_eq!(content1, content2);
       }
   }
   ```

5. **Implement the setup installer in `src/setup/installer.rs`:**
   ```rust
   use std::fs;
   use std::path::{Path, PathBuf};
   use anyhow::{Context, Result};
   use log::{debug, info, warn};
   use dirs::home_dir;

   use super::shell_detection::Shell;
   use super::profile_modification;

   const XVN_SH_CONTENT: &str = include_str!("../../shell/xvn.sh");

   pub struct SetupInstaller {
       home: PathBuf,
       shell: Shell,
   }

   impl SetupInstaller {
       /// Creates a new SetupInstaller
       pub fn new() -> Result<Self> {
           let home = home_dir().context("Could not determine home directory")?;
           let shell = Shell::detect()?;

           debug!("Detected shell: {}", shell.name());
           debug!("Home directory: {}", home.display());

           Ok(Self { home, shell })
       }

       /// Returns the xvn installation directory (~/.xvn)
       fn xvn_dir(&self) -> PathBuf {
           self.home.join(".xvn")
       }

       /// Returns the xvn bin directory (~/.xvn/bin)
       fn xvn_bin_dir(&self) -> PathBuf {
           self.xvn_dir().join("bin")
       }

       /// Returns the path where xvn.sh will be installed
       fn xvn_sh_path(&self) -> PathBuf {
           self.xvn_bin_dir().join("xvn.sh")
       }

       /// Returns the path to the default config file
       fn config_path(&self) -> PathBuf {
           self.home.join(".xvnrc")
       }

       /// Finds the best profile file to modify
       fn find_profile(&self) -> Result<PathBuf> {
           let candidates = self.shell.profile_files(&self.home);

           // Use first existing file
           for candidate in &candidates {
               if candidate.exists() {
                   debug!("Found existing profile: {}", candidate.display());
                   return Ok(candidate.clone());
               }
           }

           // No existing file, use first candidate
           let default = candidates.first()
               .context("No profile candidates found")?
               .clone();

           warn!("No existing profile found, will create: {}", default.display());
           Ok(default)
       }

       /// Checks if xvn is already installed
       pub fn is_installed(&self) -> Result<bool> {
           let xvn_sh = self.xvn_sh_path();
           if !xvn_sh.exists() {
               return Ok(false);
           }

           let profile = self.find_profile()?;
           profile_modification::is_already_installed(&profile)
       }

       /// Installs xvn shell integration
       pub fn install(&self) -> Result<()> {
           info!("Installing xvn shell integration for {}", self.shell.name());

           // Create directories
           let bin_dir = self.xvn_bin_dir();
           fs::create_dir_all(&bin_dir)
               .with_context(|| format!("Failed to create directory: {}", bin_dir.display()))?;
           info!("Created directory: {}", bin_dir.display());

           // Copy xvn.sh
           let xvn_sh = self.xvn_sh_path();
           fs::write(&xvn_sh, XVN_SH_CONTENT)
               .with_context(|| format!("Failed to write xvn.sh: {}", xvn_sh.display()))?;
           info!("Installed xvn.sh: {}", xvn_sh.display());

           // Modify profile
           let profile = self.find_profile()?;
           profile_modification::add_to_profile(&profile, &xvn_sh)?;

           // Create default config if it doesn't exist
           let config = self.config_path();
           if !config.exists() {
               self.create_default_config()?;
           } else {
               info!("Config file already exists: {}", config.display());
           }

           Ok(())
       }

       /// Creates a default ~/.xvnrc config file
       fn create_default_config(&self) -> Result<()> {
           let config = self.config_path();

           let default_config = r#"# xvn configuration file
# See https://github.com/cameronolivier/xvn for documentation

# Version files to search for (in priority order)
version_files:
  - .nvmrc
  - .node-version

# Plugin priority order
plugins:
  - nvm
  - fnm

# Auto-install mode: prompt (default), always, never
auto_install: prompt
"#;

           fs::write(&config, default_config)
               .with_context(|| format!("Failed to write config: {}", config.display()))?;

           info!("Created default config: {}", config.display());
           Ok(())
       }

       /// Prints setup instructions
       pub fn print_instructions(&self) -> Result<()> {
           let profile = self.find_profile()?;

           println!("\n✅ xvn installation complete!\n");
           println!("Configuration:");
           println!("  Shell:   {}", self.shell.name());
           println!("  Profile: {}", profile.display());
           println!("  Config:  {}", self.config_path().display());
           println!("\nTo start using xvn:");
           println!("  1. Restart your shell, or run:");
           println!("       source {}", profile.display());
           println!("  2. Navigate to a project with a .nvmrc file");
           println!("  3. xvn will automatically activate the correct Node.js version");
           println!("\nDebug mode (if you encounter issues):");
           println!("  export XVN_DEBUG=1");
           println!("\nFor more information:");
           println!("  xvn --help");
           println!();

           Ok(())
       }
   }

   impl Default for SetupInstaller {
       fn default() -> Self {
           Self::new().expect("Failed to create SetupInstaller")
       }
   }
   ```

6. **Create the module export in `src/setup/mod.rs`:**
   ```rust
   mod shell_detection;
   mod profile_modification;
   mod installer;

   pub use shell_detection::Shell;
   pub use installer::SetupInstaller;
   ```

7. **Integrate the setup command in `src/cli.rs`:**
   ```rust
   // In src/cli.rs
   use crate::setup::SetupInstaller;

   fn handle_setup() -> anyhow::Result<()> {
       let installer = SetupInstaller::new()?;

       if installer.is_installed()? {
           println!("xvn is already installed.");
           println!("Run 'xvn status' to verify your installation.");
           return Ok(());
       }

       installer.install()?;
       installer.print_instructions()?;

       Ok(())
   }
   ```

8. **Build and test:**
   ```bash
   cargo build
   cargo test setup
   ```

**Code Structure:**
- File: `src/setup/mod.rs` - Module exports
- File: `src/setup/shell_detection.rs` - Shell detection logic
- File: `src/setup/profile_modification.rs` - Profile file modification
- File: `src/setup/installer.rs` - Main setup orchestration
- File: `src/cli.rs` (modified) - Setup command handler

**Key Considerations:**
- **Idempotency**: Check if already installed before modifying files
- **Safety**: Never overwrite existing profile content; only append
- **Markers**: Use clear markers (`# >>> xvn initialize >>>`) for easy identification
- **Profile detection**: Try multiple profile files in priority order
- **Error messages**: Clear, actionable error messages if setup fails
- **Default config**: Create a sensible default ~/.xvnrc
- **Instructions**: Print clear next steps after setup

**Testing:**
- Unit tests for shell detection
- Unit tests for profile modification (with temp files)
- Integration test: Run `xvn setup` and verify files created
- Test idempotency: Run `xvn setup` twice, verify profile unchanged

**Dependencies:**
- Requires: M3.1 (needs xvn.sh to copy)

**Enables:**
- M3.4 (can test full setup flow)

---

### Task M3.4: Shell integration tests

**Objective:** Implement comprehensive tests for the shell integration, including shell script validation, FD:3 protocol testing, setup idempotency, and end-to-end shell tests.

**Implementation Steps:**

1. **Install shellcheck for shell script validation:**
   ```bash
   # macOS
   brew install shellcheck

   # Ubuntu/Debian
   apt-get install shellcheck

   # Verify installation
   shellcheck --version
   ```

2. **Create shell test directory:**
   ```bash
   mkdir -p tests/shell
   touch tests/shell/test_xvn_sh.sh
   chmod +x tests/shell/test_xvn_sh.sh
   ```

3. **Create Rust integration test file:**
   ```bash
   touch tests/shell_integration.rs
   ```

4. **Implement shellcheck validation test in `tests/shell_integration.rs`:**
   ```rust
   use std::process::Command;
   use assert_cmd::prelude::*;

   #[test]
   fn test_xvn_sh_passes_shellcheck() {
       // Validate xvn.sh with shellcheck
       let output = Command::new("shellcheck")
           .args(&["--shell=bash", "shell/xvn.sh"])
           .output();

       match output {
           Ok(output) => {
               assert!(
                   output.status.success(),
                   "shellcheck failed:\n{}",
                   String::from_utf8_lossy(&output.stdout)
               );
           }
           Err(e) => {
               eprintln!("Warning: shellcheck not available: {}", e);
               eprintln!("Install shellcheck for full test coverage");
           }
       }
   }
   ```

5. **Implement FD:3 protocol test:**
   ```rust
   // In tests/shell_integration.rs
   use std::os::unix::io::AsRawFd;
   use tempfile::NamedTempFile;

   #[test]
   fn test_fd3_protocol() {
       use xvn::shell::CommandWriter;
       use std::fs::File;
       use std::io::Read;

       // Create a temporary file for FD:3
       let temp = NamedTempFile::new().unwrap();
       let fd = temp.as_file().as_raw_fd();

       // Duplicate to FD:3
       unsafe {
           libc::dup2(fd, 3);
       }

       // Create CommandWriter (should detect FD:3)
       let mut writer = CommandWriter::new().unwrap();
       assert!(writer.is_available());

       // Write commands
       writer.write_command("export NODE_VERSION=18.20.0").unwrap();
       writer.write_command("export PATH=/path/to/node/bin:$PATH").unwrap();

       // Drop writer to flush and close
       drop(writer);

       // Read back from temp file
       let mut file = temp.reopen().unwrap();
       let mut content = String::new();
       file.read_to_string(&mut content).unwrap();

       assert!(content.contains("export NODE_VERSION=18.20.0"));
       assert!(content.contains("export PATH=/path/to/node/bin:$PATH"));

       // Clean up
       unsafe {
           libc::close(3);
       }
   }
   ```

6. **Implement setup idempotency test:**
   ```rust
   // In tests/shell_integration.rs
   use tempfile::TempDir;
   use std::env;
   use std::fs;

   #[test]
   fn test_setup_idempotency() {
       use xvn::setup::SetupInstaller;

       // Create temporary home directory
       let temp_home = TempDir::new().unwrap();
       let home_path = temp_home.path();

       // Create a fake shell profile
       let bashrc = home_path.join(".bashrc");
       fs::write(&bashrc, "# Existing content\n").unwrap();

       // Set environment for test
       env::set_var("HOME", home_path);
       env::set_var("SHELL", "/bin/bash");

       // First setup
       let installer = SetupInstaller::new().unwrap();
       assert!(!installer.is_installed().unwrap());
       installer.install().unwrap();
       assert!(installer.is_installed().unwrap());

       // Read profile content after first install
       let content1 = fs::read_to_string(&bashrc).unwrap();
       let marker_count1 = content1.matches(">>> xvn initialize >>>").count();
       assert_eq!(marker_count1, 1);

       // Second setup (should be idempotent)
       installer.install().unwrap();

       // Read profile content after second install
       let content2 = fs::read_to_string(&bashrc).unwrap();
       let marker_count2 = content2.matches(">>> xvn initialize >>>").count();

       // Should still have exactly one marker block
       assert_eq!(marker_count2, 1);
       assert_eq!(content1, content2);
   }
   ```

7. **Create end-to-end shell test script in `tests/shell/test_xvn_sh.sh`:**
   ```bash
   #!/usr/bin/env bash
   # End-to-end shell integration test

   set -euo pipefail

   # Test directory
   TEST_DIR="$(cd "$(dirname "$0")" && pwd)"
   PROJECT_ROOT="$(cd "$TEST_DIR/../.." && pwd)"

   echo "Testing xvn.sh shell integration..."

   # Source xvn.sh
   source "$PROJECT_ROOT/shell/xvn.sh"

   # Test 1: Functions are defined
   echo "✓ Test 1: Checking function definitions..."
   if ! declare -f __xvn_find_file > /dev/null; then
       echo "✗ Function __xvn_find_file not defined"
       exit 1
   fi

   if ! declare -f __xvn_activate > /dev/null; then
       echo "✗ Function __xvn_activate not defined"
       exit 1
   fi

   if ! declare -f __xvn_chpwd > /dev/null; then
       echo "✗ Function __xvn_chpwd not defined"
       exit 1
   fi

   echo "✓ All functions defined"

   # Test 2: Environment variable set
   echo "✓ Test 2: Checking XVN_SHELL_LOADED..."
   if [[ -z "${XVN_SHELL_LOADED:-}" ]]; then
       echo "✗ XVN_SHELL_LOADED not set"
       exit 1
   fi
   echo "✓ XVN_SHELL_LOADED is set"

   # Test 3: Find file function
   echo "✓ Test 3: Testing __xvn_find_file..."

   # Create temp directory with .nvmrc
   TEMP_DIR=$(mktemp -d)
   trap "rm -rf $TEMP_DIR" EXIT

   mkdir -p "$TEMP_DIR/project/subdir"
   echo "18.20.0" > "$TEMP_DIR/project/.nvmrc"

   # Should find .nvmrc from subdir
   cd "$TEMP_DIR/project/subdir"
   result=$(__xvn_find_file "$PWD" || echo "")

   if [[ "$result" != "$TEMP_DIR/project/.nvmrc" ]]; then
       echo "✗ Expected to find $TEMP_DIR/project/.nvmrc, got: $result"
       exit 1
   fi

   echo "✓ __xvn_find_file works correctly"

   # Test 4: Debug function
   echo "✓ Test 4: Testing __xvn_debug..."
   XVN_DEBUG=1 __xvn_debug "test message" 2>&1 | grep -q "test message"
   echo "✓ __xvn_debug works"

   echo ""
   echo "✅ All shell integration tests passed!"
   ```

8. **Add shell test to CI (for future):**
   ```rust
   // In tests/shell_integration.rs
   #[test]
   fn test_shell_script_execution() {
       // Run the bash test script
       let output = Command::new("bash")
           .arg("tests/shell/test_xvn_sh.sh")
           .output()
           .expect("Failed to run shell test script");

       assert!(
           output.status.success(),
           "Shell test script failed:\n{}{}",
           String::from_utf8_lossy(&output.stdout),
           String::from_utf8_lossy(&output.stderr)
       );
   }
   ```

9. **Add test for profile detection:**
   ```rust
   // In tests/shell_integration.rs
   #[test]
   fn test_profile_detection_bash() {
       use xvn::setup::Shell;
       use tempfile::TempDir;

       let temp = TempDir::new().unwrap();
       let home = temp.path();

       // Create only .bashrc
       let bashrc = home.join(".bashrc");
       fs::write(&bashrc, "# bash config\n").unwrap();

       let profiles = Shell::Bash.profile_files(home);

       // Should include .bashrc first
       assert_eq!(profiles[0], bashrc);
   }

   #[test]
   fn test_profile_detection_zsh() {
       use xvn::setup::Shell;
       use tempfile::TempDir;

       let temp = TempDir::new().unwrap();
       let home = temp.path();

       // Create only .zshrc
       let zshrc = home.join(".zshrc");
       fs::write(&zshrc, "# zsh config\n").unwrap();

       let profiles = Shell::Zsh.profile_files(home);

       // Should include .zshrc first
       assert_eq!(profiles[0], zshrc);
   }
   ```

10. **Run all tests:**
    ```bash
    # Run all tests
    cargo test

    # Run only shell tests
    cargo test shell_integration

    # Run shell script tests
    bash tests/shell/test_xvn_sh.sh

    # Run shellcheck manually
    shellcheck shell/xvn.sh
    ```

**Code Structure:**
- File: `tests/shell_integration.rs` - Rust integration tests
- File: `tests/shell/test_xvn_sh.sh` - Bash test script
- File: `shell/xvn.sh` - Shell script being tested

**Key Considerations:**
- **shellcheck**: Validates shell script syntax and best practices
- **Temp files**: Use tempfile crate for FD:3 testing
- **Idempotency**: Critical to test that multiple setups don't break anything
- **Isolation**: Tests must not modify the user's actual shell configuration
- **CI compatibility**: Tests should work in CI environments
- **Error messages**: Clear messages when tests fail

**Testing:**
- Validate xvn.sh syntax with shellcheck
- Test FD:3 protocol with mocked file descriptor
- Test setup idempotency (run twice, verify no changes)
- Test profile detection for bash and zsh
- End-to-end test: Source xvn.sh and test functions
- Test error handling (e.g., missing xvn binary)

**Dependencies:**
- Requires: M3.1 (xvn.sh must exist)
- Requires: M3.2 (FD:3 protocol implementation)
- Requires: M3.3 (setup command implementation)

**Enables:**
- Milestone 4 (shell integration tested and working)

---

## Integration Points

### Shell Script ↔ Rust Binary
- Shell script calls `xvn activate <path> 3>&1 1>&2 2>&3`
- Rust binary writes commands to FD:3
- Shell script evaluates commands with `eval "$commands"`

### Setup Command ↔ Shell Script
- Setup command copies xvn.sh to ~/.xvn/bin/
- Setup command adds source line to shell profile
- Shell profile sources xvn.sh on startup

### Configuration ↔ Shell Script
- XVN_VERSION_FILES env var controls which files to search
- XVN_DEBUG env var enables debug output
- ~/.xvnrc provides default configuration

---

## Testing Strategy

### Unit Tests (Rust)
- CommandWriter with and without FD:3
- Shell detection from $SHELL
- Profile modification (add, check, remove)
- Idempotency checks

### Integration Tests (Rust + Shell)
- shellcheck validation of xvn.sh
- FD:3 protocol with mocked file descriptor
- Setup command creates correct files
- Profile modification is idempotent

### End-to-End Tests (Pure Shell)
- Source xvn.sh and verify functions defined
- Test __xvn_find_file walks up directory tree
- Test __xvn_chpwd hook triggers correctly
- Test debug mode output

### Manual Testing
- Run `xvn setup` on clean system
- Verify shell profile modified correctly
- Test directory change triggers hook
- Test with both bash and zsh

---

## Success Criteria

✅ **Shell Script**
- xvn.sh passes shellcheck validation
- Works in both bash and zsh
- Directory changes trigger activation
- No errors in normal operation
- Debug mode provides useful output

✅ **FD:3 Protocol**
- Commands written to FD:3 when available
- Graceful degradation when FD:3 unavailable
- Multiple commands can be written
- Commands properly escaped

✅ **Setup Command**
- `xvn setup` completes without errors
- Shell profile correctly modified
- xvn.sh copied to ~/.xvn/bin/
- Default config created if missing
- Idempotent (can run multiple times safely)
- Clear instructions printed after setup

✅ **Testing**
- All unit tests pass
- All integration tests pass
- Shell script tests pass
- shellcheck validation passes
- Can manually test in live shell

---

## Common Issues and Solutions

### Issue: FD:3 not available in shell
**Solution:** The CommandWriter gracefully handles this by checking if FD:3 is open. If not, commands are silently discarded.

### Issue: Shell profile not found
**Solution:** The setup command has a priority list of profile files and creates the first one if none exist.

### Issue: Setup command runs multiple times
**Solution:** Profile modification checks for markers before adding content. xvn.sh checks XVN_SHELL_LOADED to prevent multiple initialization.

### Issue: Activation fails but breaks shell
**Solution:** The __xvn_activate function catches errors and returns instead of propagating (never breaks the shell).

### Issue: Directory changes don't trigger activation
**Solution:** Check that:
- xvn.sh is sourced in profile
- Shell profile is sourced (restart shell or `source ~/.bashrc`)
- XVN_DEBUG=1 to see debug output
- xvn binary is in PATH

---

## Next Steps After Milestone 3

After completing this milestone, you'll have a fully functional shell integration. The next milestone (M4: Version Activation & Auto-Install) will:

1. Implement the actual version activation logic in `xvn activate`
2. Integrate with the plugin system to generate activation commands
3. Add auto-install prompts for missing versions
4. Implement the full activation flow: config → version file → plugins → auto-install → commands

This milestone focuses purely on the shell integration mechanism; M4 will make it actually switch versions.

---

## Implementation Order

For optimal development flow, implement tasks in this order:

1. **M3.1**: Create xvn.sh (shell script) - This is the interface contract
2. **M3.2**: Implement FD:3 protocol (Rust) - This makes the contract work
3. **M3.3**: Implement setup command (Rust) - This installs everything
4. **M3.4**: Shell integration tests - This verifies everything works

Each task builds on the previous one, with clear dependencies and integration points.
