use crate::execution::{PassFail, TaskResult};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;

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
                result.file_path.to_owned().cell().justify(Justify::Right),
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
                "Run Time(s)".yellow().cell().bold(true),
                "File".yellow().cell().bold(true),
            ])
            .bold(true),
    )
    .is_ok());
}
