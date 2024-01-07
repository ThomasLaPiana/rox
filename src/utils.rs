//! Utility Functions
use std::fmt::Display;

use colored::Colorize;

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
