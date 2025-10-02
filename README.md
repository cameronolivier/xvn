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
npm install -g @olvrcc/xvn
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

# Silent mode (no output)
silent: false

# Version file priority
version_files:
  - .nvmrc
  - .node-version
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

## Performance

| Tool | P50 Activation | P95 Activation | Memory |
|------|---------------|---------------|---------|
| xvn  | <100ms        | <150ms        | <5MB    |
| avn  | ~200ms        | ~300ms        | ~30MB   |

*Benchmarks run on macOS M1, Node.js 20.x*

## Troubleshooting

### Shell hook not triggering

Make sure you ran `xvn setup` and restarted your shell.

Verify the hook was added:

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

### Binary not found on install

If the postinstall script fails, try reinstalling:

```bash
npm uninstall -g @olvrcc/xvn
npm install -g @olvrcc/xvn
```

Or install from source:

```bash
git clone https://github.com/cameronolivier/xvn.git
cd xvn
cargo install --path .
xvn setup
```

### Permission denied

If you see "permission denied" when running `xvn`, the binary may not be executable:

```bash
chmod +x $(which xvn)
```

## How It Works

xvn integrates with your shell using the `chpwd` hook (bash/zsh) and communicates with the parent shell via file descriptor 3 (FD:3), the same protocol used by avn. When you `cd` into a directory:

1. Shell hook triggers on directory change
2. xvn searches for `.nvmrc` or `.node-version` files
3. xvn queries configured version managers for the version
4. If version is installed, xvn generates activation command
5. If version is missing, xvn prompts to install (if configured)
6. Activation command is written to FD:3
7. Parent shell executes the command

This approach ensures xvn can modify the parent shell environment without `eval` or `source` commands.

## Development

```bash
# Clone the repository
git clone https://github.com/cameronolivier/xvn.git
cd xvn

# Build
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Install locally
cargo install --path .
xvn setup
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for guidelines.

## Architecture

For detailed architecture documentation, see [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## License

MIT - see [LICENSE](./LICENSE) for details.

## Acknowledgements

Inspired by [avn](https://github.com/wbyoung/avn) by Whitney Young. xvn reimagines the concept in Rust for improved performance and reliability.
