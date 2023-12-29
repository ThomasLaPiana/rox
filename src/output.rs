use crate::models::{AllResults, PassFail};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;

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
