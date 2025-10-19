# Phase 6-3: Supplementary Documentation Updates

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 30-45 minutes
**Depends On**: Phase 6-2 (technical documentation) completed

## Overview

Phase 6-3 completes the documentation update work by addressing supplementary documentation files. These include the roadmap, terminal-specific guides, AI agent workflows, script documentation, and verification of historical specification files.

**Why Phase 6-3 is Important:**
- ROADMAP.md communicates future direction to users and contributors
- WARP.md provides terminal-specific setup for Warp users
- AGENTS.md documents AI agent workflows for development
- scripts/README.md guides developers on release scripts
- spec/ directory verification ensures historical docs remain unchanged

**⚠️ IMPORTANT**: This phase should begin after Phase 6-2 is complete (all docs/ files updated).

**📋 Documentation Scope:**
- 4 supplementary documentation files
- 1 verification task (spec/ directory)
- Final commit of all Phase 6 work

---

## Implementation Tasks

### Task 6.3.1: Update ROADMAP.md

**File**: `ROADMAP.md`

**Changes Required**:

1. **Update title**:
   ```markdown
   # Before:
   # XVN Roadmap

   # After:
   # ANVS Roadmap
   ```

2. **Update package name references**:
   ```markdown
   # Before:
   The `xvn` project aims to...

   # After:
   The `anvs` project aims to...
   ```

3. **Add v2.0.0 milestone at the top**:
   ```markdown
   ## v2.0.0 - Project Rename (Completed - 2025-10-19)

   ### Breaking Changes
   - [x] Rename project from xvn to anvs
   - [x] Update npm package to unnamespaced `anvs`
   - [x] Update Homebrew tap (olvrcc/anvs)
   - [x] Update all documentation
   - [x] Create comprehensive migration guide
   - [x] Deprecate old `@olvrcc/xvn` package

   ### Impact
   - **Package name**: `@olvrcc/xvn` → `anvs`
   - **Binary name**: `xvn` → `anvs`
   - **Install directory**: `~/.xvn/` → `~/.anvs/`
   - **Config files**: `~/.xvnrc` → `~/.anvsrc`

   See [Migration Guide](./docs/MIGRATION.md) for upgrade instructions.

   ---
   ```

4. **Update all future version references**:
   - Update command examples to use `anvs`
   - Update feature descriptions to reference `anvs`
   - Update any technical references

**Commands**:
```bash
# Review current state
grep -n "xvn" ROADMAP.md

# After changes
grep -i "xvn" ROADMAP.md  # Should only show v2.0.0 milestone context
```

**Actions**:
- [ ] Update document title to "ANVS Roadmap"
- [ ] Add v2.0.0 rename milestone at top
- [ ] Update all package name references
- [ ] Update all future feature descriptions
- [ ] Update all command examples
- [ ] Verify chronological accuracy
- [ ] Verify no incorrect "xvn" references in future plans

---

### Task 6.3.2: Update WARP.md

**File**: `WARP.md`

**This file contains Warp terminal-specific configuration and examples.**

**Changes Required**:

1. **Update command examples**:
   ```bash
   # Before:
   xvn setup
   xvn activate

   # After:
   anvs setup
   anvs activate
   ```

2. **Update binary and package references**:
   ```markdown
   # Before:
   Install xvn: npm install -g @olvrcc/xvn

   # After:
   Install anvs: npm install -g anvs
   ```

3. **Update file paths** if present:
   - `~/.xvn/` → `~/.anvs/`
   - Configuration file references

4. **Update any Warp-specific workflow examples**:
   - Update snippets that reference xvn
   - Update keyboard shortcuts or aliases if they mention xvn

**Commands**:
```bash
# Review current state
grep -n "xvn" WARP.md

# After changes
grep -i "xvn" WARP.md  # Should return no results
```

**Actions**:
- [ ] Update all command examples
- [ ] Update package installation instructions
- [ ] Update file paths
- [ ] Update workflow examples
- [ ] Update any Warp-specific snippets or aliases
- [ ] Verify no "xvn" references remain

---

### Task 6.3.3: Update AGENTS.md

**File**: `AGENTS.md`

**This file documents AI agent workflows.**

**Changes Required**:

1. **Update agent examples** that reference xvn:
   ```markdown
   # Before:
   Agent: "Installing xvn..."

   # After:
   Agent: "Installing anvs..."
   ```

2. **Update command examples**:
   - All `xvn` → `anvs` references

3. **Update package references**:
   - `@olvrcc/xvn` → `anvs`

4. **Update workflow descriptions**:
   - Update any agent task descriptions mentioning xvn

