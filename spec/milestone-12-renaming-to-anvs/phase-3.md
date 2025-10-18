# Phase 3: Shell Integration

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 45-60 minutes

## Overview

Phase 3 updates the shell hook script (`shell/xvn.sh`) that provides automatic Node.js version switching when users change directories. This script is sourced into the user's shell (bash/zsh) during `anvs setup` and contains the core functionality that makes anvs "automatic."

**Why Phase 3 is Critical:**
- The shell script is what users actually interact with every time they `cd` to a directory
- It contains all the function names and environment variables that need renaming
- This script is sourced into user shells, so incorrect variable names will cause activation failures
- The script invokes the `anvs` binary (updated in Phase 2), so references must match

**⚠️ IMPORTANT**: Phase 3 should only begin after Phase 2 (Installation & Binary Files) is complete, as it depends on the renamed `anvs` binary existing and being referenced correctly.

---

## Implementation Tasks

### Task 3.1: Rename Shell Script File

**File**: `shell/xvn.sh` → `shell/anvs.sh`

**Commands**:
```bash
# Rename the shell script
git mv shell/xvn.sh shell/anvs.sh

# Verify rename
ls -la shell/

# Verify file still exists
test -f shell/anvs.sh && echo "✓ File exists" || echo "✗ File missing"

# Verify old file gone
test ! -f shell/xvn.sh && echo "✓ Old file removed" || echo "✗ Old file still exists"
```

**Expected output**:
```
✓ File exists
✓ Old file removed
```

**Actions**:
- [ ] Rename file: `git mv shell/xvn.sh shell/anvs.sh`
- [ ] Verify new file exists at `shell/anvs.sh`
- [ ] Verify old file no longer exists at `shell/xvn.sh`
- [ ] File permissions preserved (should be executable)

---

### Task 3.2: Update File Header Comment

**File**: `shell/anvs.sh`

**Line**: 2

**Changes Required**:

**Before**:
```bash
# xvn.sh - Shell integration for xvn (Extreme Version Switcher)
```

**After**:
```bash
# anvs.sh - Shell integration for anvs (Automatic Node Version Switcher)
```

**Commands**:
```bash
# View current header
head -n 5 shell/anvs.sh

# After editing, verify change
head -n 5 shell/anvs.sh | grep "anvs.sh"
```

**Actions**:
- [ ] Update line 2 to reference "anvs.sh" instead of "xvn.sh"
- [ ] Update description from "Extreme Version Switcher" to "Automatic Node Version Switcher"
- [ ] Verify header comment is accurate

---

### Task 3.3: Update Initialization Guard Variable

**File**: `shell/anvs.sh`

**Lines**: 12-15

**Changes Required**:

**Before**:
```bash
# Prevent multiple initialization
if [[ -n "${XVN_SHELL_LOADED:-}" ]]; then
    return 0
fi
export XVN_SHELL_LOADED=1
```

**After**:
```bash
# Prevent multiple initialization
if [[ -n "${ANVS_SHELL_LOADED:-}" ]]; then
    return 0
fi
export ANVS_SHELL_LOADED=1
```

**Explanation**: This guard variable prevents the script from being sourced multiple times in the same shell session.

**Commands**:
```bash
# View current guard section
sed -n '11,15p' shell/anvs.sh

# After editing, verify no XVN references
grep -n "XVN_SHELL_LOADED" shell/anvs.sh
# Should return no results

# Verify ANVS version exists
grep -n "ANVS_SHELL_LOADED" shell/anvs.sh
# Should show lines 12 and 15
```

**Actions**:
- [ ] Change `XVN_SHELL_LOADED` to `ANVS_SHELL_LOADED` on line 12
- [ ] Change `XVN_SHELL_LOADED` to `ANVS_SHELL_LOADED` on line 15
- [ ] Verify no remaining `XVN_SHELL_LOADED` references
- [ ] Test guard logic still works (prevents double-sourcing)

---

### Task 3.4: Rename Debug Function and Update Variable

**File**: `shell/anvs.sh`

**Lines**: 17-22

**Changes Required**:

**Function name change**:
- `__xvn_debug` → `__anvs_debug`

**Environment variable change**:
- `XVN_DEBUG` → `ANVS_DEBUG`

**Before**:
```bash
# Debug logging (enabled via XVN_DEBUG=1)
__xvn_debug() {
    if [[ "${XVN_DEBUG:-0}" == "1" ]]; then
        echo "[xvn] $*" >&2
    fi
}
```

