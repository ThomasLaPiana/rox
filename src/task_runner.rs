//! This is the module responsible for executing tasks.
use crate::models::Task;
use crate::utils::{self, color_print};
use rayon::prelude::*;
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
pub struct TaskResult {
    pub name: String,
    pub result: PassFail,
    pub parameters: String,
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

/// The struct for a Task that has been injected with Parameter values
pub struct InjectedTask {
    name: String,
    command: String,
    parameters: String,
    file_path: String,
}

// Create a vector of tasks based on the Paramaters field
// pub fn create_injected_tasks(task: Task) -> Vec<InjectedTask> {
//     let mut tasks: Vec<InjectedTask> = Vec::new();
//     let raw_command = task.command.unwrap_or_default();
//     let symbols = task.symbols.unwrap_or_default();
//     let substitutions = task.substitutions.unwrap_or_default();

//     if symbols.is_empty()
//         && !substitutions.is_empty() | !symbols.is_empty()
//         && substitutions.is_empty()
//     {
//         color_print(
//             vec!["Symbols and Substitutions must both be set if one is!"],
//             utils::ColorEnum::Red,
//         );
//         panic!()
//     }

//     if symbols.is_empty() {
//         return vec![InjectedTask {
//             name: task.name.clone(),
//             command: raw_command.clone(),
//             parameters: "N/A".to_string(),
//             file_path: task.file_path.clone().unwrap(),
//         }];
//     }

//     for substitution in &substitutions {
//         tasks.push(InjectedTask {
//             name: task.name.clone(),
//             command: {
//                 let mut replaced_command = raw_command.clone();
//                 for index in 0..substitutions.len() {
//                     replaced_command =
//                         replaced_command.replace(&symbols[index], &substitution.values[index]);
//                 }
//                 replaced_command
//             },
//             parameters: {
//                 let mut parameters = String::new();
//                 for index in 0..substitutions.len() {
//                     parameters.push_str(&format!(
//                         "{}={}\n",
//                         &symbols[index], &substitution.values[index]
//                     ))
//                 }
//                 parameters
//             },
//             file_path: task.file_path.clone().unwrap(),
//         })
//     }
//     tasks
// }

/// Run a Task
pub fn run_task(task: &InjectedTask) -> TaskResult {
    let start = std::time::Instant::now();
    println!("> Running task: {}", task.name);

    let (command, args) = utils::split_head_from_rest(&task.command);
    let command_results = Command::new(command).args(args).status();

    TaskResult {
        name: task.name.to_string(),
        result: get_result_passfail(command_results),
        parameters: task.parameters.clone(),
        elapsed_time: start.elapsed().as_secs(),
        file_path: task.file_path.clone(),
    }
}

// Recursively called executor
// pub fn execute_tasks(
//     primary_task: Task,
//     task_map: &HashMap<String, Task>,
//     parallel: bool,
// ) -> Vec<TaskResult> {
//     let current_command = primary_task.command.clone().unwrap_or_default();
//     let pre_tasks = primary_task.pre_tasks.clone().unwrap_or_default();
//     let post_tasks = primary_task.post_tasks.clone().unwrap_or_default();

//     if current_command.is_empty() && pre_tasks.is_empty() && post_tasks.is_empty() {
//         utils::color_print(vec!["Tasks must have either a command, pre_tasks, post_tasks, or any combination of the above."], utils::ColorEnum::Red);
//         std::process::exit(1);
//     }

//     // TODO: Check for non-existent tasks

//     let mut task_stack: Vec<InjectedTask> = Vec::new();

//     // Handle Pre-tasks
//     for task in pre_tasks.into_iter() {
//         let t = create_injected_tasks(task_map.get(&task).unwrap().to_owned());
//         task_stack.extend(t);
//     }

//     // Handle Primary task
//     if !current_command.is_empty() {
//         task_stack.extend(create_injected_tasks(primary_task));
//     }

//     // Handle Post-tasks
//     for task in post_tasks {
//         let t = create_injected_tasks(task_map.get(&task).unwrap().to_owned());
//         task_stack.extend(t);
//     }

//     if parallel {
//         return task_stack.par_iter().map(run_task).collect();
//     } else {
//         return task_stack.iter().map(run_task).collect();
//     }
// }
