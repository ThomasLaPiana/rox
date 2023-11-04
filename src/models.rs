//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use serde::Deserialize;
use std::error::Error;
use std::fmt;

use crate::utils::{color_print, ColorEnum};

// Create a custom Error type for Validation
#[derive(Debug, Clone)]
pub struct ValidationError {}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Roxfile parsing/validation failed!")
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
        let task_fail_message = format!("> Task '{}' failed validation with message:", self.name);

        if self.command.is_none() & self.uses.is_none() {
            color_print(vec![task_fail_message], ColorEnum::Red);
            color_print(
                vec!["A task must have either a 'command' or 'uses' a template!"],
                ColorEnum::Red,
            );
            return Err(ValidationError {});
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

/// Schema for Pipelines
///
/// Pipelines are collections of tasks.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<Vec<String>>,
    pub file_path: Option<String>,
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
        let roxfile = self.clone();
        for task in roxfile.tasks {
            task.validate()?
        }
        Ok(())
    }
}
