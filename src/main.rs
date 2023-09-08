mod parser;
mod runner;
mod syntax;
use anyhow;

fn main() -> anyhow::Result<()> {
    let file_path = "example_rox.yml".to_string();
    println!("> Loading Roxfile from path: {}", file_path);
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
