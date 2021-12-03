#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
extern crate test;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};

use test::Bencher;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2 of the day01
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || {
        solve_part1(&utils::parse_input(utils::file_to_string("input.txt")))
    });
    utils::with_measure("Part 2", || {
        solve_part2(&utils::parse_input(utils::file_to_string("input.txt")))
    });
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////
type NumberType = usize;

fn convert(input: &[String]) -> Vec<NumberType> {
    input
        .iter()
        .map(|s| NumberType::from_str_radix(s, 2).unwrap())
        .collect()
}

fn vec_as_binary_to_string(my_vec: &[NumberType]) -> String {
    let mut res = String::from("[");
    for number in my_vec {
        res += &format!("{:#07b}, ", number);
    }
    res + "]"
}

/// The part1 function calculates the result for part1 // 10110
fn solve_part1(input: &[String]) -> Result<NumberType, String> {
    let binary_length_of_number = input[0].len() as NumberType;
    let amount_of_numbers = input.len() as NumberType;
    let middle_index = ((amount_of_numbers / 2) - 1) as usize;
    let mut number_vec: Vec<NumberType> = convert(input);
    let mut result: NumberType = 0;
    for index in 0..binary_length_of_number {
        let value_half = 1 << (binary_length_of_number - index - 1);

        //////////////////////////////////////////
        // get from sorted vector the middle element
        //let x = vec_as_binary_to_string(&number_vec);
        number_vec.sort_unstable();
        // should be faster (O(n)), but isn't: number_vec.select_nth_unstable(middle_index);
        //let y = vec_as_binary_to_string(&number_vec);
        let number_middle = number_vec[middle_index];

        //////////////////////////////////////////
        // if first bit of middle element is 1, use it as leading bit for result
        if number_middle > /* 01...1 */ value_half-1 {
            // first bit is 1
            result += value_half;
        } else {
            // first bit is 0
            // result = result;
        }

        //////////////////////////////////////////
        // remove leading bit for all numbers
        for number in number_vec.iter_mut() {
            *number &= /* 01...1 */ value_half-1;
        }
    }

    let gamma = result;
    let epsilon = (!result) & ((1 << binary_length_of_number) - 1); // epsilon is just the inverted value

    Ok(gamma * epsilon)
}

fn is_bit_on(input: NumberType, n: NumberType) -> bool {
    input & (1 << n) != 0
}

fn has_more_ones(number_vec: &[NumberType], index: NumberType) -> Ordering {
    let one_amount = number_vec
        .iter()
        .filter(|&&number| is_bit_on(number, index))
        .count();
    let zero_amount = number_vec
        .iter()
        .filter(|&&number| !is_bit_on(number, index))
        .count();
    one_amount.cmp(&zero_amount)
}

fn calc_value(binary_length_of_number: NumberType, number_vec: Vec<NumberType>, need_ones: bool) -> Result<NumberType, String> {
    let mut number_vec= number_vec;
    for bit_index in (0..binary_length_of_number).rev() {
            match has_more_ones(&number_vec, bit_index) {
                Ordering::Greater | Ordering::Equal => {
                    number_vec.drain_filter(|&mut number| {
                        need_ones != is_bit_on(number, bit_index)
                    });
                }
                Ordering::Less => {
                    number_vec.drain_filter(|&mut number| {
                        need_ones == is_bit_on(number, bit_index)
                    });
                }
            }

        if number_vec.len() <= 1 {
            return Ok(number_vec[0]);
        }
    }
    Err(String::from("Shouldn't happen :-)"))
}

/// The part2 function calculates the result for part2
fn solve_part2(input: &[String]) -> Result<NumberType, String> {
    let binary_length_of_number = input[0].len() as NumberType;

    let oxygen_generator_rating = calc_value(binary_length_of_number, convert(input), true)?;
    let co2_scrubber_rating = calc_value(binary_length_of_number, convert(input), false)?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{file_to_string, parse_input};

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(&utils::parse_input(utils::file_to_string("test.txt"))).unwrap(),
            198
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(&utils::parse_input(utils::file_to_string("test.txt"))).unwrap(),
            230
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(&utils::parse_input(utils::file_to_string("input.txt"))).unwrap(),
            3429254
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(&utils::parse_input(utils::file_to_string("input.txt"))).unwrap(),
            5410338
        );
        Ok(())
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1(&utils::parse_input(utils::file_to_string("input.txt"))));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2(&utils::parse_input(utils::file_to_string("input.txt"))));
    }
}
