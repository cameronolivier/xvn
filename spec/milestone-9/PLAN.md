# Milestone 9: Homebrew Distribution - Implementation Plan

## Overview

Milestone 9 establishes Homebrew as an alternative distribution channel for xvn, allowing macOS users to install via `brew install xvn` instead of npm. This provides a more native experience for macOS developers who prefer Homebrew for CLI tool management.

**Key Goals:**
- Create Homebrew formula for xvn
- Automate formula updates via GitHub Actions
- Support both x64 and arm64 macOS builds
- Maintain npm as primary distribution method
- Provide documentation for both installation methods

**Approach:**
1. Create initial Homebrew formula
2. Test formula locally on both Intel and Apple Silicon Macs
3. Submit to homebrew-core or create tap (olvrcc/homebrew-xvn)
4. Automate formula updates on new releases
5. Update documentation with Homebrew installation instructions

---

## Prerequisites

**Required:**
- Completed Milestone 6 (npm distribution working)
- GitHub repository with releases containing macOS binaries
- Access to both x64 and arm64 Macs for testing (or CI runners)
- Homebrew installed locally for testing

**Recommended:**
- Familiarity with Homebrew formula syntax
- Understanding of Homebrew tap structure
- Experience with Homebrew bottle builds (optional but helpful)

---

## Implementation Tasks

### Task M9.1: Create Homebrew Formula

**Objective:** Write a Homebrew formula that downloads and installs the xvn binary from GitHub releases.

**Implementation Steps:**

1. **Create formula file structure:**
   ```bash
   mkdir -p homebrew
   touch homebrew/xvn.rb
   ```

2. **Write basic formula:**
   - Define download URLs for both x64 and arm64
   - Specify SHA256 checksums
   - Define installation steps
   - Add test block

3. **Local testing:**
   - Install formula locally: `brew install --build-from-source ./homebrew/xvn.rb`
   - Test installation: `xvn --version`
   - Test uninstall: `brew uninstall xvn`

**Code Structure:**

- File: `homebrew/xvn.rb`
```ruby
class Xvn < Formula
  desc "Automatic Node.js version switching for cd - 2-3x faster than avn"
  homepage "https://github.com/cameronolivier/xvn"
  version "1.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/cameronolivier/xvn/releases/download/v1.1.0/xvn-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64"
    else
      url "https://github.com/cameronolivier/xvn/releases/download/v1.1.0/xvn-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X64"
    end
  end

  def install
    bin.install "xvn"

    # Install shell integration script
    prefix.install "shell/xvn.sh" if File.exist?("shell/xvn.sh")
  end

  def caveats
    <<~EOS
      To set up xvn shell integration, run:
        xvn setup

      Then restart your shell or run:
        source ~/.bashrc  # or ~/.zshrc

      xvn requires a Node.js version manager (nvm or fnm) to be installed.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/xvn --version")
  end
end
```

**Key Considerations:**
- Use `on_macos` and `Hardware::CPU.arm?` for architecture detection
- SHA256 checksums must be updated for each release
- Include caveats for post-install setup instructions
- Test block ensures basic functionality works

**Testing:**
- Test on both Intel and Apple Silicon Macs
- Verify binary downloads correctly
- Verify `xvn --version` works
- Verify `xvn setup` works

**Dependencies:**
- Requires: Milestone 6 (binary builds must be available on GitHub releases)

**Enables:**
- M9.2 (formula can be tested before tap creation)

---

### Task M9.2: Create Homebrew Tap (Optional)

**Objective:** Create a custom Homebrew tap (olvrcc/homebrew-xvn) to host the formula before submitting to homebrew-core.

**Implementation Steps:**

1. **Create tap repository:**
   ```bash
   # On GitHub, create repository: olvrcc/homebrew-xvn
   git clone https://github.com/olvrcc/homebrew-xvn.git
   cd homebrew-xvn
   ```

2. **Add formula to tap:**
   ```bash
   mkdir -p Formula
   cp ../xvn/homebrew/xvn.rb Formula/xvn.rb
   git add Formula/xvn.rb
   git commit -m "feat: add xvn formula"
   git push
   ```

3. **Test tap installation:**
   ```bash
   brew tap olvrcc/xvn
   brew install olvrcc/xvn/xvn
   xvn --version
   ```

4. **Update README:**
   - Add installation instructions
   - Add link to main xvn repository

**Code Structure:**

- Repository: `olvrcc/homebrew-xvn`
- File: `Formula/xvn.rb` (same as M9.1)
- File: `README.md`

