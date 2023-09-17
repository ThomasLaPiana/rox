//! Contains the Structs for the Schema of the Roxfile
//! as well as the validation logic.
use serde::Deserialize;

/// Load a file into a String
pub fn load_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Roxfile!")
}

/// Parse a Roxfile into Rust structs
pub fn parse_file_contents(contents: String) -> RoxFile {
    serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!")
}

/// Rebuilds each Task with injected metadata
pub fn task_builder(task: Task, file_path: String) -> Task {
    Task {
        name: task.name,
        command: task.command,
        description: task.description,
        pre_tasks: task.pre_tasks,
        post_tasks: task.post_tasks,
        file_path: Some(file_path),
        symbols: task.symbols,
        substitutions: task.substitutions,
        hide: task.hide,
    }
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
    pub split: Option<bool>,
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
    pub command: Option<String>, // This is optional because it is valid if there are pre or post hooks
    pub description: Option<String>,
    pub pre_tasks: Option<Vec<String>>,
    pub post_tasks: Option<Vec<String>>,
    pub file_path: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub substitutions: Option<Vec<Substitution>>,
    pub hide: Option<bool>,
}

/// The name and set of values to inject into the command,
/// based on the `symbols` field.
#[derive(Deserialize, Debug, Clone)]
pub struct Substitution {
    pub name: String,
    pub values: Vec<String>,
    pub description: Option<String>,
}

/// The top-level structure of the Roxfile
#[derive(Deserialize, Debug, Default)]
pub struct RoxFile {
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub tasks: Vec<Task>,
    pub additional_files: Option<Vec<String>>,
}
