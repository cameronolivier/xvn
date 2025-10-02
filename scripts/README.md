# Scripts

Utility scripts for xvn development and releases.

## version.sh

Semantic version bumping with automatic file updates, testing, and git tagging.

**Usage:**

```bash
# Patch release (bug fixes)
./scripts/version.sh patch   # 0.6.1 -> 0.6.2

# Minor release (new features, backwards compatible)
./scripts/version.sh minor   # 0.6.1 -> 0.7.0

# Major release (breaking changes)
./scripts/version.sh major   # 0.6.1 -> 1.0.0

# Exact version
./scripts/version.sh 1.0.0   # Set to 1.0.0
```

**What it does:**

1. Prompts for confirmation
2. Updates `Cargo.toml`, `package.json`, and test files
3. Rebuilds and runs tests
4. Creates git commit with conventional commit message
5. Creates git tag (e.g., `v1.0.0`)
6. Prints push instructions

**After running:**

```bash
# Push to trigger GitHub Actions build
git push origin main
git push origin v1.0.0

# Or push everything at once
git push && git push --tags
```

## bump-version.sh (deprecated)

Original milestone-based version bumping (0.X.0 only).

**Migration:** Use `version.sh` instead for more flexible versioning.

## Other Scripts

### coverage.sh

Generate code coverage reports.

```bash
./scripts/coverage.sh
```

### bump-version.sh

Legacy milestone-based versioning (use `version.sh` instead).
