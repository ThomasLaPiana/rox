use crate::models::CiInfo;
use chrono;
use chrono::{DateTime, Utc};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use git2::Repository;
use octocrab::models::workflows::Conclusion;
use octocrab::params::workflows::Filter;

pub struct RunResult {
    name: String,
    job: String,
    status: Conclusion,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
}
impl RunResult {
    pub fn get_elapsed_time(&self) -> String {
        let elapsed_time = match self.ended_at {
            Some(ended_at) => {
                let started_at = self.started_at.as_ref().unwrap();
                let elapsed_time = ended_at.signed_duration_since(started_at).num_seconds();
                elapsed_time.to_string()
            }
            None => "N/A".to_string(),
        };
        elapsed_time
    }
}

/// Print the execution results in a pretty table format
pub fn display_ci_results(results: &[RunResult]) {
    let mut table = Vec::new();

    results.iter().for_each(|result| {
        let status = match result.status {
            Conclusion::Success => "Success"
                .to_string()
                .green()
                .cell()
                .justify(Justify::Center),
            Conclusion::Failure => "Failure".to_string().red().cell().justify(Justify::Center),
            _ => "Other".to_string().red().cell().justify(Justify::Center),
        };

        table.push(vec![
            result.name.to_owned().cell(),
            result.job.clone().cell().justify(Justify::Center),
            status,
            result.get_elapsed_time().cell().justify(Justify::Center),
        ])
    });

    assert!(print_stdout(
        table
            .table()
            .title(vec![
                "Name".yellow().cell().bold(true),
                "Job".yellow().cell().bold(true),
                "Status".yellow().cell().bold(true),
                "Elapsed Time".yellow().cell().bold(true),
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

    let mut results = Vec::new();

    let jobs = octo_instance
        .list_jobs(workflow.id)
        .per_page(100)
        .page(1u8)
        .filter(Filter::All)
        .send()
        .await
        .unwrap();

    jobs.into_iter().for_each(|job| {
        let steps = job.steps.into_iter().map(|step| RunResult {
            name: step.name,
            job: job.name.clone(),
            status: step.conclusion.unwrap(),
            started_at: step.started_at,
            ended_at: step.completed_at,
        });
        results.extend(steps);
    });

    display_ci_results(&results);
}
