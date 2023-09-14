use crate::targets::{PassFail, TargetResult};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;

pub fn display_execution_results(results: Vec<TargetResult>) {
    let mut table = Vec::new();

    for result in results {
        table.push(vec![
            result.name.cell(),
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
            result.file_path.cell().justify(Justify::Right),
        ])
    }

    assert!(print_stdout(
        table
            .table()
            .title(vec![
                "Name".yellow().cell().bold(true),
                "Result".yellow().cell().bold(true),
                "Elapsed Time (in secs)".yellow().cell().bold(true),
                "File Path".yellow().cell().bold(true),
            ])
            .bold(true),
    )
    .is_ok());
}
