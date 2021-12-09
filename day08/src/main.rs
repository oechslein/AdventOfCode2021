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
use std::hash::Hasher;
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
/// The main function prints out the results for part1 and part2 of the day01
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1(utils::file_to_string("input.txt")));
    utils::with_measure("Part 2", || solve_part2(utils::file_to_string("test.txt")));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

type NumberType = usize;

fn _parse_content(content: String) -> Vec<(Vec<String>, Vec<String>)> {
    let content: Vec<(Vec<String>, Vec<String>)> = content
        .split("\r\n")
        .filter(|substr| !substr.is_empty())
        .map(|entry| {
            let mut entry_vec: Vec<Vec<String>> = entry
                .split('|')
                .filter(|substr| !substr.is_empty())
                .map(|substr| {
                    substr
                        .split(' ')
                        .filter(|substr| !substr.is_empty())
                        .map(|subsubstr| {
                            String::from(subsubstr.chars().sorted().collect::<String>())
                                .to_uppercase()
                        })
                        .collect()
                })
                .collect();
            (entry_vec.remove(0), entry_vec.remove(0))
        })
        .collect();

    content
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Digit {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
}

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    let segment_amount_to_digits_map: HashMap<usize, Vec<Digit>> = HashMap::from([
        (2, vec![Digit::ONE]),
        (3, vec![Digit::SEVEN]),
        (4, vec![Digit::FOUR]),
        (7, vec![Digit::EIGHT]),
        (5, vec![Digit::TWO, Digit::THREE, Digit::FIVE]),
        (6, vec![Digit::ZERO, Digit::SIX, Digit::NINE]),
    ]);

    let mut result = 0;
    for (all_number_samples, result_numbers) in _parse_content(content) {
        //println!("all_number_samples: {:?}", all_number_samples);
        //println!("result_numbers: {:?}", result_numbers);

        let mut sample_to_digit_map = HashMap::<String, Digit>::new();

        for number_sample in all_number_samples.into_iter() {
            //println!("number_sample: {:?}", number_sample);
            let possibile_digits = segment_amount_to_digits_map
                .get(&(number_sample.len()))
                .unwrap();
            if possibile_digits.len() == 1 {
                // part one only easy digits
                assert!(!sample_to_digit_map.contains_key(&number_sample));
                sample_to_digit_map.insert(number_sample, possibile_digits[0]);
            }
        }

        let easy_digits_amount = result_numbers
            .into_iter()
            .filter(|result_number| sample_to_digit_map.contains_key(result_number))
            .count();
        //println!("easy_digits_amount: {:?}", easy_digits_amount);

        result += easy_digits_amount;
    }

    Ok(result)
}

