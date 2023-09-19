use crate::models::FileRequirement;
use crate::utils;
use std::fs::File;

pub fn check_file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn handle_file_requirement(requirement: FileRequirement) {
    let result = check_file_exists(&requirement.path);

    if !result && requirement.create_if_not_exists.is_some() {
        File::create(&requirement.path).expect("Failed to create file!");
        utils::color_print(
            vec!["Created file: ".to_string(), requirement.path.to_owned()],
            utils::ColorEnum::Green,
        );
    } else if !result {
        utils::color_print(
            vec!["File: ", &requirement.path, " does not exist!"],
            utils::ColorEnum::Red,
        );
        std::process::abort()
    }
}

#[test]
fn test_file_exists() {
    assert!(check_file_exists("Cargo.toml"));
}

#[test]
fn test_no_file_panics() {
    assert!(!check_file_exists("not_a_file"));
}
