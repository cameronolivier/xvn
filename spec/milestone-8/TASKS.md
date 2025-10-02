# Milestone 8: package.json Support - Task Checklist

**Version:** v1.1.0
**Timeline:** 1-2 weeks
**Status:** Not Started

---

## Phase 1: Package.json Parsing (Days 1-2)

### Setup
- [ ] Add `serde_json` dependency to Cargo.toml
- [ ] Add `semver` dependency to Cargo.toml
- [ ] Create `src/version_file/package_json.rs`
- [ ] Create `src/version_file/semver.rs`

### Package.json Parser Implementation
- [ ] Define `PackageJson` struct with path and engines fields
- [ ] Define `EnginesField` struct with node and npm fields
- [ ] Implement `PackageJson::parse()` - read and parse JSON
- [ ] Implement `PackageJson::node_version()` - extract engines.node
- [ ] Handle missing package.json gracefully (return None)
- [ ] Handle malformed JSON gracefully (return Error)
- [ ] Handle missing engines field (return None)
- [ ] Handle missing engines.node field (return None)

### Unit Tests - Parser
- [ ] Test: Parse valid package.json with engines.node
- [ ] Test: Parse package.json without engines field
- [ ] Test: Parse package.json with engines but no node field
- [ ] Test: Parse package.json with invalid JSON
- [ ] Test: Parse package.json with only engines.npm (no node)
- [ ] Test: Handle non-existent file path
- [ ] Test: Extract exact version ("18.20.0")
- [ ] Test: Extract semver range (">=18.0.0")
- [ ] Test: Extract caret range ("^20.0.0")
- [ ] Test: Extract tilde range ("~18.20.0")

---

## Phase 2: Semver Resolution (Days 3-4)

