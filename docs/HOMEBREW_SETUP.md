# Homebrew Distribution Setup Guide

This guide walks you through setting up Homebrew distribution for anvs.

## Prerequisites

- ✅ GitHub account (cameronolivier)
- ✅ Homebrew installed locally (`brew --version`)
- ✅ `gh` CLI installed and authenticated (`gh auth status`)

---

## Option A: Using Personal Account (cameronolivier)

### Step 1: Create Homebrew Tap Repository

```bash
# Create the tap repository
gh repo create cameronolivier/homebrew-anvs \
  --public \
  --description "Homebrew tap for anvs (Automatic Node Version Switcher)" \
  --clone

# Navigate to the repository
cd homebrew-anvs

# Create initial README
cat > README.md << 'EOF'
# Homebrew Tap for anvs

Official Homebrew tap for [anvs](https://github.com/olvrcc/anvs) - Automatic Node Version Switcher.

## Installation

```bash
brew tap olvrcc/anvs
brew install anvs
```

## Setup

After installation, run:

```bash
anvs setup
```

Then restart your shell.

## Requirements

- macOS (x64 or arm64)
- Node.js version manager (nvm or fnm)

## Documentation

See the main [anvs repository](https://github.com/olvrcc/anvs) for full documentation.
EOF

# Commit and push
git add README.md
git commit -m "docs: initial README for homebrew tap"
git push

# Go back to anvs directory
cd ..
```

### Step 2: Create Personal Access Token

```bash
# Option 1: Create via CLI (easier)
gh auth refresh -h github.com -s repo

# Option 2: Create via web UI
# 1. Go to: https://github.com/settings/tokens
# 2. Click "Generate new token (classic)"
# 3. Name: "Homebrew Tap Updates"
# 4. Select scopes:
#    ☑ repo (all sub-scopes)
# 5. Click "Generate token"
# 6. Copy the token
```

### Step 3: Add Token as Secret to anvs Repository

```bash
# Add the secret (will prompt for token value)
gh secret set HOMEBREW_TAP_TOKEN -R olvrcc/anvs

# Verify it was added
gh secret list -R olvrcc/anvs
```

---

## Option B: Using GitHub Organization (olvrcc) - RECOMMENDED

### Why Use an Organization?

- ✅ **FREE** for public repositories
- ✅ More professional branding (olvrcc/anvs)
- ✅ Better for multiple projects
- ✅ Easier to add collaborators later
- ✅ Matches your npm package (anvs)

### Step 1: Create GitHub Organization

```bash
# Option 1: Create via web UI (recommended for first org)
# 1. Go to: https://github.com/settings/organizations
# 2. Click "New organization"
# 3. Choose "Create a free organization"
# 4. Organization name: olvrcc
# 5. Contact email: cameronolivier@gmail.com
# 6. This organization belongs to: "My personal account"
# 7. Click "Next" and complete setup

# Option 2: Create via CLI (after you know the org name)
# gh org create olvrcc
```

### Step 2: Transfer anvs Repository to Organization (Optional)

If you want to move the main anvs repo to the org:

```bash
# Go to: https://github.com/cameronolivier/anvs/settings
# Scroll to "Danger Zone"
# Click "Transfer ownership"
# Enter: olvrcc
# Confirm transfer

# OR keep it under cameronolivier - both work fine!
```

### Step 3: Create Homebrew Tap Repository Under Organization

```bash
# Create the tap repository under the org
gh repo create olvrcc/homebrew-anvs \
  --public \
  --description "Homebrew tap for anvs (Automatic Node Version Switcher)" \
  --clone

# Navigate to the repository
cd homebrew-anvs

# Create initial README
cat > README.md << 'EOF'
# Homebrew Tap for anvs

Official Homebrew tap for [anvs](https://github.com/olvrcc/anvs) - Automatic Node Version Switcher.

## Installation

```bash
brew tap olvrcc/anvs
brew install anvs
```

## Setup

After installation, run:

```bash
anvs setup
```

Then restart your shell.

## Requirements

- macOS (x64 or arm64)
- Node.js version manager (nvm or fnm)

## Documentation

See the main [anvs repository](https://github.com/olvrcc/anvs) for full documentation.
EOF

# Commit and push
git add README.md
git commit -m "docs: initial README for homebrew tap"
git push

# Go back to anvs directory
cd ..
```

### Step 4: Create Personal Access Token for Organization

```bash
# Create a fine-grained token with access to the organization
# 1. Go to: https://github.com/settings/tokens?type=beta
# 2. Click "Generate new token"
# 3. Token name: "Homebrew Tap Updates"
# 4. Expiration: 90 days (or custom)
# 5. Resource owner: olvrcc
# 6. Repository access: "Only select repositories"
# 7. Select: olvrcc/homebrew-anvs
# 8. Permissions:
#    - Contents: Read and write
#    - Metadata: Read-only (automatic)
# 9. Click "Generate token"
# 10. Copy the token

# OR use classic token with full repo access:
gh auth refresh -h github.com -s repo
```

### Step 5: Add Token as Secret to anvs Repository

```bash
# If anvs is still under cameronolivier:
gh secret set HOMEBREW_TAP_TOKEN -R cameronolivier/anvs

# If you transferred anvs to olvrcc:
gh secret set HOMEBREW_TAP_TOKEN -R olvrcc/anvs

# Verify it was added
gh secret list -R cameronolivier/anvs
# or
gh secret list -R olvrcc/anvs
```

---

## Verification Checklist

After setup, verify everything is ready:

```bash
# 1. Check tap repository exists
gh repo view olvrcc/homebrew-anvs
# or
gh repo view cameronolivier/homebrew-anvs

# 2. Check secret is added to anvs repository
gh secret list -R olvrcc/anvs
# Should show: HOMEBREW_TAP_TOKEN

# 3. Check Homebrew is installed
brew --version

# 4. Check gh CLI is authenticated
gh auth status
```

---

## Next Steps

Once setup is complete:

1. ✅ Create Homebrew formula (`homebrew/anvs.rb`)
2. ✅ Calculate SHA256 checksums for v2.0.0 binaries
3. ✅ Test formula locally
4. ✅ Copy formula to tap repository
5. ✅ Test tap installation
6. ✅ Set up automation workflow

See `spec/milestone-9/TASKS.md` for detailed implementation tasks.

---

## Troubleshooting

### "Repository not found" when creating tap

- Make sure you have write access to the organization
- Try creating the repo via web UI first

### "Permission denied" when pushing to tap

- Check that your personal access token has `repo` scope
- Verify the token is added as a secret: `gh secret list`

### "gh: command not found"

Install GitHub CLI:
```bash
brew install gh
gh auth login
```

---

## Cost Summary

- GitHub Organization: **FREE** for public repositories
- Homebrew Distribution: **FREE**
- Personal Access Token: **FREE**
- **Total Cost: $0** 🎉
