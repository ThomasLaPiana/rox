use crate::syntax::Target;
use crate::utils;
use std::process::Command;

pub fn run_target(target: &Target) -> i32 {
    let start = std::time::Instant::now();
    let (command, args) = utils::split_head_from_rest(target.command.clone());
    let command_results = Command::new(command).args(args).status();
    let exit_status = command_results.unwrap().code().unwrap();
    println!("{}", exit_status);

    println!("> Target elapsed time: {}ms", start.elapsed().as_millis());
    exit_status
}

#[test]
#[should_panic]
fn test_invalid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: "notacommand lol".to_string(),
        description: Some("description".to_string()),
    };
    let exit = run_target(&test_target);
    assert_eq!(exit, 1);
}

#[test]
fn test_valid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: "docker --version".to_string(),
        description: Some("description".to_string()),
    };
    let exit = run_target(&test_target);
    assert_eq!(exit, 0);
}
