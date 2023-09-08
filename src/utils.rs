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
        _ => (),
    }
}
