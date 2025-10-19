# Phase 9: GitHub Repository Changes

**Status**: Completed
**Version**: v2.0.0
**Duration Estimate**: 20-30 minutes

## Overview

Phase 9 handles the critical step of renaming the main GitHub repository from `xvn` to `anvs`. This is a straightforward but important change that must be done carefully to ensure continuity of URLs, issues, pull requests, and repository metadata.

**Why Phase 9 is Critical:**
- Updates the canonical repository name to match the renamed project
- GitHub automatically sets up redirects from old URLs to new URLs
- Maintains all history, issues, PRs, and stars
- Updates repository metadata to reflect the new project identity
- Required before publishing the renamed package to npm

**⚠️ IMPORTANT NOTES:**
- GitHub provides automatic redirects: `olvrcc/xvn` → `olvrcc/anvs`
- All existing links will continue to work
- Issues, PRs, stars, and watchers are preserved
- Clone URLs will change but old URLs will redirect
- This change is reversible if needed (within a short time window)

---

## Implementation Tasks

### Task 9.1: Prepare for Repository Rename

**Pre-rename checklist**:

Before renaming the repository, ensure:
- [ ] All Phase 1-8 changes are committed and pushed
- [ ] Working directory is clean (`git status`)
- [ ] All CI/CD workflows have passed
- [ ] Latest commit is on `main` branch
- [ ] No pending pull requests that need the old name

**Commands**:
```bash
# Verify clean working directory
git status

# Verify on main branch
git branch --show-current

# Verify latest commits
git log --oneline -5

# Verify all changes pushed
git fetch origin
git status
```

**Expected output**:
```
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean
```

**Actions**:
- [ ] Verify working directory is clean
- [ ] Verify on `main` branch
- [ ] Verify all changes pushed to remote
- [ ] Verify no pending PRs that would break
- [ ] Backup repository URL: `git remote get-url origin`

---

### Task 9.2: Rename Repository on GitHub

**Navigate to repository settings on GitHub**:

1. **Open repository settings**:
   - Go to: https://github.com/olvrcc/xvn
   - Click **Settings** tab
   - Scroll down to **Danger Zone** section

2. **Rename the repository**:
   - Click **Rename** button (or **Change repository name**)
   - **Old name**: `xvn`
   - **New name**: `anvs`
   - Read the warning about redirects
   - Type the new name to confirm: `anvs`
   - Click **I understand, rename repository**

3. **GitHub's automatic actions**:
   - Redirects created: `github.com/olvrcc/xvn` → `github.com/olvrcc/anvs`
   - All issues, PRs, stars preserved
   - Commit history intact
   - Clone URLs updated

**Expected result**:
- Repository accessible at: https://github.com/olvrcc/anvs
- Old URL redirects automatically: https://github.com/olvrcc/xvn → https://github.com/olvrcc/anvs

**Actions**:
- [ ] Navigate to repository settings
- [ ] Click rename in Danger Zone
- [ ] Enter new name: `anvs`
- [ ] Confirm rename
- [ ] Verify redirect works (visit old URL)
- [ ] Verify new URL loads correctly

---

### Task 9.3: Update Local Git Remote

**Update your local repository's remote URL**:

After renaming on GitHub, update your local clone to use the new URL.

**Commands**:
```bash
# Check current remote URL
git remote get-url origin

# Should show: https://github.com/olvrcc/xvn.git
# or: git@github.com:olvrcc/xvn.git

# Update to new URL (HTTPS)
git remote set-url origin https://github.com/olvrcc/anvs.git

# Or update to new URL (SSH)
git remote set-url origin git@github.com:olvrcc/anvs.git

# Verify new URL
git remote get-url origin

# Test connection
git fetch origin

# Verify branch tracking
git branch -vv
```

**Expected output after `git remote get-url origin`**:
```
https://github.com/olvrcc/anvs.git
```

or

