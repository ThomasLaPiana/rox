use crate::syntax::Target;
use crate::utils;
use std::collections::HashMap;
use std::process::{Command, ExitStatus};

#[derive(PartialEq, Debug)]
pub enum PassFail {
    Pass,
    Fail,
}
impl std::fmt::Display for PassFail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug)]
pub struct TargetResult {
    pub name: String,
    pub result: PassFail,
    pub elapsed_time: u64,
    pub file_path: String,
}

pub fn get_result_passfail(result: Result<ExitStatus, std::io::Error>) -> PassFail {
    // If the command doesn't exist, we get an error here
    if result.is_err() {
        return PassFail::Fail;
    }

    if result.unwrap().code().unwrap() == 0 {
        return PassFail::Pass;
    }

    PassFail::Fail
}

pub fn run_target(target: &Target) -> TargetResult {
    let start = std::time::Instant::now();
    println!("> Running Target: {}", target.name);
    let (command, args) = utils::split_head_from_rest(&target.command.as_ref().unwrap().clone());
    let command_results = Command::new(command).args(args).status();

    TargetResult {
        name: target.name.to_string(),
        result: get_result_passfail(command_results),
        elapsed_time: start.elapsed().as_secs(),
        file_path: target.file_path.clone().unwrap(),
    }
}

/// Recursively called executor
pub fn execute_targets(
    primary_target: Target,
    target_map: &HashMap<String, Target>,
) -> Vec<TargetResult> {
    let current_command = primary_target.command.clone().unwrap_or_default();
    let pre_targets = primary_target.pre_targets.clone().unwrap_or_default();
    let post_targets = primary_target.post_targets.clone().unwrap_or_default();

    if current_command.is_empty() && pre_targets.is_empty() && post_targets.is_empty() {
        panic!("Targets must have either a command, pre_targets, post_targets, or any combination of the above.")
    }

    // TODO: Check for non-existent targets

    let mut command_stack: Vec<TargetResult> = Vec::new();

    // Handle Pre-Targets
    if !pre_targets.is_empty() {
        println!(
            "Target: '{}' has the following pre targets: {:?}",
            primary_target.name, pre_targets
        );
        for target in pre_targets {
            let t = target_map.get(&target).unwrap().to_owned();
            command_stack.push(run_target(&t));
        }
    }

    // Handle Primary Target
    if !current_command.is_empty() {
        command_stack.push(run_target(&primary_target));
    }

    // Handle Post-Targets
    if !post_targets.is_empty() {
        println!(
            "Target: {} has the following pre targets: {:?}",
            primary_target.name, post_targets
        );
        for target in post_targets {
            let t = target_map.get(&target).unwrap().to_owned();
            command_stack.push(run_target(&t));
        }
    }
    command_stack
}

#[test]
fn test_invalid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: Some("foo".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
        file_path: Some("test".to_string()),
    };
    let exit = run_target(&test_target);
    assert_eq!(
        exit,
        TargetResult {
            name: "foo".to_string(),
            result: PassFail::Fail,
            elapsed_time: 0,
            file_path: "test".to_string(),
        }
    );
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
        file_path: Some("test".to_string()),
    };
    run_target(&test_target);
}

#[test]
fn test_valid_command() {
    let test_target = Target {
        name: "foo".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
        file_path: Some("test".to_string()),
    };
    let exit = run_target(&test_target);
    assert_eq!(
        exit,
        TargetResult {
            name: "foo".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        }
    );
}

#[test]
fn test_execution_order() {
    let target1 = Target {
        name: "target1".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: Some(vec!["target2".to_string()]),
        post_targets: Some(vec!["target3".to_string()]),
        file_path: Some("test".to_string()),
    };
    let target2 = Target {
        name: "target2".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: Some(vec!["target3".to_string()]),
        file_path: Some("test".to_string()),
    };
    let target3 = Target {
        name: "target3".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: Some(vec!["target4".to_string()]),
        file_path: Some("test".to_string()),
    };
    // Even though Target4 is listed as a post-target for target3,
    // it shouldn't get run!
    let target4 = Target {
        name: "target4".to_string(),
        command: Some("docker --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: None,
        file_path: Some("test".to_string()),
    };
    let mut test_target_map = HashMap::new();
    test_target_map.insert("target1".to_string(), target1.clone());
    test_target_map.insert("target2".to_string(), target2);
    test_target_map.insert("target3".to_string(), target3);
    test_target_map.insert("target4".to_string(), target4);
    let output = execute_targets(target1, &test_target_map);
    let expected_output = vec![
        TargetResult {
            name: "target2".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
        TargetResult {
            name: "target1".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
        TargetResult {
            name: "target3".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
    ];
    assert_eq!(expected_output, output);
}

#[test]
fn test_duplicate_command_execution() {
    let target1 = Target {
        name: "target1".to_string(),
        command: Some("cargo --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: Some(vec!["target2".to_string(), "target3".to_string()]),
        post_targets: Some(vec!["target3".to_string()]),
        file_path: Some("test".to_string()),
    };
    let target2 = Target {
        name: "target2".to_string(),
        command: Some("cargo --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: Some(vec!["target3".to_string()]),
        file_path: Some("test".to_string()),
    };
    let target3 = Target {
        name: "target3".to_string(),
        command: Some("cargo --version".to_string()),
        description: Some("description".to_string()),
        pre_targets: None,
        post_targets: Some(vec!["target4".to_string()]),
        file_path: Some("test".to_string()),
    };
    let mut test_target_map = HashMap::new();
    test_target_map.insert("target1".to_string(), target1.clone());
    test_target_map.insert("target2".to_string(), target2);
    test_target_map.insert("target3".to_string(), target3);
    let output = execute_targets(target1, &test_target_map);
    let expected_output = vec![
        TargetResult {
            name: "target2".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
        TargetResult {
            name: "target3".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
        TargetResult {
            name: "target1".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
        TargetResult {
            name: "target3".to_string(),
            result: PassFail::Pass,
            elapsed_time: 0,
            file_path: "test".to_string(),
        },
    ];
    assert_eq!(expected_output, output);
}