```markdown
# Homebrew Tap for xvn

Official Homebrew tap for [xvn](https://github.com/cameronolivier/xvn).

## Installation

```bash
brew tap olvrcc/xvn
brew install xvn
```

## Setup

After installation, run:

```bash
xvn setup
```

Then restart your shell.

## Requirements

- macOS (x64 or arm64)
- Node.js version manager (nvm or fnm)

## Documentation

See the main [xvn repository](https://github.com/cameronolivier/xvn) for full documentation.
```

**Key Considerations:**
- Tap naming convention: `olvrcc/homebrew-xvn` (must start with `homebrew-`)
- Formula must be in `Formula/` directory
- Tap allows faster iteration before submitting to homebrew-core
- Users can install via `brew install olvrcc/xvn/xvn`

**Testing:**
- Test tap on fresh Mac (or VM)
- Verify `brew tap olvrcc/xvn` works
- Verify `brew install olvrcc/xvn/xvn` works
- Test uninstall and reinstall

**Dependencies:**
- Requires: M9.1 (formula must be created first)

**Enables:**
- M9.3 (automation can update tap formula)
- M9.4 (optional submission to homebrew-core)

---

### Task M9.3: Automate Formula Updates

**Objective:** Create GitHub Actions workflow to automatically update Homebrew formula SHA256 checksums when new releases are created.

**Implementation Steps:**

1. **Create workflow file:**
   ```bash
   touch .github/workflows/update-homebrew.yml
   ```

2. **Define workflow triggers:**
   - Run on release published
   - Run on workflow dispatch (manual testing)

3. **Implement SHA256 calculation:**
   - Download both macOS binaries
   - Calculate SHA256 for each
   - Update formula file with new checksums

4. **Commit and push to tap:**
   - Clone tap repository
   - Update Formula/xvn.rb
   - Commit and push changes

**Code Structure:**

- File: `.github/workflows/update-homebrew.yml`
```yaml
name: Update Homebrew Formula

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  update-formula:
    name: Update Homebrew Formula
    runs-on: ubuntu-latest

    steps:
      - name: Checkout xvn repository
        uses: actions/checkout@v4

      - name: Get release version
        id: version
        run: |
          VERSION="${GITHUB_REF#refs/tags/v}"
          echo "version=$VERSION" >> $GITHUB_OUTPUT

      - name: Download macOS x64 binary
        run: |
          curl -L -o xvn-x86_64.tar.gz \
            "https://github.com/cameronolivier/xvn/releases/download/v${{ steps.version.outputs.version }}/xvn-x86_64-apple-darwin.tar.gz"

      - name: Download macOS arm64 binary
        run: |
          curl -L -o xvn-aarch64.tar.gz \
            "https://github.com/cameronolivier/xvn/releases/download/v${{ steps.version.outputs.version }}/xvn-aarch64-apple-darwin.tar.gz"

      - name: Calculate SHA256 checksums
        id: checksums
        run: |
          SHA256_X64=$(shasum -a 256 xvn-x86_64.tar.gz | awk '{print $1}')
          SHA256_ARM64=$(shasum -a 256 xvn-aarch64.tar.gz | awk '{print $1}')
          echo "sha256_x64=$SHA256_X64" >> $GITHUB_OUTPUT
          echo "sha256_arm64=$SHA256_ARM64" >> $GITHUB_OUTPUT

      - name: Checkout homebrew-xvn tap
        uses: actions/checkout@v4
        with:
          repository: olvrcc/homebrew-xvn
          token: ${{ secrets.HOMEBREW_TAP_TOKEN }}
          path: homebrew-xvn

      - name: Update formula
        run: |
          cd homebrew-xvn
          sed -i "s/version \".*\"/version \"${{ steps.version.outputs.version }}\"/" Formula/xvn.rb
          sed -i "s|v[0-9.]\+/xvn-aarch64-apple-darwin.tar.gz|v${{ steps.version.outputs.version }}/xvn-aarch64-apple-darwin.tar.gz|" Formula/xvn.rb
          sed -i "s|v[0-9.]\+/xvn-x86_64-apple-darwin.tar.gz|v${{ steps.version.outputs.version }}/xvn-x86_64-apple-darwin.tar.gz|" Formula/xvn.rb

          # Update SHA256 for arm64 (first occurrence)
          sed -i "0,/sha256 \".*\"/s//sha256 \"${{ steps.checksums.outputs.sha256_arm64 }}\"/" Formula/xvn.rb

          # Update SHA256 for x64 (second occurrence)
          sed -i "0,/sha256 \".*\"/s//sha256 \"${{ steps.checksums.outputs.sha256_x64 }}\"/" Formula/xvn.rb

      - name: Commit and push
        run: |
          cd homebrew-xvn
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add Formula/xvn.rb
          git commit -m "chore: update xvn to v${{ steps.version.outputs.version }}"
          git push
```