/*

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Segment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(String::from("Too long"));
        }
        match s.chars().next().unwrap().to_ascii_uppercase() {
            'A' => Ok(Segment::A),
            'B' => Ok(Segment::B),
            'C' => Ok(Segment::C),
            'D' => Ok(Segment::D),
            'E' => Ok(Segment::E),
            'F' => Ok(Segment::F),
            'G' => Ok(Segment::G),
            _ => Err(String::from(format!("Unknown Segment '{}'", s))),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SegmentList {
    _segments: HashSet<Segment>,
}

impl std::hash::Hash for SegmentList {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut h: u64 = 0;

        for elt in self._segments.iter() {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            elt.hash(&mut hasher);
            h ^= hasher.finish();
        }

        state.write_u64(h);
    }
}

impl FromIterator<Segment> for SegmentList {
    fn from_iter<I: IntoIterator<Item = Segment>>(iter: I) -> SegmentList {
        SegmentList {
            _segments: HashSet::from_iter(iter),
        }
    }
}

impl FromStr for SegmentList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash_set: HashSet<Segment> = HashSet::from_iter(
            s.chars()
                .map(|c| Segment::from_str(&(c.to_string())).unwrap()),
        );
        Ok(SegmentList {
            _segments: hash_set,
        })
    }
}

fn _create_possible_segment_mappings() -> Vec<HashMap<char, Segment>> {
    let mut possible_segment_mappings: Vec<HashMap<char, Segment>> = Vec::new();
    for vec_samples in "abcdefg"
        .to_uppercase()
        .chars()
        .permutations("abcdefg".len())
    {
        let all_segments = vec![
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ];
        let possible_segment_mapping =
            HashMap::from_iter(vec_samples.into_iter().zip(all_segments));
        possible_segment_mappings.push(possible_segment_mapping);
    }
    possible_segment_mappings
}

lazy_static! {
    static ref SEGMENTS_TO_DIGIT_MAPPING: HashMap<SegmentList, Digit> = HashMap::from([
            (SegmentList::from_str("ABCEFG").unwrap(), Digit::ZERO),
            (SegmentList::from_str("CF").unwrap(), Digit::ONE), // unique
            (SegmentList::from_str("ACDEG").unwrap(), Digit::TWO),
            (SegmentList::from_str("ACDFG").unwrap(), Digit::THREE),
            (SegmentList::from_str("BCDF").unwrap(), Digit::FOUR), // unique
            (SegmentList::from_str("ABDFG").unwrap(), Digit::FIVE),
            (SegmentList::from_str("ABDEFG").unwrap(), Digit::SIX),
            (SegmentList::from_str("ACF").unwrap(), Digit::SEVEN), // unique
            (SegmentList::from_str("ABCDEFG").unwrap(), Digit::EIGHT), // unique
            (SegmentList::from_str("ABCDFG").unwrap(), Digit::NINE),
        ]);
}

fn filter_out_impossible_mappings(
    number_sample: String,
    possible_segment_mappings: Vec<HashMap<char, Segment>>,
) -> Vec<HashMap<char, Segment>> {
    //println!("number_sample: {:?}", number_sample);
    let mut new_possible_segment_mappings: Vec<HashMap<char, Segment>> = Vec::new();
    for possible_segment_mapping in possible_segment_mappings.iter() {
        let number_sample_segments: SegmentList =
            SegmentList::from_iter(number_sample.chars().map(|c| possible_segment_mapping[&c]));
        match SEGMENTS_TO_DIGIT_MAPPING.get(&number_sample_segments) {
            Some(_) => new_possible_segment_mappings.push(possible_segment_mapping.clone()),
            None => {},
        };
    }
    new_possible_segment_mappings
}

fn solve_part2_calc_number(result_numbers: Vec<String>, segment_mapping: &HashMap<char, Segment>) -> NumberType {
    let mut line_result = 0;
    for result_number in result_numbers.iter() {
        let segments: SegmentList = result_number.chars().map(|c| segment_mapping[&c]).collect();
        let digit_found = SEGMENTS_TO_DIGIT_MAPPING.get(&segments).unwrap();
        line_result = line_result * 10 + (*digit_found) as NumberType;
    }
    line_result
}

fn solve_part2_entry(
    all_number_samples: Vec<String>,
    result_numbers: Vec<String>,
    possible_segment_mappings: &Vec<HashMap<char, Segment>>,
) -> NumberType {
    //println!("all_number_samples: {:?}", all_number_samples);
    //println!("result_numbers: {:?}", result_numbers);

    let mut possible_segment_mappings = possible_segment_mappings.clone();
    //println!("possible_segment_mappings: {:?}", possible_segment_mappings);

    for number_sample in all_number_samples.into_iter() {
        //println!("number_sample: {:?}", number_sample);
        let new_possible_segment_mappings =
            filter_out_impossible_mappings(number_sample, possible_segment_mappings);
        //println!("Reduced possible mappings from {} to {}", possible_segment_mappings.len(), new_possible_segment_mappings.len());
        assert!(!new_possible_segment_mappings.is_empty());
        possible_segment_mappings = new_possible_segment_mappings;
    }

    // now we do have the possible mappings, hopefully it is just one
    assert!(possible_segment_mappings.len() == 1);
    solve_part2_calc_number(result_numbers, &(possible_segment_mappings[0]))
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let possible_segment_mappings = _create_possible_segment_mappings();
    let result = _parse_content(content)
        .into_iter()
        .map(|(all_number_samples, result_numbers)| {
            solve_part2_entry(
                all_number_samples,
                result_numbers,
                &possible_segment_mappings,
            )
        })
        .sum();

    Ok(result)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1(utils::file_to_string("test.txt")).unwrap(), 26);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            61229
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            349
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            1070957
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
