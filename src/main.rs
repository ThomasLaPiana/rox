mod parser;
mod runner;
mod syntax;
use anyhow;

fn main() -> anyhow::Result<()> {
    let file_path = "example_rox.yml".to_string();
    let roxfile = parser::parse_file_contents(parser::load_file(file_path));
    runner::check_version(roxfile.version_requirements[0].clone());
    Ok(())
}
