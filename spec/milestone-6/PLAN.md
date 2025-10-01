# Milestone 6: Release Preparation - Implementation Plan

## Overview

Milestone 6 establishes the complete CI/CD pipeline, binary distribution system, and npm packaging infrastructure needed to release xvn v1.0.0 to the public. This milestone transforms xvn from a development project into a production-ready tool that users can install via `npm install -g xvn` and immediately use.

**Key Goals:**
- Automate testing, building, and releases via GitHub Actions
- Build cross-platform binaries (Linux x64/arm64, macOS x64/arm64)
- Create npm package with automatic binary download
- Conduct beta testing with real users
- Release v1.0.0 to npm and announce publicly

**Approach:**
1. Start with CI/CD infrastructure (testing automation)
2. Add binary build automation with cross-compilation
3. Create npm packaging with install scripts
4. Test thoroughly on fresh systems
5. Beta test with real users, iterate on feedback
6. Release v1.0.0 and announce

---

## Prerequisites

**Required:**
- GitHub repository with push access
- npm account with publish access to `xvn` package name
- GitHub Actions enabled on repository
- Completed Milestones 1-5 (all core functionality working)

**Recommended:**
- Docker for testing on fresh systems
- Access to both x64 and arm64 test machines (or CI runners)
- List of potential beta testers (Node.js community members)

---

## Implementation Tasks

### Task M6.1: Set up CI/CD pipeline

**Objective:** Create GitHub Actions workflow for automated testing on every push/PR, with coverage reporting and multi-platform testing.

**Implementation Steps:**

1. **Create test workflow file:**
   ```bash
   mkdir -p .github/workflows
   touch .github/workflows/test.yml
   ```

2. **Define test matrix:**
   - Test on Ubuntu (latest) and macOS (latest)
   - Test with stable Rust (and optionally beta for early warning)
   - Run on every push to main and all PRs

3. **Add coverage reporting:**
   - Use `cargo-tarpaulin` for coverage
   - Upload to Coveralls or Codecov
   - Fail if coverage drops below threshold (e.g., 80%)

4. **Configure CI triggers:**
   - Push to `main` branch
   - All pull requests
   - Manual workflow dispatch for testing

**Code Structure:**

- File: `.github/workflows/test.yml`
```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:

jobs:
  test:
    name: Test on ${{ matrix.os }} with Rust ${{ matrix.rust }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable]

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --all-features --verbose

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Run coverage
        run: cargo tarpaulin --out Xml --all-features

      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml
          fail_ci_if_error: true
```

**Key Considerations:**
- Use caching to speed up CI runs (cargo registry, git index, target dir)
- Run clippy with `-D warnings` to treat warnings as errors
- Check formatting with `cargo fmt -- --check`
- Coverage should run on Ubuntu only (faster, and coverage is platform-independent)
- Use `fail-fast: false` to see all platform failures, not just the first

**Testing:**
- Push to a test branch and verify workflow runs
- Verify coverage report uploads successfully
- Test that PRs trigger the workflow
- Verify clippy and fmt checks work

**Dependencies:**
- None (this is the first task)

**Enables:**
- M6.2 (binary builds can extend this workflow)
- All future development (CI catches regressions)

---

### Task M6.2: Set up binary builds

**Objective:** Create GitHub Actions workflow to build cross-compiled binaries for all target platforms and upload them to GitHub Releases.

**Implementation Steps:**

1. **Create build workflow file:**
   ```bash
   touch .github/workflows/build.yml
   ```

2. **Define build matrix:**
   - Linux x64: `x86_64-unknown-linux-gnu`
   - Linux arm64: `aarch64-unknown-linux-gnu`
   - macOS x64: `x86_64-apple-darwin`
   - macOS arm64: `aarch64-apple-darwin`

3. **Set up cross-compilation:**
   - Use `cross` for Linux arm64 cross-compilation
   - Use native macOS runners for macOS builds
   - Install target toolchains

4. **Upload artifacts:**
   - Compress binaries with `tar.gz`
   - Generate SHA256 checksums
   - Upload to GitHub Release on tags

5. **Trigger on tags:**
   - Run on tags matching `v*` (e.g., `v0.7.0`, `v1.0.0`)
   - Also allow manual workflow dispatch for testing

**Code Structure:**

