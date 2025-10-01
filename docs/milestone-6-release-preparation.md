# Milestone 6: Release Preparation

**Timeline:** Weeks 11-12  
**Status:** Planning  
**Version:** v0.7.0-v1.0.0

---

## Plan

### Goal

Establish CI/CD pipeline, build binaries for all platforms, package for npm, and prepare for public release.

### Deliverables

- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Binary builds for all platforms
- [ ] npm package structure
- [ ] Install script with binary download
- [ ] Release automation
- [ ] Beta testing with real users
- [ ] Migration guide from avn

### CI/CD Pipeline

**Platforms:** ubuntu-latest, macos-latest  
**Targets:** x86_64, aarch64  
**Steps:** Test → Build → Upload artifacts → Publish to npm

### npm Package Structure

```
xvn/
├── package.json
├── install.js           # Postinstall: download binary
├── bin/xvn              # Wrapper script
├── native/              # Downloaded binaries
└── shell/xvn.sh         # Shell integration
```

### Release Process

1. Tag release: `git tag v0.1.0`
2. GitHub Actions builds binaries
3. Binaries uploaded to GitHub Release
4. npm package published: `npm publish`
5. Announce on social media, Reddit, HN

### Beta Testing

- Recruit 10-20 beta testers
- Test on various systems (Ubuntu, Debian, macOS Intel/M1)
- Test with nvm and fnm
- Collect feedback, iterate on bugs

### Success Criteria

- CI passing on all platforms
- Binaries successfully downloaded and installed
- Beta testers report successful installation
- Zero critical bugs from beta testing
- npm package installs without errors

---

## Architecture

### GitHub Actions Workflow

```yaml
jobs:
  test:
    matrix:
      os: [ubuntu-latest, macos-latest]
      rust: [stable]
    steps:
      - test: cargo test --all-features
      - coverage: cargo tarpaulin
      
  build:
    matrix:
      target: [x86_64-unknown-linux-gnu, aarch64-apple-darwin, ...]
    steps:
      - build: cargo build --release --target $target
      - upload: Upload artifacts to GitHub Release
```

### Binary Distribution

- Binaries hosted on GitHub Releases
- `install.js` detects platform, downloads correct binary
- Checksum verification (SHA256)
- Fallback to source compilation if platform unsupported

See [ARCHITECTURE.md](../ARCHITECTURE.md#deployment--distribution) for deployment details.

---

