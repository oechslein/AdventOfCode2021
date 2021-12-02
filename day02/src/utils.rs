use std::fmt::Debug;
use std::fs;
use std::time::Instant;

/// Reads a file and return its content as a string
pub fn file_to_string(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

/// Splits given String, trim each lines, filters empty lines and parse each line into wished type
pub fn parse_input_items<T>(contents: String) -> Vec<T>
where
    T: std::str::FromStr,
    <T>::Err: Debug, // Not sure why this is needed
{
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

pub fn parse_input2(contents: String) -> Vec<(String, u32)> {
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|substring| {
            let mylist: Vec<&str> = substring.split(' ').collect();
            (String::from(mylist[0]), mylist[1].parse::<u32>().unwrap())
        })
        .collect()
}

pub fn parse_input(contents: String) -> Vec<Vec<String>> {
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|substring| substring.split(' ').map(|x| String::from(x)).collect())
        .collect()
}

/// Runs given function, prints result and used duration
pub fn with_measure<T: Debug>(title: &str, f: fn() -> Result<T, String>) -> Result<T, String> {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();
    println!(
        "{} result: {:?} (elapsed time is: {:?})",
        title,
        res.as_ref()?,
        duration
    );
    res
}
