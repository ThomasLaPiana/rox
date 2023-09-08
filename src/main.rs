mod parser;
mod runner;
mod syntax;
mod utils;
use anyhow;

fn main() -> anyhow::Result<()> {
    let file_path = "example_rox.yml".to_string();
    utils::color_print(
        vec![
            "> Loading Roxfile from path: ".to_string(),
            file_path.to_string(),
        ],
        utils::ColorEnum::Yellow,
    );
    let roxfile = parser::parse_file_contents(parser::load_file(file_path));

    if roxfile.version_requirements.is_some() {
        println!("> Checking versions...");
        for version_check in roxfile.version_requirements.unwrap().into_iter() {
            runner::check_version(version_check.clone());
            println!("------------------------------------");
        }
    }
    Ok(())
}
