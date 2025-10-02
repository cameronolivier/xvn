#!/usr/bin/env node

const https = require('https');
const { createWriteStream, mkdirSync, chmodSync, existsSync, readFileSync, createReadStream } = require('fs');
const { join } = require('path');
const { createGunzip } = require('zlib');
const { extract: Extract } = require('tar-stream');

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
    const baseUrl = `https://github.com/cameronolivier/xvn/releases/download/v${version}`;
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
    const expectedChecksum = readFileSync(checksumPath, 'utf8').trim().split(' ')[0];
    console.log('Verifying checksum...');
    verifyChecksum(tarballPath, expectedChecksum);

    // Extract tarball using tar-stream
    console.log('Extracting binary...');
    await new Promise((resolve, reject) => {
      const extractor = Extract();
      extractor.on('entry', (header, stream, next) => {
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
      extractor.on('finish', resolve);
      extractor.on('error', reject);

      const gunzip = createGunzip();
      createReadStream(tarballPath).pipe(gunzip).pipe(extractor);
    });

    console.log('✓ xvn installed successfully!');
    console.log('');
    console.log('To set up shell integration, run:');
    console.log('  xvn setup');

  } catch (error) {
    console.error('Failed to install xvn:', error.message);
    console.error('');
    console.error('You can try installing from source:');
    console.error('  git clone https://github.com/cameronolivier/xvn.git');
    console.error('  cd xvn');
    console.error('  cargo install --path .');
    process.exit(1);
  }
}

install();