- File: `.github/workflows/build.yml`
```yaml
name: Build

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          # Linux x64
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            cross: false

          # Linux arm64
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            cross: true

          # macOS x64
          - target: x86_64-apple-darwin
            os: macos-latest
            cross: false

          # macOS arm64
          - target: aarch64-apple-darwin
            os: macos-latest
            cross: false

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross
        if: matrix.cross
        run: cargo install cross

      - name: Build binary
        run: |
          if [ "${{ matrix.cross }}" = "true" ]; then
            cross build --release --target ${{ matrix.target }}
          else
            cargo build --release --target ${{ matrix.target }}
          fi

      - name: Package binary
        run: |
          cd target/${{ matrix.target }}/release
          tar czf xvn-${{ matrix.target }}.tar.gz xvn
          shasum -a 256 xvn-${{ matrix.target }}.tar.gz > xvn-${{ matrix.target }}.tar.gz.sha256

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: xvn-${{ matrix.target }}
          path: |
            target/${{ matrix.target }}/release/xvn-${{ matrix.target }}.tar.gz
            target/${{ matrix.target }}/release/xvn-${{ matrix.target }}.tar.gz.sha256

  release:
    name: Create GitHub Release
    needs: build
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/')

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/**/*
          draft: false
          prerelease: ${{ contains(github.ref, 'beta') || contains(github.ref, 'alpha') }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

- File: `.cargo/config.toml` (for cross-compilation linker settings)
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

**Key Considerations:**
- Use `cross` for Linux arm64 to avoid complex Docker setup
- macOS runners can build both x64 and arm64 natively (with Xcode)
- Compress binaries to reduce download size
- Always generate checksums for security
- Mark pre-release versions (beta, alpha) as pre-release in GitHub
- Use `softprops/action-gh-release` for easy release creation

**Testing:**
- Manually trigger workflow with `workflow_dispatch`
- Create a test tag (e.g., `v0.7.0-test`) and push
- Verify all 4 binaries are built successfully
- Download and test each binary on target platform
- Verify checksums match

**Dependencies:**
- Requires: M6.1 (CI pipeline should be working first)

**Enables:**
- M6.3 (npm package can reference these binaries)

---

### Task M6.3: Create npm package structure

**Objective:** Build npm package structure with postinstall script that downloads the correct binary for the user's platform and sets up the xvn executable.

**Implementation Steps:**

1. **Update package.json:**
   - Set version, description, author, license
   - Add `bin` field pointing to wrapper script
   - Add `postinstall` script to run install.js
   - Define supported platforms

2. **Create install.js:**
   - Detect platform (Linux/macOS) and architecture (x64/arm64)
   - Construct download URL from GitHub Releases
   - Download binary tarball
   - Verify checksum
   - Extract to `native/` directory
   - Make executable
   - Handle errors gracefully

3. **Create bin wrapper:**
   - Shell script that executes the native binary
   - Handles case where binary not found

4. **Add .npmignore:**
   - Exclude Rust source code, tests, target directory
   - Include only: package.json, install.js, bin/, shell/, README.md, LICENSE

5. **Test locally:**
   - Run `npm pack` to create tarball
   - Install in fresh directory
   - Verify binary downloads and works

**Code Structure:**

- File: `package.json`
```json
{
  "name": "xvn",
  "version": "1.0.0",
  "description": "Automatic Node.js version switching for cd - 2-3x faster than avn",
  "main": "install.js",
  "bin": {
    "xvn": "./bin/xvn"
  },
  "scripts": {
    "postinstall": "node install.js"
  },
  "keywords": [
    "node",
    "version",
    "nvm",
    "fnm",
    "automatic",
    "switching",
    "cli"
  ],
  "author": "Your Name <your.email@example.com>",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/xvn.git"
  },
  "bugs": {
    "url": "https://github.com/yourusername/xvn/issues"
  },
  "homepage": "https://github.com/yourusername/xvn#readme",
  "engines": {
    "node": ">=14.0.0"
  },
  "os": [
    "darwin",
    "linux"
  ],
  "cpu": [
    "x64",
    "arm64"
  ]
}
```

- File: `install.js`
```javascript
#!/usr/bin/env node

const https = require('https');
const { createWriteStream, mkdirSync, chmodSync, existsSync, readFileSync, createReadStream } = require('fs');
const { execSync } = require('child_process');
const { join } = require('path');
const { createGunzip } = require('zlib');
const { Extract } = require('tar-stream');

// Get platform and architecture
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'linux' && arch === 'x64') {
    return 'x86_64-unknown-linux-gnu';
  } else if (platform === 'linux' && arch === 'arm64') {
    return 'aarch64-unknown-linux-gnu';
  } else if (platform === 'darwin' && arch === 'x64') {
    return 'x86_64-apple-darwin';
  } else if (platform === 'darwin' && arch === 'arm64') {
    return 'aarch64-apple-darwin';
  } else {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
}

// Download file from URL
async function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        return download(response.headers.location, dest)
          .then(resolve)
          .catch(reject);
      }
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', (err) => {
      reject(err);
    });
  });
}

// Verify checksum using Node.js crypto module
function verifyChecksum(file, expectedChecksum) {
  const crypto = require('crypto');
  const fileBuffer = readFileSync(file);
  const hash = crypto.createHash('sha256');
  hash.update(fileBuffer);
  const actualChecksum = hash.digest('hex');

  if (actualChecksum !== expectedChecksum) {
    throw new Error(`Checksum mismatch: expected ${expectedChecksum}, got ${actualChecksum}`);
  }
}