**After**:
```bash
# Debug logging (enabled via ANVS_DEBUG=1)
__anvs_debug() {
    if [[ "${ANVS_DEBUG:-0}" == "1" ]]; then
        echo "[anvs] $*" >&2
    fi
}
```

**Commands**:
```bash
# View current debug function
sed -n '17,22p' shell/anvs.sh

# After editing, verify changes
grep -n "__xvn_debug" shell/anvs.sh  # Should return no results (we'll update all calls later)
grep -n "XVN_DEBUG" shell/anvs.sh    # Should return no results
grep -n "__anvs_debug" shell/anvs.sh # Should show function definition
grep -n "ANVS_DEBUG" shell/anvs.sh   # Should show variable reference
```

**Actions**:
- [ ] Update comment on line 17: `XVN_DEBUG=1` → `ANVS_DEBUG=1`
- [ ] Rename function from `__xvn_debug` to `__anvs_debug` (line 18)
- [ ] Update variable check: `XVN_DEBUG` → `ANVS_DEBUG` (line 19)
- [ ] Update echo prefix: `[xvn]` → `[anvs]` (line 20)
- [ ] Verify function definition is correct
- [ ] Note: Function calls will be updated in a later task

---

### Task 3.5: Rename Find File Function and Update Variable

**File**: `shell/anvs.sh`

**Lines**: 24-54

**Changes Required**:

**Function name change**:
- `__xvn_find_file` → `__anvs_find_file`

**Environment variable change**:
- `XVN_VERSION_FILES` → `ANVS_VERSION_FILES`

**Debug function calls**:
- `__xvn_debug` → `__anvs_debug` (3 occurrences)

**Before**:
```bash
# Find version file by walking up directory tree
# Usage: __xvn_find_file <start_path>
# Returns: Full path to version file, or empty string if not found
__xvn_find_file() {
    local current_dir="${1:-$PWD}"
    local search_files="${XVN_VERSION_FILES:-.nvmrc .node-version}"

    __xvn_debug "Searching for version files: $search_files"

    while [[ "$current_dir" != "/" ]]; do
        # ... (omitted for brevity)
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

**After**:
```bash
# Find version file by walking up directory tree
# Usage: __anvs_find_file <start_path>
# Returns: Full path to version file, or empty string if not found
__anvs_find_file() {
    local current_dir="${1:-$PWD}"
    local search_files="${ANVS_VERSION_FILES:-.nvmrc .node-version}"

    __anvs_debug "Searching for version files: $search_files"

    while [[ "$current_dir" != "/" ]]; do
        # ... (omitted for brevity)
        for filename in $search_files; do
            local filepath="$current_dir/$filename"
            if [[ -f "$filepath" ]]; then
                __anvs_debug "Found version file: $filepath"
                echo "$filepath"
                return 0
            fi
        done
        current_dir="$(dirname "$current_dir")"
    done

    __anvs_debug "No version file found"
    return 1
}
```

**Commands**:
```bash
# View current function
sed -n '24,54p' shell/anvs.sh

# After editing, search for remaining old names
grep -n "__xvn_find_file" shell/anvs.sh  # Should return no results
grep -n "XVN_VERSION_FILES" shell/anvs.sh # Should return no results

