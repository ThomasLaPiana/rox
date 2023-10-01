use assert_cmd::Command;
use predicates::prelude::*;

/// Test helper to load the Rox command from bin and set the test file path
fn test_command() -> Command {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.arg("-f").arg("tests/files/test_roxfile.yml");
    cmd
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn help_succeeds() {
    test_command()
        .arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_task_succeeds() {
    test_command().arg("task").arg("passing").assert().success();
}

#[test]
fn test_pipeline_succeeds_single_stage() {
    test_command()
        .arg("pl")
        .arg("passing_single")
        .assert()
        .success();
}

#[test]
fn test_pipeline_succeeds_multi_stage() {
    test_command()
        .arg("pl")
        .arg("passing_multi")
        .assert()
        .success();
}

#[test]
fn test_pipeline_succeeds_parallel() {
    test_command()
        .arg("pl")
        .arg("-p")
        .arg("passing_single")
        .assert()
        .success();
}

#[test]
fn test_serial_processing_time() {
    let expected = "> Total elapsed time: 4s";
    test_command()
        .arg("pl")
        .arg("sleep_multi")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_parallel_processing_time() {
    let expected = "> Total elapsed time: 3s";
    test_command()
        .arg("pl")
        .arg("-p")
        .arg("sleep_multi")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_hidden_task() {
    let expected = "hidden";
    test_command()
        .arg("task")
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected).count(0));
}
