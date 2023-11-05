mod cli;
mod execution;
mod file_requirements;
mod model_injection;
pub mod models;
mod output;
mod utils;
mod version_requirements;

use crate::cli::{cli_builder, construct_cli};
use crate::execution::{execute_stages, execute_tasks, PassFail, TaskResult};
use crate::model_injection::{
    inject_pipeline_metadata, inject_task_metadata, inject_template_values,
};
use crate::models::Validate;
use std::collections::HashMap;
use std::error::Error;

type RoxResult<T> = Result<T, Box<dyn Error>>;

/// Get the filepath argument from the CLI
///
/// This is required because we might need to
/// dynamically populate the CLI based on this arg
fn get_filepath_arg_value() -> String {
    let cli = cli_builder();
    // Get the file arg from the CLI if set
    let cli_matches = cli.clone().arg_required_else_help(false).get_matches();
    cli_matches.get_one::<String>("roxfile").unwrap().to_owned()
}

/// Entrypoint for the Crate CLI
pub fn rox() -> RoxResult<()> {
    let start = std::time::Instant::now();

    // NOTE: Due to the dynamically generated nature of the CLI,
    // It is required to parse the CLI matches twice. Once to get
    // the filename arg and once to actually build the CLI.

    // Get the file arg from the CLI if set
    let file_path = get_filepath_arg_value();
    let roxfile = utils::parse_file_contents(utils::load_file(&file_path));
    roxfile.validate()?;
    utils::print_horizontal_rule();

    // Build & Generate the CLI based on the loaded Roxfile
    let tasks = inject_task_metadata(roxfile.tasks, &file_path);
    let pipelines = inject_pipeline_metadata(roxfile.pipelines);
    let cli = construct_cli(&tasks, &pipelines);
    let cli_matches = cli.get_matches();

    // Run File and Version checks
    if !cli_matches.get_flag("skip-checks") {
        // Check Versions
        if roxfile.version_requirements.is_some() {
            for version_check in roxfile.version_requirements.into_iter().flatten() {
                version_requirements::check_version(version_check);
            }
        }

        // Check Files
        if roxfile.file_requirements.is_some() {
            for requirement in roxfile.file_requirements.into_iter().flatten() {
                file_requirements::handle_file_requirement(requirement);
            }
        }
    }

    // Build Hashmaps for Tasks, Templates and Pipelines
    let template_map: HashMap<String, models::Template> = std::collections::HashMap::from_iter(
        roxfile
            .templates
            .into_iter()
            .flatten()
            .map(|template| (template.name.to_owned(), template)),
    );
    let task_map: HashMap<String, models::Task> = std::collections::HashMap::from_iter(
        tasks
            .into_iter()
            .map(|task| match task.uses.to_owned() {
                Some(task_use) => {
                    inject_template_values(task, template_map.get(&task_use).unwrap())
                }
                None => task,
            })
            .map(|task| (task.name.to_owned(), task)),
    );
    let pipeline_map: HashMap<String, models::Pipeline> = std::collections::HashMap::from_iter(
        pipelines
            .into_iter()
            .flatten()
            .map(|pipeline| (pipeline.name.to_owned(), pipeline)),
    );

    // Execute the Task(s)
    let results: Vec<Vec<TaskResult>> = match cli_matches.subcommand_name().unwrap() {
        "pl" => {
            // Deconstruct the CLI commands and get the Pipeline object that was called
            let (_, args) = cli_matches.subcommand().unwrap();
            let pipeline_name = args.subcommand_name().unwrap();
            let parallel = args.get_flag("parallel");
            let execution_results = execute_stages(
                &pipeline_map.get(pipeline_name).unwrap().stages,
                &task_map,
                parallel,
            );
            execution_results
        }
        "task" => {
            let (_, args) = cli_matches.subcommand().unwrap();
            let task_name = args.subcommand_name().unwrap().to_owned();
            let execution_results = vec![execute_tasks(vec![task_name], &task_map, false)];
            execution_results
        }
        command => {
            println!("'{}' is not a valid subcommand!", command);
            std::process::exit(2);
        }
    };
    output::display_execution_results(&results);
    println!(
        "> Total elapsed time: {}s | {}ms",
        start.elapsed().as_secs(),
        start.elapsed().as_millis(),
    );
    nonzero_exit_if_failure(&results);

    Ok(())
}

/// Throw a non-zero exit if any Task(s) had a failing result
pub fn nonzero_exit_if_failure(results: &[Vec<TaskResult>]) {
    // TODO: Figure out a way to get this info without looping again
    for result in results.iter().flatten() {
        if result.result == PassFail::Fail {
            std::process::exit(2)
        }
    }
}
