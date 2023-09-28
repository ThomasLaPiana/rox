use assert_cmd::Command;
use predicates::prelude::*;

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

#[test]
fn test_task_succeeds() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-f")
        .arg("tests/files/test_roxfile.yml")
        .arg("task")
        .arg("passing")
        .assert()
        .success();
}

#[test]
fn test_pipeline_succeeds_single_stage() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-f")
        .arg("tests/files/test_roxfile.yml")
        .arg("pl")
        .arg("passing_single")
        .assert()
        .success();
}

#[test]
fn test_pipeline_succeeds_multi_stage() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-f")
        .arg("tests/files/test_roxfile.yml")
        .arg("pl")
        .arg("passing_multi")
        .assert()
        .success();
}

#[test]
fn test_pipeline_succeeds_parallel() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-f")
        .arg("tests/files/test_roxfile.yml")
        .arg("pl")
        .arg("-p")
        .arg("passing_single")
        .assert()
        .success();
}
