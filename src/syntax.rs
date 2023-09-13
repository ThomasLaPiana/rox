/// Contains the Structs for the Syntax of the Rox file
/// Maps 1:1 with the structure of the YAML
use serde::Deserialize;

/// This function builds each target and injects related metadata
pub fn target_builder(target: Target, file_path: String) -> Target {
    Target {
        name: target.name,
        command: target.command,
        description: target.description,
        pre_targets: target.pre_targets,
        post_targets: target.post_targets,
        file_path: Some(file_path),
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct VersionRequirement {
    pub command: String,
    pub minimum_version: Option<String>,
    pub maximum_version: Option<String>,
    pub split: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileRequirement {
    pub path: String,
    pub create_if_not_exists: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    pub command: Option<String>, // This is optional because it is valid if there are pre or post hooks
    pub description: Option<String>,
    pub pre_targets: Option<Vec<String>>,
    pub post_targets: Option<Vec<String>>,
    pub file_path: Option<String>,
}

/// The top-level structure of the Roxfile
#[derive(Deserialize, Debug)]
pub struct RoxFile {
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub always_check_requirements: Option<bool>,
    pub targets: Vec<Target>,
    pub additional_files: Option<Vec<String>>,
}
