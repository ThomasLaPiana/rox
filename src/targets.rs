use crate::syntax::Target;
use crate::utils;
use std::collections::HashMap;
use std::process::Command;

pub fn run_target(target: &Target) -> i32 {
    let start = std::time::Instant::now();
    println!("> Running Target: {}", target.name);
    let (command, args) = utils::split_head_from_rest(target.command.clone());
    let command_results = Command::new(command).args(args).status();
    let exit_status = command_results.unwrap().code().unwrap();
    println!(
        "> Target '{}' elapsed time: {}s",
        target.name,
        start.elapsed().as_secs()
    );
    exit_status
}

pub fn execute_targets(primary_target: Target, target_map: &HashMap<String, Target>) {
    run_target(&primary_target);
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
