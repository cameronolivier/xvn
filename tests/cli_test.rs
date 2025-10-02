use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xvn 1.1.0"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "automatically switches your Node.js version",
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

    // Note: In Milestone 4, activate prompts to install missing versions
    // The command succeeds (exit 0) and shows installing message
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("activate")
        .arg(temp_dir.path())
        .assert()
        .success() // Command succeeds and shows install message
        .stdout(predicate::str::contains("18.20.0"))
        .stdout(predicate::str::contains("Installing"));
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
