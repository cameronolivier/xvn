# Milestone 9: Homebrew Distribution - Task Checklist

**Status:** Not Started
**Duration:** 1-2 days
**Dependencies:** Milestone 6 (npm distribution must be complete)

---

## Task M9.1: Create Homebrew Formula ⏳

**Objective:** Write a Homebrew formula that downloads and installs the xvn binary from GitHub releases.

**Subtasks:**

- [ ] Create `homebrew/` directory
- [ ] Write initial `homebrew/xvn.rb` formula
  - [ ] Add formula metadata (desc, homepage, version)
  - [ ] Define macOS x64 and arm64 download URLs
  - [ ] Add placeholder SHA256 checksums
  - [ ] Implement `install` method
  - [ ] Add `caveats` for post-install setup
  - [ ] Add `test` block
- [ ] Calculate actual SHA256 checksums for v1.1.0 binaries
  - [ ] Download x64 binary: `curl -L -o xvn-x64.tar.gz https://github.com/cameronolivier/xvn/releases/download/v1.1.0/xvn-x86_64-apple-darwin.tar.gz`
  - [ ] Calculate x64 checksum: `shasum -a 256 xvn-x64.tar.gz`
  - [ ] Download arm64 binary: `curl -L -o xvn-arm64.tar.gz https://github.com/cameronolivier/xvn/releases/download/v1.1.0/xvn-aarch64-apple-darwin.tar.gz`
  - [ ] Calculate arm64 checksum: `shasum -a 256 xvn-arm64.tar.gz`
  - [ ] Update formula with actual checksums
- [ ] Test formula locally
  - [ ] Run `brew install --build-from-source ./homebrew/xvn.rb`
  - [ ] Verify `xvn --version` works
  - [ ] Run `brew test xvn`
  - [ ] Test `xvn setup` works
  - [ ] Uninstall: `brew uninstall xvn`
- [ ] Test on both architectures
  - [ ] Test on Intel Mac (x64)
  - [ ] Test on Apple Silicon Mac (arm64)
- [ ] Commit formula: `git add homebrew/xvn.rb && git commit -m "feat: add Homebrew formula"`

**Validation:**
- ✅ Formula installs without errors
- ✅ `xvn --version` prints correct version
- ✅ `brew test xvn` passes
- ✅ Works on both x64 and arm64 Macs

---

## Task M9.2: Create Homebrew Tap ⏳

**Objective:** Create a custom Homebrew tap (olvrcc/homebrew-xvn) to host the formula.

**Subtasks:**

- [ ] Create GitHub repository: `olvrcc/homebrew-xvn`
  - [ ] Repository name must be `homebrew-xvn` (Homebrew convention)
  - [ ] Make repository public
  - [ ] Initialize with README
- [ ] Clone tap repository locally
  - [ ] `git clone https://github.com/olvrcc/homebrew-xvn.git`
- [ ] Set up tap structure
  - [ ] Create `Formula/` directory
  - [ ] Copy `homebrew/xvn.rb` to `Formula/xvn.rb`
- [ ] Write tap README
  - [ ] Add installation instructions
  - [ ] Add link to main xvn repository
  - [ ] Add requirements section
- [ ] Commit and push
  - [ ] `git add Formula/xvn.rb README.md`
  - [ ] `git commit -m "feat: add xvn formula"`
  - [ ] `git push`
- [ ] Test tap installation
  - [ ] `brew untap olvrcc/xvn` (if previously tapped)
  - [ ] `brew tap olvrcc/xvn`
  - [ ] `brew install olvrcc/xvn/xvn`
  - [ ] Verify `xvn --version` works
  - [ ] Test `xvn setup`
  - [ ] `brew uninstall xvn`
  - [ ] `brew untap olvrcc/xvn`

**Validation:**
- ✅ Repository is public and accessible
- ✅ `brew tap olvrcc/xvn` succeeds
- ✅ `brew install olvrcc/xvn/xvn` installs xvn
- ✅ Installation works on both Intel and Apple Silicon

---

## Task M9.3: Automate Formula Updates ⏳

**Objective:** Create GitHub Actions workflow to automatically update Homebrew formula SHA256 checksums on new releases.

**Subtasks:**

- [ ] Create workflow file: `.github/workflows/update-homebrew.yml`
- [ ] Define workflow triggers
  - [ ] Trigger on release published
  - [ ] Trigger on workflow dispatch (manual testing)
- [ ] Implement version extraction
  - [ ] Extract version from git tag
- [ ] Implement binary download
  - [ ] Download x64 macOS binary from release
  - [ ] Download arm64 macOS binary from release
- [ ] Implement SHA256 calculation
  - [ ] Calculate SHA256 for x64 binary
  - [ ] Calculate SHA256 for arm64 binary
- [ ] Implement formula update logic
  - [ ] Checkout tap repository
  - [ ] Update version in formula
  - [ ] Update URLs in formula
  - [ ] Update SHA256 checksums (both architectures)
- [ ] Implement commit and push
  - [ ] Configure git user
  - [ ] Commit changes
  - [ ] Push to tap repository
- [ ] Create GitHub secret for tap access
  - [ ] Generate personal access token with `repo` scope
  - [ ] Add as `HOMEBREW_TAP_TOKEN` secret in xvn repository settings
- [ ] Test workflow
  - [ ] Manually trigger workflow with existing release
  - [ ] Verify formula is updated correctly in tap
  - [ ] Test installing updated formula: `brew upgrade xvn`