async function install() {
  try {
    console.log('Installing xvn binary...');

    const target = getPlatform();
    const version = require('./package.json').version;
    const baseUrl = `https://github.com/yourusername/xvn/releases/download/v${version}`;
    const tarballName = `xvn-${target}.tar.gz`;
    const checksumName = `${tarballName}.sha256`;

    const tarballUrl = `${baseUrl}/${tarballName}`;
    const checksumUrl = `${baseUrl}/${checksumName}`;

    // Create native directory
    const nativeDir = join(__dirname, 'native');
    if (!existsSync(nativeDir)) {
      mkdirSync(nativeDir, { recursive: true });
    }

    const tarballPath = join(nativeDir, tarballName);
    const checksumPath = join(nativeDir, checksumName);

    // Download tarball and checksum
    console.log(`Downloading ${tarballUrl}...`);
    await download(tarballUrl, tarballPath);

    console.log(`Downloading ${checksumUrl}...`);
    await download(checksumUrl, checksumPath);

    // Verify checksum
    const expectedChecksum = require('fs').readFileSync(checksumPath, 'utf8').trim().split(' ')[0];
    console.log('Verifying checksum...');
    verifyChecksum(tarballPath, expectedChecksum);

    // Extract tarball using built-in modules
    console.log('Extracting binary...');
    await new Promise((resolve, reject) => {
      const extract = new Extract();
      extract.on('entry', (header, stream, next) => {
        if (header.name === 'xvn') {
          const binaryPath = join(nativeDir, 'xvn');
          const writeStream = createWriteStream(binaryPath);
          stream.pipe(writeStream);
          writeStream.on('finish', () => {
            chmodSync(binaryPath, 0o755);
            next();
          });
          writeStream.on('error', reject);
        } else {
          stream.on('end', next);
          stream.resume();
        }
      });
      extract.on('finish', resolve);
      extract.on('error', reject);

      const gunzip = createGunzip();
      createReadStream(tarballPath).pipe(gunzip).pipe(extract);
    });

    console.log('✓ xvn installed successfully!');
    console.log('');
    console.log('To set up shell integration, run:');
    console.log('  xvn setup');

  } catch (error) {
    console.error('Failed to install xvn:', error.message);
    console.error('');
    console.error('You can try installing from source:');
    console.error('  git clone https://github.com/yourusername/xvn.git');
    console.error('  cd xvn');
    console.error('  cargo install --path .');
    process.exit(1);
  }
}

install();
```

- File: `bin/xvn`
```bash
#!/bin/sh

# Wrapper script for xvn npm package

# Find the native binary
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
NATIVE_BINARY="$SCRIPT_DIR/../native/xvn"

if [ ! -f "$NATIVE_BINARY" ]; then
  echo "Error: xvn binary not found at $NATIVE_BINARY"
  echo "Please try reinstalling: npm install -g xvn"
  exit 1
fi

# Execute the native binary
exec "$NATIVE_BINARY" "$@"
```

- File: `.npmignore`
```
# Rust source
src/
tests/
target/
Cargo.toml
Cargo.lock
.cargo/

# Git
.git/
.gitignore

# CI
.github/

# Docs
docs/
spec/

# Build artifacts
*.tar.gz
*.sha256

# Keep these
!README.md
!LICENSE
!package.json
!install.js
!bin/
!shell/
```

**Key Considerations:**
- Use Node.js built-in modules only (no dependencies) to avoid npm install loops
  - Exception: `tar-stream` for tarball extraction (lightweight, essential)
  - Alternative: Could use `child_process` to call system `tar` command
- Handle all error cases gracefully (network failures, checksum mismatches, etc.)
- Provide fallback instructions (install from source) on failure
- Make bin wrapper executable with `chmod +x bin/xvn`
- Use `.npmignore` to keep package size small (exclude Rust source)
- Version in package.json must match git tag for download URL to work
- Windows users will see clear error message (Windows support in v1.1.0)

**Testing:**
- Run `npm pack` to create tarball locally
- Extract tarball to fresh directory
- Run `npm install` and verify binary downloads
- Test `./bin/xvn --version` works
- Test on both macOS and Linux
- Test with both npm and yarn

**Dependencies:**
- Requires: M6.2 (binaries must be available on GitHub Releases)

**Enables:**
- M6.4 (installation flow testing)

---

### Task M6.4: Test installation flow

**Objective:** Thoroughly test the npm installation flow on fresh systems with various configurations to ensure reliability.

**Implementation Steps:**

1. **Test local installation:**
   - Run `npm pack` to create tarball
   - Install in test directory: `npm install /path/to/xvn-1.0.0.tgz`
   - Verify binary downloads
   - Run `xvn --version`
   - Run `xvn setup` and verify shell hooks are installed

2. **Test global installation:**
   - Install globally: `npm install -g /path/to/xvn-1.0.0.tgz`
   - Verify `xvn` is in PATH
   - Run `xvn --version`
   - Run `xvn setup` and test shell integration

3. **Test on fresh systems:**
   - Use Docker containers (ubuntu:latest, ubuntu:22.04, ubuntu:20.04)
   - Use fresh macOS VM or clean user account
   - Install Node.js, npm, nvm/fnm
   - Install xvn and verify it works

4. **Test with version managers:**
   - Install nvm, create `.nvmrc` file
   - cd into directory with `.nvmrc`
   - Verify xvn detects and switches version
   - Repeat with fnm

5. **Test error cases:**
   - Simulate network failure (block download)
   - Test on unsupported platform (simulate with env vars)
   - Test with corrupted tarball
   - Verify graceful error messages

6. **Document any issues:**
   - Create issues for bugs found
   - Note edge cases or platform-specific issues
   - Document workarounds if needed

**Code Structure:**

- File: `test-install.sh` (test script)
```bash
#!/bin/bash

