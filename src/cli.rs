use crate::models::{Pipeline, Task};
use clap::{crate_version, Arg, ArgAction, Command};

/// Construct the CLI
pub fn cli_builder() -> Command {
    Command::new("rox")
        .about("Rox: The Robust Developer Experience CLI")
        .version(crate_version!())
        .arg_required_else_help(true)
        // TODO: Add a "watch" flag to run the command on file changes to a path
        .arg(
            Arg::new("skip-checks")
                .long("skip-checks")
                .short('s')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Skip the version and file requirement checks."),
        )
}

/// Build Commands to add to the CLI

/// Build the `task` subcommand with individual tasks nested as subcommands
pub fn build_task_subcommands(tasks: &Vec<Task>) -> Command {
    let subcommands: Vec<Command> = tasks
        .iter()
        .filter(|target| !target.hide.unwrap_or_default())
        .map(|task| Command::new(&task.name).about(task.description.clone().unwrap_or_default()))
        .collect();

    Command::new("task")
        .about("Single executable tasks.")
        .long_about("Discrete units of execution containing a single runnable command.")
        .subcommands(subcommands)
}

/// Build the `pipelines` subcommand with individual pipelines as subcommands
pub fn build_pipeline_subcommands(pipelines: &Vec<Pipeline>) -> Command {
    let subcommands: Vec<Command> = pipelines
        .iter()
        .map(|pipeline| {
            Command::new(&pipeline.name).about(pipeline.description.clone().unwrap_or_default())
        })
        .collect();

    Command::new("pl")
        .about("Executable pipelines.")
        .long_about("A group of tasks composed into an executable pipeline.")
        .subcommands(subcommands)
}