**Key Considerations:**
- Requires `HOMEBREW_TAP_TOKEN` secret with write access to tap repository
- SHA256 update logic must handle both x64 and arm64 checksums
- Use `sed` carefully to update only the correct lines
- Test workflow with manual trigger before relying on automatic updates

**Testing:**
- Manually trigger workflow with existing release
- Verify formula is updated correctly in tap
- Test installing updated formula: `brew upgrade xvn`

**Dependencies:**
- Requires: M9.2 (tap must exist)

**Enables:**
- Automated Homebrew releases on every GitHub release

---

### Task M9.4: Submit to homebrew-core (Optional)

**Objective:** Submit xvn formula to the official homebrew-core repository for wider distribution.

**Implementation Steps:**

1. **Review homebrew-core guidelines:**
   - Read: https://docs.brew.sh/Formula-Cookbook
   - Read: https://docs.brew.sh/Acceptable-Formulae
   - Ensure xvn meets criteria (stable, popular, no major bugs)

2. **Prepare formula for submission:**
   - Ensure formula follows homebrew-core style guide
   - Add audit checks: `brew audit --new-formula Formula/xvn.rb`
   - Fix any warnings or errors

3. **Create pull request to homebrew-core:**
   - Fork homebrew-core repository
   - Add formula to `Formula/x/xvn.rb`
   - Commit with message: `xvn 1.1.0 (new formula)`
   - Open PR with description and links

4. **Respond to maintainer feedback:**
   - Address review comments
   - Update formula as requested
   - Wait for approval and merge

**Key Considerations:**
- homebrew-core has strict guidelines (must be notable, stable, well-maintained)
- Review process can take days or weeks
- Consider waiting until xvn has >100 GitHub stars before submitting
- Alternative: Keep using tap for faster iteration

**Recommended Timing:**
- **Phase 1 (v1.0.0):** Use custom tap (olvrcc/homebrew-xvn)
- **Phase 2 (v1.5.0+):** Submit to homebrew-core after xvn gains traction

**Testing:**
- Run `brew audit --new-formula` and fix all issues
- Test installation on fresh Mac
- Verify formula meets all homebrew-core requirements

**Dependencies:**
- Requires: M9.1, M9.2 (formula must be tested and stable)
- Requires: xvn v1.0.0+ with proven stability

**Enables:**
- Official Homebrew distribution (if accepted)

---

### Task M9.5: Update Documentation

**Objective:** Update README and documentation to include Homebrew installation instructions.

**Implementation Steps:**

1. **Update README.md:**
   - Add Homebrew installation method
   - Keep npm as primary method
   - Document both approaches

2. **Update installation docs:**
   - Create docs/INSTALLATION.md if needed
   - Document differences between npm and Homebrew installs
   - Document when to use each method

3. **Update release checklist:**
   - Add Homebrew formula update to release process
   - Document manual steps if automation fails

**Code Structure:**

- File: `README.md` (updated section)
```markdown
## Installation

### Option 1: npm (Recommended)

Works on macOS and Linux:

```bash
npm install -g @olvrcc/xvn
xvn setup
```

### Option 2: Homebrew (macOS only)

```bash
brew tap olvrcc/xvn
brew install xvn
xvn setup
```

### Option 3: Cargo (from source)

```bash
cargo install --git https://github.com/cameronolivier/xvn
xvn setup
```

Then restart your shell or run:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### Updating

**npm:**
```bash
npm update -g @olvrcc/xvn
```

**Homebrew:**
```bash
brew upgrade xvn
```
```

**Key Considerations:**
- npm should remain the primary installation method (cross-platform)
- Homebrew is an alternative for macOS users who prefer it
- Document that both methods install the same binary
- Clarify that setup process is identical regardless of install method

**Testing:**
- Test both installation methods on fresh Mac
- Verify documentation is accurate
- Check that all links work

**Dependencies:**
- Requires: M9.1, M9.2 (installation methods must work)

**Enables:**
- Users can choose preferred installation method

---

## Integration Points

### npm Distribution (Milestone 6) + Homebrew Distribution (Milestone 9)

Both distribution methods should coexist:

- npm installs from GitHub releases (downloads binary in postinstall)
- Homebrew installs from GitHub releases (downloads binary in formula)
- Both methods install identical binaries
- Users can choose preferred method based on workflow

