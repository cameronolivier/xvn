---
description: Implement a specific phase following phase-P.md
args:
  - name: M:P
    description: 'Milestone:Phase (e.g., "12:2" for milestone 12, phase 2)'
    required: true
tags: [project, implementation]
---

You are tasked with implementing **Phase {{P}}** of **Milestone {{M}}** by strictly following the phase implementation plan.

## Instructions

1. **Parse the milestone and phase:**
   - Extract milestone number (M) and phase number (P) from "{{M:P}}"
   - Determine the milestone directory (e.g., for M=12: `spec/milestone-12-renaming-to-anvs/`)

2. **Read the phase documentation:**
   - Read `spec/milestone-<M>-<name>/phase-{{P}}.md` - The detailed phase implementation plan
   - This document contains:
     - Phase overview and goals
     - Detailed implementation tasks (Task P.1, P.2, P.3, etc.)
     - Action checklists for each task
     - Verification checklist
     - Success criteria
     - Exact commands to run
     - Expected outputs

3. **Create a todo list:**
   - Use TodoWrite to create a todo list from the phase document's action checklists
   - Track your progress through each task's action items
   - The phase document provides granular checkboxes for tracking

4. **For each task in the phase (Task P.1, P.2, etc.):**

   **a. Review the task requirements:**
   - Read the entire task section in phase-{{P}}.md
   - Understand the file changes required
   - Review the commands to run
   - Note any warnings or important notes

   **b. Write tests FIRST (Test-Driven Development):**
   - **EXCEPTION:** Only write tests if:
     - A testing framework already exists in the codebase
     - The task involves code changes (not just documentation/config)
   - If applicable, write comprehensive tests BEFORE implementation:
     - Write tests covering the task specification
     - Write tests for edge cases mentioned in the phase plan
     - Write tests for error conditions
     - Expect tests to FAIL initially (red phase)

   **c. Implement the task:**
   - Follow the phase plan step-by-step exactly as written
   - Use the code examples and commands provided
   - Make the exact file changes specified (with line numbers if provided)
   - Run the verification commands listed in the task
   - Do NOT deviate from the plan unless you find a blocking issue
   - If you encounter uncertainties or blockers, STOP and ask the user for guidance
   - Implement until all tests for this task pass (green phase)

   **d. Verify the task:**
   - Run the verification commands listed in the task section
   - Check that output matches the expected output
   - Verify action items are complete
   - Test syntax/compilation if applicable

   **e. Review the implementation:**
   - Use the Task tool with subagent_type "general-purpose" to spawn a review agent for significant code changes
   - The review agent should:
     - Compare implementation against phase plan requirements
     - Verify tests are comprehensive (if applicable)
     - Check code quality and style
     - Identify any gaps or issues
   - If issues found, fix them before proceeding
   - For minor changes (documentation, simple renames), you may skip the review agent

   **f. Mark task complete:**
   - Check off the action items in your TodoWrite list
   - Commit the work with a conventional commit message
   - List all files changed in the commit message (as per CLAUDE.md conventions)
   - Move on to the next task

5. **After all tasks in the phase are complete:**
   - Run through the **Verification Checklist** from the phase document
   - Verify every item in the checklist passes
   - Run all tests if code changes were made: `cargo test`
   - Ensure all **Success Criteria** from the phase plan are met
   - If the phase involves multiple related files, test integration
   - Use the Task tool to spawn a final review agent that:
     - Reviews the entire phase against the phase plan
     - Verifies all action items are checked off
     - Ensures all verification items pass
     - Checks that success criteria are met
   - If any issues found, fix them
   - Create a final commit for the phase completion

6. **Version bumping (optional for phases):**
   - Phases are typically part of a larger milestone/release
   - Do NOT bump version for individual phases unless explicitly instructed
   - Version bumping happens at milestone or major phase completion (e.g., Phase 12 completion)
   - If this is a final phase (e.g., Phase 12) or user requests it:
     - Use `./scripts/bump-version.sh <major|minor|patch>`
     - Follow the guidance in the phase plan's "Next Steps" section

## Important Rules

- **Follow the phase plan strictly** - Do not improvise or take shortcuts
- **Test-Driven Development (TDD)** - Write tests first (if applicable), then implement
- **One task at a time** - Complete each task fully before moving to the next
- **Verify as you go** - Run verification commands after each task
- **Commit after each task** - Keep commits small and focused with file lists
- **Ask when uncertain** - Don't guess or deviate from the plan
- **All tests must pass** - This is the definition of "done"
- **Use exact commands** - The phase plan provides exact commands; use them
- **Check expected outputs** - Verify your results match expected outputs in the plan

## Success Criteria

Phase {{P}} is complete when:

- ✅ All tasks (P.1, P.2, P.3, etc.) are implemented exactly as specified
- ✅ All action checklists from the phase plan are checked off
- ✅ All verification checklist items pass
- ✅ All tests pass (if code changes were made)
- ✅ All success criteria from the phase plan are met
- ✅ Code follows the plan exactly
- ✅ Review agents approve the work (for significant changes)
- ✅ Changes are committed with descriptive messages listing files changed
- ✅ No breaking changes introduced (unless documented in phase plan)

## Phase-Specific Notes

- Phase plans are extremely detailed with exact commands and file paths
- Each task has granular action checklists - use them
- Verification commands are provided - run them after each task
- Expected outputs are shown - compare your results
- Phase plans include rollback procedures if issues arise
- Time estimates help you pace the work
- Next steps guide you to the following phase

## Example Workflow

For Phase 2 of Milestone 12 (Installation & Binary Files):

1. Read `spec/milestone-12-renaming-to-anvs/phase-2.md`
2. Create TodoWrite list from all action items
3. Implement Task 2.1 (Update install.js):
   - Review task requirements
   - Make exact changes specified
   - Run verification: `cat install.js | grep -i "xvn"` (should be empty)
   - Check off action items
   - Commit: `feat(install): update install.js for anvs rename`
4. Implement Task 2.2 (Update uninstall.js):
   - Same process...
5. Continue through Tasks 2.3-2.8
6. Run Verification Checklist (14 items)
7. Verify Success Criteria (7 items)
8. Final review and commit
9. Proceed to Phase 3 (as indicated in "Next Steps")
