# Milestone 8: package.json Support (v1.1.0)

**Timeline:** 1-2 weeks
**Status:** Planning
**Branch:** `milestone-8-package-json`

---

## Overview

Add support for reading Node.js version from `package.json` "engines.node" field with semver range resolution. This is a natural extension of the existing version file detection system and provides compatibility with the npm ecosystem's standard way of specifying Node.js requirements.

## Goals

1. ✅ Parse `package.json` files to extract "engines.node" field
2. ✅ Resolve semver ranges to specific versions (e.g., ">=18.0.0" → "18.20.5")
3. ✅ Integrate package.json into version file priority system
4. ✅ Maintain backward compatibility with existing .nvmrc/.node-version files
5. ✅ Handle edge cases (missing engines field, invalid semver, etc.)

## Architecture

### Current System

The existing version file system (`src/version_file/finder.rs`):
- `VersionFile::find()` walks up directory tree
- Searches for files in priority order (`.nvmrc`, `.node-version`)
- `VersionFile::parse()` reads first non-empty line
- Returns exact version string (e.g., "18.20.0", "lts/hydrogen")

### Proposed Changes

#### 1. New Package.json Parser (`src/version_file/package_json.rs`)

```rust
pub struct PackageJson {
    pub path: PathBuf,
    pub engines: Option<EnginesField>,
}

pub struct EnginesField {
    pub node: Option<String>,  // e.g., ">=18.0.0", "^20.0.0"
    pub npm: Option<String>,
}

impl PackageJson {
    /// Parse package.json file
    pub fn parse(path: &Path) -> Result<Self>;

    /// Extract Node.js version requirement
    pub fn node_version(&self) -> Option<&str>;
}
```

#### 2. Semver Range Resolver (`src/version_file/semver.rs`)

```rust
pub struct SemverResolver {
    version_manager: Box<dyn VersionManagerPlugin>,
}

impl SemverResolver {
    /// Resolve semver range to specific version
    ///
    /// Examples:
    /// - ">=18.0.0" → "18.20.5" (latest 18.x)
    /// - "^20.0.0" → "20.11.0" (latest 20.x)
    /// - "18" → "18.20.5"
    /// - "lts/*" → "20.11.0" (current LTS)
    pub fn resolve(&self, range: &str) -> Result<String>;

    /// Get list of installed versions from version manager
    fn get_installed_versions(&self) -> Result<Vec<String>>;

    /// Find best match for semver range
    fn find_best_match(&self, range: &str, versions: &[String]) -> Option<String>;
}
```

**Resolution Strategy:**
1. Parse semver range using `semver` crate
2. Query version manager for installed versions
3. Find highest installed version matching range
4. If no match, use range as-is (triggers auto-install)

#### 3. Enhanced Version File Finder

Update `VersionFile::find()` to:
1. Check for `package.json` in addition to `.nvmrc`/`.node-version`
2. If `package.json` found, parse engines.node field
3. If engines.node is a semver range, resolve it
4. Respect priority ordering (configured in `~/.xvnrc`)

**Default Priority Order:**
1. `.nvmrc` (explicit, project-specific)
2. `.node-version` (explicit, alternative format)
3. `package.json` (implicit, may be less specific)

Users can customize in config:
```yaml
version_files:
  - .nvmrc
  - package.json      # Higher priority
  - .node-version
```

#### 4. Configuration Updates

Add `package.json` to init wizard prompts:

```rust
let options = vec![
    ".nvmrc (standard Node.js convention)",
    ".node-version (alternative format)",
    "package.json (engines.node field)",  // NEW
    ".tool-versions (asdf compatibility)",
];
```

## Implementation Phases

### Phase 1: Package.json Parsing (Week 1, Days 1-2)
- [ ] Create `src/version_file/package_json.rs`
- [ ] Implement JSON parsing with `serde_json`
- [ ] Extract engines.node field
- [ ] Handle missing/invalid package.json gracefully
- [ ] Write unit tests for parser

### Phase 2: Semver Resolution (Week 1, Days 3-4)
- [ ] Create `src/version_file/semver.rs`
- [ ] Add `semver` crate dependency
- [ ] Implement range parsing and matching
- [ ] Query version manager for installed versions
- [ ] Find best match for range
- [ ] Write unit tests for resolver

### Phase 3: Integration (Week 1, Days 5-7)
- [ ] Update `VersionFile::find()` to support package.json
- [ ] Update `VersionFile::parse()` to handle semver ranges
- [ ] Integrate `SemverResolver` into activation flow
- [ ] Update config schema to include package.json option
- [ ] Update init wizard prompts

### Phase 4: Testing & Documentation (Week 2)
- [ ] Integration tests with real package.json files
- [ ] Test semver resolution with multiple scenarios
- [ ] Test priority ordering with mixed file types
- [ ] Update README with package.json examples
- [ ] Update CHANGELOG
- [ ] Code review with clippy

## Dependencies

**New Crates:**
```toml
[dependencies]
serde_json = "1.0"    # Parse package.json
semver = "1.0"        # Semver parsing and matching
```

