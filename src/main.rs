mod execution;
mod file_requirements;
mod models;
mod utils;
mod version_requirements;
use std::collections::HashMap;
use utils::{color_print, ColorEnum};

use crate::execution::execute_tasks;
mod cli;
mod output;

// Entrypoint for the Crate CLI
pub fn main() {
    let start = std::time::Instant::now();

    // Load in the Roxfile(s)
    let file_path = "roxfile.yml".to_string();
    println!("> Loading Roxfile from path: {}", file_path);
    let roxfile = models::parse_file_contents(models::load_file(&file_path));
    color_print(vec!["> File loaded successfully!"], ColorEnum::Green);
    utils::horizontal_rule();

    // Build the CLI
    let mut cli = cli::cli_builder();

    // Build, Sort and add Tasks
    let mut sorted_tasks: Vec<models::Task> = roxfile
        .tasks
        .clone()
        .into_iter()
        .map(|task| {
            let mut new_task = task.clone();
            new_task.file_path = Some(file_path.clone());
            new_task
        })
        .collect();
    sorted_tasks.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    let task_subcommands = cli::build_task_subcommands(&sorted_tasks);
    cli = cli.subcommands(vec![task_subcommands]);

    // Build, Sort and add Pipelines
    if let Some(pipelines) = roxfile.pipelines.clone() {
        let mut sorted_pipelines: Vec<models::Pipeline> = pipelines
            .clone()
            .into_iter()
            .map(|pipeline| {
                let mut new_pipeline = pipeline.clone();
                new_pipeline.file_path = Some(file_path.clone());
                new_pipeline
            })
            .collect();
        sorted_pipelines.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
        let pipeline_subcommands = cli::build_pipeline_subcommands(&sorted_pipelines);
        cli = cli.subcommands(vec![pipeline_subcommands]);
    }
    let cli_matches = cli.get_matches();

    // Run File and Version checks
    if !cli_matches.get_flag("skip-checks") {
        // Check Versions
        if roxfile.version_requirements.is_some() {
            println!("> Checking versions...");
            for version_check in roxfile.version_requirements.unwrap().into_iter() {
                version_requirements::check_version(version_check.clone());
            }
        }
        utils::horizontal_rule();

        // Check Files
        if roxfile.file_requirements.is_some() {
            println!("Checking files...");
            for requirement in roxfile.file_requirements.unwrap().into_iter() {
                file_requirements::handle_file_requirement(requirement);
            }
        }
        utils::horizontal_rule();
    }

    // Build a HashMap of the task names and their objects
    let task_map: HashMap<String, models::Task> = std::collections::HashMap::from_iter(
        roxfile
            .tasks
            .into_iter()
            .map(|task| (task.name.clone(), task)),
    );

    // Execute the Tasks
    let results = match cli_matches.subcommand_name().unwrap() {
        "pl" => {
            // Build a HashMap of the pipeline names and their objects to use for lookup
            let pipeline_map: HashMap<String, models::Pipeline> =
                std::collections::HashMap::from_iter(
                    roxfile
                        .pipelines
                        .unwrap()
                        .into_iter()
                        .map(|pipeline| (pipeline.name.clone(), pipeline)),
                );
            // Deconstruct the CLI commands and get the Pipeline object that was called
            let (_, args) = cli_matches.subcommand().unwrap();
            let pipeline_name = args.subcommand_name().unwrap();
            execute_tasks(
                pipeline_map.get(pipeline_name).unwrap().tasks.clone(),
                &task_map,
                false,
            )
        }
        "task" => {
            let (_, args) = cli_matches.subcommand().unwrap();
            let task_name = args.subcommand_name().unwrap().to_owned();
            execute_tasks(vec![task_name], &task_map, false)
        }
        &_ => std::process::abort(),
    };
    output::display_execution_results(results);

    println!("> Total elapsed time: {}s", start.elapsed().as_secs());
}
