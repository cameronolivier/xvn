#!/usr/bin/env bash
# anvs.sh - Shell integration for anvs (Automatic Node Version Switcher)
# This script hooks into directory change events and activates Node.js versions

# Error handling
# NOTE: Do NOT use 'set -e' or 'set -u' in sourced scripts:
# - 'set -e' will terminate the user's shell on any error
# - 'set -u' causes warnings for unset shell initialization variables
# Instead, handle errors explicitly in functions and use ${VAR:-default} for variables

# Prevent multiple initialization
if [[ -n "${ANVS_SHELL_LOADED:-}" ]]; then
    return 0
fi
export ANVS_SHELL_LOADED=1

# Debug logging (enabled via ANVS_DEBUG=1)
__anvs_debug() {
    if [[ "${ANVS_DEBUG:-0}" == "1" ]]; then
        echo "[anvs] $*" >&2
    fi
}

# Find version file by walking up directory tree
# Usage: __anvs_find_file <start_path>
# Returns: Full path to version file, or empty string if not found
__anvs_find_file() {
    local current_dir="${1:-$PWD}"
    local search_files="${ANVS_VERSION_FILES:-.nvmrc .node-version}"

    __anvs_debug "Searching for version files: $search_files"

    while [[ "$current_dir" != "/" ]]; do
        # Word splitting for zsh compatibility
        # In zsh, we need to enable word splitting with setopt
        # In bash, it's enabled by default
        if [[ -n "${ZSH_VERSION:-}" ]]; then
            setopt local_options sh_word_split
        fi
        # shellcheck disable=SC2086
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

# Activate version for a given path
# Usage: __anvs_activate <version_file_path>
__anvs_activate() {
    local version_file="$1"

    # Check if already activated for this file+version (idempotency)
    # This prevents re-activation when:
    # - User runs 'cd .' in same directory
    # - User cd's into subdirectory of same project
    # - Shell re-runs hook on prompt refresh
    # We check both the file path AND its content hash to detect version changes
    local version_hash
    version_hash=$(cksum "$version_file" 2>/dev/null | cut -d' ' -f1)
    local active_key="${version_file}:${version_hash}"

    if [[ "${ANVS_ACTIVE_KEY:-}" == "$active_key" ]]; then
        __anvs_debug "Already activated for $version_file (hash: $version_hash), skipping"
        return 0
    fi

    __anvs_debug "Activating version from $version_file"

    # Call anvs binary with FD:3 protocol
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
            # This will switch to the version manager's default version (e.g., nvm default)
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
