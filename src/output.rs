use crate::models::{AllResults, CiInfo, PassFail};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use git2::Repository;
use octocrab::params::workflows::Filter;

const LOG_DIR: &str = ".rox";

/// Load execution results from a log file
pub fn display_logs(number: &i8) {
    let mut filenames = std::fs::read_dir(LOG_DIR)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    filenames.sort();

    let results = filenames
        .iter()
        .take(*number as usize)
        .map(|filename| {
            let contents = std::fs::read_to_string(filename).unwrap();
            serde_yaml::from_str(&contents).unwrap()
        })
        .collect::<Vec<AllResults>>();

    for result in results.iter() {
        println!("\n> {} | {}", result.job_name, result.execution_time);
        display_execution_results(result)
    }
}

/// Write the execution results to a log file
pub fn write_logs(results: &AllResults) -> String {
    let filename = format!("rox-{}.log.yaml", chrono::Utc::now().to_rfc3339());
    let filepath = format!("{}/{}", LOG_DIR, filename);

    // Make sure the log directory exists
    if !std::path::Path::new(LOG_DIR).exists() {
        std::fs::create_dir(LOG_DIR).unwrap();
    }

    std::fs::write(filepath, serde_yaml::to_string(results).unwrap()).unwrap();
    filename
}

/// Print the execution results in a pretty table format
pub fn display_execution_results(results: &AllResults) {
    let mut table = Vec::new();

    for result in results.results.iter() {
        table.push(vec![
            result.name.to_owned().cell(),
            format!("{}", result.stage).cell().justify(Justify::Center),
            match result.result {
                PassFail::Pass => result
                    .result
                    .to_string()
                    .green()
                    .cell()
                    .justify(Justify::Center),
                PassFail::Fail => result
                    .result
                    .to_string()
                    .red()
                    .cell()
                    .justify(Justify::Center),
            },
            result.elapsed_time.cell().justify(Justify::Center),
        ])
    }

    assert!(print_stdout(
        table
            .table()
            .title(vec![
                "Task".yellow().cell().bold(true),
                "Stage".yellow().cell().bold(true),
                "Result".yellow().cell().bold(true),
                "Run Time (sec)".yellow().cell().bold(true),
            ])
            .bold(true),
    )
    .is_ok());
}

/// Show the most recent CI workflow
pub async fn display_ci_status(ci_info: CiInfo) {
    let repo = Repository::open_from_env().unwrap();
    let head = repo.head().unwrap();
    assert!(head.is_branch());
    let branch = head.name().unwrap().split('/').last().unwrap();
    println!("> Getting CI status for branch: {}", branch);

    let instance = octocrab::instance();
    let octo_instance = instance.workflows(ci_info.repo_owner, ci_info.repo_name);

    let workflow = octo_instance
        .list_all_runs()
        .page(1u32)
        .per_page(1)
        .branch(branch)
        .send()
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    let jobs = octo_instance
        .list_jobs(workflow.id)
        // Optional Parameters
        .per_page(100)
        .page(1u8)
        .filter(Filter::All)
        // Send the request
        .send()
        .await
        .unwrap();

    jobs.into_iter().for_each(|job| {
        println!(
            "{} | {} | {:?} | {}",
            job.id, job.name, job.status, job.started_at
        );

        job.steps.into_iter().for_each(|step| {
            println!(
                "{} | {:?} | {}",
                step.name,
                step.status,
                step.started_at.unwrap()
            )
        })
    });
}
