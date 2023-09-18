//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use serde::Deserialize;

/// Load a file into a String
pub fn load_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Roxfile!")
}

/// Parse a Roxfile into Rust structs
pub fn parse_file_contents(contents: String) -> RoxFile {
    serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!")
}

/// Schema for Version Requirement Checks
///
/// Runs the specified commands and checks that
/// output is a valid version that falls within
/// the defined minimum and maximum allowed versions.
#[derive(Deserialize, Debug, Clone, Default)]
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
pub struct FileRequirement {
    pub path: String,
    pub create_if_not_exists: Option<bool>,
}

/// Schema for Tasks in the Roxfile
///
/// Tasks are discrete units of execution
/// that send commands to the shell.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Task {
    pub name: String,
    pub command: Option<String>,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub uses: Option<String>,
    pub values: Option<Vec<String>>,
    pub hide: Option<bool>,
}

/// Schema for Templates
///
/// Templates are injectable commands that
/// can be used by tasks.
#[derive(Deserialize, Debug, Default)]
pub struct Template {
    pub name: String,
    pub command: String,
    pub symbols: Vec<String>,
}

/// Schema for Pipelines
///
/// Pipelines are collections of tasks.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Pipeline {
    pub name: String,
    pub description: Option<String>,
    pub tasks: Vec<String>,
    pub file_path: Option<String>,
}

/// The top-level structure of the Roxfile
#[derive(Deserialize, Debug, Default)]
pub struct RoxFile {
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub tasks: Vec<Task>,
    pub pipelines: Option<Vec<Pipeline>>,
    pub templates: Option<Vec<Template>>,
    pub additional_files: Option<Vec<String>>,
}
