use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xvn 0.2.0"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Automatic Node.js version switching",
        ));
}

#[test]
fn test_setup_command() {
    // Note: This test can't actually run setup as it would modify the real shell
    // Instead, we just verify that the command exists and has proper help text
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("setup")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("shell"));
}

#[test]
fn test_activate_command() {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let version_file = temp_dir.path().join(".nvmrc");
    fs::write(&version_file, "18.20.0").unwrap();

    // Note: In Milestone 3, activate now uses FD:3 protocol and logs to stderr via info!
    // Since the version is not installed, it will exit with error code 1
    // We test that it finds the version file and provides install instructions
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("activate")
        .arg(temp_dir.path())
        .assert()
        .failure() // Changed from success() since version not installed
        .stderr(predicate::str::contains("Version 18.20.0 not installed"))
        .stderr(predicate::str::contains("nvm install 18.20.0"));
}

#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Plugins:"))
        .stdout(predicate::str::contains("nvm"));
}

#[test]
fn test_no_command_shows_help() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}