### GitHub Releases + Homebrew Formula Updates

The automation workflow updates the Homebrew formula automatically:

- GitHub release is published (manual or automated)
- `update-homebrew.yml` workflow triggers
- Downloads macOS binaries from release
- Calculates SHA256 checksums
- Updates formula in tap repository
- Users can upgrade via `brew upgrade xvn`

---

## Testing Strategy

### Local Testing

- **Formula testing:** Test formula locally with `brew install --build-from-source`
- **Tap testing:** Test tap installation on fresh Mac
- **Upgrade testing:** Test `brew upgrade xvn` after formula update
- **Uninstall testing:** Test `brew uninstall xvn` and verify clean removal

### CI Testing

- **Automation testing:** Test `update-homebrew.yml` workflow with manual trigger
- **Formula audit:** Run `brew audit` in CI to catch issues early
- **Cross-architecture:** Test on both Intel and Apple Silicon (GitHub Actions runners)

### User Testing

- **Beta testing:** Include Homebrew installation in beta testing checklist
- **Documentation testing:** Verify installation instructions are accurate
- **Upgrade path:** Test upgrading from v1.0.0 to v1.1.0 via Homebrew

---

## Success Criteria

### Technical Success

- ✅ Homebrew formula works on both x64 and arm64 Macs
- ✅ `brew install olvrcc/xvn/xvn` installs xvn correctly
- ✅ `xvn --version` works after Homebrew install
- ✅ `xvn setup` works after Homebrew install
- ✅ Formula updates automatically on new releases
- ✅ SHA256 checksums are correct
- ✅ `brew audit` passes with no errors

### User Success

- ✅ macOS users can install via Homebrew
- ✅ Installation instructions are clear
- ✅ `brew upgrade xvn` works correctly
- ✅ No conflicts between npm and Homebrew installs

### Distribution Success

- ✅ Homebrew tap is public and accessible
- ✅ Formula is maintained and up-to-date
- ✅ Documentation includes Homebrew instructions
- ✅ (Optional) Formula accepted into homebrew-core

---

## Timeline

**Milestone 9: 1-2 days**

- **Day 1:**
  - M9.1: Create Homebrew formula (2-3 hours)
  - M9.2: Create Homebrew tap (1-2 hours)
  - M9.3: Automate formula updates (2-3 hours)

- **Day 2:**
  - Testing on both Intel and Apple Silicon Macs (2-3 hours)
  - M9.5: Update documentation (1-2 hours)
  - M9.4: (Optional) Prepare homebrew-core submission (2-4 hours)

**Note:** M9.4 (homebrew-core submission) is optional and can be deferred to Phase 2.

---

## Notes

### Homebrew vs npm Trade-offs

**Homebrew Advantages:**
- Native macOS experience
- Integrates with other Homebrew-installed tools
- No Node.js/npm required for installation
- Easier for macOS-only users

**npm Advantages:**
- Cross-platform (macOS, Linux)
- Familiar to Node.js developers
- Works without Homebrew installed
- Can be installed per-project

**Recommendation:** Support both, document npm as primary method.

### Homebrew Tap vs homebrew-core

**Custom Tap (olvrcc/homebrew-xvn):**
- ✅ Full control over formula
- ✅ Faster iteration and updates
- ✅ No approval process
- ❌ Users must tap repository first

**homebrew-core:**
- ✅ Official Homebrew distribution
- ✅ No tap required (just `brew install xvn`)
- ✅ Higher visibility
- ❌ Strict guidelines and review process
- ❌ Slower updates (maintainers must approve)

**Recommendation for v1.1.0:** Use custom tap. Submit to homebrew-core in Phase 2 (v1.5.0+) after xvn gains traction.

### Automation Considerations

The `update-homebrew.yml` workflow automates formula updates, but requires:
- `HOMEBREW_TAP_TOKEN` secret with write access to tap repository
- Correct SHA256 calculation (using `shasum -a 256`)
- Proper `sed` commands to update formula in place

If automation fails, manual fallback:
1. Download macOS binaries from release
2. Calculate SHA256: `shasum -a 256 xvn-*.tar.gz`
3. Update `homebrew/xvn.rb` with new version and checksums
4. Push to tap repository

### Future Enhancements (Phase 2+)

- **Bottles:** Pre-compiled binaries for faster installation
- **Linux support:** Extend formula to support Linux via Linuxbrew
- **Auto-setup:** Add post-install hook to run `xvn setup` automatically
- **Homebrew services:** Daemon mode integration with `brew services`
