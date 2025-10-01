use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xvn 0.1.0"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Automatic Node.js version switching"));
}

#[test]
fn test_setup_command() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("setup")
        .assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_activate_command() {
    let mut cmd = Command::cargo_bin("xvn").unwrap();
    cmd.arg("activate")
        .arg(".")
        .assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
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
