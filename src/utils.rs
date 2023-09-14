use std::fmt::Display;

use colored::Colorize;

pub enum ColorEnum {
    Green,
    Red,
}

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

pub fn horizontal_rule() {
    println!("-------------------------------------------");
}

pub fn split_head_from_rest(snek: &str) -> (String, Vec<String>) {
    let mut split_snek = snek.split(' ');
    let head: String = split_snek.next().unwrap().to_owned();
    let remainder: Vec<String> = split_snek.map(|x| x.to_owned()).collect();
    (head, remainder)
}

#[test]
fn test_sneaky_snek_expected() {
    assert_eq!(
        (
            "Foo".to_string(),
            vec!["Bar".to_string(), "Baz".to_string()]
        ),
        split_head_from_rest("Foo Bar Baz")
    );
}

#[test]
fn test_sneaky_snek_single() {
    assert_eq!(("Foo".to_string(), vec![]), split_head_from_rest("Foo"));
}
