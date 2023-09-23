//! This is the module responsible for executing tasks.
use crate::models::Task;
use crate::utils;
use std::collections::HashMap;
use std::process::{Command, ExitStatus};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

/// Run a Task
pub fn run_task(task: &Task) -> TaskResult {
    let start = std::time::Instant::now();

    let workdir = task.workdir.clone().unwrap_or(".".to_string());

    println!("> Running command: '{}'", task.command.as_ref().unwrap());
    let (command, args) = utils::split_head_from_rest(task.command.as_ref().unwrap());
    let command_results = Command::new(command)
        .current_dir(workdir)
        .args(args)
        .status();

    TaskResult {
        name: task.name.to_string(),
        result: get_result_passfail(command_results),
        elapsed_time: start.elapsed().as_secs(),
        file_path: task.file_path.clone().unwrap(),
    }
}

/// Execute a vector of Tasks, potentially in parallel
pub fn execute_tasks(
    tasks: Vec<String>,
    task_map: &HashMap<String, Task>,
    parallel: bool,
) -> Vec<TaskResult> {
    let task_stack: Vec<Task> = tasks
        .iter()
        .map(|task| {
            task_map
                .get(task)
                .expect("Error! Task does not exist!")
                .to_owned()
        })
        .collect();
    println!(
        "> Running task(s): {:#?}",
        &task_stack
            .iter()
            .map(|task| &task.name)
            .collect::<Vec<&String>>()
    );

    // TODO: Add progress bars?
    if parallel {
        return task_stack.par_iter().map(run_task).collect();
    } else {
        return task_stack.iter().map(run_task).collect();
    }
}
