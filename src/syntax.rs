// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct VersionRequirements {
    pub command: String,
    pub minimum_version: Option<String>,
    pub maximum_version: Option<String>,
    pub split: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RoxFile {
    pub rox_version: f64,
    pub version_requirements: Option<Vec<VersionRequirements>>,
    pub file_requirements: Option<Vec<String>>,
}
