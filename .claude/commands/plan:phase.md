---
description: Generate detailed implementation plan for phase P of milestone M
args:
  - name: M:P
    description: 'Milestone:Phase (e.g., "12:2" for milestone 12, phase 2)'
    required: true
---

Create a comprehensive, actionable implementation plan for **Phase {{P}}** of **Milestone {{M}}**.

## Process

1. **Find milestone**: `ls -d spec/milestone-{{M}}*/` to locate directory
2. **Read context**: Main plan file, task checklist, existing phase files for formatting consistency
3. **Extract scope**: Identify all tasks belonging to Phase {{P}}
4. **Create plan**: `spec/milestone-{{M}}*/phase-{{P}}.md` with complete implementation details

## Document Structure

```markdown
# Phase {{P}}: [Descriptive Title]

**Status**: Not Started | **Version**: vX.Y.Z | **Duration**: 30-45 min + CI time

## Overview
[2-3 sentences: what this phase accomplishes and its role in the milestone]

**Why Phase {{P}} is Critical:**
- [Impact/dependency reason]
- [Technical/user-facing reason]
- [Risk mitigation reason]

**⚠️ CHECKPOINT**: [Pre-requisites or blocking warnings]

---

## Implementation Tasks

### Task {{P}}.1: [Action-Oriented Title]

**File**: `exact/path/to/file.ext` (new file | existing file)

**Content Requirements** (for NEW files only):
```lang
[Complete, copy-pasteable file content with actual values]
```

**Changes Required** (for EXISTING files only):
- Line/Section X: Change `old_value` to `new_value`
- [Specific before/after examples]

**Commands**:
```bash
# Descriptive comment explaining purpose
exact-command --with-real-flags value

# Expected output:
[Actual expected output, not placeholder]
```

**Actions**:
- [ ] Granular, testable step
- [ ] Verification step with command
- [ ] Commit with message: `type: description`

---

[Repeat Task {{P}}.N for each task in phase]

---

## Verification Checklist

Before Phase {{P+1}}, verify:
- [ ] Specific check with command: `verification-cmd`
- [ ] Expected result: [concrete outcome]
- [ ] All builds/tests pass
- [ ] No breaking changes (or documented)

---

## Success Criteria

1. ✅ Measurable deliverable 1
2. ✅ Testable outcome 2
3. ✅ State change verified

---

## Next Steps

1. Update milestone tracker
2. [Any cleanup or documentation]
3. **Proceed to Phase {{P+1}}**: [One-line preview]

---

## Rollback Plan

1. Specific rollback command/steps
2. Verification of rollback success

---

## Notes

- Edge cases and gotchas
- Platform-specific considerations
```

## Quality Requirements

**CRITICAL - Each task must include:**
- Real file paths (never use `path/to/file` placeholders)
- Complete code examples (fully functional, not snippets)
- Runnable commands with actual flags and values
- Expected output examples (not just "output will show...")
- Granular action checkboxes (aim for 3-7 per task)
- Conventional commit message templates

**For file modifications:**
- Show before/after examples with context
- Reference specific line numbers or section headers
- Include full code blocks, not partial snippets

**For commands:**
- Always include explanatory comment above command
- Show complete expected output
- Include failure modes if relevant

**Duration estimates:**
- Per-task time estimates for complex tasks
- Include CI/build wait times separately
- Be realistic based on similar past work

**Self-contained execution:**
- Developer should execute phase using ONLY this document
- No external references required during execution
- All context and rationale included inline
