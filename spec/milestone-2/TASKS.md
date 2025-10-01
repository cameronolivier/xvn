# Milestone 2: Plugin System - Tasks

**Timeline:** Weeks 3-4
**Version:** v0.2.0
**Status:** Not Started

---

## Tasks

### M2.1: Define plugin trait (VersionManagerPlugin)
- [x] name() method
- [x] version_files() method
- [x] is_available() method
- [x] has_version() method
- [x] activate_command() method
- [x] install_command() method
- [x] resolve_version() method

### M2.2: Implement nvm plugin
- [x] Check nvm availability (~/.nvm directory)
- [x] Check version installed (nvm which)
- [x] Generate activate command (nvm use)
- [x] Generate install command (nvm install)
- [x] Resolve version (nvm version)
- [x] Shell escaping for version strings

### M2.3: Implement fnm plugin
- [ ] Check fnm availability (fnm --version)
- [ ] Check version installed (fnm list)
- [ ] Generate activate command (fnm use)
- [ ] Generate install command (fnm install)
- [ ] Parse fnm list output

### M2.4: Implement plugin registry
- [ ] Load built-in plugins (nvm, fnm)
- [ ] Respect priority ordering from config
- [ ] find_plugin() - first match wins
- [ ] available_plugins() - filter by is_available()
- [ ] Plugin caching

### M2.5: Unit tests for plugin system
- [ ] Mock plugin implementation
- [ ] Plugin priority ordering tests
- [ ] Shell escaping tests (command injection prevention)
- [ ] Error handling (version not found)

---

## Success Criteria

- ✅ nvm plugin correctly detects nvm availability
- ✅ fnm plugin correctly detects fnm availability
- ✅ Plugins loaded in correct priority order
- ✅ Mock plugin can be tested in isolation

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
