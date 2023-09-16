/// Contains the Structs for the Syntax of the Roxfile
/// Maps 1:1 with the structure of the YAML
use serde::Deserialize;

pub fn load_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Roxfile!")
}

pub fn parse_file_contents(contents: String) -> RoxFile {
    serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!")
}

/// This function builds each task and injects related metadata
pub fn task_builder(task: Task, file_path: String) -> Task {
    Task {
        name: task.name,
        command: task.command,
        description: task.description,
        pre_tasks: task.pre_tasks,
        post_tasks: task.post_tasks,
        file_path: Some(file_path),
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct VersionRequirement {
    pub command: String,
    pub minimum_version: Option<String>,
    pub maximum_version: Option<String>,
    pub split: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FileRequirement {
    pub path: String,
    pub create_if_not_exists: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Task {
    pub name: String,
    pub command: Option<String>, // This is optional because it is valid if there are pre or post hooks
    pub description: Option<String>,
    pub pre_tasks: Option<Vec<String>>,
    pub post_tasks: Option<Vec<String>>,
    pub file_path: Option<String>,
}

/// The top-level structure of the Roxfile
#[derive(Deserialize, Debug, Default)]
pub struct RoxFile {
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub always_check_requirements: Option<bool>,
    pub tasks: Vec<Task>,
    pub additional_files: Option<Vec<String>>,
}
