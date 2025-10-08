#!/usr/bin/env node

const { copyFileSync, chmodSync, existsSync, mkdirSync, readdirSync, rmSync, statSync, unlinkSync, symlinkSync } = require('fs');
const { join } = require('path');
const os = require('os');

const XVN_DIR = join(os.homedir(), '.xvn');
const VERSION = require('./package.json').version;

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

function cleanupOldVersions() {
    const versionsDir = join(XVN_DIR, 'versions');
    if (!existsSync(versionsDir)) {
        return;
    }

    const versions = readdirSync(versionsDir)
        .filter(name => name.startsWith('v'))
        .sort((a, b) => {
            // Simple semver sort, might need a more robust solution if versions get complex
            return b.localeCompare(a, undefined, { numeric: true, sensitivity: 'base' });
        });

    const versionsToKeep = 2;
    if (versions.length > versionsToKeep) {
        const versionsToRemove = versions.slice(versionsToKeep);
        for (const version of versionsToRemove) {
            const versionPath = join(versionsDir, version);
            console.log(`Removing old version: ${versionPath}`);
            rmSync(versionPath, { recursive: true, force: true });
        }
    }
}

async function install() {
  try {
    const isUpgrade = existsSync(XVN_DIR);
    console.log(isUpgrade ? 'Upgrading xvn...' : 'Installing xvn...');

    const versionDir = join(XVN_DIR, 'versions', `v${VERSION}`);
    const binDir = join(versionDir, 'bin');
    const libDir = join(versionDir, 'lib');
    const globalBinDir = join(XVN_DIR, 'bin');

    // 1. Create directory structure
    mkdirSync(binDir, { recursive: true });
    mkdirSync(libDir, { recursive: true });
    mkdirSync(globalBinDir, { recursive: true });

    // 2. Download and install binary
    const platform = getPlatform();
    const sourceBinary = join(__dirname, 'native', platform, 'xvn');
    if (!existsSync(sourceBinary)) {
      throw new Error(`Binary not found for platform: ${platform}`);
    }
    const destBinary = join(binDir, 'xvn');
    copyFileSync(sourceBinary, destBinary);
    chmodSync(destBinary, 0o755);

    // 3. Copy shell integration scripts
    copyFileSync(join(__dirname, 'shell', 'xvn.sh'), join(libDir, 'xvn.sh'));
    copyFileSync(join(__dirname, 'shell', 'xvn.ps1'), join(libDir, 'xvn.ps1'));

    // 4. Create/update symlinks
    const binSymlink = join(globalBinDir, 'xvn');
    if (existsSync(binSymlink) || statSync(binSymlink, { throwIfNoEntry: false })?.isSymbolicLink()) {
        unlinkSync(binSymlink);
    }
    symlinkSync(destBinary, binSymlink, 'file');

    const currentSymlink = join(XVN_DIR, 'current');
    if (existsSync(currentSymlink) || statSync(currentSymlink, { throwIfNoEntry: false })?.isSymbolicLink()) {
        unlinkSync(currentSymlink);
    }
    symlinkSync(versionDir, currentSymlink, 'dir');

    // 5. Clean up old versions
    cleanupOldVersions();

    console.log(`✓ xvn v${VERSION} installed successfully to ${versionDir}`);
    console.log('');
    if (!isUpgrade) {
        console.log("To complete the installation, you need to add xvn to your shell's PATH.");
        console.log("Run the following command to get started:");
        console.log('');
        console.log('  xvn setup');
        console.log('');
        console.log('Or, you can manually add the following to your shell profile (e.g., ~/.zshrc, ~/.bashrc):');
        console.log('  export PATH="$HOME/.xvn/bin:$PATH"');
    } else {
        console.log("Your shell configuration might need to be updated.");
        console.log("Run 'xvn setup' to ensure your shell is correctly configured.");
    }

  } catch (error) {
    console.error('Failed to install xvn:', error.message);
    process.exit(1);
  }
}

install();