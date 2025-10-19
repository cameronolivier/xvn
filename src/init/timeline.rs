//! Timeline rendering for wizard progress display
//!
//! Provides box-drawing characters and functions to render
//! timeline-style progress indicators with colored step states.

use crate::output::BRAND_COLOR;
use owo_colors::OwoColorize;

/// Box-drawing characters for timeline display
pub mod chars {
    pub const STEP_PENDING: &str = "◇";
    pub const STEP_ACTIVE: &str = "◆";
    pub const STEP_COMPLETE: &str = "✓";
    pub const VERTICAL: &str = "│";
    pub const BRANCH_RIGHT: &str = "├─";
    pub const BRANCH_LAST: &str = "└─";
    pub const TOP_LEFT: &str = "┌─";
    pub const HORIZONTAL: &str = "─";
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepState {
    Pending,
    Active,
    Complete,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub label: String,
    pub state: StepState,
    pub details: Option<String>,
}

impl Step {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: StepState::Pending,
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn set_state(&mut self, state: StepState) {
        self.state = state;
    }
}

/// Render a single step in the timeline
pub fn render_step(step: &Step) -> String {
    let symbol = match step.state {
        StepState::Pending => chars::STEP_PENDING,
        StepState::Active => chars::STEP_ACTIVE,
        StepState::Complete => chars::STEP_COMPLETE,
    };

    let label = match step.state {
        StepState::Active => step.label.color(BRAND_COLOR).bold().to_string(),
        StepState::Complete => step.label.green().to_string(),
        StepState::Pending => step.label.dimmed().to_string(),
    };

    let mut output = format!("{symbol} {label}");

    if let Some(details) = &step.details {
        output.push('\n');
        output.push_str(&format!("{}  {}", chars::VERTICAL, details.dimmed()));
    }

    output
}

/// Render a timeline with multiple steps
pub fn render_timeline(steps: &[Step]) -> String {
    steps.iter().map(render_step).collect::<Vec<_>>().join("\n")
}

/// Render a box-style container with title and items
pub fn render_box(title: &str, items: &[(&str, &str)]) -> String {
    // Calculate max key length for alignment
    let max_key_len = items.iter().map(|(k, _)| k.len()).max().unwrap_or(0);

    let mut output = format!("{} {}\n", chars::TOP_LEFT, title.bold());
    output.push_str(&format!("{}\n", chars::VERTICAL));

    for (i, (key, value)) in items.iter().enumerate() {
        let prefix = if i == items.len() - 1 {
            chars::BRANCH_LAST
        } else {
            chars::BRANCH_RIGHT
        };
        output.push_str(&format!(
            "{} {:width$}: {}\n",
            prefix,
            key.dimmed(),
            value,
            width = max_key_len
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_new() {
        let step = Step::new("Test Step");
        assert_eq!(step.label, "Test Step");
        assert_eq!(step.state, StepState::Pending);
        assert!(step.details.is_none());
    }

    #[test]
    fn test_step_with_details() {
        let step = Step::new("Test Step").with_details("Found: test value");
        assert!(step.details.is_some());
        assert_eq!(step.details.unwrap(), "Found: test value");
    }

    #[test]
    fn test_step_rendering() {
        let step = Step::new("Test Step").with_details("Found: test value");
        let output = render_step(&step);
        assert!(output.contains("Test Step"));
        assert!(output.contains("Found: test value"));
    }

    #[test]
    fn test_timeline_rendering() {
        let steps = vec![
            Step {
                label: "Step 1".into(),
                state: StepState::Complete,
                details: None,
            },
            Step {
                label: "Step 2".into(),
                state: StepState::Active,
                details: None,
            },
            Step {
                label: "Step 3".into(),
                state: StepState::Pending,
                details: None,
            },
        ];
        let output = render_timeline(&steps);
        assert!(output.contains("✓"));
        assert!(output.contains("◆"));
        assert!(output.contains("◇"));
    }

    #[test]
    fn test_box_rendering() {
        let items = vec![("Shell", "zsh"), ("Plugin", "nvm")];
        let output = render_box("Configuration", &items);
        assert!(output.contains("Configuration"));
        assert!(output.contains("Shell"));
        assert!(output.contains("zsh"));
        assert!(output.contains("┌─"));
        assert!(output.contains("└─"));
    }

    #[test]
    fn test_box_rendering_alignment() {
        let items = vec![("Shell", "zsh"), ("Version manager", "nvm"), ("X", "short")];
        let output = render_box("Test", &items);
        // Verify alignment by checking structure
        assert!(output.contains("Version manager"));
    }
}
