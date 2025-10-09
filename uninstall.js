#!/usr/bin/env node

const { existsSync, readFileSync, writeFileSync, rmSync } = require('fs');
const { join } = require('path');
const os = require('os');

const XVN_DIR = join(os.homedir(), '.xvn');
const XVN_CONFIG = join(os.homedir(), '.xvnrc');
const XVN_MARKER_START = '# >>> xvn initialize >>>';
const XVN_MARKER_END = '# <<< xvn initialize <<<';

function removeShellIntegration() {
  const shells = [
    { name: 'zsh', profiles: ['.zshrc', '.zprofile'] },
    { name: 'bash', profiles: ['.bashrc', '.bash_profile', '.profile'] },
  ];

  const home = os.homedir();
  let removed = false;

  for (const shell of shells) {
    for (const profile of shell.profiles) {
      const profilePath = join(home, profile);

      if (!existsSync(profilePath)) {
        continue;
      }

      try {
        const content = readFileSync(profilePath, 'utf8');

        // Check if xvn block exists
        if (!content.includes(XVN_MARKER_START)) {
          continue;
        }

        // Remove xvn block
        const lines = content.split('\n');
        const newLines = [];
        let skipLine = false;

        for (const line of lines) {
          if (line.includes(XVN_MARKER_START)) {
            skipLine = true;
            removed = true;
            continue;
          }
          if (line.includes(XVN_MARKER_END)) {
            skipLine = false;
            continue;
          }
          if (!skipLine) {
            newLines.push(line);
          }
        }

        // Write back
        writeFileSync(profilePath, newLines.join('\n'));
        console.log(`✓ Removed xvn integration from ${profile}`);
      } catch (error) {
        console.error(`Failed to update ${profile}:`, error.message);
      }
    }
  }

  return removed;
}

function removeXvnDirectory() {
  if (!existsSync(XVN_DIR)) {
    return false;
  }

  try {
    rmSync(XVN_DIR, { recursive: true, force: true });
    console.log(`✓ Removed ${XVN_DIR}`);
    return true;
  } catch (error) {
    console.error(`Failed to remove ${XVN_DIR}:`, error.message);
    return false;
  }
}

function removeXvnConfig() {
  if (!existsSync(XVN_CONFIG)) {
    return false;
  }

  try {
    rmSync(XVN_CONFIG, { force: true });
    console.log(`✓ Removed ${XVN_CONFIG}`);
    return true;
  } catch (error) {
    console.error(`Failed to remove ${XVN_CONFIG}:`, error.message);
    return false;
  }
}

async function uninstall() {
  console.log('\nUninstalling xvn...\n');

  // Remove shell integration
  const shellRemoved = removeShellIntegration();

  // Remove xvn directory
  const dirRemoved = removeXvnDirectory();

  // Remove config
  const configRemoved = removeXvnConfig();

  console.log('\n✓ xvn uninstalled successfully!\n');

  if (shellRemoved) {
    console.log('Please restart your shell or run:');
    const shell = process.env.SHELL || 'bash';
    const profileName = shell.includes('zsh') ? '.zshrc' : '.bashrc';
    console.log(`  source ~/${profileName}\n`);
  }

  if (!dirRemoved && !configRemoved && !shellRemoved) {
    console.log('Nothing to clean up - xvn was not fully installed.\n');
  }
}

uninstall();
