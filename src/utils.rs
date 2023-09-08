use colored::Colorize;

pub enum ColorEnum {
    Green,
    Yellow,
    Red,
}

pub fn color_print(outputs: Vec<String>, color: ColorEnum) {
    let concat_output = outputs
        .iter()
        .fold("".to_string(), |x, y| format!("{}{}", x, y));

    match color {
        ColorEnum::Green => println!("{}", concat_output.green()),
        ColorEnum::Red => println!("{}", concat_output.red()),
        ColorEnum::Yellow => println!("{}", concat_output.yellow()),
    }
}

pub fn split_head_from_rest(snek: String) -> (String, Vec<String>) {
    let mut split_snek = snek.split(" ");
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
        split_head_from_rest("Foo Bar Baz".to_string())
    );
}

#[test]
fn test_sneaky_snek_single() {
    assert_eq!(
        ("Foo".to_string(), vec![]),
        split_head_from_rest("Foo".to_string())
    );
}
