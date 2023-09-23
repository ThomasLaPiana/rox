mod execution;
mod file_requirements;
mod models;
mod utils;
mod version_requirements;
use std::collections::HashMap;

use crate::execution::execute_tasks;
mod cli;
mod output;
use std::error::Error;

type RoxResult<T> = Result<T, Box<dyn Error>>;

/// Inject additional metadata into each Pipeline and sort based on name.
fn inject_pipeline_metadata(
    pipelines: Vec<models::Pipeline>,
    file_path: &str,
) -> Vec<models::Pipeline> {
    let mut sorted_pipelines: Vec<models::Pipeline> = pipelines
        .into_iter()
        .map(|mut pipeline| {
            pipeline.file_path = Some(file_path.to_owned());
            pipeline
        })
        .collect();
    sorted_pipelines.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    sorted_pipelines
}

/// Get used Template's information and inject set values
fn inject_template_values(mut task: models::Task, template: &models::Template) -> models::Task {
    task.command = {
        let mut template_command = template.command.clone();
        let template_symbols = template.symbols.clone();
        let task_values = task.values.clone().unwrap();

        for i in 0..task_values.len() {
            template_command = template_command.replace(
                template_symbols.get(i).unwrap(),
                task_values.get(i).unwrap(),
            );
        }
        Some(template_command)
    };
    task
}

#[test]
fn inject_template_values_valid() {
    let test_task = models::Task {
        name: "Test".to_string(),
        command: None,
        file_path: None,
        uses: None,
        values: Some(vec!["1".to_owned(), "2".to_owned()]),
        description: None,
        hide: None,
        workdir: None,
    };
    let test_template = models::Template {
        name: "Test".to_string(),
        command: "This is {one} and {two}".to_owned(),
        symbols: vec!["{one}".to_owned(), "{two}".to_owned()],
    };
    let output_task = inject_template_values(test_task, &test_template);
    assert_eq!(output_task.command.unwrap(), "This is 1 and 2".to_owned())
}

/// Inject additional metadata into each Task and sort based on name.
fn inject_task_metadata(tasks: Vec<models::Task>, file_path: &str) -> Vec<models::Task> {
    let mut sorted_tasks: Vec<models::Task> = tasks
        .into_iter()
        .map(|mut task| {
            task.file_path = Some(file_path.to_owned());
            task
        })
        .collect();
    sorted_tasks.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    sorted_tasks
}

// Entrypoint for the Crate CLI
pub fn rox() -> RoxResult<()> {
    let start = std::time::Instant::now();

    // Load in the Roxfile(s)
    let file_path = "roxfile.yml".to_string();
    let roxfile = utils::parse_file_contents(utils::load_file(&file_path));
    utils::horizontal_rule();

    // Build the CLI, including the various dynamically generated subcommands
    let mut cli = cli::cli_builder();

    let tasks = inject_task_metadata(roxfile.tasks, &file_path);
    let task_subcommands = cli::build_task_subcommands(&tasks);
    cli = cli.subcommands(vec![task_subcommands]);

    if let Some(pipelines) = roxfile.pipelines.clone() {
        let sorted_pipelines = inject_pipeline_metadata(pipelines, &file_path);
        let pipeline_subcommands = cli::build_pipeline_subcommands(&sorted_pipelines);
        cli = cli.subcommands(vec![pipeline_subcommands]);
    }
    let cli_matches = cli.get_matches();

    // Run File and Version checks
    if !cli_matches.get_flag("skip-checks") {
        // Check Versions
        if roxfile.version_requirements.is_some() {
            for version_check in roxfile.version_requirements.into_iter().flatten() {
                version_requirements::check_version(version_check.clone());
            }
        }

        // Check Files
        if roxfile.file_requirements.is_some() {
            for requirement in roxfile.file_requirements.into_iter().flatten() {
                file_requirements::handle_file_requirement(requirement);
            }
        }
    }

    // Build Hashmaps for Tasks, Templates and Pipelines
    let template_map: HashMap<String, models::Template> = std::collections::HashMap::from_iter(
        roxfile
            .templates
            .into_iter()
            .flatten()
            .map(|template| (template.name.clone(), template)),
    );
    let task_map: HashMap<String, models::Task> = std::collections::HashMap::from_iter(
        tasks
            .into_iter()
            .map(|task| match task.uses.clone() {
                Some(task_use) => {
                    inject_template_values(task, template_map.get(&task_use).unwrap())
                }
                None => task,
            })
            .map(|task| (task.name.clone(), task)),
    );
    let pipeline_map: HashMap<String, models::Pipeline> = std::collections::HashMap::from_iter(
        roxfile
            .pipelines
            .into_iter()
            .flatten()
            .map(|pipeline| (pipeline.name.clone(), pipeline)),
    );

    // Execute the Task(s)
    let results = match cli_matches.subcommand_name().unwrap() {
        "pl" => {
            // Deconstruct the CLI commands and get the Pipeline object that was called
            let (_, args) = cli_matches.subcommand().unwrap();
            let pipeline_name = args.subcommand_name().unwrap();
            let parallel = args.get_flag("parallel");
            execute_tasks(
                pipeline_map.get(pipeline_name).unwrap().tasks.clone(),
                &task_map,
                parallel,
            )
        }
        "task" => {
            let (_, args) = cli_matches.subcommand().unwrap();
            let task_name = args.subcommand_name().unwrap().to_owned();
            execute_tasks(vec![task_name], &task_map, false)
        }
        &_ => std::process::abort(),
    };
    // TODO: Non-zero exit if any task failed
    output::display_execution_results(results);

    println!("> Total elapsed time: {}s", start.elapsed().as_secs());
    Ok(())
}
