mod file_requirements;
mod parser;
mod syntax;
mod targets;
mod utils;
mod version_requirements;
use std::collections::HashMap;
use utils::{color_print, ColorEnum};
mod cli;

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
    let targets = roxfile.targets.clone();
    let subcommands = cli::build_sub_commands(targets.clone());
    let cli = cli::cli_builder(subcommands);
    let cli_matches = cli.get_matches();

    // Build a HashMap of the targets and their objects
    let target_map: HashMap<String, syntax::Target> = std::collections::HashMap::from_iter(
        targets
            .into_iter()
            .map(|target| (target.name.clone(), target)),
    );

    if roxfile.always_check_requirements.is_some() {
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

    // Nab the target and pass it to the runner
    let target_stuff = target_map
        .get(cli_matches.subcommand_name().unwrap())
        .unwrap();
    targets::run_target(target_stuff);

    // Print out the elapsed time
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}
