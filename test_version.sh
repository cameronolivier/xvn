#!/bin/bash

set -x

BUMP_TYPE="minor"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | awk -F '"' '{print $2}')
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

sed -i '' "s/^version = ".*"/version = "${NEW_VERSION}"" Cargo.toml