```
git@github.com:olvrcc/anvs.git
```

**Verify fetch works**:
```bash
git fetch origin
```

**Expected output**:
```
From https://github.com/olvrcc/anvs
 * branch            main       -> FETCH_HEAD
```

**Actions**:
- [ ] Check current remote URL
- [ ] Update remote URL to new repository name
- [ ] Verify new URL is set correctly
- [ ] Test `git fetch` succeeds
- [ ] Test `git pull` succeeds
- [ ] Verify branch tracking still works

---

### Task 9.4: Update Repository Description

**Update repository metadata on GitHub**:

1. **Go to repository homepage**: https://github.com/olvrcc/anvs

2. **Click the gear icon** (⚙️) next to **About** section

3. **Update description**:
   - **Old**: "Automatic Node Version Switcher for Node.js - faster avn alternative written in Rust 🚀"
   - **New**: "ANVS - Automatic Node Version Switcher for Node.js - fast, Rust-based version switching with nvm/fnm support 🚀"

4. **Update website** (if set):
   - **Old**: `https://github.com/olvrcc/xvn`
   - **New**: `https://www.npmjs.com/package/anvs` (after publishing)
   - For now, leave as: `https://github.com/olvrcc/anvs`

5. **Update topics/tags**:
   - Add: `anvs`, `node-version-switcher`, `automatic-version-switching`
   - Keep: `nodejs`, `rust`, `nvm`, `fnm`, `avn`, `version-manager`
   - Remove: `xvn` (if present)

**Actions**:
- [ ] Click gear icon in About section
- [ ] Update description to reference "ANVS"
- [ ] Update website URL (after npm publish, or use GitHub URL for now)
- [ ] Add new topics: `anvs`, `node-version-switcher`, `automatic-version-switching`
- [ ] Keep relevant existing topics: `nodejs`, `rust`, `nvm`, `fnm`, `avn`
- [ ] Remove `xvn` topic if present
- [ ] Save changes
- [ ] Verify About section displays correctly

---

### Task 9.5: Update Repository Homepage URL

**If you have a homepage URL set in repository settings**:

1. **Navigate to repository settings**: https://github.com/olvrcc/anvs/settings

2. **Scroll to "Website" field** (in the main settings, not About section)

3. **Update homepage URL**:
   - For now: `https://github.com/olvrcc/anvs`
   - After npm publish: `https://www.npmjs.com/package/anvs`

**Note**: This may be the same as the About section website. If so, this step is already complete.

**Actions**:
- [ ] Check if homepage URL is set in settings
- [ ] Update to new repository URL or npm URL
- [ ] Save changes
- [ ] Verify URL is correct

---

### Task 9.6: Update Repository Social Preview (Optional)

**Update the social media preview image** (optional but recommended):

1. **Navigate to repository settings**: https://github.com/olvrcc/anvs/settings

2. **Scroll to "Social preview" section**

3. **Upload new image** (if you have one):
   - Image should reference "ANVS" instead of "XVN"
   - Recommended size: 1280×640 pixels
   - If no custom image, GitHub will auto-generate one

4. **Skip this step if**:
   - No custom social preview currently set
   - No time to create new image
   - Default GitHub preview is acceptable

**Actions**:
- [ ] Check if custom social preview exists
- [ ] If exists and references "xvn", consider updating
- [ ] If no custom preview, skip this step
- [ ] If updated, verify preview looks correct

---

### Task 9.7: Verify Repository Settings

**Double-check all repository settings are correct**:

```bash
# From local repository, verify settings
git remote -v

# Should show:
# origin  https://github.com/olvrcc/anvs.git (fetch)
# origin  https://github.com/olvrcc/anvs.git (push)
```

