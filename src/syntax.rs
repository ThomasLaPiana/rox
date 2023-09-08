// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RoxFile {
    file_requirements: String,
}
