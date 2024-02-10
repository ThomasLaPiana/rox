pub mod model_injection;
pub mod output;
use crate::models::{JobResults, PassFail, Pipeline, Task, TaskResult};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::process::{Command, ExitStatus};

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
pub fn run_task(task: &Task, stage_number: i8) -> TaskResult {
    let start = std::time::Instant::now();

    let workdir = task.workdir.clone().unwrap_or(".".to_string());
    let command = task.command.as_ref().unwrap();

    println!("> Running command: '{}'", command);
    let command_results = Command::new("sh")
        .current_dir(workdir)
        .arg("-c")
        .arg(command)
        .status();

    TaskResult {
        name: task.name.to_string(),
        command: command.to_string(),
        stage: stage_number + 1,
        result: get_result_passfail(command_results),
        elapsed_time: start.elapsed().as_secs() as i64,
        file_path: task.file_path.to_owned().unwrap(),
    }
}

/// Execute a vector of Tasks, potentially in parallel
pub fn execute_tasks(
    tasks: Vec<String>,
    stage_number: i8,
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
        return task_stack
            .par_iter()
            .map(|task| run_task(task, stage_number))
            .collect();
    } else {
        return task_stack
            .iter()
            .map(|task| run_task(task, stage_number))
            .collect();
    }
}

/// Execute a vector of Stages
pub fn execute_stages(
    stages: &[Vec<String>],
    task_map: &HashMap<String, Task>,
    parallel: bool,
) -> Vec<Vec<TaskResult>> {
    let stage_results: Vec<Vec<TaskResult>> = stages
        .iter()
        .enumerate()
        .map(|(stage_number, stage)| {
            execute_tasks(stage.clone(), stage_number as i8, task_map, parallel)
        })
        .collect();
    stage_results
    // TODO: Return a JobResults here
}

/// Execute Pipeline
pub fn execute_pipeline(pipeline: Pipeline, task_map: &HashMap<String, Task>, parallel: bool) {
    let execution_start = chrono::Utc::now().to_rfc3339();
    let execution_results = execute_stages(&pipeline.stages, task_map, parallel);
    let results = JobResults {
        job_name: pipeline.name.to_string(),
        execution_time: execution_start,
        results: execution_results.into_iter().flatten().collect(),
    };
    results.log_results();
    results.display_results();
    results.check_results();
}

/// Execute a single user-defined Task
pub fn execute_task(task: Task) {
    let execution_start = chrono::Utc::now().to_rfc3339();
    let execution_results: TaskResult = run_task(&task, 0);
    let results = JobResults {
        job_name: task.name.to_string(),
        execution_time: execution_start,
        results: vec![execution_results],
    };

    results.log_results();
    results.display_results();
    results.check_results();
}
