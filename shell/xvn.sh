#!/usr/bin/env bash
# xvn.sh - Shell integration for xvn (Extreme Version Switcher)
# This script hooks into directory change events and activates Node.js versions

# Error handling
# NOTE: Do NOT use 'set -e' or 'set -u' in sourced scripts:
# - 'set -e' will terminate the user's shell on any error
# - 'set -u' causes warnings for unset shell initialization variables
# Instead, handle errors explicitly in functions and use ${VAR:-default} for variables

# Prevent multiple initialization
if [[ -n "${XVN_SHELL_LOADED:-}" ]]; then
    return 0
fi
export XVN_SHELL_LOADED=1

# Debug logging (enabled via XVN_DEBUG=1)
__xvn_debug() {
    if [[ "${XVN_DEBUG:-0}" == "1" ]]; then
        echo "[xvn] $*" >&2
    fi
}

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

# Activate version for a given path
# Usage: __xvn_activate <version_file_path>
__xvn_activate() {
    local version_file="$1"

    # Check if already activated for this file (idempotency)
    # This prevents re-activation when:
    # - User runs 'cd .' in same directory
    # - User cd's into subdirectory of same project
    # - Shell re-runs hook on prompt refresh
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
