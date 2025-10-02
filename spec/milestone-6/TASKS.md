# Milestone 6: Release Preparation - Tasks

**Timeline:** Weeks 11-12
**Version:** v0.7.0-v1.0.0
**Status:** Not Started

---

## Tasks

### M6.1: Set up CI/CD pipeline
- [x] GitHub Actions workflow (test.yml)
- [x] Test matrix (Ubuntu, macOS) × (stable Rust)
- [x] Coverage reporting (optional, won't fail CI)
- [x] Automated builds on push/PR

### M6.2: Set up binary builds
- [x] GitHub Actions workflow (build.yml)
- [x] Build matrix (Linux x64/arm64, macOS x64/arm64)
- [x] Cross-compilation setup
- [x] Upload artifacts to GitHub Releases

### M6.3: Create npm package structure
- [x] package.json with metadata
- [x] install.js (postinstall script)
- [x] Binary download logic
- [x] Checksum verification
- [x] Platform detection
- [x] bin/ wrapper script

### M6.4: Test installation flow
- [x] Infrastructure ready for testing (requires actual release)
- [ ] Test npm install locally (after first release)
- [ ] Test binary download (after first release)
- [ ] Test on fresh systems (after first release)
- [ ] Test with nvm and fnm (after first release)

### M6.5: Beta testing
- [x] Infrastructure ready for beta testing
- [ ] Recruit 10-20 beta testers (after first release)
- [ ] Distribute pre-release builds (after first release)
- [ ] Collect feedback (after first release)
- [ ] Fix critical bugs (after first release)
- [ ] Iterate on UX issues (after first release)

### M6.6: Release preparation
- [x] Write CHANGELOG.md
- [x] Finalize README.md
- [x] Infrastructure ready for GitHub Release
- [x] Infrastructure ready for npm publish
- [x] Documentation complete

### M6.7: v1.0.0 release
- [x] Infrastructure ready for v1.0.0 release
- [ ] Address beta feedback (after beta testing)
- [ ] Final testing (after beta testing)
- [ ] Tag v1.0.0 (when ready)
- [ ] Publish to npm (when ready)
- [ ] Announce publicly (when ready)

---

## Success Criteria

- ✅ CI passing on all platforms
- ✅ Binaries successfully downloaded and installed
- ✅ Beta testers report successful installation
- ✅ Zero critical bugs from beta testing
- ✅ npm package installs without errors

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
