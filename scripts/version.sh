#!/bin/bash
# version.sh - Semantic version bumping script
#
# Usage:
#   ./scripts/version.sh patch   # 0.6.1 -> 0.6.2
#   ./scripts/version.sh minor   # 0.6.1 -> 0.7.0
#   ./scripts/version.sh major   # 0.6.1 -> 1.0.0
#   ./scripts/version.sh 1.2.3   # Set exact version

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <patch|minor|major|X.Y.Z>"
    echo "Examples:"
    echo "  $0 patch   # Bump patch version (0.6.1 -> 0.6.2)"
    echo "  $0 minor   # Bump minor version (0.6.1 -> 0.7.0)"
    echo "  $0 major   # Bump major version (0.6.1 -> 1.0.0)"
    echo "  $0 1.0.0   # Set exact version"
    exit 1
fi

BUMP_TYPE=$1

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: ${CURRENT_VERSION}"

# Parse current version
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Calculate new version
case "$BUMP_TYPE" in
    patch)
        NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))"
        ;;
    minor)
        NEW_VERSION="${MAJOR}.$((MINOR + 1)).0"
        ;;
    major)
        NEW_VERSION="$((MAJOR + 1)).0.0"
        ;;
    *)
        # Assume it's an exact version
        if [[ ! "$BUMP_TYPE" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo "Error: Invalid version format. Use X.Y.Z format or patch/minor/major"
            exit 1
        fi
        NEW_VERSION="$BUMP_TYPE"
        ;;
esac

echo "New version: ${NEW_VERSION}"
echo ""

# Confirm
read -p "Bump version from ${CURRENT_VERSION} to ${NEW_VERSION}? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Update Cargo.toml
echo "Updating Cargo.toml..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/^version = ".*"/version = "${NEW_VERSION}"/" Cargo.toml
else
    sed -i "s/^version = ".*"/version = "${NEW_VERSION}"/" Cargo.toml
fi

# Update package.json
echo "Updating package.json..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/"version": ".*"/"version": "${NEW_VERSION}"/" package.json
else
    sed -i "s/"version": ".*"/"version": "${NEW_VERSION}"/" package.json
fi

# Update CLI test
echo "Updating test..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/anvs [0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*/anvs ${NEW_VERSION}/" tests/cli_test.rs
else
    sed -i "s/anvs [0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*/anvs ${NEW_VERSION}/" tests/cli_test.rs
fi

# Update Cargo.lock
echo "Updating Cargo.lock..."
cargo build --quiet

# Run tests
echo "Running tests..."
if ! cargo test --quiet 2>&1 | grep -q "test result: ok"; then
    echo "Error: Tests failed after version bump!"
    exit 1
fi

# Git operations
echo "Creating git commit and tag..."
git add Cargo.toml Cargo.lock package.json tests/cli_test.rs

git commit -m "chore: bump version to v${NEW_VERSION}

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"

echo ""
echo "✅ Version bumped to v${NEW_VERSION}"
echo "✅ Commit created"
echo "✅ Tag v${NEW_VERSION} created"
echo ""
echo "Next steps:"
echo "  git push origin main"
echo "  git push origin v${NEW_VERSION}"

echo ""
echo "Or to push everything at once:"
echo "  git push && git push --tags"