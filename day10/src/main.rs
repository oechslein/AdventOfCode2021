#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]

extern crate test;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::error;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::mem;
use std::ops::Sub;
use std::path::Path;
use std::str::{FromStr, Split};
use std::time::{Duration, Instant};

use counter::Counter;
use itertools::Itertools;
use test::Bencher;

#[macro_use]
extern crate lazy_static;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1(utils::file_to_string("input.txt")));
    utils::with_measure("Part 2", || solve_part2(utils::file_to_string("input.txt")));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

type NumberType = usize;

fn _parse_content(content: String) -> Vec<String> {
    content
        .split("\r\n")
        .filter(|substr| !substr.is_empty())
        .map(String::from)
        .collect()
}

lazy_static! {
    static ref BRACKETS: Vec<String> = vec!["()".to_string(), "[]".to_string(), "{}".to_string(), "<>".to_string()];
}

fn remove_pairs(s: &String) -> String {
    let mut s = s.clone();

    let mut old_len = 0;
    while old_len != s.len() {
        old_len = s.len();
        for pair in BRACKETS.iter() {
            s = s.replace(pair, "");
        }
    }
    s
}

fn _get_corrupted_char(s: &String) -> Option<char> {
    for c in remove_pairs(s).chars() {
        for pair in BRACKETS.iter() {
            let closing_char = pair.chars().nth(1).unwrap();
            if c == closing_char {
                return Some(closing_char);
            }
        }
    }

    None
}


fn _get_value_part1(c: Option<char>) -> NumberType {
    match c {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        None => 0,
        _ => { panic!() }
    }
}

/// The part1 function calculates the result for part2
fn solve_part1(content: String) -> Result<NumberType, String> {
    Ok(_parse_content(content)
        .iter()
        .map(_get_corrupted_char)
        .map(_get_value_part1)
        .sum())
}

fn _is_corrupted_line(s: &String) -> bool {
    _get_corrupted_char(&s).is_some()
}

fn _replace_opening_with_closing_char(c: char) -> char {
    for pair in BRACKETS.iter() {
        let opening_char = pair.chars().nth(0).unwrap();
        let closing_char = pair.chars().nth(1).unwrap();
        if c == opening_char {
            return closing_char;
        }
    }
    panic!();
}

fn _solve_part2_line(s: &String) -> String {
    return remove_pairs(s).chars().rev().map(_replace_opening_with_closing_char).collect();
}


fn _get_value_part2(s: String) -> NumberType {
    s.chars()
        .map(|c | ")]}>".to_string().find(c).unwrap()+1)
        .reduce(|accum, item|  5 * accum + item).unwrap()
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let result: Vec<NumberType> = _parse_content(content)
        .iter()
        .filter(|line| !_is_corrupted_line(line))
        .map(|line| _get_value_part2(_solve_part2_line(line)))
        .sorted()
        .collect();
    Ok(result[result.len() / 2])
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(_get_corrupted_char(&String::from("{()()()>")), Some('>'));
        assert_eq!(_get_corrupted_char(&String::from("(])")), Some(']'));
        assert_eq!(_get_corrupted_char(&String::from("(((()))}")), Some('}'));
        assert_eq!(_get_corrupted_char(&String::from("<([]){()}[{}])")), Some(')'));

        assert_eq!(_get_corrupted_char(&String::from("{([(<{}[<>[]}>{[]{[(<()>")), Some('}'));

        assert_eq!(_solve_part2_line(&String::from("[({(<(())[]>[[{[]{<()<>>")), "}}]])})]");
        assert_eq!(_solve_part2_line(&String::from("[(()[<>])]({[<{<<[]>>(")), ")}>]})");
        assert_eq!(_solve_part2_line(&String::from("(((({<>}<{<{<>}{[]{[]{}")), "}}>}>))))");

        assert_eq!(_get_value_part2("}}]])})]".to_string()), 288957);
        assert_eq!(_get_value_part2(")}>]})".to_string()), 5566);
        assert_eq!(_get_value_part2("}}>}>))))".to_string()), 1480781);
        assert_eq!(_get_value_part2("]]}}]}]}>".to_string()), 995444);
        assert_eq!(_get_value_part2("])}>".to_string()), 294);

        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1(utils::file_to_string("test.txt")).unwrap(), 26397);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            469755
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            288957
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            2762335572
        );
        Ok(())
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1(utils::file_to_string("input.txt")));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2(utils::file_to_string("input.txt")));
    }
}
