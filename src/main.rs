mod files_requirements;
mod parser;
mod syntax;
mod utils;
mod version_requirements;
use clap::{Arg, ArgAction, Command};
use utils::{color_print, ColorEnum};

// TODO: Write a macro that parses the file ahead-of-time?

fn cli() -> Command {
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
        .subcommand(Command::new("requirements").about("Run only the requirements checks."))
        .subcommand(Command::new("show").about("Show all available Rox targets."))
}

fn main() {
    let cli_matches = cli().get_matches();
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
            for file in roxfile.file_requirements.unwrap().into_iter() {
                files_requirements::check_file_exists(&file);
            }
        }
        utils::horizontal_rule();
    }

    if cli_matches.subcommand_matches("show").is_some() {
        if let Some(targets) = roxfile.targets {
            for target in targets {
                println!("- Target Name: {}", target.name);
                println!("  Description: {}", target.description.unwrap_or_default());
            }
        }
    }

    // Print out the elapsed time
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}
