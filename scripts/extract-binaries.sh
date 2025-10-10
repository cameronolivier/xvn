#!/usr/bin/env bash
# Extract downloaded artifacts and copy binaries to native/ directories

set -e

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v1.4.1"
    exit 1
fi

# Remove 'v' prefix if present
VERSION_NUMBER="${VERSION#v}"

TEMP_DIR="/tmp/xvn-${VERSION}-artifacts"

if [ ! -d "$TEMP_DIR" ]; then
    echo "❌ Artifacts directory not found: $TEMP_DIR"
    echo "Run: npm run download-artifacts $VERSION"
    exit 1
fi

echo "📦 Extracting binaries from $TEMP_DIR..."

# Platforms we support (not Windows for now)
PLATFORMS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

for platform in "${PLATFORMS[@]}"; do
    artifact_dir="$TEMP_DIR/xvn-$platform"
    tarball="$artifact_dir/xvn-$platform.tar.gz"

    if [ ! -f "$tarball" ]; then
        echo "⚠️  Warning: $tarball not found, skipping..."
        continue
    fi

    echo "  📂 Processing $platform..."

    # Create native directory if it doesn't exist
    mkdir -p "native/$platform"

    # Extract tarball
    tar -xzf "$tarball" -C "$artifact_dir"

    # Copy binary
    cp "$artifact_dir/xvn" "native/$platform/xvn"

    # Verify binary
    if [ -f "native/$platform/xvn" ]; then
        echo "  ✅ Copied to native/$platform/xvn"

        # Show version (macOS only since we can't run Linux binaries)
        if [[ "$platform" == *"apple-darwin"* ]]; then
            binary_version=$(./native/$platform/xvn --version 2>/dev/null || echo "unknown")
            echo "     Version: $binary_version"
        fi
    else
        echo "  ❌ Failed to copy binary for $platform"
        exit 1
    fi
done

echo ""
echo "✅ All binaries extracted successfully"
echo "📂 Binaries location: native/"
echo ""
echo "Next steps:"
echo "  1. Verify binaries: ls -lh native/*/"
echo "  2. Create package: npm pack"
echo "  3. Test install: npm install -g ./$(cat package.json | grep '\"name\"' | cut -d'"' -f4 | sed 's/@//' | sed 's/\//-/')-${VERSION_NUMBER}.tgz"
echo "  4. Publish: npm publish"
