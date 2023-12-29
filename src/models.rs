//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::utils::{color_print, ColorEnum};

/// Format for completed executions
#[derive(Serialize, Deserialize, Debug)]
pub struct AllResults {
    pub job_name: String,
    pub execution_time: String,
    pub results: Vec<TaskResult>,
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

/// Schema for Version Requirement Checks
///
/// Runs the specified commands and checks that
/// output is a valid version that falls within
/// the defined minimum and maximum allowed versions.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct VersionRequirement {
    pub command: String,
    pub minimum_version: Option<String>,
    pub maximum_version: Option<String>,
    pub split: Option<bool>, // TODO: Make this name more clear, or let the user choose with item to take after split
}
impl Validate for VersionRequirement {
    fn validate(&self) -> Result<(), ValidationError> {
        let failure_message = format!(
            "> Version Requirement '{}' failed validation!",
            self.command
        );

        // Versions must be valid Semantic Versions
        let versions: Vec<&String> = vec![&self.minimum_version, &self.maximum_version]
            .into_iter()
            .flatten()
            .collect();
        for version in versions.iter() {
            if Version::from_str(version).is_err() {
                color_print(vec![failure_message], ColorEnum::Red);
                return Err(ValidationError {
                    message: "Mininum and Maximum versions must be valid semantic version!"
                        .to_owned(),
                });
            }
        }

        // Make sure that the Maximum version isn't smaller than the Minimum version
        if self.maximum_version.is_some() && self.maximum_version.is_some() {
            let valid_version_constraints =
                VersionReq::from_str(&format!("> {}", self.minimum_version.as_ref().unwrap()))
                    .unwrap()
                    .matches(&Version::from_str(self.maximum_version.as_ref().unwrap()).unwrap());

            if !valid_version_constraints {
                color_print(vec![failure_message], ColorEnum::Red);
                return Err(ValidationError {
                    message: "The Minimum version cannot be larger than the Maximum version!"
                        .to_owned(),
                });
            }
        }

        // If Split is Some, either Min or Max Version must be Some
        if self.split.is_some() && versions.is_empty() {
            color_print(vec![failure_message], ColorEnum::Red);
            return Err(ValidationError {
                message: "If 'split' is defined, either a 'minimum_version' or a 'maximum_version' is also required!"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

/// Schema for File Requirement Checks
///
/// This verifies that the file exists locally,
/// or can be configured to create the file
/// if its missing.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct FileRequirement {
    pub path: String,
    pub create_if_not_exists: Option<bool>,
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
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub tasks: Vec<Task>,
    pub pipelines: Option<Vec<Pipeline>>,
    pub templates: Option<Vec<Template>>,
    pub additional_files: Option<Vec<String>>,
}

impl Validate for RoxFile {
    fn validate(&self) -> Result<(), ValidationError> {
        // Task Validation
        for task in &self.tasks {
            task.validate()?
        }

        // Template Validation
        if let Some(templates) = &self.templates {
            for template in templates {
                template.validate()?
            }
        }

        // Version Requirement Validation
        if let Some(version_requirements) = &self.version_requirements {
            for requirement in version_requirements {
                requirement.validate()?
            }
        }

        Ok(())
    }
}
