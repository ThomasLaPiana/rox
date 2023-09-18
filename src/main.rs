mod file_requirements;
mod models;
mod task_runner;
mod utils;
mod version_requirements;
use std::collections::HashMap;
use utils::{color_print, ColorEnum};
mod cli;
mod output;

// Entrypoint for the Crate CLI
pub fn main() {
    let start = std::time::Instant::now();

    // Load in the Roxfile(s)
    let file_path = "roxfile.yml".to_string();
    println!("> Loading Roxfile from path: {}", file_path);
    let roxfile = models::parse_file_contents(models::load_file(&file_path));
    color_print(vec!["> File loaded successfully!"], ColorEnum::Green);
    utils::horizontal_rule();

    // Build the CLI
    let mut cli = cli::cli_builder();

    // Build, Sort and add Tasks
    let mut sorted_tasks: Vec<models::Task> = roxfile
        .tasks
        .clone()
        .into_iter()
        .map(|mut task| {
            task.file_path = Some(file_path.clone());
            task
        })
        .collect();
    sorted_tasks.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    let task_subcommands = cli::build_task_subcommands(&sorted_tasks);
    cli = cli.subcommands(vec![task_subcommands]);

    // Build, Sort and add Pipelines
    if let Some(pipelines) = roxfile.pipelines {
        let mut sorted_pipelines: Vec<models::Pipeline> = pipelines
            .clone()
            .into_iter()
            .map(|mut pipeline| {
                pipeline.file_path = Some(file_path.clone());
                pipeline
            })
            .collect();
        sorted_pipelines.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
        let pipeline_subcommands = cli::build_pipeline_subcommands(&sorted_pipelines);
        cli = cli.subcommands(vec![pipeline_subcommands]);
    }
    let cli_matches = cli.get_matches();

    // Run File and Version checks
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

    // Build a HashMap of the task names and their objects
    // let task_map: HashMap<String, models::Task> = std::collections::HashMap::from_iter(
    //     tasks.into_iter().map(|task| (task.name.clone(), task)),
    // );

    // Execute the Tasks
    // let parallel = cli_matches.get_flag("parallel");
    // let task_stuff = task_map
    //     .get(cli_matches.subcommand_name().unwrap())
    //     .unwrap();
    // let results = task_runner::execute_tasks(task_stuff.to_owned(), &task_map, parallel);
    // output::display_execution_results(results);

    println!("> Total elapsed time: {}s", start.elapsed().as_secs());
}
