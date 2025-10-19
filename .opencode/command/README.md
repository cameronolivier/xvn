# Opencode Command Templates

This directory contains command templates specifically adapted for use with opencode. These templates provide structured workflows for implementing milestones and phases in the ANVS project.

## Available Commands

### Milestone Commands

- **`do:milestone.md`** - Execute a complete milestone implementation
  - Follows PLAN.md specifications strictly
  - Includes test-driven development workflow
  - Handles version bumping and git tagging

- **`plan:milestone.md`** - Generate detailed milestone implementation plans
  - Creates comprehensive PLAN.md from SPEC.md and TASKS.md
  - Includes code examples and testing strategies

### Phase Commands

- **`do:phase.md`** - Execute a specific phase within a milestone
  - Granular task execution with verification
  - Supports rollback procedures
  - Includes action checklists and success criteria

- **`plan:phase.md`** - Generate detailed phase implementation plans
  - Creates actionable phase documents
  - Includes exact commands and expected outputs
  - Self-contained execution guides

- **`plan:phase-12.md`** - Milestone 12 specific phase planning
  - Specialized for the xvn → anvs rename project
  - Includes rename-specific workflows

## Opencode Integration

These commands are specifically adapted for opencode's tool ecosystem:

- **todowrite** - Task list management and progress tracking
- **task** - Complex multi-step operations with subagents
- **read/edit/write** - File manipulation with proper context handling
- **grep/glob** - Code search and exploration
- **bash** - Command execution with timeout and security measures

## Usage with Opencode

1. **Invoke commands** using opencode's slash command system
2. **Pass arguments** as specified in each command's `args` section
3. **Follow workflows** exactly as documented - these are strict implementation guides
4. **Use tools appropriately** - leverage opencode's full capability set

## Project-Specific Conventions

- **Testing**: Use `cargo test` and `make check` for validation
- **Linting**: Run `make check` before commits
- **Commits**: Follow conventional commit format from CLAUDE.md
- **Versioning**: Use `./scripts/version.sh` for version bumps
- **Documentation**: Update PROGRESS.md and task checklists

## Error Handling

All commands include error handling procedures:

- Stop on failures and seek guidance
- Document issues clearly
- Use rollback procedures when available
- Maintain system stability during failures

## Quality Assurance

- Commands include review agent integration
- All changes require testing validation
- Code quality checks are mandatory
- Documentation updates are required

## Related Documentation

- `CLAUDE.md` - Commit conventions and development guidelines
- `spec/` - Milestone specifications and plans
- `Makefile` - Available build and test commands
- `AGENTS.md` - Opencode agent capabilities