**Existing Crates:**
- `serde` - Already used for YAML config
- `anyhow` - Error handling

## Version File Priority Examples

**Example 1: All files present**
```
project/
  ├── .nvmrc              → "18.20.0"
  ├── .node-version       → "20.0.0"
  └── package.json        → "engines.node": ">=18.0.0"
```
Result: Uses `.nvmrc` (18.20.0) - highest priority

**Example 2: Only package.json**
```
project/
  └── package.json        → "engines.node": "^20.0.0"
```
Result: Resolves "^20.0.0" → "20.11.0" (latest 20.x installed)

**Example 3: Custom priority order**
```yaml
# ~/.xvnrc
version_files:
  - package.json    # Check package.json first
  - .nvmrc
```
Result: Uses package.json if present, falls back to .nvmrc

## Edge Cases & Error Handling

### 1. Missing engines.node
```json
{
  "name": "my-app",
  "version": "1.0.0"
  // No engines field
}
```
→ Skip package.json, continue searching

### 2. Invalid semver range
```json
{
  "engines": {
    "node": "banana"
  }
}
```
→ Treat as literal version (will fail activation, show helpful error)

### 3. No matching versions
```json
{
  "engines": {
    "node": ">=22.0.0"
  }
}
```
→ Trigger auto-install with semver range (version manager handles it)

### 4. Multiple satisfying versions
```json
{
  "engines": {
    "node": ">=18.0.0"
  }
}
```
Installed: [18.20.0, 20.0.0, 20.11.0]
→ Use highest match (20.11.0)

### 5. Exact version in package.json
```json
{
  "engines": {
    "node": "18.20.0"
  }
}
```
→ Use exact version (no resolution needed)

## Testing Strategy

### Unit Tests
- Package.json parsing (valid, invalid, missing fields)
- Semver range resolution (>=, ^, ~, ||, *)
- Version matching (exact, ranges, wildcards)
- Error handling (malformed JSON, invalid semver)

### Integration Tests
- End-to-end activation with package.json
- Priority ordering with mixed file types
- Semver resolution with real version managers
- Auto-install triggered by unmet semver range

### Test Cases
```rust
#[test]
fn test_parse_package_json_with_engines() {
    // Valid package.json with engines.node
}

#[test]
fn test_parse_package_json_without_engines() {
    // package.json without engines field
}

#[test]
fn test_resolve_caret_range() {
    // "^20.0.0" → "20.11.0"
}

#[test]
fn test_resolve_gte_range() {
    // ">=18.0.0" → highest 18+ version
}

#[test]
fn test_priority_nvmrc_over_package_json() {
    // .nvmrc takes precedence over package.json
}

#[test]
fn test_package_json_fallback() {
    // Use package.json when .nvmrc missing
}
```

## Performance Considerations

1. **JSON Parsing Overhead**
   - `serde_json` is fast (<1ms for typical package.json)
   - Parse only when package.json is in priority list
   - Cache parsed result during activation

2. **Version Manager Queries**
   - Query installed versions once per activation
   - Cache result for semver resolution
   - Reuse existing plugin list_versions() API

3. **Semver Matching**
   - `semver` crate is well-optimized
   - O(n) scan of installed versions (typically <20 items)
   - Negligible impact (<5ms)

**Target:** Package.json support adds <10ms to activation time

## Backward Compatibility

✅ **Fully backward compatible:**
- Existing .nvmrc/.node-version files work unchanged
- Default config doesn't include package.json (opt-in)
- No breaking changes to APIs or config format
- New feature, purely additive

## Migration Path

**For existing users:**
1. Upgrade to v1.1.0
2. Run `xvn init` to update config
3. Select `package.json` in version files prompt
4. Config automatically updated with package.json support

**For new users:**
1. `xvn init` includes package.json option by default
2. Wizard explains semver range support
3. Default priority: .nvmrc → .node-version → package.json

## Success Metrics

1. ✅ Package.json parsing works for 100% of valid npm packages
2. ✅ Semver resolution accuracy >95% (matches npm's resolution)
3. ✅ Activation time increase <10ms (P95)
4. ✅ Zero breaking changes for existing users
5. ✅ Test coverage >85% for new code

## Documentation Updates

### README.md
- Add package.json to supported version files
- Show example with engines.node field
- Explain semver range resolution
- Document priority ordering

### CHANGELOG.md
- New feature: package.json support
- Semver range resolution
- Updated init wizard

### User Guide (Future)
- How to use package.json for version pinning
- Semver range best practices
- Troubleshooting semver resolution

## Future Enhancements (Out of Scope)

- [ ] Support for engines.npm (npm version requirement)
- [ ] Integration with package-lock.json
- [ ] Workspace support (monorepo with multiple package.json files)
- [ ] Semver range validation in init wizard
- [ ] .nvmrc generation from package.json

---

## References

- [npm engines field](https://docs.npmjs.com/cli/v10/configuring-npm/package-json#engines)
- [semver crate](https://docs.rs/semver/latest/semver/)
- [Node.js version resolution](https://nodejs.org/en/about/previous-releases)
- Existing implementation: `src/version_file/finder.rs`
