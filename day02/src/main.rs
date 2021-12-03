#![crate_name = "day02"]

//! --- Day 2: Dive! ---
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
//!
//! forward X increases the horizontal position by X units.
//! down X increases the depth by X units.
//! up X decreases the depth by X units.
//! Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//!
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
//!
//! forward 5 adds 5 to your horizontal position, a total of 5.
//! down 5 adds 5 to your depth, resulting in a value of 5.
//! forward 8 adds 8 to your horizontal position, a total of 13.
//! up 3 decreases your depth by 3, resulting in a value of 2.
//! down 8 adds 8 to your depth, resulting in a value of 10.
//! forward 2 adds 2 to your horizontal position, a total of 15.
//! After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! Your puzzle answer was 1451208.
//!
//! The first half of this puzzle is complete! It provides one gold star: *
//!
//! --- Part Two ---
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
//!
//! down X increases your aim by X units.
//! up X decreases your aim by X units.
//! forward X does two things:
//! It increases your horizontal position by X units.
//! It increases your depth by your aim multiplied by X.
//! Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//! down 5 adds 5 to your aim, resulting in a value of 5.
//! forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//! up 3 decreases your aim by 3, resulting in a value of 2.
//! down 8 adds 8 to your aim, resulting in a value of 10.
//! forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//! After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! Your puzzle answer was 1620141160.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
extern crate test;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error;
use std::fmt::{Debug, Error};
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

type NumberType = i32;

#[derive(Debug)]
enum Command {
    Forward(NumberType, bool),
    Up(NumberType, bool),
    Down(NumberType, bool),
}

impl Command {
    fn parse_str(my_str: &str, argument: NumberType, part1: bool) -> Command {
        let c = Command::from_str("forward 5");
        match my_str {
            "forward" => Command::Forward(argument, part1),
            "up" => Command::Up(argument, part1),
            "down" => Command::Down(argument, part1),
            _ => panic!("Unknown input {:?}", my_str),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<command_str>[^ ]*) (?P<argument>\d*)$").unwrap();
        }

        let res = RE.captures(s);
        let res2 = RE.captures(s).unwrap();

        let command_str = res2.name("command_str").unwrap().as_str();
        let argument: NumberType = res2
            .name("argument")
            .unwrap()
            .as_str()
            .parse::<NumberType>()
            .unwrap();
        let part1 = true;

        match command_str {
            "forward" => Ok(Command::Forward(argument, part1)),
            "up" => Ok(Command::Up(argument, part1)),
            "down" => Ok(Command::Down(argument, part1)),
            _ => Err(format!("'{}' unknown direction", s)),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    hor: NumberType,
    depth: NumberType,
    aim: NumberType,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            hor: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn apply_command(&mut self, command: Command) {
        match command {
            // Part1:
            // forward X increases the horizontal position by X units.
            // down X increases the depth by X units.
            // up X decreases the depth by X units.
            // Part2:
            // down X increases your aim by X units.
            // up X decreases your aim by X units.
            // forward X does two things:
            // It increases your horizontal position by X units.
            // It increases your depth by your aim multiplied by X.
            Command::Forward(argument, part1) => {
                if part1 {
                    self.hor += argument;
                } else {
                    self.hor += argument;
                    self.depth += self.aim * argument;
                }
            }
            Command::Up(argument, part1) => {
                if part1 {
                    self.depth -= argument;
                } else {
                    self.aim -= argument;
                }
            }
            Command::Down(argument, part1) => {
                if part1 {
                    self.depth += argument;
                } else {
                    self.aim += argument;
                }
            }
        };
    }

    fn solve_it(&mut self, commands: Vec<Command>) -> NumberType {
        commands
            .into_iter()
            .for_each(|command| self.apply_command(command));
        self.hor * self.depth
    }
}

////////////////////////////////////////////////////////////////////////////////////
fn parse_input(input: &Vec<Vec<String>>, part1: bool) -> Vec<Command> {
    input
        .iter()
        .map(|x| Command::parse_str(x[0].as_str(), x[1].parse::<NumberType>().unwrap(), part1))
        .collect()
}

/// The part1 function calculates the result for part1
fn solve_part1(input: &Vec<Vec<String>>) -> Result<NumberType, String> {
    Ok(Submarine::new().solve_it(parse_input(input, true)))
}

/// The part2 function calculates the result for part2
fn solve_part2(input: &Vec<Vec<String>>) -> Result<NumberType, String> {
    Ok(Submarine::new().solve_it(parse_input(input, false)))
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
            150
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(&utils::parse_input(utils::file_to_string("test.txt"))).unwrap(),
            900
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(&utils::parse_input(utils::file_to_string("input.txt"))).unwrap(),
            1451208
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(&utils::parse_input(utils::file_to_string("input.txt"))).unwrap(),
            1620141160
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
