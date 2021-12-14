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

type PolymerAsCounterType = Counter<String>;
type RulesType = HashMap<String, (String, String)>;

fn parse_input(file_name: &str) -> (String, RulesType) {
    let file_to_string = utils::file_to_string(file_name);
    let (polymer_content, rules_content) =
        file_to_string.split("\r\n\r\n").collect_tuple().unwrap();
    let polymer = polymer_content.to_string();
    let rules: RulesType = rules_content
        .split("\r\n")
        .map(|rule_str| {
            let (left_str, right_str) = rule_str.split(" -> ").collect_tuple().unwrap();
            let left = left_str.to_string();

            let mut right_1 = left[0..1].to_string().clone();
            let insert_char = right_str.chars().next().unwrap();
            right_1.push(insert_char);
            let mut right_2 = left[1..].to_string().clone();
            right_2.insert(0, insert_char);

            (left, (right_1, right_2))
        })
        .collect();
    //println!("rules: {:?}", rules);
    (polymer, rules)
}

fn apply_steps(steps: i32, polymer: &String, rules: RulesType) -> PolymerAsCounterType {
    let mut polymer_as_counter = (0..polymer.len() - 1)
        .map::<String, _>(|index| polymer[index..=index + 1].into())
        .collect::<PolymerAsCounterType>();
    for _ in 0..steps {
        polymer_as_counter = apply_rules(&polymer_as_counter, &rules);
    }
    polymer_as_counter
}

fn apply_rules(
    polymer_as_counter: &PolymerAsCounterType,
    rules: &RulesType,
) -> PolymerAsCounterType {
    let mut new_polymer_as_counter = Counter::new();
    for (polymer_pair, polymer_count) in polymer_as_counter.iter() {
        let (replacement_1, replacement_2) = rules.get(polymer_pair).unwrap();
        new_polymer_as_counter[replacement_1] += polymer_count;
        new_polymer_as_counter[replacement_2] += polymer_count;
    }
    new_polymer_as_counter
}

fn calc_result(polymer_as_counter: PolymerAsCounterType, polymer: String) -> usize {
    let all_chars = polymer_as_counter
        .into_iter()
        .map(|(key, value)| (key.chars().next().unwrap(), value));
    let mut frequencies = Counter::<char>::new();
    for (c, value) in all_chars {
        frequencies[&c] += value;
    }
    // last char is missing
    frequencies[&polymer.chars().rev().next().unwrap()] += 1;
    frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
}

/// The part1 function calculates the result for part2
fn solve_part1(file_name: &str) -> Result<usize, String> {
    /*
       [("CB", 1), ("NC", 1), ("NN", 1)]
       [("BC", 1), ("CH", 1), ("CN", 1), ("HB", 1), ("NB", 1), ("NC", 1)]
       [("BB", 2), ("BC", 2), ("BH", 1), ("CB", 2), ("CC", 1), ("CN", 1), ("HC", 1), ("NB", 2)]
    ++ [("BB", 2), ("BC", 2), ("BH", 1), ("CB", 2), ("CC", 1), ("CN", 1), ("HC", 1), ("NB", 2)]
       [("BB", 4), ("BC", 3), ("BH", 1), ("BN", 2), ("CC", 1), ("CH", 2), ("CN", 2), ("HB", 3), ("HH", 1), ("NB", 4), ("NC", 1)]
    ++ [("BB", 4), ("BC", 3), ("BH", 1), ("BN", 2), ("CC", 1), ("CH", 2), ("CN", 2), ("HB", 3), ("HH", 1), ("NB", 4), ("NC", 1)]

       NNCB => NN, NC, CB
       NCNBCHB => NC, CN, NB, BC, CH, HB
       NBCCNBBBCBHCB => NB BC CC CN NB BB BB BC CB BH HC CB
       NBBBCNCCNBBNBNBBCHBHHBCHB => NB BB BB BC CN NC CC CN NB BB BN NB BN NB BB BC CH HB BH HH HB BC CH HB
       NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

       [("BC", 1), ("CH", 1), ("CN", 1), ("HB", 1), ("NB", 1), ("NC", 1)]
       NCNBCH_B
       N_CNBCHB


       NNCB => NN, NC, CB (=> NCNBCHB)

       NN -> C => NN -> NCN => NN -> (NC, CN)
       NC -> B => NC -> NBC => NC -> (NB, BC)
       CB -> H => CB -> CHB => CB -> (CH, HB)

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

    let (polymer, rules) = parse_input(file_name);
    let polymer_as_counter = apply_steps(10, &polymer, rules);
    let result = calc_result(polymer_as_counter, polymer);

    Ok(result)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let (polymer, rules) = parse_input(file_name);
    let polymer_as_counter = apply_steps(40, &polymer, rules);
    let result = calc_result(polymer_as_counter, polymer);

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
