#!/usr/bin/env node

const { copyFileSync, chmodSync, existsSync, mkdirSync } = require('fs');
const { join } = require('path');

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

async function install() {
  try {
    console.log('Installing xvn binary...');

    const target = getPlatform();
    const sourceBinary = join(__dirname, 'native', target, 'xvn');
    const destDir = join(__dirname, 'native');
    const destBinary = join(destDir, 'xvn');

    // Check if source binary exists
    if (!existsSync(sourceBinary)) {
      throw new Error(`Binary not found for platform: ${target}`);
    }

    // Ensure destination directory exists
    if (!existsSync(destDir)) {
      mkdirSync(destDir, { recursive: true });
    }

    // Copy binary to native/ directory
    console.log(`Copying ${target} binary...`);
    copyFileSync(sourceBinary, destBinary);
    chmodSync(destBinary, 0o755);

    console.log('✓ xvn installed successfully!');
    console.log('');
    console.log('To set up shell integration, run:');
    console.log('  xvn setup');

  } catch (error) {
    console.error('Failed to install xvn:', error.message);
    console.error('');
    console.error('You can install from source:');
    console.error('  git clone https://github.com/cameronolivier/xvn.git');
    console.error('  cd xvn');
    console.error('  cargo install --path .');
    process.exit(1);
  }
}

install();
