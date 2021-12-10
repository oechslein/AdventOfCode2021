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

struct TwoDimArray<T> {
    _elements: Vec<Vec<T>>,
}

impl<T> TwoDimArray<T> {
    fn get_elem(&self, x: usize, y: usize) -> &T {
        self._elements.get(y).unwrap().get(x).unwrap()
    }

    fn get_x_size(&self) -> usize {
        self._elements.get(0).unwrap().len()
    }
    fn get_y_size(&self) -> usize {
        self._elements.len()
    }

    fn enumerate_neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        let mut neighbors = Vec::<(usize, usize, &T)>::new();
        if x > 0 {
            neighbors.push((x - 1, y, self.get_elem(x - 1, y)));
        }
        if x + 1 < self.get_x_size() {
            neighbors.push((x + 1, y, self.get_elem(x + 1, y)));
        }
        if y > 0 {
            neighbors.push((x, y - 1, self.get_elem(x, y - 1)));
        }
        if y + 1 < self.get_y_size() {
            neighbors.push((x, y + 1, self.get_elem(x, y + 1)));
        }
        neighbors.into_iter()
    }

    fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        let mut all = Vec::<&T>::new();
        for x in 0..self.get_x_size() {
            for y in 0..self.get_y_size() {
                all.push(self.get_elem(x, y))
            }
        }
        all.into_iter()
    }

    fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        let mut all = Vec::<(usize, usize, &T)>::new();
        for x in 0..self.get_x_size() {
            for y in 0..self.get_y_size() {
                all.push((x, y, self.get_elem(x, y)))
            }
        }
        all.into_iter()
    }
}

fn _parse_content(content: String) -> TwoDimArray<NumberType> {
    TwoDimArray {
        _elements: content
            .split("\r\n")
            .filter(|substr| !substr.is_empty())
            .map(|entry| {
                entry
                    .chars()
                    .map(|c| c.to_string().parse::<NumberType>().unwrap())
                    .collect()
            })
            .collect(),
    }
}

/// The part1 function calculates the result for part2
fn solve_part1(content: String) -> Result<NumberType, String> {
    let sea_floor = _parse_content(content);
    let mut result = 0;
    for (x, y, depth) in sea_floor.enumerate() {
        if sea_floor
            .enumerate_neighbors(x, y)
            .map(|(_, _, depth)| depth)
            .min()
            .unwrap()
            > depth
        {
            // found local minima
            result += depth + 1;
        }
    }
    Ok(result)
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let sea_floor = _parse_content(content);
    let mut all_deepest_points = Vec::<(usize, usize)>::new();
    for (x, y, depth) in sea_floor.enumerate() {
        if sea_floor
            .enumerate_neighbors(x, y)
            .map(|(_, _, depth)| depth)
            .min()
            .unwrap()
            > depth
        {
            // found local minima
            all_deepest_points.push((x, y));
        }
    }
    let mut all_basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for (deep_point_x, deep_point_y) in all_deepest_points {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((deep_point_x, deep_point_y));
        let mut points_to_check: Vec<(usize, usize)> = vec![(deep_point_x, deep_point_y)];
        let mut visited_points: HashSet<(usize, usize)> = HashSet::new();
        while !points_to_check.is_empty() {
            let (x, y) = points_to_check.pop().unwrap();
            for (neighbor_x, neighbor_y, depth) in sea_floor.enumerate_neighbors(x, y) {
                if !visited_points.contains(&(neighbor_x, neighbor_y)) {
                    visited_points.insert((neighbor_x, neighbor_y));
                    if *depth < 9 {
                        points_to_check.push((neighbor_x, neighbor_y));
                        basin.insert((neighbor_x, neighbor_y));
                    }
                }
            }
        }
        all_basins.push(basin);
    }
    let mut result = 1;
    let max_basin_size = all_basins.iter().map(|basin|basin.len()).max().unwrap();
    for nth in 0..3 {
        all_basins.select_nth_unstable_by_key(nth, |basin| max_basin_size - basin.len());
        result *= all_basins[nth].len();
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
        assert_eq!(solve_part1(utils::file_to_string("test.txt")).unwrap(), 15);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            1134
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            516
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            1023660
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
