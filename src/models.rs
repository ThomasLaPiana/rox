//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use serde::Deserialize;

// Trait for granular schema validation
pub trait Validate {
    fn validate(&self) -> Self;
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
    fn validate(&self) -> Self {
        let validated_task = self.clone();
        if validated_task.command.is_none() {
            panic!("The task doesn't have a valid command!")
        }
        validated_task
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
    fn validate(&self) -> Self {
        let mut validated_roxfile = self.clone();
        validated_roxfile.tasks = self.tasks.iter().map(|task| task.validate()).collect();
        validated_roxfile
    }
}