set -e

echo "=== Testing xvn installation flow ==="
echo ""

# Test 1: Local installation
echo "Test 1: Local installation"
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"
npm pack /path/to/xvn
npm install xvn-*.tgz
./node_modules/.bin/xvn --version
echo "✓ Local installation works"
echo ""

# Test 2: Global installation
echo "Test 2: Global installation"
npm install -g xvn-*.tgz
xvn --version
echo "✓ Global installation works"
echo ""

# Test 3: Setup command
echo "Test 3: Setup command"
xvn setup
# Verify shell hook was added
if grep -q "xvn.sh" ~/.bashrc || grep -q "xvn.sh" ~/.zshrc; then
  echo "✓ Setup command works"
else
  echo "✗ Setup command failed - no hook found in shell rc file"
  exit 1
fi
echo ""

# Test 4: Version detection
echo "Test 4: Version detection"
cd "$TMP_DIR"
echo "18.0.0" > .nvmrc
nvm install 18.0.0
# cd will trigger xvn hook
cd .
node --version | grep "v18" || (echo "✗ Version switching failed"; exit 1)
echo "✓ Version detection works"
echo ""

echo "=== All tests passed ==="
```

- File: `Dockerfile.test-ubuntu` (for Docker testing)
```dockerfile
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
  curl \
  git \
  build-essential \
  && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs

# Install nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

WORKDIR /test

# Copy xvn tarball
COPY xvn-*.tgz /test/

# Install and test
RUN npm install -g xvn-*.tgz
RUN xvn --version

CMD ["/bin/bash"]
```

**Key Considerations:**
- Test on both x64 and arm64 architectures
- Test on Ubuntu LTS versions (20.04, 22.04, 24.04)
- Test on macOS Intel and Apple Silicon
- Test with both bash and zsh shells
- Test with both nvm and fnm version managers
- Document minimum versions of dependencies (Node.js, npm)
- Test with npm and yarn (pnpm if time permits)

**Testing:**
- Run test script on at least 2 different platforms
- Run Docker tests for Ubuntu
- Test on fresh macOS installation
- Document results in test report

**Dependencies:**
- Requires: M6.3 (npm package must be complete)

**Enables:**
- M6.5 (beta testing with confidence that it works)

---

### Task M6.5: Beta testing

**Objective:** Recruit beta testers, distribute pre-release builds, collect feedback, and fix critical bugs before v1.0.0 release.

**Implementation Steps:**

1. **Recruit beta testers:**
   - Post in Node.js communities (Reddit /r/node, HN, Twitter)
   - Reach out to users of avn (search GitHub for avn usage)
   - Target 10-20 testers with diverse setups
   - Prioritize testers with different platforms (Ubuntu, macOS Intel/M1)

2. **Create beta release:**
   - Tag version `v0.7.0-beta`
   - Push tag to trigger build workflow
   - Verify binaries are built and uploaded
   - Publish to npm with beta tag: `npm publish --tag beta`

3. **Distribute to testers:**
   - Create GitHub discussion or issue for beta feedback
   - Provide installation instructions: `npm install -g xvn@beta`
   - Provide testing checklist (see below)
   - Set up feedback channels (GitHub issues, Discord, email)

4. **Collect feedback:**
   - Track installation issues (platform-specific bugs)
   - Track performance metrics (activation time)
   - Track usability issues (confusing errors, setup problems)
   - Track feature requests (for future milestones)

5. **Fix critical bugs:**
   - Prioritize: blocking bugs > critical > major > minor
   - Blocking: prevents installation or basic usage
   - Critical: causes crashes or data loss
   - Major: breaks important functionality
   - Minor: cosmetic or edge case issues

6. **Iterate:**
   - Release beta.2, beta.3 as needed
   - Re-test fixes with beta testers
   - Continue until zero critical/blocking bugs

**Code Structure:**

- File: `BETA_TESTING.md` (beta testing guide)
```markdown
# xvn Beta Testing Guide

Thank you for helping test xvn! This guide will help you install, test, and provide feedback.

## Installation

```bash
npm install -g xvn@beta
```

Verify installation:

```bash
xvn --version  # Should print v0.7.0-beta
```

## Setup

```bash
xvn setup
```

Then restart your shell or run:

```bash
source ~/.bashrc  # or ~/.zshrc
```

## Testing Checklist

Please test the following and report any issues:

### Basic Functionality

- [ ] `xvn --version` prints version
- [ ] `xvn --help` shows help text
- [ ] `xvn setup` adds shell hook
- [ ] Shell hook activates on cd
- [ ] Correct version detected from `.nvmrc`
- [ ] Correct version detected from `.node-version`
- [ ] Auto-install prompts when version missing
- [ ] Performance: activation takes <150ms (run `xvn status` for timing)

### Version Managers

- [ ] Works with nvm
- [ ] Works with fnm
- [ ] (Optional) Works with n

