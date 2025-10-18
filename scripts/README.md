# Release Scripts

Scripts to automate the anvs release process.

## Prerequisites

- `gh` CLI installed and authenticated (`brew install gh`)
- Git tag pushed to GitHub (triggers GitHub Actions build)
- Successful GitHub Actions build completed

## Quick Release Process

After pushing a new version tag (e.g., `v2.0.0`):

```bash
# 1. Download and extract binaries
./scripts/download-artifacts.sh v2.0.0
./scripts/extract-binaries.sh v2.0.0

# 2. Create the npm package
npm run release:pack

# 3. Verify the package contents
npm run release:verify

# 4. Test installation locally (optional but recommended)
npm uninstall -g anvs
rm -rf ~/.anvs ~/.anvsrc
npm install -g ./anvs-2.0.0.tgz
anvs --version

# 5. Publish to npm
npm publish
```

## Scripts Reference

### download-artifacts.sh

Downloads GitHub Actions artifacts for a specific version.

```bash
./scripts/download-artifacts.sh v2.0.0
```

**What it does:**
- Finds the successful GitHub Actions run for the tag
- Downloads all build artifacts to `/tmp/anvs-v2.0.0-artifacts/`
- Shows the run URL for reference

### extract-binaries.sh

Extracts binaries from downloaded artifacts and copies them to `native/` directories.

```bash
./scripts/extract-binaries.sh v2.0.0
```

**What it does:**
- Extracts tarballs from `/tmp/anvs-v2.0.0-artifacts/`
- Copies binaries to `native/<platform>/anvs`
- Verifies each binary (shows version for macOS binaries)
- Supports platforms:
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)
  - `x86_64-unknown-linux-gnu` (Linux x64)
  - `aarch64-unknown-linux-gnu` (Linux arm64)

## Troubleshooting

### "No successful build found for version"

The GitHub Actions build may still be running or failed. Check:
```bash
gh run list --workflow=build.yml
```

Or visit: https://github.com/olvrcc/anvs/actions

### "Artifacts directory not found"

Run the download script first:
```bash
./scripts/download-artifacts.sh v2.0.0
```

### Binary version mismatch

Ensure Cargo.toml and package.json versions match the tag:
```bash
grep version Cargo.toml
grep version package.json
```

## What Gets Included in the Package

The `npm pack` command includes:
- ✅ `native/` directories with platform binaries (from GitHub Actions)
- ✅ `shell/` directory with shell integration scripts (from git)
- ✅ `bin/` directory with npm wrapper script (from git)
- ✅ `install.js` and `uninstall.js` (from git)

The `shell/` directory is tracked in git and listed in `package.json` files array, so it's automatically included even though GitHub Actions only builds binaries.
