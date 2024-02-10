mod cli;
pub mod models;
mod modules;
mod utils;

use crate::cli::{cli_builder, construct_cli};
use crate::modules::execution::{execute_pipeline, execute_task};
use crate::modules::{ci, docs, logs};
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

    let (_, args) = cli_matches.subcommand().unwrap();
    let subcommand_name = args.subcommand_name().unwrap_or("default");

    // Execute the Command
    match cli_matches.subcommand_name() {
        Some("docs") => {
            let documentation = roxfile
                .docs
                .iter()
                .flatten()
                .find(|doc| doc.name == subcommand_name)
                .unwrap();
            docs::display_docs(documentation);
            std::process::exit(0);
        }
        Some("logs") => {
            let number = args.get_one::<i8>("number").unwrap();
            logs::display_logs(number);
            std::process::exit(0);
        }
        Some("ci") => {
            assert!(roxfile.ci.is_some());
            ci::display_ci_status(roxfile.ci.unwrap()).await;
            std::process::exit(0);
        }
        Some("pl") => {
            let parallel = args.get_flag("parallel");
            let pipeline = roxfile
                .pipelines
                .into_iter()
                .flatten()
                .find(|pipeline| pipeline.name == subcommand_name)
                .unwrap(); // Clap will catch a non-existent Pipeline for us
            execute_pipeline(pipeline, &task_map, parallel);
        }
        Some("task") => execute_task(task_map.get(subcommand_name).unwrap().to_owned()),
        _ => unreachable!("Invalid subcommand"),
    };

    println!(
        "> Total elapsed time: {}s | {}ms",
        start.elapsed().as_secs(),
        start.elapsed().as_millis(),
    );

    Ok(())
}