### Edge Cases

- [ ] Handles missing version file gracefully
- [ ] Handles invalid version format gracefully
- [ ] Handles version manager not installed
- [ ] Handles version not installed (auto-install prompt)

### Performance

Please run `xvn status` and report the activation time:

- Activation time: ___ms
- Platform: Ubuntu / macOS Intel / macOS M1 / Other
- Version manager: nvm / fnm / n / other

## Reporting Issues

Please report issues at: https://github.com/yourusername/xvn/issues

Include:

- Platform (OS, architecture)
- Node.js version (`node --version`)
- Version manager (nvm, fnm, etc.)
- Shell (bash, zsh)
- Error message or unexpected behavior
- Steps to reproduce

## Feedback

We'd love to hear your thoughts:

- What do you like about xvn?
- What could be improved?
- Any features you'd like to see?
- How does it compare to avn?

Post feedback in: https://github.com/yourusername/xvn/discussions
```

- File: `BETA_ANNOUNCEMENT.md` (template for announcing beta)
```markdown
# xvn Beta Release - Seeking Testers

I'm excited to announce the beta release of **xvn**, a Rust-based reimagining of automatic Node.js version switching (similar to avn).

**Key Features:**
- 2-3x faster than avn (<100ms activation time)
- Works with nvm, fnm, and n
- Auto-install prompts for missing versions
- Simple setup: `npm install -g xvn@beta && xvn setup`

**Why beta?**
I need help testing on different platforms (Ubuntu, macOS Intel/M1) and version managers before releasing v1.0.0.

**How to help:**
1. Install: `npm install -g xvn@beta`
2. Follow testing guide: [BETA_TESTING.md](link)
3. Report issues: [GitHub Issues](link)

**Requirements:**
- Node.js 14+
- nvm or fnm installed
- bash or zsh shell
- Linux or macOS (x64 or arm64)

Looking for 10-20 testers. Thanks in advance!
```

**Key Considerations:**
- Beta testing should last at least 1 week (ideally 2 weeks)
- Aim for at least 10 testers to get diverse feedback
- Prioritize blocking/critical bugs over feature requests
- Document all bugs in GitHub issues for tracking
- Keep communication open - respond to questions quickly
- Consider creating a Discord/Slack channel for real-time feedback

**Testing:**
- Verify npm package installs correctly with `@beta` tag
- Test the beta on your own system first
- Have at least one other developer test before broader release

**Dependencies:**
- Requires: M6.4 (installation flow must be working)

**Enables:**
- M6.6 (beta feedback informs release preparation)

---

### Task M6.6: Release preparation

**Objective:** Finalize documentation, create GitHub release for v0.7.0-beta, publish to npm, and test end-to-end.

**Implementation Steps:**

1. **Write CHANGELOG.md:**
   - Document all features in v0.7.0-beta
   - Follow Keep a Changelog format
   - Include breaking changes, new features, bug fixes
   - Credit contributors

2. **Finalize README.md:**
   - Add installation instructions
   - Add quick start guide
   - Add configuration examples
   - Add performance benchmarks
   - Add troubleshooting section
   - Add links to docs, issues, discussions

3. **Create GitHub Release:**
   - Tag version `v0.7.0-beta`
   - Write release notes (summary of CHANGELOG)
   - Mark as pre-release
   - Verify binaries are attached

4. **Set up npm authentication:**
   - Log in to npm: `npm login`
   - Or set NPM_TOKEN environment variable for CI
   - Verify access: `npm whoami`

5. **Publish to npm:**
   - Run `npm publish --tag beta` (not `latest` yet)
   - Verify package appears on npmjs.com
   - Test installation: `npm install -g xvn@beta`

6. **Test end-to-end:**
   - Fresh system installation test
   - Verify all features work
   - Verify docs are accurate
   - Verify links work

**Code Structure:**

- File: `CHANGELOG.md`
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0-beta] - 2025-01-15

### Added

- Initial beta release
- Automatic Node.js version switching on cd
- Support for nvm and fnm version managers
- Shell integration for bash and zsh
- Version file detection (.nvmrc, .node-version)
- Auto-install prompts for missing versions
- Configuration via ~/.xvnrc and .xvn.yaml
- Plugin system for version managers
- Performance: <100ms activation time (P50), <150ms (P95)

### Changed

- N/A (first release)

### Fixed

- N/A (first release)

### Security

- Checksum verification for binary downloads

## [Unreleased]

### Planned for v1.0.0

- Incorporate beta feedback
- Final performance tuning
- Final documentation polish
```

