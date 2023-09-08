mod files_requirements;
mod parser;
mod syntax;
mod utils;
mod version_requirements;
use anyhow;
use colored::Colorize;
use utils::{color_print, ColorEnum};

fn main() -> anyhow::Result<()> {
    println!("{}", "\t~~~~~~ Rox ~~~~~~".underline().bright_green());
    // Load in the Roxfile(s)
    let file_path = "example_rox.yml".to_string();
    println!("{}{}", "> Loading Roxfile from path: ", file_path);
    let roxfile = parser::parse_file_contents(parser::load_file(file_path));
    color_print(
        vec!["> File loaded successfully!".to_string()],
        ColorEnum::Green,
    );
    utils::horizontal_rule();

    // Check Versions
    if roxfile.version_requirements.is_some() {
        println!("{}", "> Checking versions...");
        for version_check in roxfile.version_requirements.unwrap().into_iter() {
            version_requirements::check_version(version_check.clone());
        }
    }
    utils::horizontal_rule();

    // Check Files
    if roxfile.file_requirements.is_some() {
        for file in roxfile.file_requirements.unwrap().into_iter() {
            files_requirements::check_file_exists(&file);
        }
    }
    Ok(())
}
