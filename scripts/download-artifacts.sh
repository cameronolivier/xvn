#!/usr/bin/env bash
# Download GitHub Actions artifacts for a release

set -e

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v1.4.1"
    exit 1
fi

# Remove 'v' prefix if present for comparison
VERSION_NUMBER="${VERSION#v}"

echo "🔍 Finding GitHub Actions run for $VERSION..."

# Get the latest run ID for this tag (success or failure, we just need artifacts)
RUN_ID=$(gh run list --workflow=build.yml --json databaseId,headBranch \
    --jq ".[] | select(.headBranch == \"$VERSION\") | .databaseId" \
    | head -1)

if [ -z "$RUN_ID" ]; then
    echo "❌ No build found for $VERSION"
    echo "Check: https://github.com/cameronolivier/xvn/actions"
    exit 1
fi

echo "✅ Found run ID: $RUN_ID"
echo "🔗 View at: https://github.com/cameronolivier/xvn/actions/runs/$RUN_ID"

# Create temp directory
TEMP_DIR="/tmp/xvn-${VERSION}-artifacts"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"

echo "📥 Downloading artifacts to $TEMP_DIR..."
gh run download "$RUN_ID" --dir "$TEMP_DIR"

echo "✅ Artifacts downloaded successfully"
echo "📂 Location: $TEMP_DIR"