**Commands**:
```bash
# Review current state
grep -n "xvn" AGENTS.md

# After changes
grep -i "xvn" AGENTS.md  # Should return no results
```

**Actions**:
- [ ] Update all agent workflow examples
- [ ] Update command references in workflows
- [ ] Update package name
- [ ] Update task descriptions
- [ ] Verify no "xvn" references remain

---

### Task 6.3.4: Update scripts/README.md

**File**: `scripts/README.md`

**Changes Required**:

1. **Update script descriptions**:
   ```markdown
   # Before:
   Build and publish xvn to npm

   # After:
   Build and publish anvs to npm
   ```

2. **Update command examples**:
   ```bash
   # Before:
   ./scripts/download-artifacts.sh v1.6.2
   # Downloads: xvn-x86_64-apple-darwin.tar.gz

   # After:
   ./scripts/download-artifacts.sh v2.0.0
   # Downloads: anvs-x86_64-apple-darwin.tar.gz
   ```

3. **Update artifact name references**:
   - `xvn-*.tar.gz` → `anvs-*.tar.gz`

4. **Update binary name references**:
   - `native/*/xvn` → `native/*/anvs`

5. **Update version examples**:
   - Update example version numbers to v2.0.0+

**Commands**:
```bash
# Review current state
grep -n "xvn" scripts/README.md

# After changes
grep -i "xvn" scripts/README.md  # Should return no results
```

**Actions**:
- [ ] Update all script descriptions
- [ ] Update command examples
- [ ] Update artifact name patterns
- [ ] Update binary paths
- [ ] Update version examples
- [ ] Verify all examples accurate
- [ ] Verify no "xvn" references remain

---

### Task 6.3.5: Verify Spec Directory (No Changes Required)

**Directory**: `spec/`

**⚠️ IMPORTANT**: Do NOT update historical planning documents!

**Files to VERIFY are UNCHANGED**:
- `spec/milestone-1/` through `spec/milestone-11/` - Historical, must not be modified
- `spec/PROJECT_PLAN.md` - Historical project planning
- `spec/PROJECT_SPEC.md` - Historical specifications
- All `PLAN.md`, `SPEC.md`, `TASKS.md` in old milestones

**Files you MAY update** (if needed):
- `spec/BACKLOG.md` - Update only if it contains future references to xvn
- `spec/PROGRESS.md` - Update only if tracking current milestone progress

**Commands**:
```bash
# Verify historical docs are untouched
git status spec/milestone-{1..11}/

# Should show: nothing to commit (or only phase-6*.md additions)

# Review BACKLOG for current references
grep -n "xvn" spec/BACKLOG.md 2>/dev/null

# Review PROGRESS for current references
grep -n "xvn" spec/PROGRESS.md 2>/dev/null
```

**Actions**:
- [ ] Verify `git status spec/milestone-{1..11}/` shows no changes
- [ ] Review `spec/BACKLOG.md` for future xvn references
- [ ] Update BACKLOG only if future features mention xvn
- [ ] Review `spec/PROGRESS.md` for current tracking
- [ ] Update PROGRESS only if actively tracking with xvn refs
- [ ] Verify historical milestones (1-11) remain unchanged
- [ ] Verify milestone-12 planning documents unchanged (they document the rename)
- [ ] Confirm only phase-6*.md files are new in spec/

---

## Verification Checklist

Before marking Phase 6-3 (and entire Phase 6) complete, verify ALL of the following:

### Supplementary Files
- [ ] ROADMAP.md updated with v2.0.0 milestone
- [ ] ROADMAP.md title is "ANVS Roadmap"
- [ ] ROADMAP.md future plans use "anvs"
- [ ] WARP.md commands and examples updated
- [ ] AGENTS.md workflow examples updated
- [ ] scripts/README.md script documentation updated

### Spec Directory Verification
- [ ] Historical milestones 1-11 unchanged
- [ ] BACKLOG.md reviewed (updated if needed)
- [ ] PROGRESS.md reviewed (updated if needed)
- [ ] No unintended changes to historical docs

### Comprehensive Phase 6 Verification
- [ ] All main docs updated (Phase 6-1): README, CLAUDE, CHANGELOG, CONTRIBUTING
- [ ] All technical docs updated (Phase 6-2): MIGRATION, ARCHITECTURE, HOMEBREW_SETUP, TEST_REVIEW
- [ ] All supplementary docs updated (Phase 6-3): ROADMAP, WARP, AGENTS, scripts/README
- [ ] No "xvn" in README.md: `grep -i "xvn" README.md`
- [ ] No "xvn" in ROADMAP.md future sections: `grep -i "xvn" ROADMAP.md`
- [ ] No ".xvnrc" references: `grep -r "\.xvnrc" *.md docs/*.md`
- [ ] No ".xvn" directory references: `grep -r "\.xvn[^s]" *.md docs/*.md`
- [ ] All repository URLs updated: `grep -r "olvrcc/xvn" *.md docs/*.md`

