# Milestone 13: Wizard Redesign

**Status**: Planning
**Version Target**: v2.1.0
**Priority**: Medium
**Estimated Duration**: 4-6 hours

## Overview

Redesign the `anvs init` wizard to be faster, cleaner, and more visually appealing. Inspired by modern CLI tools like Vite and ShadCN, the new wizard should optimize for speed with smart defaults while maintaining clarity through excellent visual design.

**Current Problems:**
- Too verbose/wordy - lots of explanatory text
- Visual design needs polish - lacks the clean, modern feel of tools like Vite
- Not optimized for speed - takes too many steps

**Design Inspiration:**
- **Vite CLI**: Clean timeline-style progress indicators, clear visual hierarchy
- **ShadCN CLI**: Simple interface, minimal prompts, smart defaults
- **inquire crate examples**: Modern Rust CLI patterns

## Goals

1. **Speed First**: Get users set up in 30 seconds or less
2. **Visual Excellence**: Clean, modern interface with progress indicators
3. **Smart Defaults**: Auto-detect everything possible, minimal user input
4. **Inline Feedback**: Show detected values as part of prompts
5. **Progressive Disclosure**: Simple by default, advanced options available

## Design Principles

### 1. Timeline-Style Progress
Use visual indicators to show progress through setup:
```
◇  Shell detection
│  Found: zsh (/bin/zsh)
│
◆  Version manager detection
│  Found: nvm (v0.39.0)
│
◇  Configuration
   Auto-install: Prompt (recommended)
```

### 2. Minimal Prompts
**Target: 2-3 prompts maximum**
- Only ask questions when detection fails or user needs choice
- Use smart defaults for everything else
- Provide "Advanced setup" option for customization

### 3. Inline Detection Display
Show what was detected as part of the prompt:
```
◆  Which version manager? (detected: nvm)
   ● nvm (recommended)
   ○ fnm
   ○ Customize
```

### 4. Visual Hierarchy
- Use unicode box drawing characters (◇ ◆ │ ├ └)
- Color coding: lime green for brand, cyan for info, green for success
- Spacing and alignment for clarity
- Icons/symbols for status (✓ ● ◇ ◆)

## Proposed New Flow

### Quick Mode (Default: `anvs init` or `anvs init --quick`)

**Single screen with auto-detection summary + confirmation:**

```
⚡ Automatic Node Version Switcher:

┌─ Initializing anvs
│
├─ ✓ Shell: zsh (/bin/zsh)
├─ ✓ Version manager: nvm (v0.39.0)
├─ ✓ Config location: ~/.anvsrc
└─ ℹ Auto-install: Prompt when needed

? Proceed with this configuration? ›
  ● Yes, continue
  ○ Customize settings
  ○ Cancel
```

**If user selects "Yes":**
- Install immediately
- Show progress
- Done in 3-5 seconds

**If user selects "Customize":**
- Drop into advanced mode (see below)

### Advanced Mode (`anvs init --advanced` or via "Customize" choice)

**Step 1: Shell Selection**
```
◇  Step 1 of 3: Shell Configuration
│
◆  Which shell? (detected: zsh)
   ● zsh (recommended)
   ○ bash
   ○ Custom path
```

**Step 2: Version Manager**
```
◇  Step 2 of 3: Version Manager
│
◆  Which version manager?
   ● nvm (detected, recommended)
   ○ fnm
   ○ Multiple (advanced)
```

**Step 3: Auto-install Behavior**
```
◇  Step 3 of 3: Configuration
│
◆  Auto-install missing versions?
   ○ Always (automatic)
   ● Prompt (recommended)
   ○ Never (manual)
```

**Summary & Confirmation:**
```
┌─ Configuration Summary
│
├─ Shell: zsh
├─ Version manager: nvm
├─ Auto-install: Prompt
└─ Config: ~/.anvsrc

? Apply this configuration? › Yes / No
```

### Installation Progress

```
⚡ Automatic Node Version Switcher:

◇  Installing
├─ ✓ Creating config at ~/.anvsrc
├─ ✓ Installing shell hook to ~/.zshrc
├─ ✓ Validating installation
└─ ✓ Testing activation

✓ Setup complete!

Next steps:
  1. Restart your shell or run: source ~/.zshrc
  2. Navigate to a project with .nvmrc
  3. Watch anvs activate automatically!
```

## Technical Implementation

### Visual Components (using `inquire` crate)

**Current dependencies:**
```toml
[dependencies]
inquire = "0.7"
owo-colors = "4.0"
```

**New visual helpers to create:**

