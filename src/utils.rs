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
    serde_yaml::from_str(&contents).expect("Failed to parse the Roxfile!")
}

pub enum ColorEnum {
    Green,
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
        ColorEnum::Green => println!("{}", concat_output.green()),
        ColorEnum::Red => println!("{}", concat_output.red()),
    }
}

/// A reusable println! to serve as a visual break
pub fn horizontal_rule() {
    println!("-------------------------------------------");
}

/// Split a string on spaces and return the head + the remainder
pub fn split_head_from_rest(snek: &str) -> (String, Vec<String>) {
    let mut split_snek = snek.split(' ');
    let head: String = split_snek.next().unwrap().to_owned();
    let remainder: Vec<String> = split_snek.map(|x| x.to_owned()).collect();
    (head, remainder)
}

#[test]
fn split_head_valid() {
    assert_eq!(
        (
            "Foo".to_string(),
            vec!["Bar".to_string(), "Baz".to_string()]
        ),
        split_head_from_rest("Foo Bar Baz")
    );
}

#[test]
fn split_head_single() {
    assert_eq!(("Foo".to_string(), vec![]), split_head_from_rest("Foo"));
}