**On GitHub, verify**:
- [ ] Repository name is `anvs`
- [ ] Repository URL is `https://github.com/olvrcc/anvs`
- [ ] Old URL redirects: `https://github.com/olvrcc/xvn` → `https://github.com/olvrcc/anvs`
- [ ] Description updated
- [ ] Topics/tags updated
- [ ] Website URL updated (or points to GitHub for now)
- [ ] Default branch is still `main`
- [ ] All issues, PRs, stars intact

**Check redirect works**:
```bash
# Test old URL redirect (using curl)
curl -I https://github.com/olvrcc/xvn

# Should see HTTP 301 redirect to:
# Location: https://github.com/olvrcc/anvs
```

**Actions**:
- [ ] All repository settings verified
- [ ] Redirects working
- [ ] Repository accessible at new URL
- [ ] All data preserved (issues, PRs, stars, etc.)

---

### Task 9.8: Test Git Operations

**Verify all git operations work with the new URL**:

```bash
# Test fetch
git fetch origin

# Test pull
git pull origin main

# Make a test commit (update this phase file to mark task complete)
echo "# Phase 9 verification: $(date)" >> /tmp/test-phase-9.txt
cat /tmp/test-phase-9.txt

# Stage a small change (update spec docs to mark phase complete)
git add spec/milestone-12-renaming-to-anvs/phase-9.md

# Commit
git commit -m "docs(spec): verify Phase 9 - repository renamed to anvs"

# Push
git push origin main

# Verify push succeeded
git log --oneline -1
```

**Expected output**: Push succeeds without errors

**Actions**:
- [ ] `git fetch` works
- [ ] `git pull` works
- [ ] `git push` works
- [ ] No authentication issues
- [ ] No URL-related errors
- [ ] Commit appears on GitHub at new URL

---

### Task 9.9: Update Documentation References (If Any Remain)

**Check if any documentation still references the old repository URL**:

```bash
# Search for old repository references
grep -r "github.com/olvrcc/xvn" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude="*.lock"
```

**If any references found**:
- Most should have been updated in Phase 1-8
- If any remain (especially in spec/milestone-12-renaming-to-anvs/), update them
- Historical docs in other spec/ directories should remain as-is

**Expected result**:
- Should find NO references in active documentation
- May find references in historical spec docs (milestone-1 through milestone-11) - leave those as-is

**Actions**:
- [ ] Search for old URL references
- [ ] Update any found in active documentation
- [ ] Leave historical references intact
- [ ] Commit any changes: `docs: update remaining repository URL references`

---

### Task 9.10: Notify Collaborators (If Any)

**If you have collaborators or contributors**:

1. **Notify them of the rename**:
   - Create a GitHub Discussion or Issue
   - Explain the repository has been renamed
   - Provide instructions to update their local clones
   - Link to Phase 9 migration instructions

2. **Example notification**:
   ```markdown
   # Repository Renamed: xvn → anvs

   The repository has been renamed from `xvn` to `anvs` as part of our v2.0 release.

   ## For Contributors

   Update your local clone's remote URL:

   ```bash
   git remote set-url origin https://github.com/olvrcc/anvs.git
   # or for SSH:
   git remote set-url origin git@github.com:olvrcc/anvs.git
   ```

   ## What Changed

   - Repository: `olvrcc/xvn` → `olvrcc/anvs`
   - Package: `@olvrcc/xvn` → `anvs`
   - Binary: `xvn` → `anvs`

   ## Redirects

   GitHub automatically redirects the old URL, so existing clones will continue to work, but we recommend updating to the new URL.

   See the [migration guide](docs/XVN_TO_ANVS_MIGRATION.md) for full details.
   ```

**Actions**:
- [ ] Check if there are active collaborators
- [ ] If yes, create notification issue or discussion
- [ ] If no, skip this task
- [ ] Pin notification if created

---

## Verification Checklist

Before proceeding to Phase 10, verify ALL of the following:

