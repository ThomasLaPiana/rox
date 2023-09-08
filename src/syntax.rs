// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct VersionRequirements {
    pub command: String,
    pub minimum_version: Option<Vec<String>>,
    pub maximum_version: Option<Vec<String>>,
    pub versions: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct RoxFile {
    pub rox_version: f64,
    pub version_requirements: Vec<VersionRequirements>,
}
