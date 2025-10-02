---
description: Milestone number (1-6) to implement following PLAN.md
tags: [project, implementation]
---

You are tasked with implementing Milestone {{N}} by strictly following the implementation plan.

## Instructions

1. **Read the milestone documentation:**
   - Read `spec/milestone-{{N}}/PLAN.md` - The detailed implementation plan
   - Read `spec/milestone-{{N}}/TASKS.md` - The task checklist
   - Read `spec/milestone-{{N}}/SPEC.md` - The specification (for context)

2. **Create a todo list:**
   - Use TodoWrite to create a todo list with all tasks from TASKS.md
   - Track your progress through the milestone

3. **For each task in order:**

   **a. Write tests FIRST (Test-Driven Development):**
   - **EXCEPTION:** Only write tests if a testing framework already exists in the codebase
   - If this is Milestone 1 or the testing framework isn't set up yet, skip test writing until the test infrastructure is created
   - Once test infrastructure exists (after M1.7), write comprehensive tests BEFORE implementation:
     - Write tests covering the full specification
     - Write tests for edge cases
     - Write tests for error conditions
     - Expect tests to FAIL initially (red phase)

   **b. Implement the task:**
   - Follow PLAN.md step-by-step exactly as written
   - Use the code examples provided in PLAN.md
   - Do NOT deviate from the plan unless you find a blocking issue
   - If you encounter uncertainties or blockers, STOP and ask the user for guidance
   - Implement until all tests for this task pass (green phase)

   **c. Review the implementation:**
   - Use the Task tool with subagent_type "general-purpose" to spawn a review agent
   - The review agent should:
     - Compare implementation against PLAN.md requirements
     - Verify tests are comprehensive
     - Check code quality and style
     - Identify any gaps or issues
   - If issues found, fix them before proceeding

   **d. Mark task complete:**
   - Update `spec/milestone-{{N}}/TASKS.md` - check off the completed task
   - Commit the work with a conventional commit message
   - Update your TodoWrite list to mark the task completed
   - Move on to the next task

4. **After all tasks are complete:**
   - Run ALL tests to ensure everything passes
   - Use the Task tool to spawn a final review agent that:
     - Reviews the entire milestone against PLAN.md
     - Verifies all tasks in TASKS.md are checked off
     - Ensures all tests pass
     - Checks that success criteria from PLAN.md are met
   - If any issues found, fix them
   - Update `spec/PROGRESS.md` to mark the milestone complete
   - Create a final commit for the milestone completion
   - **Bump the version** using the appropriate method:
     - For milestone completions: `./scripts/version.sh minor` (bumps 0.X.0)
     - For bug fixes within a milestone: `./scripts/version.sh patch` (bumps 0.X.Y)
     - For major releases: `./scripts/version.sh major` or `./scripts/version.sh 1.0.0`
     - This will update Cargo.toml, package.json, tests, and create a commit and git tag automatically

## Important Rules

- **Follow PLAN.md strictly** - Do not improvise or take shortcuts
- **Test-Driven Development (TDD)** - Write tests first (after M1.7), then implement
- **One task at a time** - Complete each task fully before moving to the next
- **Commit after each task** - Keep commits small and focused
- **Ask when uncertain** - Don't guess or deviate from the plan
- **All tests must pass** - This is the definition of "done"
- **Exception for early milestones** - Skip test-first approach if testing framework doesn't exist yet

## Success Criteria

- All tasks in TASKS.md are checked off ✅
- All tests pass (cargo test)
- Code follows the plan exactly
- Review agents approve the work
- PROGRESS.md updated to mark milestone complete
- Final commit marks milestone as complete
- Version bumped appropriately with git tag created
  - Milestone completion: minor version bump (e.g., 0.6.0 -> 0.7.0)
  - Bug fixes: patch version bump (e.g., 0.6.0 -> 0.6.1)
  - Major release: major version bump (e.g., 0.9.0 -> 1.0.0)