# Verify new names exist
grep -n "__anvs_find_file" shell/anvs.sh  # Should show function definition + calls
grep -n "ANVS_VERSION_FILES" shell/anvs.sh # Should show variable usage
```

**Actions**:
- [ ] Update function usage comment (line 25): `__xvn_find_file` → `__anvs_find_file`
- [ ] Rename function (line 27): `__xvn_find_file` → `__anvs_find_file`
- [ ] Update variable (line 29): `XVN_VERSION_FILES` → `ANVS_VERSION_FILES`
- [ ] Update debug call (line 31): `__xvn_debug` → `__anvs_debug`
- [ ] Update debug call (line 44): `__xvn_debug` → `__anvs_debug`
- [ ] Update debug call (line 52): `__xvn_debug` → `__anvs_debug`
- [ ] Verify all function references updated
- [ ] Note: Function calls from other functions will be updated in later tasks

---

### Task 3.6: Rename Activate Function and Update Variables

**File**: `shell/anvs.sh`

**Lines**: 56-102

**Changes Required**:

**Function name change**:
- `__xvn_activate` → `__anvs_activate`

**Environment variable changes**:
- `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY` (4 occurrences)

**Binary invocation change**:
- `xvn activate` → `anvs activate` (1 occurrence)

**Debug function calls**:
- `__xvn_debug` → `__anvs_debug` (5 occurrences)

**Before**:
```bash
# Activate version for a given path
# Usage: __xvn_activate <version_file_path>
__xvn_activate() {
    local version_file="$1"

    # ... idempotency check ...
    local version_hash
    version_hash=$(cksum "$version_file" 2>/dev/null | cut -d' ' -f1)
    local active_key="${version_file}:${version_hash}"

    if [[ "${XVN_ACTIVE_KEY:-}" == "$active_key" ]]; then
        __xvn_debug "Already activated for $version_file (hash: $version_hash), skipping"
        return 0
    fi

    __xvn_debug "Activating version from $version_file"

    # Call xvn binary with FD:3 protocol
    # ... (FD:3 explanation omitted) ...
    local commands
    commands=$(xvn activate "$(dirname "$version_file")" 3>&1 1>&2 2>&3) || {
        # Activation failed, but don't break the shell
        __xvn_debug "Activation failed (exit code $?)"
        return 1
    }

    if [[ -n "$commands" ]]; then
        __xvn_debug "Evaluating commands: $commands"
        eval "$commands"
        export XVN_ACTIVE_KEY="$active_key"
    else
        __xvn_debug "No commands returned"
    fi
}
```

**After**:
```bash
# Activate version for a given path
# Usage: __anvs_activate <version_file_path>
__anvs_activate() {
    local version_file="$1"

    # ... idempotency check ...
    local version_hash
    version_hash=$(cksum "$version_file" 2>/dev/null | cut -d' ' -f1)
    local active_key="${version_file}:${version_hash}"

    if [[ "${ANVS_ACTIVE_KEY:-}" == "$active_key" ]]; then
        __anvs_debug "Already activated for $version_file (hash: $version_hash), skipping"
        return 0
    fi

    __anvs_debug "Activating version from $version_file"

    # Call anvs binary with FD:3 protocol
    # ... (FD:3 explanation omitted) ...
    local commands
    commands=$(anvs activate "$(dirname "$version_file")" 3>&1 1>&2 2>&3) || {
        # Activation failed, but don't break the shell
        __anvs_debug "Activation failed (exit code $?)"
        return 1
    }

    if [[ -n "$commands" ]]; then
        __anvs_debug "Evaluating commands: $commands"
        eval "$commands"
        export ANVS_ACTIVE_KEY="$active_key"
    else
        __anvs_debug "No commands returned"
    fi
}
```

**Commands**:
```bash
# View current function
sed -n '56,102p' shell/anvs.sh

# After editing, check for old references
grep -n "__xvn_activate" shell/anvs.sh  # Should return no results
grep -n "XVN_ACTIVE_KEY" shell/anvs.sh  # Should return no results
grep -n "xvn activate" shell/anvs.sh    # Should return no results

