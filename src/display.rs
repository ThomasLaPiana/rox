use crate::targets::TargetResult;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

pub fn display_execution_results(results: Vec<TargetResult>) {
    let mut table = Vec::new();

    for result in results {
        table.push(vec![
            result.name.cell(),
            result.result.cell().justify(Justify::Center),
            result.elapsed_time.cell().justify(Justify::Right),
        ])
    }

    assert!(print_stdout(
        table
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "Exit Code".cell().bold(true),
                "Elapsed Time (in secs)".cell().bold(true)
            ])
            .bold(true),
    )
    .is_ok());
}
