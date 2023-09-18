use assert_cmd::Command;

#[test]
fn test_no_subcmd_fails() {
    let mut cmd = Command::cargo_bin("rox").unwrap();
    cmd.assert().code(2);
}
