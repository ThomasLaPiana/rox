mod file_requirements;
mod parser;
mod syntax;
mod targets;
mod utils;
mod version_requirements;
use clap::{Arg, ArgAction, Command};
use utils::{color_print, ColorEnum};

// TODO: Write a macro that parses the file ahead-of-time?

fn cli_builder(additional_commands: Vec<Command>) -> Command {
    Command::new("rox")
        .about("Robust Developer Experience CLI")
        .arg_required_else_help(true)
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Runs all checks and targets in parallel."),
        )
        .subcommand(
            Command::new("requirements").about("[Default] Run only the file and version checks."),
        )
        .subcommands(additional_commands)
}

fn main() {
    let start = std::time::Instant::now();

    // Load in the Roxfile(s)
    let file_path = "example_rox.yml".to_string();
    println!("> Loading Roxfile from path: {}", file_path);
    let roxfile = parser::parse_file_contents(parser::load_file(file_path));
    color_print(
        vec!["> File loaded successfully!".to_string()],
        ColorEnum::Green,
    );
    utils::horizontal_rule();

    // Build the CLI
    let additional_commands = roxfile
        .targets
        .clone()
        .iter()
        .map(|target| {
            Command::new(&target.name).about(target.description.clone().unwrap_or_default())
        })
        .collect();
    let cli = cli_builder(additional_commands);
    let cli_matches = cli.get_matches();

    // Build a HashMap of the targets and their objects
    let mut target_map = std::collections::HashMap::new();
    for target in roxfile.targets {
        target_map.insert(target.name.clone(), target);
    }

    // Nab the target and pass it to the runner
    let (_, target_stuff) = target_map
        .get_key_value(cli_matches.subcommand_name().unwrap())
        .unwrap();
    targets::run_target(target_stuff);

    if cli_matches.subcommand_matches("requirements").is_some()
        || roxfile.always_check_requirements.is_some()
    {
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

    // Print out the elapsed time
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}
