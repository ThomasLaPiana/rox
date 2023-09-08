use crate::syntax;

pub fn load_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the Rox file!")
}
