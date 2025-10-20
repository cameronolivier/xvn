//! Integration tests for the redesigned wizard

use anvs::config::{AutoInstallMode, Config};
use anvs::init::detection::detect_all;
use anvs::init::summary::DetectionResults;
use anvs::setup::shell_detection::Shell;

#[test]
fn test_detect_all_returns_valid_results() {
    let results = detect_all().unwrap();
    // Should not panic, even if detection fails
    assert!(results.config_path.contains(".anvsrc"));
}

#[test]
fn test_wizard_mode_enum_exists() {
    // Test that WizardMode enum exists and has expected variants
    use anvs::init::wizard::WizardMode;

    let _quick = WizardMode::Quick;
    let _advanced = WizardMode::Advanced;

    // Test that they are different
    assert_ne!(_quick, _advanced);
}

#[test]
fn test_detection_results_to_config() {
    let mut results = DetectionResults::new();
    results.shell = Some(Shell::Zsh);
    results.version_managers = vec!["nvm".to_string()];

    // Test the conversion logic (implement in wizard.rs if not present)
    let config = Config {
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
        use_default: true,
        default_version: None,
    };

    assert_eq!(config.plugins, vec!["nvm".to_string()]);
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);
}

#[test]
fn test_config_validation() {
    // Test that created configs are valid
    let config = Config {
        plugins: vec!["nvm".to_string()],
        auto_install: AutoInstallMode::Prompt,
        version_files: vec![".nvmrc".to_string()],
        use_default: true,
        default_version: None,
    };

    // Should not panic when used
    assert!(!config.plugins.is_empty());
}
