# PROJECT_SPEC.md

**Project:** avn (Automatic Version Switching for Node.js)
**Version:** 0.2.4
**Date:** September 30, 2025
**License:** MIT
**Original Author:** Whitney Young

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Project Purpose & Goals](#project-purpose--goals)
3. [Core Functionality](#core-functionality)
4. [System Architecture](#system-architecture)
5. [Technical Implementation Details](#technical-implementation-details)
6. [Plugin System Specification](#plugin-system-specification)
7. [Shell Integration Specification](#shell-integration-specification)
8. [Setup & Installation Flow](#setup--installation-flow)
9. [Configuration System](#configuration-system)
10. [User Interface & Commands](#user-interface--commands)
11. [Error Handling & Edge Cases](#error-handling--edge-cases)
12. [Testing Requirements](#testing-requirements)
13. [Known Issues & Limitations](#known-issues--limitations)
14. [Performance Considerations](#performance-considerations)
15. [Security Considerations](#security-considerations)
16. [Future Enhancements](#future-enhancements)

---

## Executive Summary

**avn** is a shell automation layer that monitors directory changes and automatically invokes Node.js version managers (nvm, n, nodebrew) to switch versions based on project configuration files.

**Critical Distinction:** avn does NOT manage Node.js versions itself. It's a thin automation wrapper that:
1. Detects when you `cd` into a directory
2. Looks for a `.node-version` or `.nvmrc` file
3. Reads the desired version
4. Calls the appropriate version manager command (`nvm use X`, `n X`, etc.)

**Key Components:**
- **avn core** (~1000 LOC): Shell hooks + directory monitoring + version file reading
- **avn plugins** (~50-100 LOC each): Thin wrappers that translate version requests into version manager commands
- **Version managers** (external): The actual tools (nvm, n, nodebrew) that manage Node.js installations

**The Innovation:** Uses file descriptor #3 to pass shell commands from a Node.js child process back to the parent shell, allowing automated environment modification that normal child processes cannot achieve.

**In Practice:**
```bash
# User has nvm installed with Node 14 and 16

cd ~/project-using-node-14    # avn reads .node-version → "14.17.0"
                               # avn calls: nvm use 14.17.0
                               # Output: avn activated 14.17.0 (nvm v14.17.0)

cd ~/project-using-node-16    # avn reads .node-version → "16.13.0"
                               # avn calls: nvm use 16.13.0
                               # Output: avn activated 16.13.0 (nvm v16.13.0)
```

---

## Project Purpose & Goals

### Primary Goal
**Eliminate the manual step of running version manager commands when switching between projects.**

Without avn:
```bash
cd ~/project-a
nvm use        # or: nvm use 14.17.0, or: source .nvmrc, etc.

cd ~/project-b
nvm use        # Must remember to do this every time
```

With avn:
```bash
cd ~/project-a  # Automatically activates correct version
cd ~/project-b  # Automatically switches to different version
```

### What avn IS
- **Directory change monitor** - Hooks into shell cd/pushd/popd
- **Version file reader** - Parses `.node-version`, `.nvmrc`, etc.
- **Version manager orchestrator** - Calls the right command for the right tool
- **User feedback provider** - Shows what version was activated

### What avn IS NOT
- ❌ **NOT a version manager** - Doesn't install, uninstall, or manage Node.js versions
- ❌ **NOT a replacement for nvm/n** - Works alongside them, requires them to be installed
- ❌ **NOT a version resolver** - Delegates version matching to underlying tools
- ❌ **NOT cross-platform** - Shell-specific, Unix-only

### User Experience Goals
1. **Zero-friction switching** - Automatic activation without user intervention
2. **Multi-project support** - Seamless switching between projects with different Node.js requirements
3. **Version manager agnostic** - Works with nvm, n, nodebrew through plugins
4. **Transparent operation** - Clear feedback when versions switch, silent when nothing changes
5. **Shell agnostic** - Support for bash and zsh
6. **One-time setup** - Install once, works everywhere

### Developer Goals
1. **Extensible plugin architecture** - Easy to add new version manager support
2. **Minimal plugin complexity** - Plugins are just command wrappers (~50-100 LOC)
3. **Reliable operation** - Must not break existing shell workflows
4. **Minimal performance impact** - Should not noticeably slow down directory navigation
5. **Self-contained** - Install to user home directory, no system-wide changes

---

## Core Functionality

### Functional Requirements

#### FR1: Directory Change Detection
- **Description:** Detect when the user changes directories in their shell
- **Mechanism:** Hook into bash/zsh `chpwd` event system
- **Scope:** Must work with `cd`, `pushd`, and `popd` commands
- **Trigger:** Execute on every directory change
- **Responsibility:** avn core

#### FR2: Version File Discovery
- **Description:** Locate version specification files in the directory hierarchy
- **Search Pattern:** Walk up from current directory to HOME directory
- **Supported Files:**
  - `.node-version` (default, always enabled)
  - Plugin-defined files (e.g., `.nvmrc` for avn-nvm plugin)
- **Search Termination:** Stop at first match or when reaching HOME directory
- **File Format:** First line of file, trimmed of whitespace, is the version string
- **Responsibility:** avn core (shell script does file search, Node.js does file read)

#### FR3: Version String Parsing
- **Description:** Extract version specification from file content
- **Format:** Version string as understood by underlying version manager
- **Examples:** `0.10.26`, `v14.17.0`, `14`, `lts/fermium`, `iojs-1.4`
- **Multiline Files:** Only read first line, ignore subsequent lines/comments
- **Whitespace:** Trim leading/trailing whitespace
- **Responsibility:** avn core (minimal parsing, passes to plugin as-is)

#### FR4: Plugin Selection & Invocation
- **Description:** Find a plugin that can handle the requested version
- **Strategy:** Iterate through plugins in configured order until one succeeds
- **Plugin Responsibility:** Determine if they can handle the version
- **Failure Handling:** Display warning if no plugin can satisfy the version
- **Responsibility:** avn core orchestrates, plugins execute

#### FR5: Version Manager Command Execution
- **Description:** Execute version manager command in parent shell
- **Mechanism:** Plugin returns shell command string (e.g., `nvm use 14.17.0`)
- **Transport:** Write command to file descriptor #3, shell evaluates it
- **Output:** Display success message with version and plugin name
- **Idempotency:** Only activate if version file has changed since last check
- **Responsibility:** Plugin generates command, avn transports it, shell executes it

#### FR6: Setup & Installation
- **Description:** Configure user environment for avn operation
- **Actions:**
  - Copy avn installation to `~/.avn/`
  - Copy installed plugins to `~/.avn/plugins/`
  - Modify shell profile files to source `avn.sh`
  - Create/update `~/.avnrc` configuration file
- **Validation:** Check versions, skip if already up-to-date
- **Atomicity:** All setup steps run in parallel, report all errors
- **Responsibility:** avn setup command

---

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         User Shell                          │
│                      (bash/zsh/etc)                         │
│  Current Node.js: Managed by nvm/n/nodebrew                 │
└────────────────────────┬────────────────────────────────────┘
                         │ sources on shell startup
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  ~/.avn/bin/avn.sh                          │
│  ROLE: Shell Integration Layer                              │
│  • Hooks into chpwd_functions array                         │
│  • Searches for version files (__avn_find_file)             │
│  • Calls _avn when version file found/changed               │
│  • Evaluates commands from file descriptor #3               │
└────────────────────────┬────────────────────────────────────┘
                         │ spawns on chpwd (if version file exists)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  ~/.avn/bin/_avn chpwd                      │
│  ROLE: Orchestration Layer (Node.js process)                │
│  • Reads version file content                               │
│  • Loads and iterates through plugins                       │
│  • Calls plugin.match(version)                              │
│  • Writes command to fd:3, messages to stdout/stderr        │
└────────────────────────┬────────────────────────────────────┘
                         │ requires plugin
                         ▼
┌─────────────────────────────────────────────────────────────┐
│          ~/.avn/plugins/avn-nvm/ (or avn-n, etc)           │
│  ROLE: Version Manager Adapter (50-100 LOC)                 │
│  • Receives version string (e.g., "14.17.0")                │
│  • Checks if version manager can handle it                  │
│  • Returns command string: "nvm use 14.17.0"                │
│  • OR throws error if version not available                 │
└────────────────────────┬────────────────────────────────────┘
                         │ returns command string
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  Command written to fd:3, captured by shell, eval'd         │
│  Example: eval "nvm use 14.17.0"                            │
└────────────────────────┬────────────────────────────────────┘
                         │ executed by shell
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              nvm / n / nodebrew (user-installed)            │
│  ROLE: Actual Version Manager (external to avn)             │
│  • Has Node.js versions installed: ~/.nvm/, /usr/local/n/   │
│  • "nvm use X" modifies PATH, sets environment variables    │
│  • "n X" switches active version via symlinks               │
│  • These are THE tools that manage Node.js installations    │
└─────────────────────────────────────────────────────────────┘
```

### Responsibility Breakdown

| Component | Responsibility | Complexity | LOC |
|-----------|---------------|------------|-----|
| **avn core** | Shell hooks, file discovery, orchestration | Medium | ~1000 |
| **avn plugin** | Translate version → version manager command | Very Low | ~50-100 |
| **Version manager** | Install/manage Node.js, switch versions | High | External |

### Component Breakdown

#### 1. Public CLI (`bin-public/avn`)
- **Purpose:** User-facing command line interface
- **Commands:**
  - `avn setup` - Install and configure avn
  - `avn --version` - Display version
- **Responsibility:** Entry point for user-initiated actions
- **Dependencies:** Commander.js for argument parsing
- **Entry Point:** `#!/usr/bin/env node` shebang for direct execution

#### 2. Internal CLI (`bin/_avn`)
- **Purpose:** Called by shell hooks, not intended for direct user invocation
- **Commands:**
  - `_avn chpwd <path> [versionFile]` - Handle directory change
  - `_avn explain <path> [versionFile]` - Debug mode (verbose output)
- **Special I/O:** Uses file descriptor #3 for command output
- **Responsibility:** Orchestrate plugin loading and matching
- **Options:**
  - `--color` - Enable colored output
  - `--verbose` - Show detailed error information

#### 3. Shell Integration (`bin/avn.sh`)
- **Purpose:** Bash/Zsh script loaded by user's shell profile
- **Responsibility:**
  - Monitor directory changes
  - Search for version files in directory hierarchy
  - Invoke `_avn` when version file found
  - Execute returned commands
- **Exports:**
  - `__avn_files` - Array of version filenames to search for
  - `__avn_active_file` - Path to currently active version file
- **Functions:**
  - `__avn_eval()` - Execute `_avn` and eval results from fd:3
  - `__avn_chpwd()` - Hook called on directory change
  - `__avn_find_file()` - Search parent directories for version file (MAIN WORK)
  - `__avn_debug()` - User-callable debug function
  - `__zsh_like_cd()` - Bash compatibility layer for chpwd functions
- **Hooks:** Integrates with `chpwd_functions` array
- **Compatibility:** Handles both bash and zsh, with RVM compatibility

#### 4. Core Library (`lib/avn.js`)
- **Purpose:** Main entry point, exports hooks and setup modules
- **Promise Configuration:** Configures Bluebird as promise implementation
- **Exports:**
  - `hooks` - Directory change handling
  - `setup` - Installation and configuration

#### 5. Hooks Module (`lib/hooks.js`)
- **Purpose:** Orchestration logic for plugin invocation
- **Key Function:** `chpwd(dir, [versionFile], [options])`
  - Reads version file in specified directory
  - Calls `plugins.first()` to find matching plugin
  - Writes activation command to `process.stdcmd` (fd:3)
  - Writes user messages to stdout/stderr
- **Responsibility:** Plugin orchestration, NOT version management
- **Error Handling:**
  - Silent on ENOENT (no version file)
  - Warns on PREDICATE_FAILED (no matching plugin)
  - Throws on other errors

#### 6. Plugins Module (`lib/plugins.js`)
- **Purpose:** Plugin discovery, loading, and selection
- **Key Functions:**
  - `all()` - Get all loaded plugins (cached)
  - `_all()` - Load plugins from config and default locations
  - `first(predicate)` - Find first plugin matching predicate
- **Loading Order:**
  1. Read `~/.avnrc` for plugin preference order
  2. Append default plugins: `['nvm', 'n']`
  3. Try loading from `~/.avn/plugins/avn-{name}`
  4. Fall back to global `avn-{name}` module
- **Error Handling:** Silently skip missing plugins, throw on syntax/load errors
- **Responsibility:** Plugin lifecycle management

#### 7. Formatting Module (`lib/fmt.js`)
- **Purpose:** Generate colored terminal output messages
- **Functions:**
  - `success(version, result, via)` - Success message with version info
  - `failure(version, error, verbose)` - Error message with optional details
- **Styling:** Uses chalk library for terminal colors

#### 8. Setup Module (`lib/setup.js`)
- **Purpose:** Orchestrate installation process
- **Execution:** Runs all setup steps in parallel using `Promise.all()`
- **Steps:**
  - Install avn and plugins to `~/.avn/`
  - Update shell profile files
  - Update configuration file
- **Error Handling:** Collects all errors, displays at end

#### 9-12. Setup Subsystem (`lib/setup/`)
- **install.js** - Copy avn and plugins to `~/.avn/` using `/bin/cp -RL`
- **profile.js** - Modify `~/.bash_profile`, `~/.zshrc` to source avn.sh
- **config.js** - Create/update `~/.avnrc` with plugin list
- **plugins.js** - Discover globally installed avn-* plugins via npm API

#### 13. Utility Modules (`lib/util/`)
- **codes.js** - Error code checking utilities (`isNoEntry(error)`)

---

## Plugin System Specification

### Plugin Architecture Philosophy

**Plugins are intentionally minimal.** Their ONLY job is to:
1. Check if a version is available in their version manager
2. Return the shell command to activate it

**Plugins do NOT:**
- ❌ Install Node.js versions
- ❌ Manage version downloads
- ❌ Parse complex version specifications (delegate to version manager)
- ❌ Modify the environment directly

### Plugin Interface Contract

```javascript
module.exports = {
  // Required: Plugin name (string)
  // Typically matches the version manager name
  name: 'nvm',

  // Required: Check if version can be activated, return command
  // @param {string} version - Version string from .node-version file
  // @returns {Promise<Object>} - Resolves to { command, version }
  //   - command: Shell command string to execute (e.g., "nvm use 14.17.0")
  //   - version: Actual version that will be activated (e.g., "v14.17.0")
  // @throws {Error} - If version cannot be matched/activated
  match: function(version) {
    // 1. Query version manager to check if version exists
    //    Example: run "nvm which <version>" or check filesystem
    // 2. If exists, return command to activate it
    // 3. If not exists, throw descriptive error

    return Promise.resolve({
      command: 'nvm use ' + version,  // Shell command (as string)
      version: 'v14.17.0'             // Actual resolved version
    });
  }
};
```

### Real-World Plugin Examples

#### avn-nvm Plugin (Simplified)

```javascript
// The avn-nvm plugin is ~100 LOC that basically does:

module.exports = {
  name: 'nvm',

  match: function(version) {
    // 1. Run: nvm which <version>
    //    This queries nvm to see if version exists
    return exec('nvm which ' + version)
      .then(function(path) {
        // 2. If nvm returns a path, version exists
        if (path) {
          return {
            command: 'nvm use ' + version,  // Just return the command!
            version: extractVersion(path)   // Parse version from path
          };
        }
        // 3. If not found, throw error
        throw new Error('Version not installed: ' + version);
      });
  }
};
```

**That's it!** The plugin doesn't install anything, doesn't manage anything. It just:
- Checks: "Does this version exist in nvm?"
- Returns: "Run `nvm use <version>`"

#### avn-n Plugin (Simplified)

```javascript
// The avn-n plugin is even simpler (~80 LOC):

module.exports = {
  name: 'n',

  match: function(version) {
    // 1. Check filesystem for installed versions
    //    n stores versions in: $N_PREFIX/n/versions/node/<version>
    let installedVersions = fs.readdirSync(N_PREFIX + '/n/versions/node');

    // 2. See if requested version matches any installed version
    let matched = semver.maxSatisfying(installedVersions, version);

    // 3. Return command or throw
    if (matched) {
      return Promise.resolve({
        command: 'n ' + matched,  // Just the command string!
        version: matched
      });
    }
    throw new Error('Version not installed: ' + version);
  }
};
```

### Plugin Responsibilities (Minimal)

1. **Version Availability Check**
   - Query version manager: "Do you have version X?"
   - Methods: Run version manager command, check filesystem, etc.
   - Return true/false (or throw error)

2. **Command Generation**
   - Return shell command string that activates the version
   - Examples: `"nvm use 14.17.0"`, `"n 16.13.0"`, `"nodebrew use v12.18.0"`
   - Command should be idempotent (safe to run multiple times)

3. **Version Resolution**
   - Accept version string from version file
   - Optionally resolve to actual version (e.g., "14" → "14.17.0")
   - Return actual version that will be activated
   - Can delegate resolution to version manager

4. **Error Handling**
   - Throw descriptive errors for missing versions
   - Include plugin name in error context
   - Support promise rejection for async operations

### Plugin Discovery Locations

Plugins are searched in the following order:

1. **Local Plugins:** `~/.avn/plugins/avn-{name}/`
   - Installed by `avn setup`
   - User-specific, version-locked with avn
   - Example: `~/.avn/plugins/avn-nvm/index.js`

2. **Global Plugins:** Global npm module `avn-{name}`
   - Installed with `npm install -g avn-{name}`
   - Shared across Node.js versions
   - Example: `/usr/local/lib/node_modules/avn-nvm/`

### Plugin Priority Order

Plugin priority is determined by `~/.avnrc` configuration:

```json
{
  "plugins": ["nvm", "n"]
}
```

- **First Match Wins:** First plugin to successfully match is used
- **Sequential Testing:** Plugins tried in order until one succeeds
- **Default Order:** `["nvm", "n"]` if not specified

**Example Scenario:**
- User has both nvm and n installed
- Config: `{"plugins": ["nvm", "n"]}`
- Version file: `14.17.0`
- Process:
  1. Try avn-nvm: Checks nvm, version exists → Returns `"nvm use 14.17.0"` ✅
  2. Never tries avn-n (first match wins)

### Shell Integration (Optional)

Plugins MAY include a `load.sh` file for shell-level integration:

**Location:** `~/.avn/plugins/avn-{name}/load.sh`

**Purpose:**
- Add additional version file names to `__avn_files` array
- Set environment variables
- Define helper functions

**Example (avn-nvm load.sh):**
```bash
# Add .nvmrc to list of searched files
__avn_files+=(".nvmrc")
```

This allows avn to detect `.nvmrc` files in addition to `.node-version`.

**Loading:** Sourced by `avn.sh` on shell startup:
```bash
for plugin in $HOME/.avn/plugins/*
do
  [[ -f "$plugin/load.sh" ]] && . "$plugin/load.sh"
done
```

### What Version Managers Do (Outside avn)

For context, here's what version managers actually do:

**nvm (Node Version Manager):**
- Installs Node.js versions to `~/.nvm/versions/node/<version>/`
- `nvm install <version>` - Downloads and installs Node.js
- `nvm use <version>` - Modifies PATH to point to specific version
- `nvm which <version>` - Returns path to node binary

**n:**
- Installs Node.js versions to `$N_PREFIX/n/versions/node/<version>/`
- `n <version>` - Downloads (if needed) and activates version via symlinks
- `n ls` - Lists installed versions
- Uses symlinks in `$N_PREFIX/bin/` to active version

**nodebrew:**
- Installs Node.js versions to `~/.nodebrew/node/<version>/`
- `nodebrew install <version>` - Downloads and installs
- `nodebrew use <version>` - Activates version
- Similar PATH manipulation approach

**avn's role:** Just calls these commands automatically based on directory!

### Plugin Development Guidelines

1. **Keep It Simple:** Plugins should be 50-100 LOC
2. **Delegate to Version Manager:** Don't reimplement version manager logic
3. **Query, Don't Install:** Check if version exists, don't install it
4. **Return Command Strings:** Let the shell execute commands
5. **Handle Errors Gracefully:** Descriptive errors when version not found
6. **Support Promises:** All async operations should return promises
7. **Minimal Dependencies:** Avoid heavy dependencies

---

## Technical Implementation Details

### The File Descriptor #3 Innovation

**Problem:** Node.js child processes cannot modify parent shell environment

Normal child process behavior:
```bash
$ node -e "process.env.FOO='bar'"  # Runs in child process
$ echo $FOO                         # Empty! Child can't modify parent
```

**avn's Solution:** Custom file descriptor protocol

1. **Shell opens fd:3** before spawning Node.js:
   ```bash
   actions=$(_avn chpwd /path 3>&1 1>&2)
   ```
   - `3>&1` - Redirect fd:3 to stdout (capture it)
   - `1>&2` - Redirect stdout to stderr (separate user messages)

2. **Node.js writes to fd:3:**
   ```javascript
   process.stdcmd = fs.createWriteStream(null, { fd: 3 });
   process.stdcmd.write('nvm use 14.17.0\n');
   ```

3. **Shell captures fd:3 output:**
   ```bash
   actions=$(_avn chpwd /path 3>&1 1>&2)
   # actions now contains: "nvm use 14.17.0"
   ```

4. **Shell evaluates captured string:**
   ```bash
   eval "$actions"
   # Executes: nvm use 14.17.0
   # This runs in the PARENT shell, modifying its environment!
   ```

**Benefits:**
- Clean separation of concerns:
  - **fd:3** - Shell commands to execute
  - **stdout** - User messages (success output)
  - **stderr** - Error/warning messages
- Allows child process to effectively modify parent environment
- Standard Unix mechanism, no hacks

### Shell Hook Integration

#### chpwd_functions Mechanism

**Zsh** has native support for `chpwd_functions`:
```bash
# Zsh automatically calls functions in this array after cd
chpwd_functions+=(__avn_chpwd)
```

**Bash** doesn't have native support, so avn emulates it:
```bash
# Wrap cd/pushd/popd to call chpwd functions manually
function cd() {
  builtin cd "$@" && __call_chpwd_functions
}
function pushd() {
  builtin pushd "$@" && __call_chpwd_functions
}
function popd() {
  builtin popd "$@" && __call_chpwd_functions
}
```

#### Version File Search Algorithm

Implemented in shell (not Node.js) for performance:

```bash
function __avn_find_file() {
  local found
  local dir=$PWD

  # Walk up directory tree
  while [[ -z "$found" ]] && [[ "$dir" != "" ]]; do
    # Check each configured filename
    for file in "${__avn_files[@]}"; do
      if [[ -f "$dir/$file" ]]; then
        found="$dir/$file"
        break
      fi
    done

    # Stop at HOME directory
    if [[ "$dir" == "$HOME" ]]; then
      break
    fi

    # Move up one directory
    dir=${dir%/*}
  done

  echo $found
}
```

**Why in shell?**
- Faster than spawning Node.js for filesystem checks
- Allows early exit if no version file found
- Simple string operations, shell excels at this

### Complete Activation Flow

```
1. User runs: cd ~/my-project
   ↓
2. Shell executes cd command (changes directory)
   ↓
3. Shell triggers chpwd_functions
   ↓
4. __avn_chpwd executes:
   - Calls __avn_find_file (shell function)
   - Searches: ~/my-project/.node-version
   - Searches: ~/my-project/../.node-version
   - Searches: ~/...
   - Finds: ~/my-project/.node-version
   ↓
5. Compare to __avn_active_file
   - If same file as last time: EXIT (no action needed)
   - If different: Continue
   ↓
6. Call __avn_eval:
   actions=$(_avn chpwd ~/my-project .node-version 3>&1 1>&2)
   ↓
7. _avn Node.js process starts:
   - Reads file: cat ~/my-project/.node-version
   - Content: "14.17.0\n"
   - Parses: version = "14.17.0"
   ↓
8. Load plugins (cached):
   - Load ~/.avn/plugins/avn-nvm/
   - Load ~/.avn/plugins/avn-n/
   ↓
9. Try first plugin (avn-nvm):
   - Call: plugin.match("14.17.0")
   - Plugin runs: nvm which 14.17.0
   - nvm returns: /Users/me/.nvm/versions/node/v14.17.0/bin/node
   - Plugin returns: {
       command: "nvm use 14.17.0",
       version: "v14.17.0"
     }
   - Success! ✅
   ↓
10. Write outputs:
    - fd:3 (stdcmd): "nvm use 14.17.0\n"
    - stdout: "avn activated 14.17.0 (nvm v14.17.0)\n"
    ↓
11. Node.js process exits
    ↓
12. Shell captures fd:3 output:
    actions="nvm use 14.17.0"
    ↓
13. Shell evaluates:
    eval "nvm use 14.17.0"
    ↓
14. nvm executes:
    - Modifies PATH
    - Sets NVM_BIN, NVM_PATH, etc.
    - Node.js 14.17.0 is now active!
    ↓
15. User sees avn's stdout message:
    "avn activated 14.17.0 (nvm v14.17.0)"
    ↓
16. Shell prompt returns
    ↓
17. User can now run: node --version
    Output: v14.17.0
```

**Total time:** ~150-200ms (imperceptible to user)

### Promise Architecture

**Library:** Bluebird (configured via `any-promise/register`)

**Why Bluebird (in 2019)?**
- More features than native promises at the time
- Better performance
- Promise cancellation support
- Long stack traces for debugging

**Modern Rebuild:** Would use native async/await

**Patterns Used:**
```javascript
// Context sharing with .bind({})
return Promise.bind({})
  .then(function() {
    this.version = readVersion();
  })
  .then(function() {
    return match(this.version);  // Access shared context
  });

// Error-tolerant parallel execution with .reflect()
Promise.all([
  step1().reflect(),  // Won't reject Promise.all
  step2().reflect(),
  step3().reflect()
])
.then(function(results) {
  // Check which succeeded/failed
  results.forEach(function(result) {
    if (result.isRejected()) {
      console.error(result.reason());
    }
  });
});

// Sequential testing with .reduce()
plugins.reduce(function(foundPlugin, plugin) {
  return foundPlugin || plugin.match(version)
    .then(function(result) { return result && plugin; })
    .catch(function() { return null; });
}, null);

// Predicate-based error handling
fs.readFile(file)
  .catch(isNoEntry, function() {
    // Only handle ENOENT errors
    return null;
  });
```

### Installation Copy Strategy

**Command:** `/bin/cp -RL source/ destination`

**Flags:**
- `-R` - Recursive copy
- `-L` - Follow symbolic links (dereference)

**Why `-L` (dereference symlinks)?**
- npm sometimes uses symlinks in node_modules
- Want actual files copied, not broken symlinks
- Ensures installation works even if source is removed

**Version Check Before Copy:**
```javascript
let srcVersion = JSON.parse(fs.readFileSync(src + '/package.json')).version;
let dstVersion = JSON.parse(fs.readFileSync(dst + '/package.json')).version;

if (srcVersion === dstVersion) {
  // Skip copy, already up to date
  return;
}

// Version differs, perform copy
cp('-RL', src, dst);
```

---

## Shell Integration Specification

### Shell Support Matrix

| Shell | Support Status | Notes |
|-------|----------------|-------|
| bash  | ✅ Full Support | Requires chpwd emulation via function wrapping |
| zsh   | ✅ Full Support | Native chpwd_functions support |
| fish  | ❌ Not Supported | Different architecture, would need rewrite |
| other | ❌ Not Supported | Would require custom integration |

### Shell Startup Flow

```
1. User opens terminal
   ↓
2. Shell loads profile (~/.bash_profile or ~/.zshrc)
   ↓
3. Profile sources ~/.avn/bin/avn.sh
   ↓
4. avn.sh initialization:
   - Adds ~/.avn/bin to PATH
   - Initializes __avn_files=(".node-version")
   - Initializes __avn_active_file=""
   - Defines shell functions
   ↓
5. avn.sh sources plugin load.sh files:
   for plugin in ~/.avn/plugins/*; do
     source $plugin/load.sh  # May add to __avn_files
   done
   ↓
6. avn.sh calls __avn_chpwd once (activate for current directory)
   ↓
7. avn.sh adds __avn_chpwd to chpwd_functions array
   ↓
8. Shell is ready, avn will trigger on cd/pushd/popd
```

### Environment Variables

#### Set by avn.sh
- `PATH` - Prepended with `$HOME/.avn/bin`
- `__avn_files` - Array of version filenames to search
- `__avn_active_file` - Full path to currently active version file

#### Used by avn
- `HOME` - User home directory for config and installation
- `N_PREFIX` - (Plugin-specific) Location of n installations

#### Modified by Version Managers (not avn)
- `PATH` - Modified by `nvm use` or `n` commands
- `NVM_BIN`, `NVM_PATH` - Set by nvm
- Various others depending on version manager

### Exported Functions

All functions prefixed with `__avn_` to avoid namespace collisions:

- `__avn_eval` - Execute Node.js CLI and eval result
- `__avn_chpwd` - Hook called on directory change
- `__avn_find_file` - Search for version file (does the walking)
- `__avn_debug` - User-callable debug function

### Color Detection

avn.sh checks if stdout is a TTY before enabling color:
```bash
[[ -t 1 ]] && options="--color"
_avn ${cmd} $options "$@" 3>&1 1>&2
```

### RVM Compatibility

avn.sh includes special handling for RVM (Ruby Version Manager):
```bash
# Support rvm until chpwd_functions are integrated
[[ " ${chpwd_functions[*]} " == *" __rvm"* ]] ||
  chpwd_functions+=(__rvm_cd_functions_set)
```

This ensures both avn and RVM hooks work together.

---

## Setup & Installation Flow

### Pre-Installation Requirements

1. **Node.js:** Must be installed (any version)
2. **Version Manager:** Must have nvm, n, or nodebrew installed
3. **Shell:** Must be using bash or zsh
4. **npm:** Used for global installation

### Installation Steps

```bash
# 1. Install avn and desired plugins globally
npm install -g avn avn-nvm avn-n

# 2. Run setup to configure shell
avn setup

# 3. Restart terminal (or source profile)
exec $SHELL
# OR: source ~/.bash_profile (or ~/.zshrc)
```

### What `avn setup` Does

Runs three operations **in parallel**:

#### Operation 1: Install Files (`lib/setup/install.js`)

**Copy avn to ~/.avn/:**
1. Find where avn is installed globally (e.g., `/usr/local/lib/node_modules/avn/`)
2. Copy entire directory to `~/.avn/` using `cp -RL`
3. Compare package.json versions first (skip if same)

**Copy plugins to ~/.avn/plugins/:**
1. Query npm for all global `avn-*` packages
2. For each plugin:
   - Source: Global npm location
   - Destination: `~/.avn/plugins/avn-{name}/`
   - Copy using `cp -RL`
   - Compare versions first (skip if same)

**Output:**
```
avn: installation complete
avn-nvm: installation complete
avn-n: installation complete
```

#### Operation 2: Modify Shell Profiles (`lib/setup/profile.js`)

**Check for existing profiles:**
- `~/.bash_profile`
- `~/.zshrc`

**For each existing file:**
1. Check if avn already configured (search for "avn.sh")
2. If not found, append:
   ```bash
   [[ -s "$HOME/.avn/bin/avn.sh" ]] && source "$HOME/.avn/bin/avn.sh" # load avn
   ```

**If no profiles exist:**
- Create `~/.bash_profile`
- Add avn configuration

**Output:**
```
avn: profile setup complete (~/.bash_profile)
avn: restart your terminal to start using avn
```

#### Operation 3: Create Config (`lib/setup/config.js`)

**Discover plugins:**
1. Query npm for all global `avn-*` packages
2. Extract plugin names (remove "avn-" prefix)
3. Exclude built-in plugins (nvm, n) from config

**Create/update ~/.avnrc:**
```json
{
  "plugins": ["custom-plugin", "nvm", "n"]
}
```

**Logic:**
- If file exists: Merge discovered plugins with existing config
- If file doesn't exist: Create with discovered plugins
- Only write if changed

**Output:**
```
avn: configuration complete (~/.avnrc)
```

### Post-Installation

**User must restart terminal** or run:
```bash
source ~/.bash_profile  # or ~/.zshrc
```

**Verification:**
```bash
cd /path/to/project/with/.node-version
# Should see: avn activated X.X.X (plugin-name vX.X.X)
```

### Update Process

Running `avn setup` again:
- ✅ Updates avn if version changed
- ✅ Updates plugins if versions changed
- ✅ Updates profile if not already configured
- ✅ Updates config with any new plugins
- ⚠️ Preserves existing config plugin order

### Uninstallation

**No uninstall command.** Manual process:

```bash
# 1. Remove global packages
npm rm -g avn avn-nvm avn-n

# 2. Remove local installation
rm -r ~/.avn

# 3. Remove config
rm ~/.avnrc

# 4. Edit shell profiles (manual)
# Remove lines containing "avn.sh" from:
#   ~/.bash_profile
#   ~/.zshrc
```

---

## Configuration System

### Configuration File

**Path:** `~/.avnrc`
**Format:** JSON
**Creation:** Automatically created by `avn setup`
**Editing:** User can manually edit

### Schema

```json
{
  "plugins": ["array", "of", "plugin", "names"]
}
```

**Properties:**

- `plugins` (array of strings)
  - Plugin names **without** "avn-" prefix
  - Defines priority order (first = highest priority)
  - Default: `["nvm", "n"]` (if file doesn't exist)

### Examples

#### Default (both nvm and n installed)
```json
{
  "plugins": ["nvm", "n"]
}
```
**Behavior:** Try avn-nvm first, fall back to avn-n

#### Prefer n over nvm
```json
{
  "plugins": ["n", "nvm"]
}
```
**Behavior:** Try avn-n first, fall back to avn-nvm

#### Only use nvm
```json
{
  "plugins": ["nvm"]
}
```
**Behavior:** Only try avn-nvm, fail if it can't match

#### Custom plugin
```json
{
  "plugins": ["my-custom-plugin", "nvm", "n"]
}
```
**Behavior:** Try custom plugin first, then nvm, then n

### Configuration Loading

```javascript
// lib/plugins.js logic
function loadPlugins() {
  // 1. Read config file
  let config = JSON.parse(fs.readFileSync('~/.avnrc')) || {};

  // 2. Union with defaults
  let pluginNames = union(config.plugins, ['nvm', 'n']);

  // 3. Load each plugin
  return pluginNames.map(loadPlugin);
}
```

**Key Point:** Config plugins take priority, but defaults are always added

---

## User Interface & Commands

### Command Line Interface

#### avn setup
```bash
avn setup
```
- **Description:** Install and configure avn
- **Requirements:** Must run with Node.js version that installed avn
- **Actions:**
  - Copies avn to ~/.avn/
  - Copies plugins to ~/.avn/plugins/
  - Modifies shell profile files
  - Creates/updates ~/.avnrc
- **Output:** Progress for each step
- **Errors:** Displays all errors at end, doesn't stop on first error

#### avn --version
```bash
avn --version
```
- **Description:** Display avn version
- **Output:** `0.2.4`

#### avn --help
```bash
avn --help
```
- **Description:** Display help
- **Output:** Usage and commands

### Shell Functions

#### __avn_debug
```bash
__avn_debug
```
- **Description:** Debug avn in current directory
- **When to use:** Troubleshooting version activation issues
- **Output:** Verbose output showing:
  - Version file found (or not)
  - Version string parsed
  - Plugins tried
  - Errors from each plugin
  - Command that would be executed

**Example output:**
```
avn activated 14.17.0 via ../parent-dir/.node-version (nvm v14.17.0)
```

**Example error output:**
```
avn could not activate node 99.99.99
error: no plugin passed predicate
  nvm: Version '99.99.99' not found - try `nvm ls-remote` to browse available versions.
  n: Version not installed: 99.99.99
```

### Output Messages

#### Success
```
avn activated 14.17.0 (nvm v14.17.0)
```

**Components:**
- `avn` - Bold magenta
- `activated 14.17.0` - Cyan (version from file)
- `(nvm v14.17.0)` - Gray (plugin name and resolved version)

#### Success (from parent directory)
```
avn activated 14.17.0 via ../.node-version (nvm v14.17.0)
```

**Additional:**
- `via ../.node-version` - Gray (shows which file triggered activation)

#### Error (simple)
```
avn could not activate node 14.17.0
```

**Components:**
- `avn` - Red
- `could not activate node 14.17.0` - Yellow

#### Error (verbose, with --verbose or __avn_debug)
```
avn could not activate node 14.17.0
error: no plugin passed predicate
  nvm: N/A: version "v14.17.0 -> N/A" is not yet installed
  n: Version not installed: 14.17.0
```

**Components:**
- Error summary
- Per-plugin errors with plugin name in magenta

### Silent Cases

**No output when:**
- No version file exists in directory hierarchy
- Version file is same as previous directory (already active)

---

## Error Handling & Edge Cases

### Error Categories

1. **Silent** - No version file → no output (normal)
2. **Warning** - Version file exists but can't activate → stderr warning
3. **Fatal** - Unexpected errors → throw, display message

### Edge Cases

#### 1. No Version File
**Scenario:** User is in directory without `.node-version`

**Behavior:**
- `__avn_find_file` returns empty
- `__avn_chpwd` doesn't call `_avn`
- No output, no error
- Node.js version remains whatever it was

#### 2. Version Not Installed
**Scenario:** `.node-version` says `14.17.0`, but not installed in nvm/n

**Behavior:**
- Plugin `match()` throws error
- Error caught by avn core
- Warning displayed to stderr
- Shell continues normally
- Node.js version unchanged

**Output:**
```
avn could not activate node 14.17.0
```

#### 3. Multiple Plugins, First Fails
**Scenario:** Config has `["nvm", "n"]`, nvm doesn't have version, n does

**Behavior:**
- Try avn-nvm: Throws "version not installed"
- Try avn-n: Returns command successfully
- Execute avn-n command
- Version activated via n

**Output:**
```
avn activated 14.17.0 (n 14.17.0)
```

#### 4. Nested Version Files
**Scenario:**
```
/project/.node-version (14.17.0)
/project/subdir/.node-version (16.13.0)
```

**Behavior:**
- In `/project/`: Uses 14.17.0
- In `/project/subdir/`: Uses 16.13.0 (child takes precedence)

#### 5. Multiline Version File
**Scenario:**
```
14.17.0
# This is a comment
```

**Behavior:**
- Only first line read: `"14.17.0"`
- Comment ignored

#### 6. Version File with Whitespace
**Scenario:** `"  14.17.0  \n"`

**Behavior:**
- Trimmed: `"14.17.0"`

#### 7. Version File Change in Current Directory
**Scenario:** User edits `.node-version` without changing directories

**Behavior:**
- No activation (only triggers on `cd`)
- Workaround: `cd .` or `cd $PWD`

#### 8. Same Version File in New Directory
**Scenario:**
- `/project-a/.node-version` (14.17.0)
- `/project-b/.node-version` (14.17.0)

**Behavior:**
- `cd /project-a`: Activates 14.17.0
- `cd /project-b`: Compares file paths, they differ, activates again
- Optimization opportunity: Could compare content, not just path

#### 9. Rapid Directory Changes
**Scenario:** User runs `cd a && cd b && cd c` quickly

**Behavior:**
- Each `cd` spawns separate `_avn` process
- Processes may overlap
- Last activation wins
- Potential race condition (harmless)

#### 10. HOME Directory
**Scenario:** `.node-version` in HOME directory

**Behavior:**
- Search stops AT HOME, doesn't read it
- Prevents unwanted global activation
- Intentional design decision

---

## Testing Requirements

### Test Structure

**Framework:** Mocha + Chai + Sinon + Istanbul

**Test Files:**
- `test/helpers.js` - Test utilities
- `test/hooks.js` - Core functionality tests
- `test/plugins.js` - Plugin loading tests
- `test/setup.js` - Setup process tests
- `test/shell.js` - Shell integration test runner
- `test/shell/*.sh` - Shell test scripts
- `test/fixtures/` - Test data

### Unit Tests (JavaScript)

**test/hooks.js** - Core chpwd functionality:
- ✅ No action when no version file
- ✅ Calls plugin match with correct version
- ✅ Reads only first line of file
- ✅ Handles plugin errors gracefully
- ✅ Validates plugin response format
- ✅ Supports verbose error output

**test/plugins.js** - Plugin system:
- ✅ Loads plugins from config
- ✅ Falls back to defaults if no config
- ✅ Skips missing plugins
- ✅ Throws on plugin syntax errors
- ✅ Caches plugin list

**test/setup.js** - Installation:
- ✅ Creates/updates shell profiles
- ✅ Detects existing avn configuration
- ✅ Installs to ~/.avn/
- ✅ Creates/updates ~/.avnrc
- ✅ Handles permission errors

### Shell Integration Tests

**test/shell/*.sh** - Real shell execution:
- ✅ Searches parent directories
- ✅ Supports custom version files
- ✅ Doesn't re-activate unchanged files
- ✅ Passes correct arguments
- ✅ Evaluates fd:3 commands
- ✅ Works with `set -e`
- ✅ Compatible with RVM
- ✅ Loads plugin load.sh files

**Test execution:**
```bash
# Each .sh script runs in both bash and zsh
for shell in bash zsh; do
  $shell test/shell/example.sh
done
```

### Coverage

**Tool:** Istanbul

**Command:** `npm test` (runs linting + tests + coverage)

**Report:** HTML in `coverage/` directory

**CI:** LCOV report sent to Coveralls

---

## Known Issues & Limitations

### Current Limitations

1. **Shell Support:** Only bash and zsh
2. **Manual File Changes:** Not detected without `cd`
3. **Node.js Version Dependency:** avn binary tied to installation Node.js version
4. **No Previous Version Restoration:** Leaving directory doesn't restore previous version
5. **No Windows Support:** Unix shells only
6. **Archived Plugins:** avn-nvm and avn-n are archived/unmaintained

### Known Issues

1. **Race Conditions:** Rapid directory changes may interleave output
2. **Setup Node Version:** Must run `avn setup` with same Node.js version as installation
3. **Plugin Discovery:** Relies on npm programmatic API (slow)

---

## Performance Considerations

### Critical Path Timing

**Directory change → activation:**
- Shell hook: <1ms
- File search (shell): ~10ms
- Node.js spawn: ~50ms
- Plugin match: ~30ms
- Version manager: ~50ms
- **Total:** ~150-200ms (acceptable)

### Optimizations

1. **File search in shell** - Avoids Node.js spawn if no version file
2. **Plugin caching** - Load once per process
3. **Early exit** - Skip if version file unchanged
4. **Parallel setup** - All setup steps run concurrently

---

## Security Considerations

### Trust Model

User trusts:
1. avn core code
2. All installed plugins
3. Version files in projects
4. Underlying version managers

### Attack Vectors

1. **Malicious Plugins:** Full system access during `match()`
2. **Shell Injection:** Plugins return arbitrary commands
3. **Version Files:** Trigger activation of attacker-chosen versions

**Mitigation:** Trust-based system, review plugins before installation

---

## Future Enhancements

### From README
- **Automatic version restoration:** Switch back when leaving directory

### Potential Improvements
1. **Fish shell support:** Different architecture required
2. **File watching:** Detect version file changes without `cd`
3. **Daemon mode:** Persistent process for faster response
4. **Built-in plugins:** Ship with nvm/n support
5. **TypeScript rewrite:** Type safety, better DX
6. **Modern dependencies:** Replace Bluebird, lodash with native alternatives

---

## Appendix A: Technology Stack

### Current (2019)
- **Runtime:** Node.js 0.10 - 5.5 (tested)
- **Promises:** Bluebird via any-promise
- **CLI:** Commander
- **Testing:** Mocha, Chai, Sinon, Istanbul
- **Linting:** JSHint, JSCS
- **CI:** Travis CI

### Recommended for Rebuild (2025)
- **Runtime:** Node.js 18+ LTS
- **Language:** TypeScript + ESM
- **Promises:** Native async/await
- **CLI:** Commander or Yargs
- **Testing:** Vitest or Jest
- **Linting:** ESLint + Prettier
- **CI:** GitHub Actions

---

## Appendix B: Related Projects

### Version Managers (what avn wraps)
- **nvm:** https://github.com/nvm-sh/nvm
- **n:** https://github.com/tj/n
- **nodebrew:** https://github.com/hokaccha/nodebrew
- **nodenv:** https://github.com/nodenv/nodenv
- **fnm:** https://github.com/Schniz/fnm
- **asdf-nodejs:** https://github.com/asdf-vm/asdf-nodejs

### Auto-Switching Tools (avn alternatives)
- **volta:** https://volta.sh/ (Rust-based, cross-platform, auto-switching built-in)
- **direnv:** https://direnv.net/ (Generic environment switcher)
- **nvs:** https://github.com/jasongin/nvs (Cross-platform, Windows support)
- **chnode:** https://github.com/chmln/chnode (Simple bash auto-switcher)

### avn Plugins
- **avn-nvm:** https://github.com/wbyoung/avn-nvm (archived)
- **avn-n:** https://github.com/wbyoung/avn-n (archived)
- **avn-nodebrew:** https://github.com/kuy/avn-nodebrew (community)

---

**End of PROJECT_SPEC.md**

This specification provides complete information to rebuild avn from scratch with a modern technology stack while preserving its core architecture and functionality.