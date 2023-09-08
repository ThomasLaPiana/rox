mod parser;
mod syntax;
use anyhow;

fn main() -> anyhow::Result<()> {
    let file_path = "example_rox.yml".to_string();
    let loaded_file = parser::load_file(file_path);
    println!("{}", loaded_file);
    Ok(())
}
