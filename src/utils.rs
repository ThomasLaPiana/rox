//! Utility Functions
use crate::models::RoxFile;
use std::fmt::Display;

use colored::Colorize;

/// Load a file into a String
pub fn load_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Roxfile!")
}

/// Parse a Roxfile into Rust structs
pub fn parse_file_contents(contents: String) -> RoxFile {
    let roxfile: RoxFile = serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!");
    roxfile
}

pub enum ColorEnum {
    Red,
}

/// Print a string as a certain color
pub fn color_print<T>(outputs: Vec<T>, color: ColorEnum)
where
    T: Display + Into<String>,
{
    let concat_output = outputs
        .iter()
        .fold("".to_string(), |x, y| format!("{}{}", x, y));

    match color {
        ColorEnum::Red => println!("{}", concat_output.red()),
    }
}

/// A reusable println! to serve as a visual break
pub fn print_horizontal_rule() {
    println!("-------------------------------------------");
}
