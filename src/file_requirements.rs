use crate::syntax::FileRequirement;
use crate::utils;

pub fn check_file_exists(path: &str) -> bool {
    println!("Checking for file at path: {}", path);
    std::path::Path::new(path).exists()
}

pub fn handle_file_requirement(requirement: FileRequirement) {
    let result = check_file_exists(&requirement.path);

    if !result && requirement.create_if_not_exists.is_some() {
        println!("> Creating file: {}", &requirement.path);
    } else if !result {
        utils::color_print(
            vec![
                "File: ".to_string(),
                requirement.path.to_owned(),
                " does not exist!".to_string(),
            ],
            utils::ColorEnum::Red,
        );
    } else {
        utils::color_print(
            vec!["File Check succeeded!".to_string()],
            utils::ColorEnum::Green,
        );
    }
}

#[test]
fn test_file_exists() {
    assert_eq!(check_file_exists("Cargo.toml"), true);
}

#[test]
fn test_no_file_panics() {
    assert_eq!(check_file_exists("not_a_file"), false);
}
