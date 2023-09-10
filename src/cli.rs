use crate::syntax::Target;
use clap::{Arg, ArgAction, Command};

/// Construct the CLI
pub fn cli_builder(subcommands: Vec<Command>) -> Command {
    Command::new("rox")
        .about("Rox: The Robust Developer Experience CLI")
        .arg_required_else_help(true)
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Runs all checks and targets in parallel."),
        )
        .subcommands(subcommands)
}

/// Build Subcommands to add to the CLI
pub fn build_sub_commands(targets: Vec<Target>) -> Vec<Command> {
    let additional_commands = targets
        .iter()
        .map(|target| {
            Command::new(&target.name).about(target.description.clone().unwrap_or_default())
        })
        .collect();
    additional_commands
}
