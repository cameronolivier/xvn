---
description: Generate detailed implementation plan for phase P of milestone M
args:
  - name: M:P
    description: 'Milestone:Phase (e.g., "12:2" for milestone 12, phase 2)'
    required: true
---

You are tasked with creating a comprehensive, step-by-step implementation plan for **Phase {{P}}** of **Milestone {{M}}** (the xvn → anvs rename).

## Instructions

1. **Parse the milestone and phase:**
   - Extract milestone number (M) and phase number (P) from "{{M:P}}"
   - For milestone 12, the milestone directory is `spec/milestone-12-renaming-to-anvs/`

2. **Read the context:**
   - Read `spec/milestone-12-renaming-to-anvs/RENAME_PLAN.md` to understand the overall rename plan
   - Read `spec/milestone-12-renaming-to-anvs/RENAME_PLAN_TASKS.md` to see the task checklist
   - Look at `spec/milestone-12-renaming-to-anvs/phase-0.md` as a reference for the format and level of detail
   - Identify which section of RENAME_PLAN.md corresponds to Phase {{P}}

3. **Create the phase implementation plan:**
   - Create a new file: `spec/milestone-12-renaming-to-anvs/phase-{{P}}.md`
   - Structure the plan following the phase-0.md format
   - Include:
     - Phase status and metadata (version, duration estimate)
     - Overview explaining the phase goals
     - Detailed implementation tasks with specific file changes
     - Exact commands to run
     - Action checklists for each task
     - Verification checklist
     - Success criteria
     - Next steps

4. **Plan structure:**
   ```markdown
   # Phase {{P}}: [Phase Title from RENAME_PLAN.md]

   **Status**: Not Started
   **Version**: [Target version, e.g., v2.0.0]
   **Duration Estimate**: [Time estimate]

   ## Overview

   [Detailed explanation of what this phase accomplishes and why it's important]

   **Why Phase {{P}} is [Important/Essential/Critical]:**
   - [Reason 1]
   - [Reason 2]
   - [Reason 3]

   **⚠️ CHECKPOINT** (if applicable): [Any important notes before starting]

   ---

   ## Implementation Tasks

   ### Task {{P}}.1: [Task Title]

   **File**: `path/to/file` (new/existing file)

   **Content Requirements** (for new files):
   ```markdown
   [Example content or structure]
   ```

   **Changes Required** (for existing files):
   - Line X: Change `old value` to `new value`
   - Section Y: Update Z

   **Commands** (if applicable):
   ```bash
   # Command 1
   command to run

   # Command 2
   another command
   ```

   **Expected Output** (if applicable):
   ```
   Expected command output
   ```

   **Actions**:
   - [ ] Specific action 1
   - [ ] Specific action 2
   - [ ] Specific action 3

   ---

   ### Task {{P}}.2: [Next Task Title]

   [Repeat structure for each task in this phase]

   ---

   ## Verification Checklist

   Before proceeding to Phase {{P+1}}, verify ALL of the following:

   - [ ] Verification item 1
   - [ ] Verification item 2
   - [ ] Verification item 3
   - [ ] No breaking changes introduced (or documented if intentional)

   ---

   ## Success Criteria

   Phase {{P}} is complete when:

   1. ✅ Criterion 1
   2. ✅ Criterion 2
   3. ✅ Criterion 3

   ---

   ## Next Steps

   After completing Phase {{P}}:

   1. [Next action]
   2. [Next action]
   3. **Proceed to Phase {{P+1}}**: [Brief description of next phase]

   ---

   ## Rollback Plan

   If issues are discovered:

   1. [Rollback step 1]
   2. [Rollback step 2]

   ---

   ## Notes

   - [Important note 1]
   - [Important note 2]
   ```

5. **Be extremely detailed:**
   - For each file that needs to be modified, specify:
     - The exact file path
     - What needs to change (with before/after examples)
     - Line numbers or sections to modify
   - For each command to run:
     - Provide the exact command
     - Explain what it does
     - Show expected output
   - For verification:
     - Provide specific commands to verify changes
     - Include expected results

6. **Follow the phase-0.md style:**
   - Use the same level of detail and structure
   - Include action checklists with `[ ]` checkboxes
   - Provide context and explanations
   - Add warnings and important notes
   - Include time estimates

7. **Quality assurance:**
   - After creating the phase plan, review it to ensure:
     - All tasks from RENAME_PLAN.md for this phase are covered
     - Task numbers match the phase (e.g., Phase 2 tasks are 2.1, 2.2, etc.)
     - Commands are accurate and tested
     - File paths are correct
     - The plan is actionable and complete

## Important Notes

- Phase plans should be as detailed as phase-0.md - developers should be able to execute the phase using ONLY this document
- Include actual commands, file paths, and code examples
- Make checklists granular enough to track progress
- Consider edge cases and potential issues
- Provide clear success criteria so developers know when they're done
- Link to related documentation where helpful
- For milestone 12 specifically, focus on the xvn → anvs rename changes
