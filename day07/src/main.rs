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

type NumberType = BigUint;

fn _create_number(num: u64) -> NumberType {
    num.to_biguint().unwrap()
}

/*
type NumberType = u64;
fn _create_number(x: u64) -> NumberType {
    x
}
 */


fn solve_it(content: String, max_days: u32) -> NumberType {
    const YOUNG_FISH_BREADING_CYCLE_TIME: usize = 8;
    const FISH_BREADING_CYCLE_TIME: usize = 6;
    //let mut pop: [NumberType; YOUNG_FISH_BREADING_CYCLE_TIME+1] = [_create_number(0); YOUNG_FISH_BREADING_CYCLE_TIME];
    let mut pop: [NumberType; YOUNG_FISH_BREADING_CYCLE_TIME + 1] =
        itertools::repeat_n(_create_number(0), YOUNG_FISH_BREADING_CYCLE_TIME + 1)
            .collect::<Vec<NumberType>>()
            .try_into()
            .unwrap();

    content
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .for_each(|days_until_bread| {
            pop[days_until_bread] += _create_number(1);
        });

    for _day in 1..=max_days {
        for i in 1..=YOUNG_FISH_BREADING_CYCLE_TIME {
            pop.swap(i - 1, i);
        }
        // pop[YOUNG_FISH_BREADING_CYCLE_TIME] has now the value from pop[0]!
        pop[FISH_BREADING_CYCLE_TIME] += pop[YOUNG_FISH_BREADING_CYCLE_TIME].clone();
    }

    pop.into_iter().reduce(|accum, item| accum + item).unwrap()
}

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    Ok(solve_it(content, 80))
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    //Ok(solve_it(content, 10_000_000))
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
            _create_number(5934)
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            _create_number(26984457539)
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            _create_number(362666)
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            _create_number(1640526601595)
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
