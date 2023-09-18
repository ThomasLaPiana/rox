use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn help_succeeds() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

/// This is a useful example for testing output
#[test]
#[ignore]
fn build_binary_output() {
    let expected = "tests/expected/help_output.txt";
    let output = fs::read_to_string(expected).unwrap();
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-s task build-release-binary")
        .assert()
        .success()
        .stdout(output);
}
