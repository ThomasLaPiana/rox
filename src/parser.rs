// All logic related to file loading/reading/parsing
use crate::syntax;
use serde_yaml;

pub fn load_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Roxfile!")
}

pub fn parse_file_contents(contents: String) -> syntax::RoxFile {
    serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!")
}
