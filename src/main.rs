mod parser;
mod runner;
mod syntax;
mod utils;
use anyhow;
use utils::{color_print, ColorEnum};

fn main() -> anyhow::Result<()> {
    // Load in the Roxfile(s)
    let file_path = "example_rox.yml".to_string();
    color_print(
        vec![
            "> Loading Roxfile from path: ".to_string(),
            file_path.to_string(),
        ],
        ColorEnum::Yellow,
    );
    let roxfile = parser::parse_file_contents(parser::load_file(file_path));
    color_print(
        vec!["> File loaded successfully!".to_string()],
        ColorEnum::Green,
    );

    // Check Versions
    if roxfile.version_requirements.is_some() {
        color_print(
            vec!["> Checking versions...".to_string()],
            ColorEnum::Yellow,
        );
        for version_check in roxfile.version_requirements.unwrap().into_iter() {
            runner::check_version(version_check.clone());
            println!("------------------------------------");
        }
    }
    Ok(())
}