### Quality Checks
- [ ] All markdown files render correctly
- [ ] All links are valid (no 404s)
- [ ] All code examples use correct syntax
- [ ] Migration guide tested (if possible)
- [ ] No broken internal references

---

## Success Criteria

Phase 6-3 (and entire Phase 6) is complete when:

1. ✅ ROADMAP.md updated with v2.0.0 milestone and future plans
2. ✅ WARP.md terminal-specific docs updated
3. ✅ AGENTS.md workflow docs updated
4. ✅ scripts/README.md script docs updated
5. ✅ Historical spec/ documents verified unchanged
6. ✅ All verification commands pass
7. ✅ All Phase 6 work committed with file lists
8. ✅ No incorrect "xvn" references remain in any documentation

---

## Next Steps

After completing Phase 6-3:

1. **Final verification**:
   ```bash
   # Run comprehensive grep to find any remaining "xvn" references
   grep -r "xvn" *.md docs/*.md scripts/*.md 2>/dev/null | grep -v "xvn→anvs" | grep -v "from xvn" | grep -v "milestone-12"
   ```

2. **Commit supplementary documentation changes**:
   ```bash
   git add ROADMAP.md WARP.md AGENTS.md scripts/README.md spec/BACKLOG.md spec/PROGRESS.md
   git commit -m "docs: update supplementary documentation for anvs rename (Phase 6-3)

   Files changed:
   - ROADMAP.md: Added v2.0.0 milestone, updated future plans
   - WARP.md: Updated terminal-specific examples
   - AGENTS.md: Updated AI workflow examples
   - scripts/README.md: Updated script documentation
   - spec/BACKLOG.md: Updated future references (if applicable)
   - spec/PROGRESS.md: Updated tracking references (if applicable)

   Phase 6 (Documentation Files) now complete."
   ```

3. **Mark Phase 6 complete**: Update phase-6.md status to "Completed"

4. **Proceed to Phase 7**: Build & Release Scripts (update scripts in `scripts/` directory)

---

## Rollback Plan

If issues are discovered:

1. **Identify specific issues**: What documentation is incorrect?
2. **Fix incrementally**: Update specific files rather than reverting all
3. **Test fixes**: Verify markdown renders correctly
4. **Re-commit**: Apply fixes in separate commit for clarity

---

## Notes

- Phase 6-3 completes all documentation updates for the anvs rename
- These are lower priority than main/technical docs but still important for completeness
- ROADMAP.md v2.0.0 milestone provides clear communication about the rename
- Historical spec/ documents (milestones 1-11) must remain unchanged - they're reference material
- After Phase 6-3, all user-facing and developer documentation will be fully updated
- Phase 7 will update the actual build/release scripts

**Estimated time breakdown**:
- ROADMAP.md: 10 minutes
- WARP.md: 5 minutes
- AGENTS.md: 5 minutes
- scripts/README.md: 10 minutes
- spec/ verification: 5 minutes
- Final verification and commit: 5 minutes
- **Total**: 30-45 minutes

---

## Phase 6 Summary

When Phase 6-3 is complete, the entire Phase 6 will have accomplished:

**Phase 6-1** (Main Documentation):
- ✅ README.md - Primary user documentation
- ✅ CLAUDE.md - AI assistant instructions
- ✅ CHANGELOG.md - Version history with v2.0.0 entry
- ✅ CONTRIBUTING.md - Contributor guidelines

**Phase 6-2** (Technical Documentation):
- ✅ docs/MIGRATION.md - Comprehensive xvn→anvs migration guide
- ✅ docs/ARCHITECTURE.md - Technical architecture documentation
- ✅ docs/HOMEBREW_SETUP.md - Homebrew tap setup guide
- ✅ docs/TEST_REVIEW.md - Testing guidelines

**Phase 6-3** (Supplementary Documentation):
- ✅ ROADMAP.md - Project roadmap with v2.0.0 milestone
- ✅ WARP.md - Warp terminal-specific guide
- ✅ AGENTS.md - AI agent workflow documentation
- ✅ scripts/README.md - Script documentation
- ✅ spec/ directory verification

**Total**: 13 documentation files updated + spec verification = Complete Phase 6

---
