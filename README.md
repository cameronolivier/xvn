# ANVS - Automatic Node Version Switcher

[![CI](https://github.com/olvrcc/anvs/workflows/Test/badge.svg)](https://github.com/olvrcc/anvs/actions)
[![npm version](https://badge.fury.io/js/anvs.svg)](https://www.npmjs.com/package/anvs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Automatic Node Version Switcher is written in Rust, with a loving tip-of-the-hat to [avn](https://github.com/wbyoung/avn)

**ANVS** is a Rust-based tool that automatically switches your Node.js version when you `cd` into a directory with a `.nvmrc`, `.node-version`, or `package.json` file. It automatically returns to your default version when you leave.

## Features

- 🚀 **Fast**: <100ms activation time (2-3x faster than avn)
- 🔌 **Compatible**: Works with nvm, fnm, and n
- 🔄 **Smart**: Automatically returns to default version when leaving projects
- 🤖 **Auto-install**: Prompts to install missing versions
- ⚙️  **Configurable**: Customize behavior via `~/.anvsrc`
- 🔒 **Safe**: Written in Rust with checksum verification
- 📦 **Easy**: Install via npm, no manual binary downloads

## Quick Start

Initialize anvs with auto-detection (recommended):

```bash
# Install
npm install -g anvs

# Quick setup with auto-detection (default, < 30 seconds)
anvs init
```

Or use advanced mode for full customization:

```bash
anvs init --advanced
```

The wizard detects your shell and Node.js version manager, then guides you through setup.

### Setup Modes

**Quick Mode** (default):
- Auto-detects shell and version managers
- Shows summary and asks for confirmation
- Completes in < 30 seconds
- Recommended for most users

**Advanced Mode** (`--advanced`):
- Step-by-step customization
- Full control over all settings
- Takes 1-2 minutes
- For users with specific requirements

## Installation

`anvs` is installed to a central directory (`~/.anvs`) to ensure it's always available, regardless of the active Node.js version.

### Option 1: npm (Recommended for Linux and macOS)

```bash
# Step 1: Install the package
npm install -g anvs

# Step 2: Set up your shell
anvs init
```

### Option 2: Homebrew (macOS only)

```bash
# Step 1: Tap the repository
brew tap olvrcc/anvs

# Step 2: Install anvs
brew install anvs

# Step 3: Set up your shell
anvs init
```

### Option 3: Cargo (Build from source)

```bash
# Install from source
cargo install --git https://github.com/olvrcc/anvs

# Set up your shell
anvs init
```

### Complete the Installation

After installation via any method, restart your shell or run:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### Upgrading

**npm:**
```bash
npm update -g anvs
```

**Note for nvm users:** Global packages are Node version-specific in nvm. If you installed anvs while on Node v20 but later switched to Node v22, the global package only exists in v20. To uninstall completely, switch back to the Node version where anvs was installed before running `npm uninstall -g anvs`.

**Homebrew:**
```bash
brew upgrade anvs
```

**Cargo:**
```bash
cargo install --git https://github.com/olvrcc/anvs --force
```

## Migration from XVN

If you're upgrading from `xvn` to `anvs`, see the [Migration Guide](docs/XVN_TO_ANVS_MIGRATION.md).

**Quick migration**:
```bash
xvn uninstall
npm install -g anvs
anvs init
```

For detailed instructions, troubleshooting, and configuration migration, see [docs/XVN_TO_ANVS_MIGRATION.md](docs/XVN_TO_ANVS_MIGRATION.md).

## Usage

Just `cd` into a directory with a version file:

```bash
cd ~/my-project  # anvs automatically switches Node.js version
cd ..            # anvs switches back to your default Node.js version
```

### Automatic Default Version

When you leave a project directory (one with a `.nvmrc` or other version file), **anvs automatically switches back to your default Node.js version**. This ensures you're always on your preferred version when not in a project.

**For nvm users:**
- anvs uses your `default` alias: `nvm alias default 20.11.0`
- Check your default: `nvm version default`

**For fnm users:**
- anvs uses fnm's default version
- Check your default: `fnm default`

**Configuration:**
```yaml
# In ~/.anvsrc
use_default: true  # (default: true)
```

Disable this behavior if you prefer manual version switching:
```bash
anvs set use-default  # Interactive toggle
```

### Supported Version Files

anvs supports multiple version file formats:

- **`.nvmrc`** - Standard nvm format with exact version or alias
  ```
  18.20.0
  ```

- **`.node-version`** - Alternative format, same as .nvmrc
  ```
  20.11.0
  ```

- **`package.json`** - npm standard with semver ranges
  ```json
  {
    "engines": {
      "node": ">=18.0.0"
    }
  }
  ```
  Supports semver ranges: `^20.0.0`, `~18.20.0`, `>=18 <21`, `18.x`

### Manual Activation

```bash
anvs activate  # Activate version for current directory
```

### Check Status

```bash
anvs status  # Show current configuration and activation timing
```

### Configuration

Create `~/.anvsrc`:

```yaml
# Version managers (in priority order)
plugins:
  - nvm
  - fnm

# Auto-install missing versions
auto_install: prompt  # or 'always' or 'never'

# Automatically switch to default version when leaving projects
use_default: true  # or 'false' to disable

# Silent mode (no output)
silent: false

# Version file priority (first match wins)
version_files:
  - .nvmrc
  - .node-version
  - package.json  # optional, supports semver ranges
```

Project-level configuration (`.anvs.yaml` in project root):

```yaml
# Override global settings for this project
auto_install: always
silent: true
```

### Supported Version Managers

- ✅ nvm (Node Version Manager)
- ✅ fnm (Fast Node Manager)
- ⏳ n (planned for future release)
- ⏳ asdf (planned for future release)

## Requirements

- Node.js 14+
- nvm or fnm installed
- bash or zsh shell
- Linux or macOS (x64 or arm64)
- Windows support planned for future release

## Uninstalling

To completely remove anvs and clean up all configuration:

```bash
anvs uninstall
```

This command will:
- Detect all anvs installations (npm, Homebrew, Cargo)
- Remove `~/.anvs` directory
- Remove `~/.anvsrc` configuration
- Remove shell integration from `.bashrc`/`.zshrc`
- Provide instructions for uninstalling external packages

Use `--force` to skip the confirmation prompt:
```bash
anvs uninstall --force
```

## Troubleshooting

### `anvs: command not found`

This can happen after installation if your shell hasn't been restarted. Make sure you have run `anvs init` and restarted your shell.

Verify that `~/.anvs/bin` is in your `PATH`:

```bash
echo $PATH
```

Verify the `anvs` binary is in the right place:

```bash
which anvs
# Should output: /Users/your-name/.anvs/bin/anvs
```

### Shell hook not triggering

Make sure you ran `anvs init` and restarted your shell.

Verify the hook was added to your profile:

```bash
grep anvs ~/.bashrc  # or ~/.zshrc
```

### Version not switching

Check that your version manager is installed:

```bash
nvm --version  # or fnm --version
```

Check that anvs detects your version file:

```bash
anvs status
```

### Setup Issues

**Shell not detected:**
```bash
anvs init --shell zsh  # or bash
```

**Version manager not detected:**
- Ensure nvm or fnm is installed and in PATH
- Use advanced mode to specify manually

**Permission denied:**
- Check write permissions for ~/.anvsrc and shell config files
- Run with sudo if necessary (not recommended)

### Debug mode

Enable debug output to see what anvs is doing:

```bash
ANVS_DEBUG=1 cd my-project
```

## How It Works

anvs is installed to `~/.anvs/bin` and this directory is added to your shell's `PATH`. It integrates with your shell using the `chpwd` hook (bash/zsh) and communicates with the parent shell via file descriptor 3 (FD:3).

When you `cd` into a directory:

1. Shell hook triggers on directory change.
2. anvs searches for version files (`.nvmrc`, etc.).
3. anvs queries configured version managers (nvm, fnm) for the version.
4. If the version is missing, anvs prompts to install it.
5. An activation command is generated and written to FD:3.
6. The parent shell executes the command, changing the Node.js version.

This approach ensures anvs can modify the parent shell environment safely.

## Development

```bash
# Clone the repository
git clone https://github.com/olvrcc/anvs.git
cd anvs

# Build
cargo build

# Run tests
cargo test

# Install locally for development
cargo install --path .
anvs init
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Architecture

For detailed architecture documentation, see [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## License

MIT - see [LICENSE](./LICENSE) for details.

## Acknowledgements

Inspired by [avn](https://github.com/wbyoung/avn) by Whitney Young. anvs reimagines the concept in Rust for improved performance and reliability.
