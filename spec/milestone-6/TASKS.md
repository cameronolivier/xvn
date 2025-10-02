# Milestone 6: Release Preparation - Tasks

**Timeline:** Weeks 11-12
**Version:** v0.7.0-v1.0.0
**Status:** Not Started

---

## Tasks

### M6.1: Set up CI/CD pipeline
- [x] GitHub Actions workflow (test.yml)
- [x] Test matrix (Ubuntu, macOS) × (stable Rust)
- [x] Coverage reporting (Codecov)
- [x] Automated builds on push/PR

### M6.2: Set up binary builds
- [ ] GitHub Actions workflow (build.yml)
- [ ] Build matrix (Linux x64/arm64, macOS x64/arm64)
- [ ] Cross-compilation setup
- [ ] Upload artifacts to GitHub Releases

### M6.3: Create npm package structure
- [ ] package.json with metadata
- [ ] install.js (postinstall script)
- [ ] Binary download logic
- [ ] Checksum verification
- [ ] Platform detection
- [ ] bin/ wrapper script

### M6.4: Test installation flow
- [ ] Test npm install locally
- [ ] Test binary download
- [ ] Test on fresh systems (Ubuntu, macOS)
- [ ] Test with nvm and fnm

### M6.5: Beta testing
- [ ] Recruit 10-20 beta testers
- [ ] Distribute pre-release builds
- [ ] Collect feedback (setup, performance, errors)
- [ ] Fix critical bugs
- [ ] Iterate on UX issues

### M6.6: Release preparation
- [ ] Write CHANGELOG.md
- [ ] Finalize README.md
- [ ] Create GitHub Release (v0.7.0-beta)
- [ ] Publish to npm (beta tag)
- [ ] Test end-to-end installation

### M6.7: v1.0.0 release
- [ ] Address beta feedback
- [ ] Final testing
- [ ] Tag v1.0.0
- [ ] Publish to npm (latest tag)
- [ ] Announce on social media, Reddit, HN

---

## Success Criteria

- ✅ CI passing on all platforms
- ✅ Binaries successfully downloaded and installed
- ✅ Beta testers report successful installation
- ✅ Zero critical bugs from beta testing
- ✅ npm package installs without errors

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
