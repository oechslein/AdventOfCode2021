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
type NumberType = BigUint;

fn solve_it_slow(content: String, max_days: u32) -> NumberType {
    type FishPopulationType = Counter<NumberType, NumberType>;

    lazy_static! {
        static ref AFTER_BREAD_LIFE_TIME: NumberType = 6.to_biguint().unwrap();
        static ref NEW_FISH_LIFE_TIME: NumberType = 8.to_biguint().unwrap();
    }
    let mut pop: FishPopulationType = content
        .split(",")
        .map(|n| n.trim().parse::<NumberType>().unwrap())
        .collect();
    //println!("pop: {:?}", pop);

    for _day in 1..=max_days {
        let mut next_day_pop = FishPopulationType::new();
        match pop.remove(&NumberType::zero()) {
            Some(breading_fish_amount) => {
                next_day_pop[&AFTER_BREAD_LIFE_TIME] += &breading_fish_amount;
                next_day_pop[&NEW_FISH_LIFE_TIME] += &breading_fish_amount;
            }
            None => {}
        };
        for (days_until_bread, frequency) in pop.into_iter() {
            let days_until_bread_minus_one = days_until_bread - 1.to_biguint().unwrap();
            next_day_pop[&days_until_bread_minus_one] += frequency;
        }
        //println!("day: {:?}, next_day_pop: {:?}", _day, next_day_pop);

        pop = next_day_pop;
    }

    pop.iter()
        .map(|(_, frequency)| frequency)
        .sum::<NumberType>() as NumberType
}

fn solve_it(content: String, max_days: u32) -> NumberType {
    /*
    Part 1 result: 0 (elapsed time is: 614.8┬╡s)
Part 2 result: 0 (elapsed time is: 24.3508195s)
*/
    let mut pop = [
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
    ];

    content
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .for_each(|days_until_bread| {
            pop[days_until_bread] += 1.to_biguint().unwrap();
        });

    for _day in 1..=max_days {
        for i in 1..=8 {
            pop.swap(i - 1, i);
        }
        // pop[8] has now the value from pop[0]!
        pop[6] += pop[8].clone();
    }

    //pop.iter().sum()
    BigUint::zero()
}

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    Ok(solve_it(content, 80))
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    Ok(solve_it(content, 10_000_000))
    //Ok(solve_it(content, 256))
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
