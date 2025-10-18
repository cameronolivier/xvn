> **⚠️ DEPRECATION NOTICE**: This package has been renamed to [`anvs`](https://www.npmjs.com/package/anvs) (Automatic Node Version Switcher).
> Please install the new package: `npm install -g anvs`
> See [README.DEPRECATION.md](./README.DEPRECATION.md) for migration instructions.
> This package will continue to work but won't receive updates.

---

# XVN - Automatic Node Version Switcher

[![CI](https://github.com/olvrcc/xvn/workflows/Test/badge.svg)](https://github.com/olvrcc/xvn/actions)
[![npm version](https://badge.fury.io/js/@olvrcc%2Fxvn.svg)](https://www.npmjs.com/package/@olvrcc/xvn)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Automatic Node.js version switching written in Rust, with a loving tip-of-the-hat to [avn](https://github.com/wbyoung/avn)

**xvn** is a Rust-based tool that automatically switches your Node.js version when you `cd` into a directory with a `.nvmrc`, `.node-version`, or `package.json` file. It automatically returns to your default version when you leave.

## Features

- 🚀 **Fast**: <100ms activation time (2-3x faster than avn)
- 🔌 **Compatible**: Works with nvm, fnm, and n
- 🔄 **Smart**: Automatically returns to default version when leaving projects
- 🤖 **Auto-install**: Prompts to install missing versions
- ⚙️  **Configurable**: Customize behavior via `~/.xvnrc`
- 🔒 **Safe**: Written in Rust with checksum verification
- 📦 **Easy**: Install via npm, no manual binary downloads

## Installation

`xvn` is installed to a central directory (`~/.xvn`) to ensure it's always available, regardless of the active Node.js version.

### Option 1: npm (Recommended for Linux and macOS)

```bash
# Step 1: Install the package
npm install -g @olvrcc/xvn

# Step 2: Set up your shell
xvn setup
```

### Option 2: Homebrew (macOS only)

```bash
# Step 1: Tap the repository
brew tap olvrcc/xvn

# Step 2: Install xvn
brew install xvn

# Step 3: Set up your shell
xvn setup
```

### Option 3: Cargo (Build from source)

```bash
# Install from source
cargo install --git https://github.com/olvrcc/xvn

# Set up your shell
xvn setup
```

### Complete the Installation

After installation via any method, restart your shell or run:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### Upgrading

**npm:**
```bash
npm update -g @olvrcc/xvn
```

**Note for nvm users:** Global packages are Node version-specific in nvm. If you installed xvn while on Node v20 but later switched to Node v22, the global package only exists in v20. To uninstall completely, switch back to the Node version where xvn was installed before running `npm uninstall -g @olvrcc/xvn`.

**Homebrew:**
```bash
brew upgrade xvn
```

**Cargo:**
```bash
cargo install --git https://github.com/olvrcc/xvn --force
```

### Upgrading from v1.1.x

If you are upgrading from an older version of `xvn`, the installation process has changed. Please follow our **[Migration Guide](./docs/MIGRATION.md)** to upgrade your existing installation.

## Migration to ANVS

This project has been renamed to **anvs**. For migration instructions, see [README.DEPRECATION.md](./README.DEPRECATION.md).

**Quick migration**:
```bash
xvn uninstall
npm install -g anvs
anvs setup
```

## Usage

Just `cd` into a directory with a version file:

```bash
cd ~/my-project  # xvn automatically switches Node.js version
cd ..            # xvn switches back to your default Node.js version
```

### Automatic Default Version

When you leave a project directory (one with a `.nvmrc` or other version file), **xvn automatically switches back to your default Node.js version**. This ensures you're always on your preferred version when not in a project.

**For nvm users:**
- xvn uses your `default` alias: `nvm alias default 20.11.0`
- Check your default: `nvm version default`

**For fnm users:**
- xvn uses fnm's default version
- Check your default: `fnm default`

**Configuration:**
```yaml
# In ~/.xvnrc
use_default: true  # (default: true)
```

Disable this behavior if you prefer manual version switching:
```bash
xvn set use-default  # Interactive toggle
```

### Supported Version Files

xvn supports multiple version file formats:

- **`.nvmrc`** - Standard nvm format with exact version or alias
  ```
  18.20.0
  ```

- **`.node-version`** - Alternative format, same as .nvmrc
  ```
  20.11.0
  ```

- **`package.json`** - npm standard with semver ranges *(new in v1.1.0)*
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
xvn activate  # Activate version for current directory
```

### Check Status

```bash
xvn status  # Show current configuration and activation timing
```

### Configuration

Create `~/.xvnrc`:

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

Project-level configuration (`.xvn.yaml` in project root):

```yaml
# Override global settings for this project
auto_install: always
silent: true
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
- Windows support planned for v1.1.0

## Uninstalling

To completely remove xvn and clean up all configuration:

```bash
xvn uninstall
```

This command will:
- Detect all xvn installations (npm, Homebrew, Cargo)
- Remove `~/.xvn` directory
- Remove `~/.xvnrc` configuration
- Remove shell integration from `.bashrc`/`.zshrc`
- Provide instructions for uninstalling external packages

Use `--force` to skip the confirmation prompt:
```bash
xvn uninstall --force
```

## Troubleshooting

### `xvn: command not found`

This can happen after installation if your shell hasn't been restarted. Make sure you have run `xvn setup` and restarted your shell.

Verify that `~/.xvn/bin` is in your `PATH`:

```bash
echo $PATH
```

Verify the `xvn` binary is in the right place:

```bash
which xvn
# Should output: /Users/your-name/.xvn/bin/xvn
```

### Shell hook not triggering

Make sure you ran `xvn setup` and restarted your shell.

Verify the hook was added to your profile:

```bash
grep xvn ~/.bashrc  # or ~/.zshrc
```

### Version not switching

Check that your version manager is installed:

```bash
nvm --version  # or fnm --version
```

Check that xvn detects your version file:

```bash
xvn status
```

## How It Works

xvn is installed to `~/.xvn/bin` and this directory is added to your shell's `PATH`. It integrates with your shell using the `chpwd` hook (bash/zsh) and communicates with the parent shell via file descriptor 3 (FD:3).

When you `cd` into a directory:

1. Shell hook triggers on directory change.
2. xvn searches for version files (`.nvmrc`, etc.).
3. xvn queries configured version managers (nvm, fnm) for the version.
4. If the version is missing, xvn prompts to install it.
5. An activation command is generated and written to FD:3.
6. The parent shell executes the command, changing the Node.js version.

This approach ensures xvn can modify the parent shell environment safely.

## Development

```bash
# Clone the repository
git clone https://github.com/olvrcc/xvn.git
cd xvn

# Build
cargo build

# Run tests
cargo test

# Install locally for development
cargo install --path .
xvn setup
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Architecture

For detailed architecture documentation, see [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## License

MIT - see [LICENSE](./LICENSE) for details.

## Acknowledgements

Inspired by [avn](https://github.com/wbyoung/avn) by Whitney Young. xvn reimagines the concept in Rust for improved performance and reliability.