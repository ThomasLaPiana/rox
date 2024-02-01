use crate::models::{JobResults, PassFail};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;

/// Print the execution results in a pretty table format
pub fn display_execution_results(results: &JobResults) {
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
