mod file_requirements;
mod roxfile;
mod task_runner;
mod utils;
mod version_requirements;
use std::collections::HashMap;
use utils::{color_print, ColorEnum};
mod cli;
mod output;

// Entrypoint for the Crate CLI
fn main() {
    let start = std::time::Instant::now();

    // Load in the Roxfile(s)
    let file_path = "roxfile.yml".to_string();
    println!("> Loading Roxfile from path: {}", file_path);
    let roxfile = roxfile::parse_file_contents(roxfile::load_file(file_path));
    color_print(vec!["> File loaded successfully!"], ColorEnum::Green);
    utils::horizontal_rule();

    // Build the CLI
    let mut unsorted_tasks: Vec<roxfile::Task> = roxfile
        .tasks
        .clone()
        .iter()
        .map(|task| roxfile::task_builder(task.to_owned(), "roxfile.yml".to_string()))
        .collect();

    unsorted_tasks.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    let tasks = unsorted_tasks;
    let subcommands = cli::build_sub_commands(tasks.clone());
    let cli = cli::cli_builder(subcommands);
    let cli_matches = cli.get_matches();

    // Build a HashMap of the task names and their objects
    let task_map: HashMap<String, roxfile::Task> = std::collections::HashMap::from_iter(
        tasks.into_iter().map(|task| (task.name.clone(), task)),
    );

    if !cli_matches.get_flag("skip-checks") {
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

    // Execute the Tasks
    let parallel = cli_matches.get_flag("parallel");
    let task_stuff = task_map
        .get(cli_matches.subcommand_name().unwrap())
        .unwrap();
    let results = task_runner::execute_tasks(task_stuff.to_owned(), &task_map, parallel);
    output::display_execution_results(results);

    println!("> Total elapsed time: {}s", start.elapsed().as_secs());
}
