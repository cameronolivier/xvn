#!/usr/bin/env node

const { existsSync, readFileSync, writeFileSync, rmSync } = require('fs');
const { join } = require('path');
const os = require('os');

const ANVS_DIR = join(os.homedir(), '.anvs');
const ANVS_CONFIG = join(os.homedir(), '.anvsrc');
const ANVS_MARKER_START = '# >>> anvs initialize >>>';
const ANVS_MARKER_END = '# <<< anvs initialize <<<';

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

        // Check if anvs block exists
        if (!content.includes(ANVS_MARKER_START)) {
          continue;
        }

        // Remove anvs block
        const lines = content.split('\n');
        const newLines = [];
        let skipLine = false;

        for (const line of lines) {
          if (line.includes(ANVS_MARKER_START)) {
            skipLine = true;
            removed = true;
            continue;
          }
          if (line.includes(ANVS_MARKER_END)) {
            skipLine = false;
            continue;
          }
          if (!skipLine) {
            newLines.push(line);
          }
        }

        // Write back
        writeFileSync(profilePath, newLines.join('\n'));
        console.log(`✓ Removed anvs integration from ${profile}`);
      } catch (error) {
        console.error(`Failed to update ${profile}:`, error.message);
      }
    }
  }

  return removed;
}

function removeAnvsDirectory() {
  if (!existsSync(ANVS_DIR)) {
    return false;
  }

  try {
    rmSync(ANVS_DIR, { recursive: true, force: true });
    console.log(`✓ Removed ${ANVS_DIR}`);
    return true;
  } catch (error) {
    console.error(`Failed to remove ${ANVS_DIR}:`, error.message);
    return false;
  }
}

function removeAnvsConfig() {
  if (!existsSync(ANVS_CONFIG)) {
    return false;
  }

  try {
    rmSync(ANVS_CONFIG, { force: true });
    console.log(`✓ Removed ${ANVS_CONFIG}`);
    return true;
  } catch (error) {
    console.error(`Failed to remove ${ANVS_CONFIG}:`, error.message);
    return false;
  }
}

async function uninstall() {
  console.log('\nUninstalling anvs...\n');

  // Remove shell integration
  const shellRemoved = removeShellIntegration();

  // Remove anvs directory
  const dirRemoved = removeAnvsDirectory();

  // Remove config
  const configRemoved = removeAnvsConfig();

  console.log('\n✓ anvs uninstalled successfully!\n');

  if (shellRemoved) {
    console.log('Please restart your shell or run:');
    const shell = process.env.SHELL || 'bash';
    const profileName = shell.includes('zsh') ? '.zshrc' : '.bashrc';
    console.log(`  source ~/${profileName}\n`);
  }

  if (!dirRemoved && !configRemoved && !shellRemoved) {
    console.log('Nothing to clean up - anvs was not fully installed.\n');
  }
}

uninstall();
