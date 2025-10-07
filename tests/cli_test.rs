use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xvn 1.1.2"));
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

    // Note: This test may fail in CI environments without version managers installed.
    // The command will either:
    // - Succeed and prompt to install the version (if a version manager is available)
    // - Fail with "no version manager plugins available" (if no version managers installed)
    // We just verify the command runs and processes the version file correctly.
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    let output = cmd.arg("activate").arg(temp_dir.path()).output().unwrap();

    // The command should either succeed with install prompt, or fail with helpful error
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Either we get a successful install prompt, or a helpful "no plugins" error
    let has_valid_output = stdout.contains("18.20.0")
        || stdout.contains("Install a Node.js version manager")
        || stderr.contains("no version manager plugins available");

    assert!(
        has_valid_output,
        "Expected version reference or plugin error, got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
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
