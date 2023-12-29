use crate::execution::{PassFail, TaskResult};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct StageResults {
    stage_number: i8,
    results: Vec<TaskResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogFormat {
    job_name: String,
    stages: Vec<StageResults>,
}

pub fn format_log_results(subcommand_name: &str, results: &[Vec<TaskResult>]) -> String {
    let log_dir = ".rox";
    let filename = format!("rox-{}.log", chrono::Utc::now().to_rfc3339());
    let filepath = format!("{}/{}", log_dir, filename);

    // Make sure the log directory exists
    if !std::path::Path::new(log_dir).exists() {
        std::fs::create_dir(log_dir).unwrap();
    }

    let all_results = results
        .iter()
        .enumerate()
        .map(|(i, x)| StageResults {
            stage_number: i as i8,
            results: x.to_vec(),
        })
        .collect();

    let logs = LogFormat {
        job_name: subcommand_name.to_string(),
        stages: all_results,
    };

    std::fs::write(filepath, serde_yaml::to_string(&logs).unwrap()).unwrap();
    filename
}

pub fn display_execution_results(results: &[Vec<TaskResult>]) {
    let mut table = Vec::new();

    for (i, stage_results) in results.iter().enumerate() {
        for result in stage_results {
            table.push(vec![
                result.name.to_owned().cell(),
                format!("{}", i + 1).cell().justify(Justify::Center),
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
                result.command.to_owned().cell().justify(Justify::Right),
            ])
        }
    }

    assert!(print_stdout(
        table
            .table()
            .title(vec![
                "Task".yellow().cell().bold(true),
                "Stage".yellow().cell().bold(true),
                "Result".yellow().cell().bold(true),
                "Run Time (sec)".yellow().cell().bold(true),
                "Command".yellow().cell().bold(true),
            ])
            .bold(true),
    )
    .is_ok());
}
