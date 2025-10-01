# Milestone 2: Plugin System

**Timeline:** Weeks 3-4  
**Status:** Planning  
**Version:** v0.2.0

---

## Plan

### Goal

Implement the plugin trait definition, registry, and built-in nvm/fnm plugins to enable extensible version manager support.

### Deliverables

- [ ] Plugin trait definition (`VersionManagerPlugin`)
- [ ] Built-in nvm plugin (compiled into binary)
- [ ] Built-in fnm plugin (compiled into binary)
- [ ] Plugin registry and priority ordering
- [ ] Plugin match/install interface
- [ ] Plugin availability caching

### Testing

- Mock plugin implementation for testing
- Plugin loading/unloading lifecycle tests
- Priority ordering with multiple plugins
- Error handling for plugin failures

### Success Criteria

- nvm plugin correctly detects nvm availability
- fnm plugin correctly detects fnm availability
- Plugins loaded in correct priority order
- Tests passing with >85% coverage

---

## Architecture

### Plugin Trait

```rust
pub trait VersionManagerPlugin: Debug + Send + Sync {
    fn name(&self) -> &str;
    fn version_files(&self) -> Vec<&str>;
    fn is_available(&self) -> Result<bool>;
    fn has_version(&self, version: &str) -> Result<bool>;
    fn activate_command(&self, version: &str) -> Result<String>;
    fn install_command(&self, version: &str) -> Result<String>;
    fn resolve_version(&self, version: &str) -> Result<String> {
        Ok(version.to_string()) // Default implementation
    }
}
```

### Plugin Registry

- Discovers plugins from config
- Loads built-in plugins (nvm, fnm)
- Future: Dynamic loading from ~/.xvn/plugins/
- Caches loaded plugins
- Provides priority ordering based on config

### NVM Plugin Implementation

- Check availability: Test for ~/.nvm directory
- Check version: Run `bash -c "source ~/.nvm/nvm.sh && nvm which <version>"`
- Activate command: Return `"nvm use <version>"`
- Install command: Return `"nvm install <version>"`

### FNM Plugin Implementation

- Check availability: Run `fnm --version`
- Check version: Parse `fnm list` output
- Activate command: Return `"fnm use <version>"`
- Install command: Return `"fnm install <version>"`

See [ARCHITECTURE.md](../docs/ARCHITECTURE.md#plugin-system-architecture) for high-level plugin system design.

---

