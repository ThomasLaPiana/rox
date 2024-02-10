//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use crate::logs;
use crate::modules::execution::model_injection::{inject_task_metadata, inject_template_values};
use crate::modules::execution::output;
use crate::utils::{color_print, ColorEnum};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CiInfo {
    pub provider: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub token_env_var: String,
}

/// Format for completed executions
#[derive(Serialize, Deserialize, Debug)]
pub struct JobResults {
    pub job_name: String,
    pub execution_time: String,
    pub results: Vec<TaskResult>,
}

impl JobResults {
    pub fn log_results(&self) {
        let log_path = logs::write_logs(self);
        println!("> Log file written to: {}", log_path);
    }

    pub fn display_results(&self) {
        output::display_execution_results(self);
    }

    /// Triggers a non-zero exit if any job failed, otherwise passes.
    pub fn check_results(&self) {
        // TODO: Figure out a way to get this info without looping again
        self.results.iter().for_each(|result| {
            if result.result == PassFail::Fail {
                std::process::exit(2)
            }
        });
    }
}

/// Enum for task command status
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum PassFail {
    Pass,
    Fail,
}
impl std::fmt::Display for PassFail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DocsKind {
    Markdown,
    Text,
    URL,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Docs {
    pub name: String,
    pub description: Option<String>,
    pub kind: DocsKind,
    pub path: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct TaskResult {
    pub name: String,
    pub command: String,
    pub stage: i8,
    pub result: PassFail,
    pub elapsed_time: i64,
    pub file_path: String,
}

// Create a custom Error type for Validation
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}
impl Default for ValidationError {
    fn default() -> Self {
        ValidationError {
            message: String::from("Error: Roxfile syntax is invalid!"),
        }
    }
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ValidationError {}

// Trait for granular schema validation
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Schema for Tasks in the Roxfile
///
/// Tasks are discrete units of execution
/// that send commands to the shell.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub name: String,
    pub command: Option<String>,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub uses: Option<String>,
    pub values: Option<Vec<String>>,
    pub hide: Option<bool>,
    pub workdir: Option<String>,
}

impl Validate for Task {
    fn validate(&self) -> Result<(), ValidationError> {
        let task_fail_message = format!("> Task '{}' failed validation!", self.name);

        // Command and Uses cannot both be none
        if self.command.is_none() & self.uses.is_none() {
            color_print(vec![task_fail_message], ColorEnum::Red);
            return Err(ValidationError {
                message: "A Task must implement either 'command' or 'uses'!".to_owned(),
            });
        }

        // Command and Uses cannot both be Some
        if self.uses.is_some() & self.command.is_some() {
            color_print(vec![task_fail_message], ColorEnum::Red);
            return Err(ValidationError {
                message: "A Task cannot implement both 'command' & 'uses'!".to_owned(),
            });
        }

        // If Uses is Some, Values must also be Some
        if self.uses.is_some() & self.values.is_none() {
            color_print(vec![task_fail_message], ColorEnum::Red);
            return Err(ValidationError {
                message: "A Task that implements 'uses' must also implement 'values'!".to_owned(),
            });
        }

        // If Uses is None, Values must also be None
        if self.uses.is_none() & self.values.is_some() {
            color_print(vec![task_fail_message], ColorEnum::Red);
            return Err(ValidationError {
                message: "A Task that implements 'values' must also implement 'uses'!".to_owned(),
            });
        }

        Ok(())
    }
}

/// Schema for Templates
///
/// Templates are injectable commands that
/// can be used by tasks.
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Template {
    pub name: String,
    pub command: String,
    pub symbols: Vec<String>,
}
impl Validate for Template {
    fn validate(&self) -> Result<(), ValidationError> {
        let failure_message = format!("> Template '{}' failed validation!", self.name);

        // All of the 'Symbol' items must exist within the 'Command'
        for symbol in &self.symbols {
            let exists = self.command.contains(symbol);
            if !exists {
                color_print(vec![failure_message], ColorEnum::Red);
                return Err(ValidationError {
                    message: "A Template's 'symbols' must all exist within its 'command'!"
                        .to_owned(),
                });
            }
        }

        Ok(())
    }
}

/// Schema for Pipelines
///
/// Pipelines are collections of tasks.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<Vec<String>>,
}

/// The top-level structure of the Roxfile
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct RoxFile {
    pub ci: Option<CiInfo>,
    pub docs: Option<Vec<Docs>>,
    pub tasks: Vec<Task>,
    pub pipelines: Option<Vec<Pipeline>>,
    pub templates: Option<Vec<Template>>,
}

impl RoxFile {
    /// Create a new instance of RoxFile from a file path and
    /// run all additional validation and metadata injection.
    pub fn build(file_path: &str) -> Result<Self> {
        let file_string = std::fs::read_to_string(file_path)?;
        let mut roxfile: RoxFile = serde_yaml::from_str(&file_string)?;

        // Templates
        let _ = roxfile
            .templates
            .iter()
            .flatten()
            .try_for_each(|template| template.validate());
        let template_map: HashMap<String, &Template> = std::collections::HashMap::from_iter(
            roxfile
                .templates
                .as_deref()
                .into_iter()
                .flatten()
                .map(|template| (template.name.to_owned(), template)),
        );

        // Tasks
        let _ = roxfile.tasks.iter().try_for_each(|task| task.validate());
        roxfile.tasks = inject_task_metadata(roxfile.tasks, file_path);
        roxfile.tasks = roxfile
            .tasks
            .into_iter()
            .map(|task| match task.uses.to_owned() {
                Some(task_use) => {
                    inject_template_values(task, template_map.get(&task_use).unwrap())
                }
                None => task,
            })
            .collect();

        Ok(roxfile)
    }
}