- File: `README.md` (complete version)
```markdown
# xvn

> Automatic Node.js version switching for cd - 2-3x faster than avn

**xvn** is a Rust-based tool that automatically switches your Node.js version when you `cd` into a directory with a `.nvmrc` or `.node-version` file.

## Features

- 🚀 **Fast**: <100ms activation time (2-3x faster than avn)
- 🔌 **Compatible**: Works with nvm, fnm, and n
- 🤖 **Auto-install**: Prompts to install missing versions
- ⚙️  **Configurable**: Customize behavior via `~/.xvnrc`
- 🔒 **Safe**: Written in Rust with checksum verification
- 📦 **Easy**: Install via npm, no manual binary downloads

## Installation

```bash
npm install -g xvn
xvn setup
```

Then restart your shell or run:

```bash
source ~/.bashrc  # or ~/.zshrc
```

## Usage

Just `cd` into a directory with a `.nvmrc` or `.node-version` file:

```bash
cd ~/my-project  # xvn automatically switches Node.js version
```

### Manual Activation

```bash
xvn activate  # Activate version for current directory
```

### Configuration

Create `~/.xvnrc`:

```yaml
# Version managers (in priority order)
plugins:
  - nvm
  - fnm

# Auto-install missing versions
auto_install: prompt  # or 'never'

# Silent mode (no output)
silent: false
```

### Supported Version Managers

- ✅ nvm (Node Version Manager)
- ✅ fnm (Fast Node Manager)
- ⏳ n (planned for v1.1.0)
- ⏳ asdf (planned for v1.2.0)

## Requirements

- Node.js 14+
- nvm or fnm installed
- bash or zsh shell
- Linux or macOS (x64 or arm64)

## Performance

| Tool | P50 Activation | P95 Activation | Memory |
|------|---------------|---------------|---------|
| xvn  | <100ms        | <150ms        | <5MB    |
| avn  | ~200ms        | ~300ms        | ~30MB   |

## Troubleshooting

### Shell hook not triggering

Make sure you ran `xvn setup` and restarted your shell.

### Version not switching

Check that your version manager is installed:

```bash
nvm --version  # or fnm --version
```

### Binary not found on install

If the postinstall script fails, try installing from source:

```bash
git clone https://github.com/yourusername/xvn.git
cd xvn
cargo install --path .
```

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
```

**Key Considerations:**
- CHANGELOG should be comprehensive but concise
- README should focus on getting started quickly
- Include clear troubleshooting section (common issues)
- Add badges (build status, npm version, license)
- Use `--tag beta` for npm publish (not `latest`)
- Double-check all links work (GitHub, npm, docs)

**Testing:**
- Test fresh installation from npm registry
- Verify README instructions are accurate
- Check that all links in README work
- Test on at least 2 platforms

**Dependencies:**
- Requires: M6.5 (beta testing should be complete or nearly complete)

**Enables:**
- M6.7 (v1.0.0 release)

---

### Task M6.7: v1.0.0 release

**Objective:** Address beta feedback, perform final testing, tag v1.0.0, publish to npm as `latest`, and announce publicly.

**Implementation Steps:**

1. **Address beta feedback:**
   - Fix all critical/blocking bugs from beta
   - Fix high-priority major bugs
   - Document workarounds for minor bugs (fix in v1.1.0)
   - Update docs based on feedback

2. **Final testing:**
   - Run full test suite: `cargo test --all-features`
   - Run benchmarks: `cargo bench`
   - Test on fresh systems (Ubuntu, macOS)
   - Test with nvm and fnm
   - Verify no regressions

3. **Update version:**
   - Bump version in `Cargo.toml` to `1.0.0`
   - Bump version in `package.json` to `1.0.0`
   - Update CHANGELOG.md with final release notes
   - Commit: `git commit -m "chore: bump version to 1.0.0"`

4. **Tag and build:**
   - Create git tag: `git tag v1.0.0`
   - Push tag: `git push origin v1.0.0`
   - Wait for GitHub Actions to build binaries
   - Verify binaries are uploaded to GitHub Release

5. **Publish to npm:**
   - Run `npm publish` (no `--tag` flag, publishes as `latest`)
   - Verify package appears on npmjs.com
   - Test installation: `npm install -g xvn`
   - Verify `xvn --version` shows `1.0.0`

6. **Announce publicly:**
   - Post on Reddit /r/node
   - Post on Hacker News (Show HN)
   - Post on Twitter/X
   - Post in Node.js Discord/Slack communities
   - Update GitHub repo description and topics

7. **Monitor feedback:**
   - Watch GitHub issues for bug reports
   - Respond to questions on announcement posts
   - Prepare hotfix release if critical bugs found

**Code Structure:**