**Validation:**
- ✅ Workflow triggers on new release
- ✅ Formula is updated with correct version and checksums
- ✅ Tap repository receives automated commit
- ✅ `brew upgrade xvn` works after formula update

---

## Task M9.4: Submit to homebrew-core (Optional) ⏸️

**Objective:** Submit xvn formula to the official homebrew-core repository for wider distribution.

**Status:** Deferred to Phase 2 (after xvn gains traction)

**Subtasks:**

- [ ] Review homebrew-core guidelines
  - [ ] Read Formula Cookbook
  - [ ] Read Acceptable Formulae guidelines
  - [ ] Ensure xvn meets criteria
- [ ] Prepare formula for submission
  - [ ] Run `brew audit --new-formula Formula/xvn.rb`
  - [ ] Fix any warnings or errors
  - [ ] Ensure formula follows style guide
- [ ] Create pull request to homebrew-core
  - [ ] Fork homebrew/homebrew-core
  - [ ] Add formula to `Formula/x/xvn.rb`
  - [ ] Commit with message: `xvn X.X.X (new formula)`
  - [ ] Open PR with description and links
- [ ] Respond to maintainer feedback
  - [ ] Address review comments
  - [ ] Update formula as requested
  - [ ] Wait for approval and merge

**Note:** This task is optional for v1.1.0. Consider submitting after xvn has >100 GitHub stars and proven stability.

---

## Task M9.5: Update Documentation ✅

**Objective:** Update README and documentation to include Homebrew installation instructions.

**Subtasks:**

- [ ] Update README.md
  - [ ] Add Homebrew installation section
  - [ ] Keep npm as "Option 1 (Recommended)"
  - [ ] Add Homebrew as "Option 2 (macOS only)"
  - [ ] Add upgrade instructions for both methods
  - [ ] Clarify that both methods install the same binary
- [ ] Update installation docs (if exists)
  - [ ] Document differences between npm and Homebrew installs
  - [ ] Document when to use each method
  - [ ] Add troubleshooting for Homebrew-specific issues
- [ ] Update release checklist (if exists)
  - [ ] Add Homebrew formula update to release process
  - [ ] Document manual steps if automation fails
- [ ] Test documentation
  - [ ] Follow Homebrew instructions on fresh Mac
  - [ ] Verify all links work
  - [ ] Ensure instructions are accurate

**Validation:**
- ✅ README includes Homebrew installation instructions
- ✅ Documentation is clear and accurate
- ✅ Both installation methods are documented
- ✅ All links work

---

## Testing Checklist

### Pre-Release Testing

- [ ] Formula works on Intel Mac (x64)
- [ ] Formula works on Apple Silicon Mac (arm64)
- [ ] `brew install olvrcc/xvn/xvn` succeeds
- [ ] `xvn --version` prints correct version
- [ ] `xvn setup` works after Homebrew install
- [ ] `brew test xvn` passes
- [ ] `brew audit xvn` passes with no errors
- [ ] `brew upgrade xvn` works correctly
- [ ] `brew uninstall xvn` removes all files
- [ ] No conflicts between npm and Homebrew installs

### Post-Release Testing

- [ ] Automation workflow triggers on release
- [ ] Formula is updated automatically in tap
- [ ] SHA256 checksums are correct
- [ ] Users can upgrade via `brew upgrade xvn`
- [ ] Documentation is accurate

---

## Success Metrics

- [ ] **Installation success rate:** >95% on macOS
- [ ] **Formula audit:** Zero errors or warnings
- [ ] **Automation success:** Formula updates automatically on every release
- [ ] **Documentation quality:** Clear instructions for both npm and Homebrew
- [ ] **User feedback:** Positive feedback on Homebrew installation experience

---

## Rollback Plan

If Homebrew distribution has critical issues:

1. **Mark formula as deprecated:**
   ```ruby
   deprecate! date: "YYYY-MM-DD", because: "critical bug description"
   ```

2. **Revert to previous version:**
   ```bash
   cd homebrew-xvn
   git revert HEAD
   git push
   ```

3. **Notify users:**
   - Update tap README with notice
   - Post issue in tap repository
   - Recommend using npm installation temporarily

4. **Fix and re-release:**
   - Fix the bug in xvn
   - Release new version
   - Update formula with fix
   - Remove deprecation notice

---

## Notes

### Homebrew Formula Versioning

- Formula version must match git tag (e.g., `v1.1.0`)
- SHA256 checksums must be updated for every release
- URLs must point to the correct release tag

### Tap Maintenance

- Tap repository should only contain Formula files and README
- Keep tap in sync with main xvn releases
- Monitor tap issues for Homebrew-specific bugs

### Future Enhancements

Ideas for Phase 2:

- [ ] Add bottles (pre-compiled binaries) for faster installation
- [ ] Support Linux via Linuxbrew
- [ ] Add post-install hook to run `xvn setup` automatically
- [ ] Integrate with `brew services` for daemon mode (v2.0+)

---

## Completion Criteria

Milestone 9 is complete when:

- ✅ All subtasks in M9.1, M9.2, M9.3, M9.5 are complete
- ✅ Formula works on both Intel and Apple Silicon Macs
- ✅ Tap is public and accessible
- ✅ Automation updates formula on new releases
- ✅ Documentation includes Homebrew instructions
- ✅ All tests pass
- ✅ Zero critical bugs in Homebrew distribution
