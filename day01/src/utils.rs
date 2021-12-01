use std::fmt::Debug;
use std::fs;
use std::time::Instant;

////////////////////////////////////////////////////////////////////////////////////
/// Helper functions
pub fn file_to_string(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

pub fn parse_input(contents: String) -> Vec<i32> {
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn get_input(file_name: &str) -> Vec<i32> {
    parse_input(file_to_string(file_name))
}

pub fn with_measure<T: Debug>(title: &str, f: fn() -> Result<T, String>) -> Result<T, String> {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();
    println!(
        "{} result: {:?} Time elapsed is: {:?}\n",
        title,
        res.as_ref()?,
        duration
    );
    res
}
