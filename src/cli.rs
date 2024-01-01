use crate::models::{CiInfo, Docs, Pipeline, Task};
use clap::{crate_version, Arg, ArgAction, Command};

/// Dyanmically construct the CLI from the Roxfile
pub fn construct_cli(
    tasks: &[Task],
    pipelines: &Option<Vec<Pipeline>>,
    docs: &Option<Vec<Docs>>,
    ci: &Option<CiInfo>,
) -> clap::Command {
    let mut cli = cli_builder(true);

    // CI
    if ci.is_some() {
        // TODO: Add a flag to only show failures
        cli = cli.subcommand(Command::new("ci").about("View CI pipeline information."));
    }

    // Docs
    if let Some(docs) = docs {
        let docs_subcommands = build_docs_subcommands(docs);
        cli = cli.subcommands(vec![docs_subcommands]);
    }

    // Tasks
    let task_subcommands = build_task_subcommands(tasks);
    cli = cli.subcommands(vec![task_subcommands]);

    // Pipelines
    if let Some(pipelines) = pipelines {
        let pipeline_subcommands = build_pipeline_subcommands(pipelines);
        cli = cli.subcommands(vec![pipeline_subcommands]);
    }
    cli
}

/// Construct the CLI
pub fn cli_builder(strict_subcommands: bool) -> Command {
    Command::new("rox")
        .about("Rox: The Robust Developer Experience CLI")
        .version(crate_version!())
        .arg_required_else_help(true)
        .allow_external_subcommands(!strict_subcommands)
        // TODO: Add a "watch" flag to run the command on file changes to a path?
        .arg(
            Arg::new("roxfile")
                .long("file")
                .short('f')
                .default_value("roxfile.yml")
                .help("Path to a Roxfile"),
        )
        .subcommand(
            Command::new("logs")
                .about("View logs for Rox invocations.")
                .arg(
                    Arg::new("number")
                        .help("The number of logs to view.")
                        .required(false)
                        .value_parser(clap::value_parser!(i8))
                        .default_value("1"),
                ),
        )
}

pub fn build_docs_subcommands(docs: &[Docs]) -> Command {
    let subcommands: Vec<Command> = docs
        .iter()
        .map(|doc| Command::new(&doc.name).about(doc.description.clone().unwrap_or_default()))
        .collect();

    Command::new("docs")
        .about("Display various kinds of documentation.")
        .arg_required_else_help(true)
        .subcommands(subcommands)
}

/// Build the `task` subcommand with individual tasks nested as subcommands
pub fn build_task_subcommands(tasks: &[Task]) -> Command {
    let subcommands: Vec<Command> = tasks
        .iter()
        .filter(|target| !target.hide.unwrap_or_default())
        .map(|task| Command::new(&task.name).about(task.description.to_owned().unwrap_or_default()))
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
        .long_about("Set(s) of task(s) composed into multiple stages.")
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
