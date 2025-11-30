use std::fs;

pub fn read_src(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap_or_else(|e| panic!("Failed to read file '{}': {}", filename, e))
}
