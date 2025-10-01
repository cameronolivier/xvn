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
echo " Test 1: Checking function definitions..."
if ! declare -f __xvn_find_file > /dev/null; then
    echo " Function __xvn_find_file not defined"
    exit 1
fi

if ! declare -f __xvn_activate > /dev/null; then
    echo " Function __xvn_activate not defined"
    exit 1
fi

if ! declare -f __xvn_chpwd > /dev/null; then
    echo " Function __xvn_chpwd not defined"
    exit 1
fi

echo " All functions defined"

# Test 2: Environment variable set
echo " Test 2: Checking XVN_SHELL_LOADED..."
if [[ -z "${XVN_SHELL_LOADED:-}" ]]; then
    echo " XVN_SHELL_LOADED not set"
    exit 1
fi
echo " XVN_SHELL_LOADED is set"

# Test 3: Find file function
echo " Test 3: Testing __xvn_find_file..."

# Create temp directory with .nvmrc
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

mkdir -p "$TEMP_DIR/project/subdir"
echo "18.20.0" > "$TEMP_DIR/project/.nvmrc"

# Call __xvn_find_file directly without cd (to avoid triggering __xvn_chpwd)
result=$(__xvn_find_file "$TEMP_DIR/project/subdir" || echo "")

if [[ "$result" != "$TEMP_DIR/project/.nvmrc" ]]; then
    echo " Expected to find $TEMP_DIR/project/.nvmrc, got: $result"
    exit 1
fi

echo " __xvn_find_file works correctly"

# Test 4: Debug function
echo " Test 4: Testing __xvn_debug..."
XVN_DEBUG=1 __xvn_debug "test message" 2>&1 | grep -q "test message"
echo " __xvn_debug works"

echo ""
echo " All shell integration tests passed!"
