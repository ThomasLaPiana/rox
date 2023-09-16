use crate::roxfile::Task;
use clap::{Arg, ArgAction, Command};

/// Construct the CLI
pub fn cli_builder(subcommands: Vec<Command>) -> Command {
    Command::new("rox")
        .about("Rox: The Robust Developer Experience CLI")
        .arg_required_else_help(true)
        // TODO: Add a flag to ignore pre/post tasks?
        // TODO: Add a "watch" flag to run the command on file changes to a path
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Force tasks to run in parallel."),
        )
        .arg(
            Arg::new("skip-checks")
                .long("skip-checks")
                .short('s')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Skip the version and file requirement checks."),
        )
        .subcommands(subcommands)
}

/// Build Subcommands to add to the CLI
pub fn build_sub_commands(tasks: Vec<Task>) -> Vec<Command> {
    let additional_commands = tasks
        .iter()
        .map(|task| Command::new(&task.name).about(task.description.clone().unwrap_or_default()))
        .collect();
    additional_commands
}
