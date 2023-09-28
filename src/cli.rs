use crate::models::{Pipeline, Task};
use clap::{crate_version, Arg, ArgAction, Command};

/// Construct the CLI
pub fn cli_builder() -> Command {
    Command::new("rox")
        .about("Rox: The Robust Developer Experience CLI")
        .version(crate_version!())
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // TODO: Add a "watch" flag to run the command on file changes to a path?
        // TODO: Add the option to log the command outputs into a file?
        .arg(
            Arg::new("roxfile")
                .long("file")
                .short('f')
                .default_value("roxfile.yml")
                .help("Path to a Roxfile"),
        )
        .arg(
            Arg::new("skip-checks")
                .long("skip-checks")
                .short('s')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Skip the version and file requirement checks."),
        )
}

/// Build the `task` subcommand with individual tasks nested as subcommands
pub fn build_task_subcommands(tasks: &[Task]) -> Command {
    let subcommands: Vec<Command> = tasks
        .iter()
        .filter(|target| !target.hide.unwrap_or_default())
        .map(|task| Command::new(&task.name).about(task.description.clone().unwrap_or_default()))
        .collect();

    Command::new("task")
        .about("Discrete executable tasks.")
        .long_about("Discrete units of execution containing a single runnable command.")
        .arg_required_else_help(true)
        .subcommands(subcommands)
}

/// Build the `pipelines` subcommand with individual pipelines as subcommands
pub fn build_pipeline_subcommands(pipelines: &[Pipeline]) -> Command {
    let subcommands: Vec<Command> = pipelines
        .iter()
        .map(|pipeline| {
            Command::new(&pipeline.name).about(pipeline.description.clone().unwrap_or_default())
        })
        .collect();

    Command::new("pl")
        .about("Pipelines composed of multiple tasks.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Run the pipeline's tasks in parallel."),
        )
        .subcommands(subcommands)
}
