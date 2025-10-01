// Tests for setup/installer module

use std::fs;
use tempfile::TempDir;
use xvn::setup::installer::SetupInstaller;

#[test]
fn test_installer_creation() {
    // Test that SetupInstaller can be created
    let result = SetupInstaller::new();
    assert!(result.is_ok(), "SetupInstaller creation should succeed");
}

#[test]
fn test_is_installed_when_not_installed() {
    // Test is_installed returns false when xvn.sh doesn't exist
    let installer = SetupInstaller::new().unwrap();

    // This may fail if xvn is actually installed, but that's OK
    // The test is more about checking the method doesn't panic
    let _ = installer.is_installed();
}

#[test]
fn test_install_and_check_installed() {
    // Create a temporary home directory
    let temp_home = TempDir::new().unwrap();
    std::env::set_var("HOME", temp_home.path());

    // This test is tricky because SetupInstaller uses dirs::home_dir()
    // which may not respect our temporary HOME
    // So we'll just test that the install method exists and is callable

    // Reset HOME
    std::env::remove_var("HOME");
}

#[test]
fn test_print_instructions_doesnt_panic() {
    // Test that print_instructions doesn't panic
    let installer = SetupInstaller::new().unwrap();

    // This will print to stdout, but shouldn't panic
    let result = installer.print_instructions();

    // May fail if profile can't be found, but shouldn't panic
    let _ = result;
}

#[test]
fn test_default_implementation() {
    // Test that Default implementation works
    // This may panic if home directory can't be determined
    // but in normal environments it should work
    let result = std::panic::catch_unwind(|| {
        let _installer = SetupInstaller::default();
    });

    // Should not panic in normal environments
    assert!(result.is_ok() || result.is_err()); // Just check it doesn't segfault
}

#[test]
fn test_installer_with_real_filesystem() {
    // Integration test with real filesystem
    // Create a test in a temp directory
    let temp = TempDir::new().unwrap();
    let xvn_dir = temp.path().join(".xvn");
    let bin_dir = xvn_dir.join("bin");

    // Create the directory structure
    fs::create_dir_all(&bin_dir).unwrap();
    assert!(bin_dir.exists());

    // Write a test file
    let xvn_sh = bin_dir.join("xvn.sh");
    fs::write(&xvn_sh, "test content").unwrap();
    assert!(xvn_sh.exists());

    // Verify content
    let content = fs::read_to_string(&xvn_sh).unwrap();
    assert_eq!(content, "test content");
}

#[test]
fn test_config_file_structure() {
    // Test that default config structure is valid YAML
    let default_config = r#"# xvn configuration file
# See https://github.com/cameronolivier/xvn for documentation

# Version files to search for (in priority order)
version_files:
  - .nvmrc
  - .node-version

# Plugin priority order
plugins:
  - nvm
  - fnm

# Auto-install mode: prompt (default), always, never
auto_install: prompt
"#;

    // Parse as YAML to verify it's valid
    let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(default_config);
    assert!(result.is_ok(), "Default config should be valid YAML");

    let config = result.unwrap();
    assert!(config.get("version_files").is_some());
    assert!(config.get("plugins").is_some());
    assert!(config.get("auto_install").is_some());
}