1. **Timeline module** (`src/init/timeline.rs`)
   - Functions for drawing progress indicators
   - Box drawing characters: ◇ ◆ │ ├ └ ┌ ─
   - Step states: pending, active, complete

2. **Summary display** (`src/init/summary.rs`)
   - Format detection results
   - Create configuration preview
   - Show next steps

3. **Compact prompts** (`src/init/prompts.rs` - refactor)
   - Simplify existing prompts
   - Add inline detection display
   - Remove verbose help text (move to `--help` if needed)

### Flow Control

**Update `src/init/wizard.rs`:**
```rust
pub enum WizardMode {
    Quick,      // Auto-detect + single confirmation
    Advanced,   // Full customization
}

pub fn run_wizard(mode: WizardMode) -> Result<Config> {
    match mode {
        WizardMode::Quick => run_quick_wizard(),
        WizardMode::Advanced => run_advanced_wizard(),
    }
}
```

**Quick wizard steps:**
1. Detect shell, version managers, existing config
2. Show summary with detected values
3. Single confirmation prompt
4. Install & configure
5. Show completion message

**Advanced wizard steps:**
1. Show detected values but allow customization
2. Step-by-step prompts (3 steps max)
3. Summary confirmation
4. Install & configure
5. Show completion message

### CLI Updates

**Add `--advanced` flag:**
```rust
#[command(Commands)]
Init {
    /// Skip wizard, use all defaults
    #[arg(long)]
    quick: bool,

    /// Advanced setup with full customization
    #[arg(long)]
    advanced: bool,

    // ... existing flags
}
```

**Default behavior:**
- `anvs init` → Quick mode (new default)
- `anvs init --quick` → Quick mode (explicit)
- `anvs init --advanced` → Advanced mode
- `anvs init --non-interactive` → Fully automated (no prompts)

## Visual Design Examples

### Box Drawing Characters
```
┌─ Title
│  Content
├─ Item 1
├─ Item 2
└─ Last item

◇ Step (pending)
◆ Step (active)
✓ Step (complete)
```

### Color Scheme
- **Brand**: Lime green (RGB: 50, 205, 50)
- **Info/prompts**: Cyan
- **Success**: Bright green
- **Warnings**: Yellow
- **Errors**: Red
- **Muted text**: Dimmed gray

### Spacing & Alignment
- Consistent indentation (2 or 3 spaces)
- Blank lines for section separation
- Aligned text and symbols
- Compact but not cramped

## Success Criteria

**Milestone 13 is complete when:**

1. ✅ Quick mode is the default, completes in < 30 seconds
2. ✅ Visual design uses timeline/progress indicators
3. ✅ Prompts are concise and well-formatted
4. ✅ Detection results shown inline with prompts
5. ✅ Advanced mode available via `--advanced` flag
6. ✅ Installation progress shows clear feedback
7. ✅ Completion message is helpful and concise
8. ✅ All tests pass
9. ✅ Documentation updated (README, --help)
10. ✅ Wizard feels as polished as Vite/ShadCN CLIs

## Phases

### Phase 1: Visual Components
- Create timeline module with box drawing
- Create summary display helpers
- Create compact prompt templates

### Phase 2: Quick Mode Implementation
- Implement auto-detection and summary
- Single confirmation prompt
- Progress indicators during installation

### Phase 3: Advanced Mode Refinement
- Simplify existing wizard prompts
- Add inline detection display
- Reduce to 3 steps maximum

### Phase 4: Polish & Testing
- Visual alignment and spacing
- Color consistency
- Test on different terminals
- Update documentation

## Non-Goals (Out of Scope)

- ❌ Changing functionality (only improving UX)
- ❌ Adding new configuration options
- ❌ Changing config file format
- ❌ Rewriting the entire init system (refactor only what's needed)

## Future Enhancements (Post-Milestone)

- Interactive config editing (`anvs config edit`)
- Animated progress spinners
- More advanced detection (project-specific defaults)
- Onboarding tips/hints system

## References

- Vite CLI: https://github.com/vitejs/vite
- ShadCN CLI: https://github.com/shadcn-ui/ui/tree/main/packages/cli
- inquire examples: https://github.com/mikaelmello/inquire/tree/main/inquire/examples
- Unicode box drawing: https://en.wikipedia.org/wiki/Box-drawing_character

## Notes

- Maintain backward compatibility with `--quick` and `--non-interactive` flags
- Keep `setup` command alias working
- Test on both light and dark terminal themes
- Consider terminal width for layout (80 chars safe minimum)
- Optimize for common case (quick mode) while supporting advanced users
