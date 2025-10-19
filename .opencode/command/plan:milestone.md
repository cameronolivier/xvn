---
description: Generate detailed implementation plan for milestone N
args:
  - name: N
    description: Milestone number (1-6)
    required: true
---

You are tasked with creating a comprehensive, step-by-step implementation plan for milestone {{N}}.

## Instructions

1. **Read the milestone documentation:**
   - Read `spec/milestone-{{N}}/SPEC.md` to understand the architectural design and requirements
   - Read `spec/milestone-{{N}}/TASKS.md` to see the task checklist

2. **Create the implementation plan:**
   - Create a new file `spec/milestone-{{N}}/PLAN.md`
   - Structure the plan as a detailed, step-by-step implementation guide
   - For each task in TASKS.md, create a corresponding section in PLAN.md with:
     - Task name (matching exactly the task name from TASKS.md)
     - Detailed implementation steps
     - Code structure and file organization
     - Key considerations and design decisions
     - Testing requirements
     - Dependencies on other tasks
   - Include concrete code examples where helpful
   - Provide full context for each task so it can be implemented independently

3. **Plan structure:**
   ```markdown
   # Milestone {{N}}: [Title] - Implementation Plan

   ## Overview
   [Brief summary of milestone goals and approach]

   ## Prerequisites
   [Any setup or dependencies needed before starting]

   ## Implementation Tasks

   ### Task 1: [Task Name from TASKS.md]
   **Objective:** [What this task accomplishes]

   **Implementation Steps:**
   1. [Detailed step]
   2. [Detailed step]
   3. [Detailed step]

   **Code Structure:**
   - File: `path/to/file.rs`
     - [What goes in this file]
   - File: `path/to/another.rs`
     - [What goes in this file]

   **Key Considerations:**
   - [Important design decision]
   - [Edge case to handle]

   **Testing:**
   - [What tests to write]
   - [How to verify it works]

   **Dependencies:**
   - Requires: [Other tasks that must be done first]
   - Enables: [Tasks that can be done after this]

   [Repeat for each task...]

   ## Integration Points
   [How tasks work together]

   ## Testing Strategy
   [Overall testing approach for milestone]

   ## Success Criteria
   [What "done" looks like for this milestone]
   ```

4. **Quality assurance:**
   - After creating PLAN.md, use task with subagent_type "general" to launch a review agent
   - The review agent should:
     - Read all three files: SPEC.md, TASKS.md, and PLAN.md
     - Verify that every task in TASKS.md has a corresponding detailed section in PLAN.md
     - Check that task names match exactly between TASKS.md and PLAN.md
     - Ensure implementation steps are concrete and actionable
     - Identify any gaps, ambiguities, or missing details
     - Return a report with any issues found

5. **Fix any issues:**
   - If the review agent identifies problems, update PLAN.md to address them
   - Ensure the plan is complete, accurate, and ready for implementation

## Important Notes

- Be thorough - developers should be able to implement each task using only the PLAN.md
- Include actual code snippets and file structures where helpful
- Consider dependencies between tasks and call them out explicitly
- Make sure the plan flows logically from one task to the next
- Focus on practical implementation details, not just theory