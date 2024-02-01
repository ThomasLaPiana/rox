use crate::models::CiInfo;
use chrono::{DateTime, Utc};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use git2::Repository;
use octocrab::models::workflows::Conclusion;
use octocrab::params::workflows::Filter;

pub enum StepStatus {
    Success,
    Failed,
    Skipped,
    InProgress,
    Cancelled,
    Other,
}

impl std::fmt::Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            StepStatus::Success => "Success",
            StepStatus::Failed => "Failed",
            StepStatus::Skipped => "Skipped",
            StepStatus::InProgress => "In Progress",
            StepStatus::Cancelled => "Cancelled",
            StepStatus::Other => "Other",
        };
        write!(f, "{}", message)
    }
}

/// Convert the OctoCrab Conclusion enum to a StepStatus enum
/// for more user-friendly messaging.
pub fn step_conclusion_lookup(conclusion: &Conclusion) -> StepStatus {
    match conclusion {
        Conclusion::Success => StepStatus::Success,
        Conclusion::Failure | Conclusion::TimedOut => StepStatus::Failed,
        Conclusion::Skipped => StepStatus::Skipped,
        Conclusion::Cancelled => StepStatus::Cancelled,
        Conclusion::ActionRequired | Conclusion::Neutral => StepStatus::Other,
        _ => StepStatus::InProgress,
    }
}

pub struct RunResult {
    name: String,
    job: String,
    status: StepStatus,
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
pub fn display_results_table(results: &[RunResult]) {
    let mut table = Vec::new();

    results.iter().for_each(|result| {
        // Convert the StepStatus enum to a string with the correct color
        let status_string = result.status.to_string();
        let status = match result.status {
            StepStatus::Success => "Success"
                .to_string()
                .green()
                .cell()
                .justify(Justify::Center),
            StepStatus::Failed => status_string.red().cell().justify(Justify::Center),
            _ => status_string.yellow().cell().justify(Justify::Center),
        };

        // Add a row to the table
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
                "Elapsed Time (sec)".yellow().cell().bold(true),
            ])
            .bold(true),
    )
    .is_ok());
}

/// Show the most recent CI workflow
pub async fn display_ci_status(ci_info: CiInfo) {
    // Configure Git and retrieve repo info
    let repo = Repository::open_from_env().unwrap();
    let head = repo.head().unwrap();
    assert!(head.is_branch());
    let branch = head.name().unwrap().split('/').last().unwrap();
    println!("> Getting CI status for branch: {}", branch);

    // Build an Authenticated GitHub Client
    let token = std::env::var(ci_info.token_env_var).expect("Failed to get token from env var!");
    let octo_instance = octocrab::OctocrabBuilder::new()
        .personal_token(token)
        .build()
        .unwrap();

    // Verify that the client is authorized
    if octo_instance.current().user().await.is_err() {
        panic!("GitHub client is not authorized!");
    }

    let workflow_instance = octo_instance.workflows(ci_info.repo_owner, ci_info.repo_name);
    let workflow = workflow_instance
        .list_all_runs()
        .page(1u32)
        .per_page(1)
        .branch(branch)
        .send()
        .await
        .expect("Failed to retrieve workflow data!")
        .into_iter()
        .next()
        .unwrap();

    let results: Vec<RunResult> = workflow_instance
        .list_jobs(workflow.id)
        .per_page(100)
        .page(1u8)
        .filter(Filter::All)
        .send()
        .await
        .unwrap()
        .into_iter()
        .flat_map(|job| {
            let results: Vec<RunResult> = job
                .steps
                .into_iter()
                .map(|step| RunResult {
                    name: step.name,
                    job: job.name.clone(),
                    status: if step.conclusion.is_none() {
                        StepStatus::InProgress
                    } else {
                        step_conclusion_lookup(step.conclusion.as_ref().unwrap())
                    },
                    started_at: step.started_at,
                    ended_at: step.completed_at,
                })
                .collect();
            results
        })
        .collect();

    display_results_table(&results);
}
