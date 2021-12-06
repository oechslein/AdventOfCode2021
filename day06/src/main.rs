#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
extern crate test;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::ops::Sub;
use std::path::Path;
use std::str::{FromStr, Split};
use std::time::{Duration, Instant};
use test::Bencher;
use counter::Counter;

use itertools::Itertools;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2 of the day01
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || {
        solve_part1(utils::file_to_string("input.txt"))
    });
    utils::with_measure("Part 2", || {
        solve_part2(utils::file_to_string("input.txt"))
    });
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////
type NumberType = u64;
type FishPopulationType = Counter<NumberType, NumberType>;

fn solve_it(content: String, max_days: NumberType) -> NumberType {
    let mut pop: FishPopulationType = content
    .split(",")
    .map(|n| n.trim().parse::<NumberType>().unwrap())
    .collect();
    //println!("pop: {:?}", pop);

    for _day in 1..=max_days {
        let mut next_day_pop = FishPopulationType::new();
        match pop.remove(&0) {
            Some(breading_fish_amount) => {
                next_day_pop[&6] += breading_fish_amount;
                next_day_pop[&8] += breading_fish_amount;
            },
            None => {}
        };
        for (days_until_bread, frequency) in pop.into_iter() {
            let days_until_bread_minus_one = (*days_until_bread)-1;
            next_day_pop[&days_until_bread_minus_one] += *frequency;
        }
        //println!("day: {:?}, next_day_pop: {:?}", _day, next_day_pop);

        pop = next_day_pop;
    }

   pop.iter().map(|(_, frequency)| *frequency).sum::<NumberType>() as NumberType
}

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    Ok(solve_it(content, 80))
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    Ok(solve_it(content, 256))
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
            5934
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            26984457539
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            5632
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            22213
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
