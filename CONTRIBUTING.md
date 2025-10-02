# Contributing to xvn

Thank you for your interest in contributing to xvn!

## Quick Start

### Prerequisites

- Rust 1.70+ ([install via rustup](https://rustup.rs/))
- Node.js 14+ (for npm packaging)
- nvm or fnm (for testing)

### Development Setup

**Option 1: Using Make (recommended)**

```bash
# Clone the repository
git clone https://github.com/cameronolivier/xvn.git
cd xvn

# Install and set up for development
make dev

# See all available commands
make help
```

**Option 2: Using cargo directly**

```bash
# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .

# Set up shell integration
xvn setup
```

**Option 3: Using npm scripts**

```bash
# Install dependencies and build
npm run dev

# Run tests
npm test

# Check code quality
npm run check
```

## Common Development Tasks

### Building

```bash
make build          # Debug build
make release        # Release build (optimized)
cargo build         # Same as make build
cargo build --release
```

### Testing

```bash
make test           # Run all tests
make test-watch     # Auto-run tests on file changes
make coverage       # Generate coverage report
make bench          # Run benchmarks
cargo test          # Run tests directly
```

### Code Quality

```bash
make check          # Run all checks (fmt, clippy, test)
make lint           # Run clippy
make fmt            # Format code
cargo clippy        # Lint directly
cargo fmt           # Format directly
```

### Local Testing

```bash
# Install from source
make install
# or
cargo install --path .

# Set up shell integration
xvn setup

# Test it
cd /path/to/project-with-nvmrc
# Should automatically switch Node.js version
```

### Making Changes

1. **Create a branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes**
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   make check  # Runs fmt, clippy, and tests
   ```

4. **Commit with conventional commits**
   ```bash
   git commit -m "feat: add new feature"
   git commit -m "fix: resolve bug"
   git commit -m "docs: update README"
   ```

5. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```

## Conventional Commits

We use [conventional commits](https://www.conventionalcommits.org/) for clear commit history:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks
- `perf:` - Performance improvements

## Project Structure

```
xvn/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI parsing
│   ├── config/              # Configuration loading
│   ├── plugins/             # Plugin system
│   ├── shell/               # Shell integration
│   ├── activation/          # Activation orchestration
│   ├── version_file/        # Version file detection
│   └── setup/               # Setup command
├── tests/                   # Integration tests
├── shell/xvn.sh            # Shell hook script
├── install.js              # npm postinstall script
├── bin/xvn                 # npm bin wrapper
└── Makefile                # Development commands
```

## Testing Guidelines

- Write tests for all new features
- Ensure existing tests pass
- Aim for >85% code coverage
- Test edge cases and error conditions
- Use integration tests for end-to-end flows

## Release Process

Only maintainers can create releases. The process is:

```bash
# Bump version
make version-patch   # Bug fixes (0.6.1 -> 0.6.2)
make version-minor   # New features (0.6.1 -> 0.7.0)
make version-major   # Breaking changes (0.6.1 -> 1.0.0)

# Push
git push && git push --tags

# Wait for CI to build binaries
# Then publish to npm
npm publish --access public
```

## Getting Help

- **Issues:** [GitHub Issues](https://github.com/cameronolivier/xvn/issues)
- **Discussions:** [GitHub Discussions](https://github.com/cameronolivier/xvn/discussions)
- **Documentation:** [Architecture docs](./docs/ARCHITECTURE.md)

## Code of Conduct

Be respectful and constructive. We want xvn to be a welcoming project for everyone.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