- [ ] Repository accessible at: `https://github.com/olvrcc/anvs`
- [ ] Old URL redirects: `https://github.com/olvrcc/xvn` → `https://github.com/olvrcc/anvs`
- [ ] Local git remote updated to new URL
- [ ] `git fetch`, `git pull`, `git push` all work
- [ ] Repository description updated
- [ ] Repository topics/tags updated
- [ ] All issues, PRs, stars preserved
- [ ] Default branch still `main`
- [ ] No broken links in documentation
- [ ] Collaborators notified (if applicable)

---

## Success Criteria

Phase 9 is complete when:

1. ✅ GitHub repository renamed from `xvn` to `anvs`
2. ✅ Old URL automatically redirects to new URL
3. ✅ Local git remote updated and tested
4. ✅ Repository metadata updated (description, topics)
5. ✅ All git operations working correctly
6. ✅ All issues, PRs, stars, and history preserved
7. ✅ Collaborators notified (if any)

---

## Next Steps

After completing Phase 9:

1. **Proceed to Phase 10**: Homebrew Tap Changes
   - Rename the `homebrew-xvn` repository to `homebrew-anvs`
   - Update the Homebrew formula from `xvn.rb` to `anvs.rb`
   - Update formula contents with new URLs and binary names

2. **Note**: Phase 10 is independent of Phase 9 and can be done immediately

3. **After Phase 10**: Continue with Phase 11 (Migration Guide) and Phase 12 (Build, Test, and Publish)

---

## Rollback Plan

If issues are discovered after renaming:

1. **Rename back to `xvn`** (GitHub allows this if done quickly):
   - Go to repository settings
   - Rename back to `xvn`
   - Update local remote: `git remote set-url origin https://github.com/olvrcc/xvn.git`

2. **Fix the issue** in the code before attempting rename again

3. **Re-attempt rename** once issues resolved

**Note**: Renaming back should only be done if critical issues are discovered. Once npm package is published, renaming back becomes much more difficult.

---

## Notes

- Repository renaming is **non-destructive** - all data is preserved
- GitHub's automatic redirects make this a low-risk operation
- Existing clones will continue to work even without updating remote URL (due to redirects)
- However, it's best practice to update local remotes to the canonical URL
- This change must be done before publishing the `anvs` package to npm
- The repository rename does not affect local files - only the remote URL
- If you have GitHub Pages enabled, update the URL in settings after rename

---

## Common Issues and Solutions

### Issue: "Repository name already exists"
**Solution**:
- Check if `anvs` repository already exists in your account
- If it's an old repository, delete or rename it first
- If it's in use, choose a different name or transfer ownership

### Issue: Git push fails after rename
**Solution**:
```bash
# Verify remote URL
git remote -v

# Update to new URL
git remote set-url origin https://github.com/olvrcc/anvs.git

# Try push again
git push origin main
```

### Issue: Old URL doesn't redirect
**Solution**:
- Wait a few minutes - redirects may take time to propagate
- Clear browser cache
- Check GitHub status page for issues
- Verify rename completed successfully in repository settings

### Issue: Lose access to repository after rename
**Solution**:
- Check repository visibility settings (should remain public)
- Verify you're logged into correct GitHub account
- Check organization permissions if applicable
- Contact GitHub support if access truly lost

---

## Timeline

**Estimated time for Phase 9**: 20-30 minutes

**Breakdown**:
- Task 9.1 (Prepare): 3-5 minutes
- Task 9.2 (Rename on GitHub): 2-3 minutes
- Task 9.3 (Update local remote): 2-3 minutes
- Task 9.4 (Update description): 3-5 minutes
- Task 9.5 (Update homepage): 1-2 minutes
- Task 9.6 (Social preview - optional): 5-10 minutes or skip
- Task 9.7 (Verify settings): 2-3 minutes
- Task 9.8 (Test git operations): 3-5 minutes
- Task 9.9 (Check documentation): 2-3 minutes
- Task 9.10 (Notify collaborators): 5-10 minutes or skip

**Total**: 20-30 minutes (or up to 45 minutes with optional tasks)
