// Contains the Structs for the Syntax of the Rox file
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoxFile {
    rox_version: f64,
}
