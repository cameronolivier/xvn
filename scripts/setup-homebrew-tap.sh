#!/usr/bin/env bash
# Setup script for Homebrew tap repository

set -e

echo "🍺 Homebrew Tap Setup Script"
echo ""

# Check if gh is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed"
    echo "Install it with: brew install gh"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Not authenticated with GitHub CLI"
    echo "Run: gh auth login"
    exit 1
fi

echo "✅ GitHub CLI is installed and authenticated"
echo ""

# Ask for organization name
read -p "Use GitHub organization? (y/n) [recommended: y]: " use_org

if [[ "$use_org" =~ ^[Yy]$ ]]; then
    read -p "Organization name (default: olvrcc): " org_name
    org_name=${org_name:-olvrcc}
    REPO_OWNER="$org_name"

    echo ""
    echo "📝 To create the organization:"
    echo "   1. Go to: https://github.com/settings/organizations"
    echo "   2. Click 'New organization'"
    echo "   3. Choose 'Create a free organization'"
    echo "   4. Organization name: $org_name"
    echo "   5. Complete the setup"
    echo ""
    read -p "Press Enter after creating the organization..."
else
    REPO_OWNER=$(gh api user -q .login)
    echo "Using personal account: $REPO_OWNER"
fi

echo ""
echo "📦 Creating homebrew-xvn repository under $REPO_OWNER..."

# Create the repository
if gh repo view "$REPO_OWNER/homebrew-xvn" &> /dev/null; then
    echo "⚠️  Repository $REPO_OWNER/homebrew-xvn already exists"
    read -p "Continue anyway? (y/n): " continue
    if [[ ! "$continue" =~ ^[Yy]$ ]]; then
        exit 0
    fi
else
    gh repo create "$REPO_OWNER/homebrew-xvn" \
        --public \
        --description "Homebrew tap for xvn (Extreme Version Switcher for Node.js)" \
        --clone

    echo "✅ Repository created: $REPO_OWNER/homebrew-xvn"
fi

# Clone or navigate to the repository
if [ -d "homebrew-xvn" ]; then
    cd homebrew-xvn
else
    gh repo clone "$REPO_OWNER/homebrew-xvn"
    cd homebrew-xvn
fi

# Create initial README if it doesn't exist
if [ ! -f "README.md" ]; then
    echo "📝 Creating README.md..."
    cat > README.md << EOF
# Homebrew Tap for xvn

Official Homebrew tap for [xvn](https://github.com/cameronolivier/xvn) - Extreme Version Switcher for Node.js.

## Installation

\`\`\`bash
brew tap $REPO_OWNER/xvn
brew install xvn
\`\`\`

## Setup

After installation, run:

\`\`\`bash
xvn setup
\`\`\`

Then restart your shell.

## Requirements

- macOS (x64 or arm64)
- Node.js version manager (nvm or fnm)

## Documentation

See the main [xvn repository](https://github.com/cameronolivier/xvn) for full documentation.
EOF

    git add README.md
    git commit -m "docs: initial README for homebrew tap"
    git push

    echo "✅ README.md created and pushed"
fi

cd ..

echo ""
echo "🔐 Setting up GitHub token..."
echo ""
echo "Choose token type:"
echo "  1. Fine-grained token (recommended, more secure)"
echo "  2. Classic token (simpler setup)"
read -p "Enter choice (1 or 2): " token_type

if [ "$token_type" = "1" ]; then
    echo ""
    echo "📝 Create a fine-grained token:"
    echo "   1. Go to: https://github.com/settings/tokens?type=beta"
    echo "   2. Click 'Generate new token'"
    echo "   3. Token name: Homebrew Tap Updates"
    echo "   4. Expiration: 90 days (or custom)"
    echo "   5. Resource owner: $REPO_OWNER"
    echo "   6. Repository access: Only select repositories"
    echo "   7. Select: $REPO_OWNER/homebrew-xvn"
    echo "   8. Permissions:"
    echo "      - Contents: Read and write"
    echo "   9. Click 'Generate token'"
    echo "   10. Copy the token"
else
    echo ""
    echo "📝 Create a classic token:"
    echo "   1. Go to: https://github.com/settings/tokens"
    echo "   2. Click 'Generate new token (classic)'"
    echo "   3. Token name: Homebrew Tap Updates"
    echo "   4. Select scopes: ☑ repo (all sub-scopes)"
    echo "   5. Click 'Generate token'"
    echo "   6. Copy the token"
fi

echo ""
read -p "Press Enter after creating the token, then paste it when prompted..."

# Determine xvn repository location
if gh repo view "olvrcc/xvn" &> /dev/null; then
    XVN_REPO="olvrcc/xvn"
elif gh repo view "cameronolivier/xvn" &> /dev/null; then
    XVN_REPO="cameronolivier/xvn"
else
    read -p "Enter xvn repository (owner/repo): " XVN_REPO
fi

echo ""
echo "📝 Adding secret to $XVN_REPO..."
gh secret set HOMEBREW_TAP_TOKEN -R "$XVN_REPO"

echo ""
echo "✅ Verifying secret was added..."
if gh secret list -R "$XVN_REPO" | grep -q "HOMEBREW_TAP_TOKEN"; then
    echo "✅ Secret HOMEBREW_TAP_TOKEN added successfully"
else
    echo "❌ Failed to add secret"
    exit 1
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Summary:"
echo "  - Tap repository: https://github.com/$REPO_OWNER/homebrew-xvn"
echo "  - Secret added to: https://github.com/$XVN_REPO"
echo ""
echo "Next steps:"
echo "  1. Create formula: homebrew/xvn.rb"
echo "  2. Test locally: brew install --build-from-source ./homebrew/xvn.rb"
echo "  3. Copy to tap: cp homebrew/xvn.rb ../homebrew-xvn/Formula/xvn.rb"
echo "  4. Set up automation: .github/workflows/update-homebrew.yml"
echo ""
echo "See docs/HOMEBREW_SETUP.md for detailed instructions."
