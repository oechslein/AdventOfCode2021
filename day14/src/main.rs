#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]
#![feature(destructuring_assignment)]

extern crate test;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::mem;
use std::ops::Sub;
use std::path::Path;
use std::str::{FromStr, Split};
use std::time::{Duration, Instant};

use counter::Counter;
//use counter::Counter;
use itertools::{all, Itertools};
use test::Bencher;

//#[macro_use]
//extern crate lazy_static;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("input.txt"));
    utils::with_measure("Part 2", || solve_part2("input.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

type CountedPolymerPairs = Counter<String>;
type Rules = HashMap<String, (String, String)>;

struct PolymerProblem {
    _start_polymer: String,
    _rules: Rules,
}

/*
Expand rules
    CN -> CC CN
    HN -> HC CN
    NB -> NB BB
    BB -> BN NB
    BN -> BB BN
    BH -> BH HH
    BC -> BB BC
    CC -> CN NC
    CB -> CH HB
    HH -> HN NH
    HC -> HB BC
    NC -> NB BC
    HB -> HC CB
    NN -> NC CN
    CH -> CB BH
    NH -> NC CH
*/
fn parse_rule(rule_str: &str) -> (String, (String, String)) {
    let (pattern, right_str) = rule_str.split(" -> ").collect_tuple().unwrap();
    let insert_char = right_str.chars().next().unwrap();
    let left_expansion = pattern[0..1].to_string() + &insert_char.to_string();
    let right_expansion = insert_char.to_string() + &pattern[1..];
    (pattern.to_string(), (left_expansion, right_expansion))
}

impl FromStr for PolymerProblem {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (polymer_content, rules_content) = content.split("\r\n\r\n").collect_tuple().unwrap();
        let polymer = polymer_content.to_string();
        let rules: Rules = rules_content.split("\r\n").map(parse_rule).collect();
        //println!("rules: {:?}", rules);
        Ok(PolymerProblem {
            _start_polymer: polymer,
            _rules: rules,
        })
    }
}

impl PolymerProblem {
    fn solve(&self, steps: i32) -> usize {
        self.calc_result(self.apply_steps(steps))
    }

    fn apply_steps(&self, steps: i32) -> CountedPolymerPairs {
        /*
               From string to pairs, then apply extended rules, then back to string
               NNCB => NN, NC, CB => [NN -> (NC, CN)] [NC -> (NB, BC)] [CB -> (CH, HB)] => NC CN NB BC CH HB => NCNBCHB
               Just work on pairs and count occurance (order is lost, but not relevant for solution)
        */

        let mut counted_polymer_pairs = self.create_counted_polymer_pairs();
        for _ in 0..steps {
            counted_polymer_pairs = self.apply_rules(&counted_polymer_pairs);
        }
        counted_polymer_pairs
    }

    fn create_counted_polymer_pairs(&self) -> Counter<String> {
        let polymer = &self._start_polymer;
        (0..polymer.len() - 1)
            .map::<String, _>(|index| polymer[index..=index + 1].to_string())
            .collect::<CountedPolymerPairs>()
    }

    fn apply_rules(&self, counted_polymer_pairs: &CountedPolymerPairs) -> CountedPolymerPairs {
        let rules = &self._rules;
        let mut new_counted_polymer_pairs = Counter::new();
        for (polymer_pair, polymer_count) in counted_polymer_pairs.iter() {
            let (left_expansion, right_expansion) = rules.get(polymer_pair).unwrap();
            new_counted_polymer_pairs[left_expansion] += polymer_count;
            new_counted_polymer_pairs[right_expansion] += polymer_count;
        }
        new_counted_polymer_pairs
    }

    fn calc_result(&self, counted_polymer_pairs: CountedPolymerPairs) -> usize {
        let all_chars = counted_polymer_pairs
            .into_iter()
            .map(|(key, value)| (key.chars().next().unwrap(), value));
        let mut frequencies = Counter::<char>::new();
        for (c, value) in all_chars {
            frequencies[&c] += value;
        }
        // last char is missing
        frequencies[&self._start_polymer.chars().rev().next().unwrap()] += 1;
        frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
    }
}

/// The part1 function calculates the result for part2
fn solve_part1(file_name: &str) -> Result<usize, String> {
    let polymer_problem = PolymerProblem::from_str(&utils::file_to_string(file_name)).unwrap();
    let result = polymer_problem.solve(10);

    Ok(result)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let polymer_problem = PolymerProblem::from_str(&utils::file_to_string(file_name)).unwrap();
    let result = polymer_problem.solve(40);

    Ok(result)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 1588);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 2657);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 2188189693529);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 2911561572630);
        Ok(())
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
}
