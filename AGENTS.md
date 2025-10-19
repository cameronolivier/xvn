# Repository Guidelines

## Build/Test Commands
- `make check` - Run fmt, clippy, test (use before PRs)
- `cargo test -- --exact test_name` - Run single test
- `make test` - Run all tests, `make test-watch` for watch mode
- `make coverage` - Generate coverage report
- `make lint` - Run clippy with warnings as errors
- `make fmt` - Format code, `make fmt-check` to verify

## Code Style Guidelines
- Rust defaults: 4-space indent, `snake_case` for functions/modules, `CamelCase` for types
- Use `anyhow::Result<T>` for error handling, `thiserror` for custom error types
- Prefer integration tests in `tests/` over unit tests in `src/`
- Follow conventional commits: `feat(rust):`, `fix(shell):`, etc.
- Run `make check` before committing - it chains fmt, clippy, and test

## Project Structure
- `src/main.rs` entrypoint, feature logic in `config/`, `activation/`, `plugins/`, `shell/`, `setup/`
- Use `shell/anvs.sh` for shell integration, `scripts/` for automation
- Maintain >85% coverage for activation/setup flows
