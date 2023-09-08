// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VersionRequirements {
    command: String,
    minimum_version: Option<Vec<String>>,
    maximum_version: Option<Vec<String>>,
    versions: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct RoxFile {
    rox_version: f64,
    version_requirements: Vec<VersionRequirements>,
}
