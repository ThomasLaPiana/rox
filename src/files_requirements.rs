use crate::utils;

pub fn check_file_exists(path: &str) {
    let result = std::path::Path::new(path).exists();

    if !result {
        utils::color_print(
            vec![
                "File: ".to_string(),
                path.to_owned(),
                " does not exist!".to_string(),
            ],
            utils::ColorEnum::Red,
        );
        panic!()
    }
}

#[test]
fn test_file_exists() {
    check_file_exists("Cargo.toml")
}

#[test]
#[should_panic]
fn test_no_file_panics() {
    check_file_exists("not_a_file")
}