# Verify new references exist
grep -n "__anvs_activate" shell/anvs.sh  # Should show function + calls
grep -n "ANVS_ACTIVE_KEY" shell/anvs.sh  # Should show 4 occurrences
grep -n "anvs activate" shell/anvs.sh    # Should show 2 occurrences
```

**Actions**:
- [ ] Update function usage comment (line 57): `__xvn_activate` → `__anvs_activate`
- [ ] Rename function (line 58): `__xvn_activate` → `__anvs_activate`
- [ ] Update variable check (line 71): `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`
- [ ] Update debug call (line 72): `__xvn_debug` → `__anvs_debug`
- [ ] Update debug call (line 76): `__xvn_debug` → `__anvs_debug`
- [ ] Update comment (line 78): "Call xvn binary" → "Call anvs binary"
- [ ] Update binary call (line 89): `xvn activate` → `anvs activate`
- [ ] Update debug call (line 92): `__xvn_debug` → `__anvs_debug`
- [ ] Update debug call (line 97): `__xvn_debug` → `__anvs_debug`
- [ ] Update export (line 98): `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`
- [ ] Update debug call (line 100): `__xvn_debug` → `__anvs_debug`
- [ ] Verify all references updated

---

### Task 3.7: Rename Main Hook Function and Update Variables

**File**: `shell/anvs.sh`

**Lines**: 104-135

**Changes Required**:

**Function name change**:
- `__xvn_chpwd` → `__anvs_chpwd`

**Function calls**:
- `__xvn_debug` → `__anvs_debug` (5 occurrences)
- `__xvn_find_file` → `__anvs_find_file` (1 occurrence)
- `__xvn_activate` → `__anvs_activate` (1 occurrence)

**Environment variable**:
- `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY` (2 occurrences)

**Binary invocation**:
- `xvn activate` → `anvs activate` (1 occurrence)

**Before**:
```bash
# Main hook function called on directory change
__xvn_chpwd() {
    __xvn_debug "Directory changed to: $PWD"

    local version_file
    if version_file=$(__xvn_find_file "$PWD"); then
        __xvn_activate "$version_file"
    else
        # No version file found - switch to default version if configured
        if [[ -n "${XVN_ACTIVE_KEY:-}" ]]; then
            __xvn_debug "Left project directory, switching to default version"

            # Call xvn activate with --use-default flag
            # ... (explanation omitted) ...
            local commands
            commands=$(xvn activate "$PWD" --use-default 3>&1 1>&2 2>&3) || {
                __xvn_debug "Default version activation failed (exit code $?)"
                # Clear active key even if activation fails
                unset XVN_ACTIVE_KEY
                return 0
            }

            if [[ -n "$commands" ]]; then
                __xvn_debug "Evaluating default activation commands: $commands"
                eval "$commands"
            fi

            # Clear active key to allow re-activation if entering another project
            unset XVN_ACTIVE_KEY
        fi
    fi
}
```

**After**:
```bash
# Main hook function called on directory change
__anvs_chpwd() {
    __anvs_debug "Directory changed to: $PWD"

    local version_file
    if version_file=$(__anvs_find_file "$PWD"); then
        __anvs_activate "$version_file"
    else
        # No version file found - switch to default version if configured
        if [[ -n "${ANVS_ACTIVE_KEY:-}" ]]; then
            __anvs_debug "Left project directory, switching to default version"

            # Call anvs activate with --use-default flag
            # ... (explanation omitted) ...
            local commands
            commands=$(anvs activate "$PWD" --use-default 3>&1 1>&2 2>&3) || {
                __anvs_debug "Default version activation failed (exit code $?)"
                # Clear active key even if activation fails
                unset ANVS_ACTIVE_KEY
                return 0
            }

            if [[ -n "$commands" ]]; then
                __anvs_debug "Evaluating default activation commands: $commands"
                eval "$commands"
            fi

            # Clear active key to allow re-activation if entering another project
            unset ANVS_ACTIVE_KEY
        fi
    fi
}
```

**Commands**:
```bash
# View current function
sed -n '104,135p' shell/anvs.sh

# After editing, check for old references
grep -n "__xvn_chpwd" shell/anvs.sh     # Should only show in shell integration sections
grep -n "xvn activate" shell/anvs.sh     # Should return no results
grep -n "XVN_ACTIVE_KEY" shell/anvs.sh   # Should return no results

# Verify new references
grep -n "__anvs_chpwd" shell/anvs.sh     # Should show function + integration hooks
grep -n "anvs activate" shell/anvs.sh    # Should show 2 occurrences total
grep -n "ANVS_ACTIVE_KEY" shell/anvs.sh  # Should show all occurrences
```

**Actions**:
- [ ] Rename function (line 105): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Update debug call (line 106): `__xvn_debug` → `__anvs_debug`
- [ ] Update function call (line 109): `__xvn_find_file` → `__anvs_find_file`
- [ ] Update function call (line 110): `__xvn_activate` → `__anvs_activate`
- [ ] Update variable check (line 113): `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`
- [ ] Update debug call (line 114): `__xvn_debug` → `__anvs_debug`
- [ ] Update comment (line 116): "Call xvn activate" → "Call anvs activate"
- [ ] Update binary call (line 119): `xvn activate` → `anvs activate`
- [ ] Update debug call (line 120): `__xvn_debug` → `__anvs_debug`
- [ ] Update unset (line 122): `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`
- [ ] Update debug call (line 127): `__xvn_debug` → `__anvs_debug`
- [ ] Update unset (line 132): `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`
- [ ] Verify all references updated

---

### Task 3.8: Update Bash Integration Section

**File**: `shell/anvs.sh`

**Lines**: 137-171

**Changes Required**:

**Function names**:
- `__xvn_debug` → `__anvs_debug` (1 occurrence)
- `__xvn_original_cd` → `__anvs_original_cd` (3 occurrences)
- `__xvn_original_pushd` → `__anvs_original_pushd` (2 occurrences)
- `__xvn_original_popd` → `__anvs_original_popd` (2 occurrences)
- `__xvn_chpwd` → `__anvs_chpwd` (4 occurrences)

**Before**:
```bash
# Bash-specific integration
if [[ -n "${BASH_VERSION:-}" ]]; then
    __xvn_debug "Detected bash shell"

    # Bash doesn't have native chpwd support, so we wrap cd, pushd, popd
    if ! declare -f __xvn_original_cd > /dev/null; then
        # Only wrap once - store original builtin as function
        __xvn_original_cd() { builtin cd "$@" || return; }

        cd() {
            __xvn_original_cd "$@" || return $?
            __xvn_chpwd
        }

        # Also wrap pushd and popd if they exist
        if declare -f pushd > /dev/null 2>&1 || command -v pushd > /dev/null 2>&1; then
            __xvn_original_pushd() { builtin pushd "$@" || return; }
            pushd() {
                __xvn_original_pushd "$@" || return $?
                __xvn_chpwd
            }
        fi

        if declare -f popd > /dev/null 2>&1 || command -v popd > /dev/null 2>&1; then
            __xvn_original_popd() { builtin popd "$@" || return; }
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