### Semver Resolver Implementation
- [ ] Define `SemverResolver` struct with version_manager field
- [ ] Implement `SemverResolver::new()` constructor
- [ ] Implement `get_installed_versions()` - query version manager
- [ ] Implement semver range parsing using `semver` crate
- [ ] Implement `find_best_match()` - highest version matching range
- [ ] Implement `resolve()` - main public API
- [ ] Handle exact versions (no resolution needed)
- [ ] Handle semver ranges (>=, ^, ~, ||, *)
- [ ] Handle wildcards (18.x, 18.*, *)
- [ ] Handle LTS aliases (lts/*, lts/hydrogen)
- [ ] Handle no matching versions (return original range)
- [ ] Add logging for resolution steps

### Unit Tests - Semver Resolver
- [ ] Test: Resolve exact version (18.20.0 → 18.20.0)
- [ ] Test: Resolve caret range (^20.0.0 → 20.11.0)
- [ ] Test: Resolve tilde range (~18.20.0 → 18.20.5)
- [ ] Test: Resolve >= range (>=18.0.0 → 20.11.0)
- [ ] Test: Resolve wildcard (18.x → 18.20.5)
- [ ] Test: Resolve || (||18 ||20 → 20.11.0)
- [ ] Test: No matching versions returns original
- [ ] Test: Empty installed versions list
- [ ] Test: Invalid semver range (pass through as-is)
- [ ] Test: LTS alias resolution

---

## Phase 3: Integration (Days 5-7)

### Version File Finder Updates
- [ ] Update `VersionFile` struct to include source type (nvmrc/package/etc)
- [ ] Update `VersionFile::find()` to check for package.json
- [ ] Add package.json to search list (respecting priority)
- [ ] Parse package.json when found
- [ ] Extract engines.node field
- [ ] Integrate `SemverResolver` for range resolution
- [ ] Update `VersionFile::parse()` to handle package.json path
- [ ] Maintain backward compatibility with existing version files
- [ ] Add debug logging for package.json detection

### Configuration Schema Updates
- [ ] Add package.json to default version_files list (as option)
- [ ] Update config validation to accept "package.json"
- [ ] Ensure backward compatibility (don't break existing configs)

### Init Wizard Updates
- [ ] Add "package.json (engines.node field)" to version files prompt
- [ ] Update prompt help text to explain semver ranges
- [ ] Update default selections (keep .nvmrc and .node-version)
- [ ] Add package.json as optional third choice

### Activation Orchestrator Updates
- [ ] Pass version manager to SemverResolver
- [ ] Handle semver-resolved versions in activation flow
- [ ] Log when semver resolution occurs
- [ ] Ensure auto-install works with resolved versions

### Integration Tests
- [ ] Test: Find package.json in current directory
- [ ] Test: Find package.json in parent directory
- [ ] Test: Priority - .nvmrc over package.json
- [ ] Test: Priority - custom order in config
- [ ] Test: package.json only (no .nvmrc)
- [ ] Test: package.json with exact version
- [ ] Test: package.json with semver range
- [ ] Test: End-to-end activation with package.json
- [ ] Test: Auto-install triggered by semver range
- [ ] Test: Multiple satisfying versions (use highest)

---

## Phase 4: Testing & Documentation (Week 2)

### Additional Testing
- [ ] Test with real-world package.json files (Next.js, React, Express)
- [ ] Test priority ordering with all combinations
- [ ] Test error messages are helpful
- [ ] Test performance impact (<10ms overhead)
- [ ] Run full test suite (cargo test)
- [ ] Check code coverage (>85% for new code)

### Code Quality
- [ ] Run clippy (cargo clippy --all-targets)
- [ ] Fix all warnings
- [ ] Format code (cargo fmt)
- [ ] Review error handling
- [ ] Review logging levels

### Documentation
- [ ] Update README.md - Add package.json to supported files
- [ ] Update README.md - Add example with engines.node
- [ ] Update README.md - Explain semver resolution
- [ ] Update README.md - Document priority ordering
- [ ] Update CHANGELOG.md - New feature entry
- [ ] Update CHANGELOG.md - Breaking changes (none)
- [ ] Add inline documentation to new modules
- [ ] Update module-level docs

### Examples & Demos
- [ ] Create example project with package.json
- [ ] Test activation with Next.js project
- [ ] Test activation with React project
- [ ] Document common semver patterns

---

## Release Preparation

### Version Bump
- [ ] Update version in Cargo.toml (0.8.0 → 1.1.0)
- [ ] Update version in package.json (0.8.0 → 1.1.0)
- [ ] Update version in tests/cli_test.rs
- [ ] Run version script (./scripts/version.sh)

### Pre-release Checklist
- [ ] All tests passing (cargo test)
- [ ] No clippy warnings (cargo clippy)
- [ ] Code formatted (cargo fmt --check)
- [ ] Documentation complete
- [ ] CHANGELOG updated
- [ ] Git commit with conventional commit message
- [ ] Git tag v1.1.0

### Post-release
- [ ] Update PROGRESS.md - Mark Milestone 8 complete
- [ ] Merge to main branch
- [ ] Push tag to GitHub
- [ ] Create GitHub release
- [ ] Monitor for issues

---

## Edge Cases Checklist

- [ ] package.json with no engines field → Skip, continue search
- [ ] package.json with engines but no node → Skip, continue search
- [ ] package.json with invalid JSON → Error with helpful message
- [ ] package.json with invalid semver → Treat as literal, let version manager handle
- [ ] Semver range with no matching versions → Trigger auto-install with range
- [ ] Multiple versions satisfy range → Use highest
- [ ] Exact version in package.json → No resolution, use directly
- [ ] Empty installed versions list → Pass through range
- [ ] Concurrent .nvmrc and package.json → Respect priority order
- [ ] package.json in parent directory → Found via walk-up search

---

## Success Criteria

- [x] ✅ Package.json parsing works for 100% of valid npm packages
- [x] ✅ Semver resolution matches npm behavior (>95% accuracy)
- [x] ✅ Activation time overhead <10ms (P95)
- [x] ✅ Zero breaking changes for existing users
- [x] ✅ Test coverage >85% for new code
- [x] ✅ All existing tests still pass
- [x] ✅ Init wizard includes package.json option
- [x] ✅ Documentation complete and clear

---

## Notes

- Keep package.json support opt-in (not in default config initially)
- Focus on common semver patterns first (^, ~, >=)
- Leverage existing version manager APIs (list_versions)
- Maintain performance target (<100ms total activation)
- Follow existing error handling patterns
- Use existing logging infrastructure

---

**Last Updated:** October 2, 2025
**Next Review:** After Phase 1 completion
