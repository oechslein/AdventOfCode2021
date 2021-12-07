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

use num::bigint::{BigUint, ToBigUint};
use num::Zero;

#[macro_use]
extern crate lazy_static;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2 of the day01
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1(utils::file_to_string("input.txt")));
    utils::with_measure("Part 2", || solve_part2(utils::file_to_string("input.txt")));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

type NumberType = i32;

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    let mut positions = content.split(",").map(|n| n.trim().parse::<NumberType>().unwrap()).collect::<Vec::<NumberType>>();

    let middle_index = positions.len() / 2;
    positions.select_nth_unstable(middle_index);
    let median_position = positions[middle_index];

    let result = positions.into_iter().map(|pos: NumberType| (pos - median_position).abs()).sum();

    Ok(result)
}

fn _part2_costs(positions: &[NumberType], goal_position: &NumberType) -> NumberType {
    // cost function: 1 => 1, 2 => 3, 3 => 6, 4 => 10, 5 => 15, f(n) = sum_0..n(n) => f(n) => n(n+1)/2
    positions.iter().map(|pos| {
        let n = (pos - goal_position).abs() as f64;
        (n*(n+1.0)/2.0).round() as NumberType
    }).sum()
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let positions = content.split(",").map(|n| n.trim().parse::<NumberType>().unwrap()).collect::<Vec::<NumberType>>();
    let average_position: NumberType = positions.iter().sum::<NumberType>() / (positions.len() as NumberType);
    let possible_average_positions = vec![average_position-1,average_position,average_position+1];

    let result = possible_average_positions.iter().map(|goal_position| _part2_costs(&positions, goal_position)).min().unwrap();

    Ok(result)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("test.txt")).unwrap(),
            37
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            168
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            344297
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            97164301
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