**After**:
```bash
# Bash-specific integration
if [[ -n "${BASH_VERSION:-}" ]]; then
    __anvs_debug "Detected bash shell"

    # Bash doesn't have native chpwd support, so we wrap cd, pushd, popd
    if ! declare -f __anvs_original_cd > /dev/null; then
        # Only wrap once - store original builtin as function
        __anvs_original_cd() { builtin cd "$@" || return; }

        cd() {
            __anvs_original_cd "$@" || return $?
            __anvs_chpwd
        }

        # Also wrap pushd and popd if they exist
        if declare -f pushd > /dev/null 2>&1 || command -v pushd > /dev/null 2>&1; then
            __anvs_original_pushd() { builtin pushd "$@" || return; }
            pushd() {
                __anvs_original_pushd "$@" || return $?
                __anvs_chpwd
            }
        fi

        if declare -f popd > /dev/null 2>&1 || command -v popd > /dev/null 2>&1; then
            __anvs_original_popd() { builtin popd "$@" || return; }
            popd() {
                __anvs_original_popd "$@" || return $?
                __anvs_chpwd
            }
        fi
    fi

    # Trigger on shell startup
    __anvs_chpwd
fi
```

**Commands**:
```bash
# View current section
sed -n '137,171p' shell/anvs.sh

# After editing, verify all function names updated
grep -n "__xvn_original" shell/anvs.sh  # Should return no results
grep -n "__anvs_original" shell/anvs.sh # Should show cd, pushd, popd wrappers
```

**Actions**:
- [ ] Update debug call (line 139): `__xvn_debug` → `__anvs_debug`
- [ ] Update check (line 142): `__xvn_original_cd` → `__anvs_original_cd`
- [ ] Rename function (line 144): `__xvn_original_cd` → `__anvs_original_cd`
- [ ] Update call (line 147): `__xvn_original_cd` → `__anvs_original_cd`
- [ ] Update call (line 148): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Rename function (line 153): `__xvn_original_pushd` → `__anvs_original_pushd`
- [ ] Update call (line 155): `__xvn_original_pushd` → `__anvs_original_pushd`
- [ ] Update call (line 156): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Rename function (line 161): `__xvn_original_popd` → `__anvs_original_popd`
- [ ] Update call (line 163): `__xvn_original_popd` → `__anvs_original_popd`
- [ ] Update call (line 164): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Update startup trigger (line 170): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Verify all bash integration updated

---

### Task 3.9: Update Zsh Integration Section

**File**: `shell/anvs.sh`

**Lines**: 173-185

**Changes Required**:

**Function names**:
- `__xvn_debug` → `__anvs_debug` (1 occurrence)
- `__xvn_chpwd` → `__anvs_chpwd` (3 occurrences)

**Before**:
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

**After**:
```bash
# Zsh-specific integration
if [[ -n "${ZSH_VERSION:-}" ]]; then
    __anvs_debug "Detected zsh shell"

    # Zsh has native chpwd_functions support
    if [[ -z "${chpwd_functions[(r)__anvs_chpwd]}" ]]; then
        chpwd_functions+=(__anvs_chpwd)
    fi

    # Trigger on shell startup
    __anvs_chpwd
fi
```

