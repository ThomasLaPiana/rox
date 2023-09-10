// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

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
    pub command: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RoxFile {
    pub version_requirements: Option<Vec<VersionRequirement>>,
    pub file_requirements: Option<Vec<FileRequirement>>,
    pub always_check_requirements: Option<bool>,
    pub targets: Option<Vec<Target>>,
    pub additional_files: Option<Vec<String>>,
}
