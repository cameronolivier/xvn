.PHONY: help build run test coverage install clean dev release setup check lint fmt bench

# Default target
help:
	@echo "xvn development commands:"
	@echo ""
	@echo "Development:"
	@echo "  make dev              Build and install for local development"
	@echo "  make run              Quick compile and run"
	@echo "  make build            Build debug binary"
	@echo "  make release          Build optimized release binary"
	@echo ""
	@echo "Testing:"
	@echo "  make test             Run all tests"
	@echo "  make test-watch       Run tests on file changes"
	@echo "  make coverage         Generate code coverage report"
	@echo "  make bench            Run performance benchmarks"
	@echo ""
	@echo "Code Quality:"
	@echo "  make check            Run all checks (fmt, clippy, test)"
	@echo "  make lint             Run clippy linter"
	@echo "  make fmt              Format code"
	@echo "  make fmt-check        Check code formatting"
	@echo ""
	@echo "Installation:"
	@echo "  make install          Install xvn to ~/.cargo/bin"
	@echo "  make setup            Install and run setup (local development)"
	@echo "  make uninstall        Remove xvn from ~/.cargo/bin"
	@echo ""
	@echo "Release:"
	@echo "  make version-patch    Bump patch version (0.6.1 -> 0.6.2)"
	@echo "  make version-minor    Bump minor version (0.6.1 -> 0.7.0)"
	@echo "  make version-major    Bump major version (0.6.1 -> 1.0.0)"
	@echo ""
	@echo "Misc:"
	@echo "  make clean            Clean build artifacts"
	@echo "  make npm-pack         Create npm package tarball"

# Development
dev: install setup
	@echo "✅ xvn installed and configured for development"
	@echo "Run 'xvn --help' to see available commands"

run:
	@cargo run --quiet -- $(ARGS)

build:
	@cargo build --quiet
	@echo "✅ Debug build complete: target/debug/xvn"

release:
	@cargo build --release --quiet
	@echo "✅ Release build complete: target/release/xvn"

# Testing
test:
	@cargo test --quiet

test-watch:
	@cargo watch -x test

coverage:
	@./scripts/coverage.sh

bench:
	@cargo bench

# Code Quality
check: fmt-check lint test
	@echo "✅ All checks passed"

lint:
	@cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@cargo fmt

fmt-check:
	@cargo fmt -- --check

# Installation
install:
	@cargo install --path . --quiet
	@echo "✅ xvn installed to ~/.cargo/bin"

setup: install
	@echo "Running xvn setup..."
	@xvn setup --force
	@echo ""
	@echo "✅ Setup complete. Restart your shell or run:"
	@echo "   source ~/.bashrc  # or ~/.zshrc"

uninstall:
	@cargo uninstall xvn
	@echo "✅ xvn uninstalled"

# Release
version-patch:
	@./scripts/version.sh patch

version-minor:
	@./scripts/version.sh minor

version-major:
	@./scripts/version.sh major

# Misc
clean:
	@cargo clean
	@rm -rf native/
	@echo "✅ Build artifacts cleaned"

npm-pack:
	@npm pack
	@echo "✅ npm package created"