**Commands**:
```bash
# View current section
sed -n '173,185p' shell/anvs.sh

# After editing, verify all function names updated
grep -n "__xvn_chpwd" shell/anvs.sh   # Should return no results
grep -n "__anvs_chpwd" shell/anvs.sh  # Should show zsh integration
```

**Actions**:
- [ ] Update debug call (line 175): `__xvn_debug` → `__anvs_debug`
- [ ] Update array check (line 178): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Update array append (line 179): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Update startup trigger (line 183): `__xvn_chpwd` → `__anvs_chpwd`
- [ ] Verify all zsh integration updated

---

### Task 3.10: Final Verification - Search for Remaining XVN References

**File**: `shell/anvs.sh`

**Search commands**:

```bash
# Search for any remaining "xvn" references (case-insensitive)
grep -in "xvn" shell/anvs.sh

# Expected: No results (all should be "anvs" now)

# Search for any XVN_ environment variables
grep -n "XVN_" shell/anvs.sh

# Expected: No results

# Search for __xvn_ function names
grep -n "__xvn_" shell/anvs.sh

# Expected: No results

# Verify all ANVS references are present
grep -c "ANVS" shell/anvs.sh

# Expected: Should show multiple occurrences (at least 15+)

# Count function definitions
grep -c "^__anvs_" shell/anvs.sh

# Expected: Should show 5 functions (debug, find_file, activate, chpwd, original_cd/pushd/popd)
```

**Actions**:
- [ ] No "xvn" string found (case-insensitive search)
- [ ] No "XVN_" environment variables found
- [ ] No "__xvn_" function names found
- [ ] All "ANVS" references present and correct
- [ ] 5 main functions renamed: `__anvs_debug`, `__anvs_find_file`, `__anvs_activate`, `__anvs_chpwd`, `__anvs_original_*`
- [ ] 4 environment variables renamed: `ANVS_SHELL_LOADED`, `ANVS_DEBUG`, `ANVS_VERSION_FILES`, `ANVS_ACTIVE_KEY`
- [ ] Binary invocations changed to `anvs activate`
- [ ] All comments and documentation updated

---

### Task 3.11: Test Shell Script Syntax

**Bash syntax validation**:

```bash
# Test bash syntax
bash -n shell/anvs.sh

# Expected: No output (silence means success)

# If errors, they'll show like:
# shell/anvs.sh: line XX: syntax error near unexpected token `...'
```

**Zsh syntax validation**:

```bash
# Test zsh syntax
zsh -n shell/anvs.sh

# Expected: No output (silence means success)
```

**Shellcheck linting** (if available):

```bash
# Run shellcheck
shellcheck shell/anvs.sh

# Address any warnings or errors
# Note: Some warnings may be intentional (like word splitting on line 41)
```

**Actions**:
- [ ] Bash syntax check passes: `bash -n shell/anvs.sh`
- [ ] Zsh syntax check passes: `zsh -n shell/anvs.sh`
- [ ] Shellcheck passes or warnings are documented/suppressed
- [ ] No syntax errors introduced during renaming

---

### Task 3.12: Manual Inspection and Quality Check

**Review critical sections**:

```bash
# 1. Check all function definitions
grep "^__anvs_" shell/anvs.sh

# Expected output:
# __anvs_debug() {
# __anvs_find_file() {
# __anvs_activate() {
# __anvs_chpwd() {
# __anvs_original_cd() { builtin cd "$@" || return; }
# __anvs_original_pushd() { builtin pushd "$@" || return; }
# __anvs_original_popd() { builtin popd "$@" || return; }

# 2. Check all environment variables
grep -o "ANVS_[A-Z_]*" shell/anvs.sh | sort -u

# Expected output:
# ANVS_ACTIVE_KEY
# ANVS_DEBUG
# ANVS_SHELL_LOADED
# ANVS_VERSION_FILES

# 3. Check binary invocations
grep "anvs activate" shell/anvs.sh

# Expected: Should show 2 occurrences (one normal, one with --use-default)

