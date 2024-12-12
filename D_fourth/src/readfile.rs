use std::fs;

pub fn readfile(filepath: &str) -> Vec<String> {
    fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}