- File: `scripts/release.sh` (automation helper)
```bash
#!/bin/bash

set -e

if [ -z "$1" ]; then
  echo "Usage: ./scripts/release.sh <version>"
  echo "Example: ./scripts/release.sh 1.0.0"
  exit 1
fi

VERSION=$1

echo "=== Releasing xvn v$VERSION ==="
echo ""

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
  echo "Error: Uncommitted changes detected. Please commit or stash them."
  exit 1
fi

# Detect sed variant (GNU vs BSD)
if sed --version 2>/dev/null | grep -q GNU; then
  SED_INPLACE="sed -i"
else
  SED_INPLACE="sed -i ''"
fi

# Update Cargo.toml
echo "Updating Cargo.toml..."
eval "$SED_INPLACE 's/^version = .*/version = \"$VERSION\"/' Cargo.toml"

# Update package.json
echo "Updating package.json..."
eval "$SED_INPLACE 's/\"version\": \".*\"/\"version\": \"$VERSION\"/' package.json"

# Update CHANGELOG.md
echo "Updating CHANGELOG.md..."
DATE=$(date +%Y-%m-%d)
eval "$SED_INPLACE 's/## \\[Unreleased\\]/## [$VERSION] - $DATE/' CHANGELOG.md"

# Commit
echo "Committing version bump..."
git add Cargo.toml package.json CHANGELOG.md
git commit -m "chore: bump version to $VERSION"

# Tag
echo "Creating git tag v$VERSION..."
git tag "v$VERSION"

# Push
echo "Pushing to GitHub..."
git push origin main
git push origin "v$VERSION"

echo ""
echo "✓ Release v$VERSION initiated!"
echo ""
echo "Next steps:"
echo "1. Wait for GitHub Actions to build binaries"
echo "2. Verify binaries at https://github.com/yourusername/xvn/releases/tag/v$VERSION"
echo "3. Run: npm publish"
echo "4. Announce on Reddit, HN, Twitter"
```

- File: `ANNOUNCEMENT_v1.0.0.md` (announcement template)
```markdown
# Show HN: xvn - Automatic Node.js version switching, 2-3x faster than avn

Hi HN! I built **xvn**, a Rust-based tool for automatically switching Node.js versions when you `cd` into a project directory.

**Why xvn?**

I've been using avn for years, but it's slow (~200ms per cd) and requires Node.js/npm overhead. xvn is written in Rust and activates in <100ms, making directory navigation feel instant again.

**Key features:**

- 🚀 Fast: <100ms activation (2-3x faster than avn)
- 🔌 Works with nvm, fnm, and n
- 🤖 Auto-install: Prompts to install missing versions
- 📦 Easy: `npm install -g xvn && xvn setup`

**How it works:**

xvn hooks into your shell's `cd` command (via chpwd in bash/zsh) and checks for `.nvmrc` or `.node-version` files. If the current Node.js version doesn't match, xvn activates the correct version using your version manager (nvm, fnm, etc.).

**Performance:**

xvn activates in <100ms (P50) and <150ms (P95), compared to avn's ~200ms (P50) / ~300ms (P95). This is because Rust has zero runtime overhead and I optimized the plugin system.

**Installation:**

```bash
npm install -g xvn
xvn setup
# Restart shell and cd into a project with .nvmrc
```

**GitHub:** https://github.com/yourusername/xvn
**npm:** https://www.npmjs.com/package/xvn

I'd love feedback! What features would you find useful?
```

**Key Considerations:**
- Only release v1.0.0 when you're confident in quality
- Zero critical bugs should remain after beta testing
- Test thoroughly before publishing to npm as `latest`
- Announce timing: Tuesday-Thursday 9am-12pm PT for best visibility
- Monitor GitHub issues closely for first 48 hours after release
- Be prepared to release hotfix (v1.0.1) if critical bugs found
- Update README badges (replace beta with latest)

**Testing:**
- Full test suite passes
- Benchmarks meet targets (<100ms P50, <150ms P95)
- Fresh install from npm works on multiple platforms
- No regressions from beta version

**Dependencies:**
- Requires: M6.6 (release preparation complete)
- Requires: M6.5 (beta testing complete, feedback addressed)

**Enables:**
- Public v1.0.0 release
- Begin work on Phase 2 features (v1.1.0+)

---

### Task M6.8: Post-release hotfix/rollback procedures

**Objective:** Establish procedures for handling critical bugs discovered after v1.0.0 release, including hotfix releases and rollback strategies.

**Hotfix Procedure:**

1. **Assess severity:**
   - **Critical:** Causes crashes, data loss, security issues → Immediate hotfix
   - **Major:** Breaks important functionality → Hotfix within 24-48 hours
   - **Minor:** Cosmetic or edge case → Fix in next minor release

2. **Create hotfix branch:**
   ```bash
   git checkout v1.0.0
   git checkout -b hotfix/v1.0.1
   ```

3. **Fix the bug:**
   - Make minimal changes (only fix the bug, no new features)
   - Add regression test
   - Update CHANGELOG.md

4. **Release hotfix:**
   ```bash
   ./scripts/release.sh 1.0.1
   npm publish  # publishes as latest
   ```

5. **Announce hotfix:**
   - Update GitHub release notes
   - Post in issues/discussions
   - Notify users who reported the bug

**Rollback Strategy:**

npm does not allow unpublishing versions after 24 hours, so rollback options are limited:

1. **Deprecate the broken version:**
   ```bash
   npm deprecate xvn@1.0.0 "Critical bug - please upgrade to 1.0.1"
   ```

2. **Release hotfix immediately:**
   - Users installing fresh will get latest (1.0.1)
   - Users on 1.0.0 will see deprecation warning

3. **If bug is catastrophic (extremely rare):**
   - Contact npm support to unpublish within 24 hours
   - Update GitHub release to mark as broken
   - Document issue in CHANGELOG.md

**Key Considerations:**
- Never delete git tags (breaks existing installs)
- Always increment version for hotfixes (1.0.0 → 1.0.1)
- Keep hotfix changes minimal and focused
- Test hotfix thoroughly before releasing
- Document all known issues in GitHub issues