# 4. Verify comments and documentation
head -n 10 shell/anvs.sh
grep -n "^#" shell/anvs.sh | head -20
```

**Actions**:
- [ ] All 5 function names correct (`__anvs_*`)
- [ ] All 4 environment variables correct (`ANVS_*`)
- [ ] Binary invocations correct (`anvs activate`)
- [ ] Comments and documentation accurate
- [ ] File header updated
- [ ] No typos or formatting issues
- [ ] Line breaks and spacing preserved

---

### Task 3.13: Commit Changes

**Stage and commit all changes from Phase 3**:

```bash
# Check status
git status

# Should show:
# renamed: shell/xvn.sh -> shell/anvs.sh

# Review changes
git diff --staged shell/anvs.sh

# Stage changes
git add shell/anvs.sh

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
feat(shell): rename shell integration script for anvs

- Rename shell/xvn.sh → shell/anvs.sh
- Update all function names: __xvn_* → __anvs_*
  - __xvn_debug → __anvs_debug
  - __xvn_find_file → __anvs_find_file
  - __xvn_activate → __anvs_activate
  - __xvn_chpwd → __anvs_chpwd
  - __xvn_original_cd/pushd/popd → __anvs_original_cd/pushd/popd
- Update all environment variables: XVN_* → ANVS_*
  - XVN_SHELL_LOADED → ANVS_SHELL_LOADED
  - XVN_DEBUG → ANVS_DEBUG
  - XVN_VERSION_FILES → ANVS_VERSION_FILES
  - XVN_ACTIVE_KEY → ANVS_ACTIVE_KEY
- Update binary invocations: xvn activate → anvs activate
- Update file header and all comments
- Update debug output prefix: [xvn] → [anvs]

Files changed:
- shell/xvn.sh → shell/anvs.sh

Part of Phase 3: Shell Integration
EOF
)"
```

**Verify commit**:

```bash
# Check commit was created
git log -1 --oneline

