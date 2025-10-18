# Repository Guidelines

## Project Structure & Module Organization
- `src/` hosts the Rust CLI: keep entrypoints in `main.rs`/`cli.rs`, while feature logic lives under `config/`, `activation/`, `plugins/`, `shell/`, and `setup/`.
- `tests/` contains integration coverage; add scenario fixtures beside the test modules. Use `shell/anvs.sh` for shell hook updates and `scripts/` for automation helpers (coverage, version bumps, artifact handling).
- `docs/` and `spec/` capture architecture notes and phased plans—refresh them when workflows shift. Packaging assets reside in `bin/`, `native/`, `install.js`, and the `homebrew*/` directories.

## Build, Test, and Development Commands
- `make dev` installs the binary locally and runs `anvs setup` for shell integration.
- `cargo run -- ARGS` or `make run` executes the CLI quickly for manual testing.
- `make build` / `make release` produce debug and optimized binaries under `target/`.
- `make test`, `make test-watch`, and `npm test` exercise the Rust suites; `./scripts/coverage.sh` emits coverage reports.
- `make check` chains `fmt`, `clippy`, and `test`; rely on it before pushing or opening a PR.

## Coding Style & Naming Conventions
- Follow Rust defaults: 4-space indentation, `snake_case` for modules/functions, `CamelCase` for types. Split larger features into submodules inside `src/<domain>/`.
- Run `cargo fmt` (or `make fmt`) before committing; the repository relies on rustfmt defaults.
- Treat clippy warnings as errors (`make lint`). Use `#[allow(...)]` only with an inline comment explaining the exception.

## Testing Guidelines
- Prefer integration coverage in `tests/`; mirror command scenarios using files like `tests/<feature>_spec.rs`. For shell behavior, exercise `test_version.sh`.
- Maintain >85% coverage when altering activation or setup flows; run `./scripts/coverage.sh` to verify.
- Use descriptive test names (`should_switch_to_default_version`) tied to CLI behavior, and add regression cases for every bug fix.

## Commit & Pull Request Guidelines
- Commits follow Conventional Commits with scopes (`feat(rust):`, `fix(shell):`). Squash trivial fixups locally (`git rebase -i`) to keep history readable.
- Each PR should include a summary, linked issue or roadmap reference, screenshots or `asciinema` clips for UX changes, and a checklist highlighting `make check` results.
- Avoid combining feature work and release chores; use `scripts/version.sh` targets for version bumps and keep packaging updates isolated.
