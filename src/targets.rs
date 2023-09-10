use crate::syntax::Target;
use crate::utils;
use std::collections::HashMap;
use std::process::Command;

pub fn run_target(target: &Target) -> i32 {
    let start = std::time::Instant::now();
    println!("> Running Target: {}", target.name);
    let (command, args) = utils::split_head_from_rest(target.command.as_ref().unwrap().clone());
    let command_results = Command::new(command).args(args).status();
    let exit_status = command_results.unwrap().code().unwrap();
    println!(
        "> Target '{}' elapsed time: {}s",
        target.name,
        start.elapsed().as_secs()
    );
    exit_status
}

/// Recursively called executor
pub fn execute_targets(primary_target: Target, target_map: &HashMap<String, Target>) {
    // TODO: Deduplicate target calls?
    // TODO: Check for circular dependencies
    let current_command = primary_target.command.clone().unwrap_or_default();
    let pre_targets = primary_target.pre_targets.clone().unwrap_or_default();
    let post_targets = primary_target.post_targets.clone().unwrap_or_default();

    if current_command.is_empty() && pre_targets.is_empty() && post_targets.is_empty() {
        panic!("Targets must have either a command, pre_targets, post_targets, or any combination of the above.")
    }

    if !pre_targets.is_empty() {
        println!(
            "Target: '{}' has the following pre targets: {:?}",
            primary_target.name, pre_targets
        );
    }
    if !post_targets.is_empty() {
        println!(
            "Target: {} has the following pre targets: {:?}",
            primary_target.name, post_targets
        );
    }

    // Recursively run pre targets
    for target in pre_targets {
        let t = target_map.get(&target).unwrap().to_owned();
        execute_targets(t, target_map);
    }

    // Run the primary target once all pre targets are exhausted
    if !current_command.is_empty() {
        run_target(&primary_target);
    }

    // Recursively run the post targets
    for target in post_targets {
        let t = target_map.get(&target).unwrap().to_owned();
        execute_targets(t, target_map);
    }
}

#[test]
#[should_panic]
fn test_invalid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: Some("notacommand lol".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
    };
    let exit = run_target(&test_target);
    assert_eq!(exit, 1);
}

#[test]
#[should_panic]
fn test_no_command_no_pre_no_post() {
    let test_target = Target {
        name: "foo".to_string(),
        command: None,
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
    };
    let exit = run_target(&test_target);
    assert_eq!(exit, 1);
}

#[test]
fn test_valid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
    };
    let exit = run_target(&test_target);
    assert_eq!(exit, 0);
}
