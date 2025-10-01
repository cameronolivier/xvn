#!/bin/bash
# bump-version.sh - Bump version for milestone completion
#
# Usage: ./scripts/bump-version.sh <milestone-number>
# Example: ./scripts/bump-version.sh 3
#
# This will bump the version from 0.2.0 to 0.3.0

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <milestone-number>"
    echo "Example: $0 3 (bumps to v0.3.0)"
    exit 1
fi

MILESTONE=$1
NEW_VERSION="0.${MILESTONE}.0"

# Validate milestone is a number
if ! [[ "$MILESTONE" =~ ^[0-9]+$ ]]; then
    echo "Error: Milestone must be a number"
    exit 1
fi

# Update Cargo.toml
echo "Bumping version to ${NEW_VERSION}..."
sed -i '' "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Verify the change
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Updated Cargo.toml to version: ${CURRENT_VERSION}"

# Build to update Cargo.lock
echo "Updating Cargo.lock..."
cargo build --quiet

# Git operations
echo "Creating git commit and tag..."
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to v${NEW_VERSION} for milestone ${MILESTONE} completion

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION} - Milestone ${MILESTONE} Complete"

echo ""
echo "✅ Version bumped to v${NEW_VERSION}"
echo "✅ Commit created"
echo "✅ Tag v${NEW_VERSION} created"
echo ""
echo "To push changes and tag:"
echo "  git push && git push --tags"
