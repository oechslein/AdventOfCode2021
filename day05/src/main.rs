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
type NumberType = i32;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: NumberType,
    y: NumberType,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Point {
    // 0,9
    type Err = String;

    fn from_str(point_str: &str) -> Result<Self, Self::Err> {
        let rows_vec: Vec<NumberType> = point_str
            .split(",")
            .map(|n| n.trim().parse::<NumberType>().unwrap())
            .collect();

        if rows_vec.len() > 2 { return Err(String::from("Parsing Error")); }

        Ok(Point { x: rows_vec[0], y: rows_vec[1] })
    }
}

impl FromStr for Line {
    // 0,9 -> 5,9
    type Err = String;

    fn from_str(point_str: &str) -> Result<Self, Self::Err> {
        let mut rows_vec: Vec<Point> = point_str
            .split(" -> ")
            .map(|n| n.trim().parse::<Point>().unwrap())
            .collect();

        let from = rows_vec.remove(0);
        let to = rows_vec.remove(0);
        if !rows_vec.is_empty() { Err(String::from("Parsing Error")) } else {
            Ok(Line { from, to })
        }
    }
}

impl Line {
    fn is_horizontal_or_vertical_line(&self) -> bool {
        (self.from.x == self.to.x) || (self.from.y == self.to.y)
    }

    fn _get_increment(start: NumberType, end: &NumberType) -> i32 {
        match start.cmp(&end) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    fn all_covered_points(&self) -> Vec<Point> {
        let mut x = self.from.x;
        let x_inc = Self::_get_increment(self.from.x, &self.to.x);
        let mut y = self.from.y;
        let y_inc = Self::_get_increment(self.from.y, &self.to.y);
        let mut all_covered_points = vec![Point { x, y }];
        while (x != self.to.x) || (y != self.to.y) {
            x += x_inc;
            y += y_inc;
            all_covered_points.push(Point { x, y });
        }

        //println!("point: {:?}", self);
        //println!("all_covered_points: {:?}", all_covered_points);

        all_covered_points
    }

}

/// The part1 function calculates the result for part1
fn solve_part1(content: String) -> Result<NumberType, String> {
    let lines: Vec<Line> = content.split("\r\n").map(|n| n.trim().parse::<Line>().unwrap()).collect();
    //println!("lines: {:?}", lines);
    let horizontal_or_vertical_lines: Vec<Line> = lines.into_iter().filter(|line| line.is_horizontal_or_vertical_line()).collect();
    //println!("horizontal_or_vertical_lines: {:?}", horizontal_or_vertical_lines);
    let frequency: Counter<Point> = horizontal_or_vertical_lines.into_iter().map(|line| line.all_covered_points()).flatten().collect();
    //println!("frequency: {:?}", frequency);
    let result = frequency.into_iter().filter(|(_, frequency)| (*frequency) > &1).count() as NumberType;

    Ok(result)
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let lines: Vec<Line> = content.split("\r\n").map(|n| n.trim().parse::<Line>().unwrap()).collect();
    //println!("lines: {:?}", lines);
    let frequency: Counter<Point> = lines.into_iter().map(|line| line.all_covered_points()).flatten().collect();
    //println!("frequency: {:?}", frequency);

    let result = frequency.into_iter().filter(|(_, frequency)| (*frequency) > &1).count() as NumberType;

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
            5
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            12
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
