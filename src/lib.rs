mod ci;
mod cli;
mod docs;
mod execution;
mod logs;
mod model_injection;
pub mod models;
mod output;
mod utils;

use crate::cli::{cli_builder, construct_cli};
use crate::execution::{execute_stages, execute_tasks};
use crate::models::JobResults;
use std::collections::HashMap;
use std::error::Error;

type RoxResult<T> = Result<T, Box<dyn Error>>;

/// Get the filepath argument from the CLI
///
/// This is required because we might need to
/// dynamically populate the CLI based on this arg
fn get_filepath_arg_value() -> String {
    let cli = cli_builder(false);
    // Get the file arg from the CLI if set
    let cli_matches = cli.clone().arg_required_else_help(false).get_matches();
    cli_matches.get_one::<String>("roxfile").unwrap().to_owned()
}

/// Entrypoint for the Crate CLI
pub async fn rox() -> RoxResult<()> {
    let start = std::time::Instant::now();
    let execution_start = chrono::Utc::now().to_rfc3339();

    // NOTE: Due to the dynamically generated nature of the CLI,
    // It is required to parse the CLI matches twice. Once to get
    // the filename arg and once to actually build the CLI.

    // Get the file arg from the CLI if set
    let file_path = get_filepath_arg_value();
    let roxfile = models::RoxFile::build(&file_path)?;
    utils::print_horizontal_rule();

    // Build & Generate the CLI based on the loaded Roxfile
    let cli = construct_cli(
        &roxfile.tasks,
        &roxfile.pipelines,
        &roxfile.docs,
        &roxfile.ci,
    );
    let cli_matches = cli.get_matches();

    let task_map: HashMap<String, models::Task> = std::collections::HashMap::from_iter(
        roxfile
            .tasks
            .into_iter()
            .map(|task| (task.name.to_owned(), task)),
    );

    // Deconstruct the CLI commands and get the Pipeline object that was called
    let (_, args) = cli_matches.subcommand().unwrap();
    let subcommand_name = args.subcommand_name().unwrap_or("default");

    // Execute the Command
    match cli_matches.subcommand_name().unwrap() {
        "docs" => {
            let docs_map: HashMap<String, models::Docs> = std::collections::HashMap::from_iter(
                roxfile
                    .docs
                    .into_iter()
                    .flatten()
                    .map(|doc| (doc.name.to_owned(), doc)),
            );
            docs::display_docs(docs_map.get(subcommand_name).unwrap());
            std::process::exit(0);
        }
        "logs" => {
            let number = args.get_one::<i8>("number").unwrap();
            logs::display_logs(number);
            std::process::exit(0);
        }
        "ci" => {
            assert!(roxfile.ci.is_some());
            ci::display_ci_status(roxfile.ci.unwrap()).await;
            std::process::exit(0);
        }
        "pl" => {
            let pipeline_map: HashMap<String, models::Pipeline> =
                std::collections::HashMap::from_iter(
                    roxfile
                        .pipelines
                        .into_iter()
                        .flatten()
                        .map(|pipeline| (pipeline.name.to_owned(), pipeline)),
                );
            let parallel = args.get_flag("parallel");
            let execution_results = execute_stages(
                &pipeline_map.get(subcommand_name).unwrap().stages,
                &task_map,
                parallel,
            );
            let results = JobResults {
                job_name: subcommand_name.to_string(),
                execution_time: execution_start,
                results: execution_results.into_iter().flatten().collect(),
            };
            results.log_results();
            results.display_results();
            results.check_results();
        }
        "task" => {
            let execution_results = vec![execute_tasks(
                vec![subcommand_name.to_string()],
                0,
                &task_map,
                false,
            )];
            let results = JobResults {
                job_name: subcommand_name.to_string(),
                execution_time: execution_start,
                results: execution_results.into_iter().flatten().collect(),
            };
            results.log_results();
            results.display_results();
            results.check_results();
        }
        _ => unreachable!("Invalid subcommand"),
    };

    println!(
        "> Total elapsed time: {}s | {}ms",
        start.elapsed().as_secs(),
        start.elapsed().as_millis(),
    );

    Ok(())
}
