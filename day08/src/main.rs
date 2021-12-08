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

type NumberType = usize;

fn _parse_content(content: String) -> Vec<(Vec<String>, Vec<String>)> {
    let mut content: Vec<(Vec<String>, Vec<String>)> = content
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
    SEVEN= 7 ,
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
        println!("all_number_samples: {:?}", all_number_samples);
        println!("result_numbers: {:?}", result_numbers);

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
        println!("easy_digits_amount: {:?}", easy_digits_amount);

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

// amount of segments
1: 2
7: 3
4: 4
8: 7

2: 5
3: 5
5: 5

0: 6
6: 6
9: 6

// identify:
1+7 => a klar, c/f vertauschbar
1+7+4 => a klar, c/f und b/d vertauschbar
1+7+4+8 => (8 bringt nichts)
0+8 => d klar (aber 0 nicht eindeutig identifizierbar)
6+8 => c klar (aber 6 nicht eindeutig identifizierbar)

mit allen möglichen mappings starten A=>(a,b,c,d,e,f,g) ...
dann für eindeutige Zahlen aus mapping werfen zB für 1 C=>(c,f)/F=>(c,f)
dafür braucht man
1: {c, f}
7: {a, c, f}
Ich denke man bruacht noch die positionen (hat man mit den chars schon)



2+3 => e+f klar, a/c/d/g nicht klar
2+3+5 => e+f+b klar, a/c/d/g nicht klar
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

fn _get_digit(digit_to_segments_mapping: &HashMap<Digit, HashSet<Segment>>, number_sample_segments: HashSet<Segment> ) -> Option::<Digit> {
    let mut digit_found = Option::<Digit>::None;
    for (digit, digit_segments) in digit_to_segments_mapping.iter() {
        if number_sample_segments.eq(digit_segments) {
            assert!(digit_found.is_none());
            digit_found = Some(*digit);
        }
    }
    digit_found
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let segment_amount_to_digits_map: HashMap<usize, Vec<Digit>> = HashMap::from([
        (2, vec![Digit::ONE]),
        (3, vec![Digit::SEVEN]),
        (4, vec![Digit::FOUR]),
        (7, vec![Digit::EIGHT]),
        (5, vec![Digit::TWO, Digit::THREE, Digit::FIVE]),
        (6, vec![Digit::ZERO, Digit::SIX, Digit::NINE]),
    ]);

    let digit_to_segments_mapping: HashMap<Digit, HashSet<Segment>> = HashMap::from([
        (
            Digit::ZERO,
            HashSet::from([
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G,
            ]),
        ),
        (Digit::ONE, HashSet::from([Segment::C, Segment::F])), // unique
        (
            Digit::TWO,
            HashSet::from([Segment::A, Segment::C, Segment::D, Segment::E, Segment::G]),
        ),
        (
            Digit::THREE,
            HashSet::from([Segment::A, Segment::C, Segment::D, Segment::F, Segment::G]),
        ),
        (
            Digit::FOUR,
            HashSet::from([Segment::B, Segment::C, Segment::D, Segment::F]),
        ), // unique
        (
            Digit::FIVE,
            HashSet::from([Segment::A, Segment::B, Segment::D, Segment::F, Segment::G]),
        ),
        (
            Digit::SIX,
            HashSet::from([
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]),
        ),
        (
            Digit::SEVEN,
            HashSet::from([Segment::A, Segment::C, Segment::F]),
        ), // unique
        (
            Digit::EIGHT,
            HashSet::from([
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]),
        ), // unique
        (
            Digit::NINE,
            HashSet::from([
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G,
            ]),
        ),
    ]);

    let mut result = 0;
    for (all_number_samples, result_numbers) in _parse_content(content) {
        println!("all_number_samples: {:?}", all_number_samples);
        println!("result_numbers: {:?}", result_numbers);

        /* size = n*(n-1)*(n-2)
           [<(a-A),(b-B),(c-C), ...>,
            <(a-B),(b-A),(c-C), ...>,
           ]
        */
        let mut possible_segment_mappings: Vec<HashMap<char, Segment>> = Vec::new();
        for vec_samples in "abcdefg".to_uppercase().chars().permutations("abcdefg".len()) {
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
        //println!("possible_segment_mappings: {:?}", possible_segment_mappings);

        for number_sample in all_number_samples.into_iter() {
            //println!("number_sample: {:?}", number_sample);
            let mut new_possible_segment_mappings: Vec<HashMap<char, Segment>> = Vec::new();
            for possible_segment_mapping in possible_segment_mappings.iter() {
                let number_sample_segments: HashSet<Segment> =
                    HashSet::from_iter(number_sample.chars().map(|c| possible_segment_mapping[&c]));
                // / println!("number_sample_segments: {:?}", number_sample_segments);
                // we now have one mapping e.g. <(a-B),(b-A),(c-C), ...>
                // 1. Convert chars into Segments (based on mapping)
                //    => list of Segments
                // 2. Get unique! number for list of Segments (is unique since we do have a mapping)
                // 3. Check if there is not contradication introduced (HOW?)
                //    a) there is no corresponding digit found
                //    b) there is a digit found that we already have a mapping for (not sure if that is possible?)

                let digit_found = _get_digit(&digit_to_segments_mapping, number_sample_segments);
                if digit_found.is_some() {
                    new_possible_segment_mappings.push(possible_segment_mapping.clone());
                }
            }
            //println!("Reduced possible mappings from {} to {}", possible_segment_mappings.len(), new_possible_segment_mappings.len());
            assert!(!new_possible_segment_mappings.is_empty());
            possible_segment_mappings = new_possible_segment_mappings;
        }

        // now we do have the possible mappings, hopefully it is just one
        assert!(possible_segment_mappings.len() == 1);
        let segment_mapping = &(possible_segment_mappings[0]);
        let mut line_result = 0;
        for result_number in result_numbers.iter() {
            let segments: HashSet<Segment> = result_number.chars().map(|c| segment_mapping[&c]).collect();
            let digit_found = _get_digit(&digit_to_segments_mapping, segments).unwrap();
            line_result = line_result * 10 + digit_found as NumberType;
        }
        result += line_result;
    }

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
        assert_eq!(solve_part2(utils::file_to_string("test.txt")).unwrap(), 61229);
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