# View commit details
git show HEAD --stat
```

**Actions**:
- [ ] Changes staged
- [ ] Commit message follows conventional commit format
- [ ] Commit message lists all changed functions and variables
- [ ] Commit references "Part of Phase 3: Shell Integration"
- [ ] Commit created successfully
- [ ] Git history clean and readable

---

## Verification Checklist

Before proceeding to Phase 4, verify ALL of the following:

### File and Naming
- [ ] File renamed: `shell/xvn.sh` → `shell/anvs.sh`
- [ ] Old file no longer exists: `shell/xvn.sh`
- [ ] New file exists and is readable: `shell/anvs.sh`
- [ ] File permissions preserved (executable if needed)

### Function Names (5 functions)
- [ ] `__xvn_debug` → `__anvs_debug`
- [ ] `__xvn_find_file` → `__anvs_find_file`
- [ ] `__xvn_activate` → `__anvs_activate`
- [ ] `__xvn_chpwd` → `__anvs_chpwd`
- [ ] `__xvn_original_cd` → `__anvs_original_cd` (and pushd/popd variants)

### Environment Variables (4 variables)
- [ ] `XVN_SHELL_LOADED` → `ANVS_SHELL_LOADED`
- [ ] `XVN_DEBUG` → `ANVS_DEBUG`
- [ ] `XVN_VERSION_FILES` → `ANVS_VERSION_FILES`
- [ ] `XVN_ACTIVE_KEY` → `ANVS_ACTIVE_KEY`

### Binary Invocations
- [ ] All `xvn activate` calls → `anvs activate`
- [ ] No remaining `xvn` binary invocations

### Documentation and Comments
- [ ] File header updated (line 2)
- [ ] Debug output prefix updated: `[xvn]` → `[anvs]`
- [ ] All function usage comments updated
- [ ] All inline comments referencing xvn updated

### Code Quality
- [ ] Bash syntax valid: `bash -n shell/anvs.sh`
- [ ] Zsh syntax valid: `zsh -n shell/anvs.sh`
- [ ] Shellcheck passes (or warnings documented)
- [ ] No syntax errors introduced

### Completeness
- [ ] No "xvn" strings remain (case-insensitive search)
- [ ] No "XVN_" constants remain
- [ ] No "__xvn_" function names remain
- [ ] All changes committed to git

---

## Success Criteria

Phase 3 is complete when:

1. ✅ Shell script renamed from `shell/xvn.sh` to `shell/anvs.sh`
2. ✅ All 5 function names updated to use `__anvs_` prefix
3. ✅ All 4 environment variables updated to use `ANVS_` prefix
4. ✅ All binary invocations changed from `xvn` to `anvs`
5. ✅ All comments and documentation updated
6. ✅ Shell syntax is valid (bash and zsh)
7. ✅ No references to "xvn" remain in the file
8. ✅ Changes committed with descriptive message

---

## Next Steps

After completing Phase 3:

1. **Do NOT test shell integration yet** - The Rust code still references old names
2. **Proceed to Phase 4**: Update Rust source code (all `src/` files)
3. **After Phase 4**: Build the new binary and test shell integration end-to-end
4. **Integration testing**: After all phases complete, verify that:
   - `anvs setup` correctly installs `shell/anvs.sh`
   - Shell sources `~/.anvs/bin/anvs.sh` correctly
   - Directory change triggers `__anvs_chpwd`
   - Binary activation works with `anvs activate`

---

## Rollback Plan

If issues are discovered:

1. **Revert git commit**: `git revert <commit-hash>` for Phase 3 changes
2. **Restore original file**: `git checkout HEAD~1 shell/anvs.sh`
3. **Restore original filename**: `git mv shell/anvs.sh shell/xvn.sh`
4. **Fix issues**: Address problems in a new commit
5. **Re-test**: Verify fixes before proceeding
6. **Re-commit**: Create new Phase 3 commit with fixes

**Emergency restore**:
```bash
# If needed, restore from git history
git log --all --full-history -- shell/xvn.sh
git show <commit-hash>:shell/xvn.sh > shell/xvn.sh
```

---

## Notes

### Critical Considerations
- The shell script is sourced into user shells, so errors will affect their terminal sessions
- Typos in function names will cause "command not found" errors in user shells
- Typos in environment variables will break idempotency checks
- The script must work in both bash and zsh without modification

### Integration Points
- This script is installed by `install.js` to `~/.anvs/bin/anvs.sh`
- User's shell profile (`.bashrc`/`.zshrc`) sources this script via setup
- The script invokes the `anvs` binary (updated in Phase 2)
- Rust code (Phase 4) will need to reference `shell/anvs.sh` for setup

### Testing Strategy
- Syntax testing can be done immediately (bash/zsh -n)
- Functional testing must wait until:
  - Phase 4 completes (Rust code updated)
  - Binary is rebuilt with new names
  - Installation scripts place file in correct location
- End-to-end testing requires complete rename (all phases)

### Common Pitfalls to Avoid
- Don't forget the underscore prefixes: `__anvs_` not `_anvs_` or `anvs_`
- Don't mix old and new names (all must be updated together)
- Don't modify shell script logic, only names/references
- Don't break bash/zsh compatibility (test both)
- Don't introduce trailing spaces or formatting changes

### Performance Considerations
- This script runs on every `cd`, so it must be fast
- Function renames don't affect performance
- No new logic is being added, just renaming

---

## Appendix: Complete Rename Mapping

### Functions (7 total)
| Old Name                | New Name                | Occurrences |
|-------------------------|-------------------------|-------------|
| `__xvn_debug`           | `__anvs_debug`          | 12+         |
| `__xvn_find_file`       | `__anvs_find_file`      | 4+          |
| `__xvn_activate`        | `__anvs_activate`       | 3+          |
| `__xvn_chpwd`           | `__anvs_chpwd`          | 8+          |
| `__xvn_original_cd`     | `__anvs_original_cd`    | 3           |
| `__xvn_original_pushd`  | `__anvs_original_pushd` | 2           |
| `__xvn_original_popd`   | `__anvs_original_popd`  | 2           |

### Environment Variables (4 total)
| Old Name              | New Name              | Occurrences |
|-----------------------|-----------------------|-------------|
| `XVN_SHELL_LOADED`    | `ANVS_SHELL_LOADED`   | 2           |
| `XVN_DEBUG`           | `ANVS_DEBUG`          | 2           |
| `XVN_VERSION_FILES`   | `ANVS_VERSION_FILES`  | 1           |
| `XVN_ACTIVE_KEY`      | `ANVS_ACTIVE_KEY`     | 4           |

### Binary Invocations (2 total)
| Old Command                            | New Command                            |
|----------------------------------------|----------------------------------------|
| `xvn activate "$path" 3>&1 1>&2 2>&3`  | `anvs activate "$path" 3>&1 1>&2 2>&3` |
| `xvn activate "$PWD" --use-default ...`| `anvs activate "$PWD" --use-default ...`|

### Debug Output
| Old Prefix | New Prefix |
|------------|------------|
| `[xvn]`    | `[anvs]`   |

---

**Phase 3 Complete** ✅