**Dependencies:**
- None (this is a reference procedure for post-release)

---

## Integration Points

### CI/CD Pipeline (M6.1 + M6.2)

The test workflow (M6.1) and build workflow (M6.2) work together:

- Test workflow runs on every push/PR to catch regressions early
- Build workflow runs on tags to create releases
- Both use GitHub Actions caching to speed up builds
- Both use matrix builds to test/build across platforms

### npm Package (M6.3) + Binary Builds (M6.2)

The npm package's `install.js` script downloads binaries from GitHub Releases:

- M6.2 uploads binaries to GitHub Releases on tags
- M6.3's `install.js` constructs URLs based on package.json version
- Version in package.json must match git tag for downloads to work

### Beta Testing (M6.5) + Release (M6.7)

Beta testing informs the final release:

- Beta testers identify bugs that must be fixed before v1.0.0
- Beta feedback shapes documentation and UX improvements
- Beta testing validates installation flow on real systems

---

## Testing Strategy

### Automated Testing

- **Unit tests:** All Rust code has unit tests (>85% coverage)
- **Integration tests:** Shell integration tests verify end-to-end flows
- **CI tests:** Every push/PR runs full test suite on Ubuntu and macOS
- **Coverage reporting:** Codecov tracks coverage over time

### Manual Testing

- **Installation testing:** Fresh system tests via Docker and VMs
- **Platform testing:** Test on Ubuntu 20.04/22.04/24.04, macOS Intel/M1
- **Version manager testing:** Test with nvm and fnm
- **Shell testing:** Test with bash and zsh

### Beta Testing

- **Real users:** 10-20 beta testers with diverse setups
- **Feedback collection:** GitHub issues and discussions
- **Bug tracking:** All bugs documented and prioritized
- **Iteration:** Multiple beta releases until zero critical bugs

---

## Success Criteria

### Technical Success

- ✅ CI passing on all platforms (Ubuntu, macOS)
- ✅ Binaries successfully built for all targets (x64, arm64)
- ✅ npm package installs without errors
- ✅ Binary downloads and extracts correctly
- ✅ Shell integration works on bash and zsh
- ✅ Version switching works with nvm and fnm
- ✅ Performance: <100ms activation (P50), <150ms (P95)
- ✅ Coverage: >85% line coverage

### Release Success

- ✅ Zero critical/blocking bugs in beta
- ✅ Beta testers report successful installation (>90%)
- ✅ Documentation is accurate and complete
- ✅ v1.0.0 published to npm as `latest`
- ✅ GitHub Release created with binaries attached

### User Success

- ✅ Users can install with `npm install -g xvn`
- ✅ Users can set up with `xvn setup`
- ✅ Version switching "just works" on cd
- ✅ Error messages are clear and actionable
- ✅ Performance is noticeably faster than avn

### Post-Release Success

- GitHub stars: >100 in first week
- npm downloads: >500 in first week
- Positive feedback on HN/Reddit (>50 upvotes)
- Active community engagement (issues, discussions)
- Minimal bug reports (<5 critical bugs in first month)

---

## Notes

### Timeline Breakdown

**Milestone 6: 2 weeks (weeks 11-12)**

**Week 1: Infrastructure and packaging**
- Days 1-2: M6.1 (CI/CD pipeline) + M6.2 (Binary builds) in parallel
- Days 3-4: M6.3 (npm package structure)
- Day 5: M6.4 (Installation testing)

**Week 2: Beta testing and release**
- Days 1-7: M6.5 (Beta testing) - runs for 1 week minimum
- Days 6-7: M6.6 (Release preparation) - overlaps with beta end
- Day 8+: M6.7 (v1.0.0 release) - after beta feedback addressed

**Note:** If beta testing reveals critical issues, M6.7 may slip into week 13.

### Parallelization Opportunities

- M6.1 and M6.2 (CI and builds) can be worked on simultaneously
- M6.4 (testing) can start while M6.5 (beta) is recruiting testers
- M6.6 (docs) can be drafted during M6.5 (beta testing)

### Blocking Dependencies

- M6.3 blocks M6.4 (can't test install without package)
- M6.4 blocks M6.5 (must verify install works before beta)
- M6.5 blocks M6.7 (must address beta feedback before v1.0.0)

### Risk Areas

- Cross-compilation for arm64 (may require debugging)
- npm package installation on Windows (unsupported but may be attempted)
- Beta tester recruitment (may take longer than expected)
- Critical bugs found late in beta testing (may delay v1.0.0)

### Migration Guide from avn

**Status:** Not included in v1.0.0 scope

**Rationale:**
- xvn is not backward compatible with avn (different config format, different plugins)
- Migration is simple: uninstall avn, install xvn, run setup
- Most avn features work the same way (cd triggers version switch)
- Can be added as documentation if users request it

**If migration guide is needed later:**
1. Document differences between avn and xvn config
2. Provide script to convert ~/.avnrc to ~/.xvnrc
3. Document plugin equivalents (avn-nvm → xvn nvm plugin)
4. Add to docs/MIGRATION.md in Phase 2
