//! Summary and status display formatting for the wizard
//!
//! Provides functions to format detection results, configuration previews,
//! and completion messages using the timeline module.

use crate::config::{AutoInstallMode, Config};
use crate::init::timeline;
use crate::setup::shell_detection::Shell;
use owo_colors::OwoColorize;

/// Results from auto-detection of shell and version managers
#[derive(Debug, Clone)]
pub struct DetectionResults {
    pub shell: Option<Shell>,
    pub shell_path: Option<String>,
    pub version_managers: Vec<String>,
    pub config_path: String,
    pub auto_install: AutoInstallMode,
}

impl DetectionResults {
    pub fn new() -> Self {
        Self {
            shell: None,
            shell_path: None,
            version_managers: Vec::new(),
            config_path: "~/.anvsrc".to_string(),
            auto_install: AutoInstallMode::Prompt,
        }
    }
}

impl Default for DetectionResults {
    fn default() -> Self {
        Self::new()
    }
}

/// Format detection results as a box-style summary
pub fn format_detection_summary(results: &DetectionResults) -> String {
    let mut items = Vec::new();

    // Shell
    let shell_info;
    if let Some(shell) = &results.shell {
        shell_info = if let Some(path) = &results.shell_path {
            format!("{} ({})", shell.name(), path)
        } else {
            shell.name().to_string()
        };
        items.push(("Shell", shell_info.as_str()));
    } else {
        items.push(("Shell", "Not detected"));
    }

    // Version manager
    let vm_list;
    if !results.version_managers.is_empty() {
        vm_list = results.version_managers.join(", ");
        items.push(("Version manager", vm_list.as_str()));
    } else {
        items.push(("Version manager", "Not detected"));
    }

    // Config location
    items.push(("Config location", results.config_path.as_str()));

    // Auto-install mode
    let mode_str = match results.auto_install {
        AutoInstallMode::Always => "Always",
        AutoInstallMode::Prompt => "Prompt when needed",
        AutoInstallMode::Never => "Never",
    };
    items.push(("Auto-install", mode_str));

    timeline::render_box("Initializing anvs", &items)
}

/// Format a configuration preview before applying
pub fn format_config_preview(config: &Config, shell: &Shell) -> String {
    let vm_list = config.plugins.join(", ");
    let auto_install_str = format_auto_install(&config.auto_install);

    let items = vec![
        ("Shell", shell.name()),
        ("Version manager", vm_list.as_str()),
        ("Auto-install", auto_install_str.as_str()),
        ("Config", "~/.anvsrc"),
    ];

    timeline::render_box("Configuration Summary", &items)
}

/// Format auto-install mode as a string
fn format_auto_install(mode: &AutoInstallMode) -> String {
    match mode {
        AutoInstallMode::Always => "Always".to_string(),
        AutoInstallMode::Prompt => "Prompt".to_string(),
        AutoInstallMode::Never => "Never".to_string(),
    }
}

/// Format next steps message after successful setup
pub fn format_next_steps(shell: &Shell) -> String {
    let shell_rc = match shell {
        Shell::Zsh => "~/.zshrc",
        Shell::Bash => "~/.bashrc",
    };

    let mut output = String::new();
    output.push_str(&"Next steps:".bold().to_string());
    output.push('\n');
    output.push_str(&format!(
        "  1. Restart your shell or run: {}\n",
        format!("source {shell_rc}").cyan()
    ));
    output.push_str("  2. Navigate to a project with .nvmrc\n");
    output.push_str("  3. Watch anvs activate automatically!\n");
    output.push('\n');
    output.push_str(&format!(
        "Example: {}\n",
        "cd ~/my-project && anvs status".dimmed()
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_results_new() {
        let results = DetectionResults::new();
        assert!(results.shell.is_none());
        assert!(results.version_managers.is_empty());
        assert_eq!(results.config_path, "~/.anvsrc");
    }

    #[test]
    fn test_detection_summary_with_all_detected() {
        let mut results = DetectionResults::new();
        results.shell = Some(Shell::Zsh);
        results.shell_path = Some("/bin/zsh".to_string());
        results.version_managers = vec!["nvm".to_string()];

        let output = format_detection_summary(&results);
        assert!(output.contains("zsh"));
        assert!(output.contains("nvm"));
        assert!(output.contains("Initializing anvs"));
    }

    #[test]
    fn test_detection_summary_with_nothing_detected() {
        let results = DetectionResults::new();
        let output = format_detection_summary(&results);
        assert!(output.contains("Not detected"));
    }

    #[test]
    fn test_config_preview() {
        let config = Config {
            plugins: vec!["nvm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string()],
            use_default: true,
            default_version: None,
        };
        let shell = Shell::Zsh;

        let output = format_config_preview(&config, &shell);
        assert!(output.contains("zsh"));
        assert!(output.contains("nvm"));
        assert!(output.contains("Prompt"));
        assert!(output.contains("Configuration Summary"));
    }

    #[test]
    fn test_next_steps_zsh() {
        let output = format_next_steps(&Shell::Zsh);
        assert!(output.contains("Next steps:"));
        assert!(output.contains("source ~/.zshrc"));
        assert!(output.contains("Navigate to a project"));
    }

    #[test]
    fn test_next_steps_bash() {
        let output = format_next_steps(&Shell::Bash);
        assert!(output.contains("source ~/.bashrc"));
    }

    #[test]
    fn test_format_auto_install() {
        assert_eq!(format_auto_install(&AutoInstallMode::Always), "Always");
        assert_eq!(format_auto_install(&AutoInstallMode::Prompt), "Prompt");
        assert_eq!(format_auto_install(&AutoInstallMode::Never), "Never");
    }
}
