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
use std::collections::{HashMap, HashSet};
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
use itertools::Itertools;
use test::Bencher;

#[macro_use]
extern crate lazy_static;

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct TwoDimArray<T> {
    _elements: Vec<Vec<T>>,
}

impl<T> TwoDimArray<T> {
    fn get_elem(&self, x: usize, y: usize) -> &T {
        let y_vec: &Vec<T> = &self._elements[y];
        &y_vec[x]
    }

    fn set_elem(&mut self, x: usize, y: usize, new_elem: T) {
        let y_vec: &mut Vec<T> = &mut self._elements[y];
        y_vec[x] = new_elem;
    }

    fn get_x_size(&self) -> usize {
        self._elements.get(0).unwrap().len()
    }
    fn get_y_size(&self) -> usize {
        self._elements.len()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::<(usize, usize)>::new();
        for (new_x, new_y) in
            (x as i64 - 1..=x as i64 + 1).cartesian_product(y as i64 - 1..=y as i64 + 1)
        {
            // incl diagonals
            if (new_x, new_y) != (x as i64, y as i64)
                && new_x >= 0
                && new_x < self.get_x_size() as i64
                && new_y >= 0
                && new_y < self.get_y_size() as i64
            {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }
        neighbors
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

impl<T> FromStr for TwoDimArray<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TwoDimArray {
            _elements: s
                .replace("\r", "")
                .split("\n")
                .map(|entry| {
                    entry
                        .chars()
                        .map(|c| c.to_string().parse::<T>().unwrap())
                        .collect()
                })
                .collect(),
        })
    }
}

impl fmt::Display for MyTwoDimArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.get_y_size() {
            for x in 0..self.get_x_size() {
                result = format!("{}{:#01x}", result, self.get_elem(x, y));
            }
            result.push('\n');
        }
        write!(f, "{}", result.replace("0x", ""))
    }
}

type NumberType = isize;
type MyTwoDimArray = TwoDimArray<NumberType>;

impl MyTwoDimArray {
    fn increment_all(&mut self, inc: NumberType) {
        for x in 0..self.get_x_size() {
            for y in 0..self.get_y_size() {
                self.set_elem(x, y, self.get_elem(x, y) + inc);
            }
        }
    }

    fn reset_flashed_ones(&mut self) {
        for x in 0..self.get_x_size() {
            for y in 0..self.get_y_size() {
                if self.get_elem(x, y) > &9 {
                    self.set_elem(x, y, 0);
                }
            }
        }
    }

    fn flash(&mut self, flash_positions: &mut HashSet<(usize, usize)>) {
        let all_new_flash_positions: Vec<(usize, usize)> = self
            .enumerate()
            .filter(|(x, y, elem)| (elem >= &&10) && !flash_positions.contains(&(*x, *y)))
            .map(|(x, y, _)| (x, y))
            .collect();

        for (flash_position_x, flash_position_y) in all_new_flash_positions.iter() {
            let neighbor_positions = self.get_neighbors(*flash_position_x, *flash_position_y);
            for (x_neighbor, y_neighbor) in neighbor_positions {
                self.set_elem(
                    x_neighbor,
                    y_neighbor,
                    self.get_elem(x_neighbor, y_neighbor) + 1,
                );
            }
        }

        flash_positions.extend(all_new_flash_positions);
    }

    fn step(&mut self) -> usize {
        self.increment_all(1);

        let mut flash_positions: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let old_flash_positions_len = flash_positions.len();
            self.flash(&mut flash_positions);
            if old_flash_positions_len == flash_positions.len() {
                break;
            }
        }

        self.reset_flashed_ones();
        flash_positions.len()
    }

    fn steps(&mut self, amount: usize) -> usize {
        (0..amount).map(|_| self.step()).sum()
    }

    fn steps_until_flashed(&mut self) -> usize {
        let mut steps = 1;
        loop {
            self.step();
            if self.iter().all(|elem| elem == &0) {
                return steps;
            }
            steps += 1;
        }
    }
}

fn _parse_content(file_name: &str) -> MyTwoDimArray {
    TwoDimArray::from_str(fs::read_to_string(file_name).unwrap().as_str()).unwrap()
}

/// The part1 function calculates the result for part2
fn solve_part1(file_name: &str) -> Result<usize, String> {
    let mut world = _parse_content(file_name);
    let total_flashes = world.steps(100);

    Ok(total_flashes)
}

/// The part2 function calculates the result for part2
fn solve_part2(file_name: &str) -> Result<usize, String> {
    let steps = _parse_content(file_name).steps_until_flashed();
    Ok(steps)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        let a = MyTwoDimArray::from_str(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        )
        .unwrap();
        assert_eq!(a.get_y_size(), 10);
        assert_eq!(a.get_x_size(), 10);
        assert_eq!(a.get_neighbors(0, 0), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(a.get_neighbors(9, 9), vec![(8, 8), (8, 9), (9, 8)]);

        assert_eq!(
            _parse_content("test.txt"),
            MyTwoDimArray::from_str(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )
            .unwrap()
        );

        let mut test_a = _parse_content("test.txt");
        assert_eq!(test_a.steps(1), 0);
        assert_eq!(
            test_a,
            MyTwoDimArray::from_str(
                "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
            )
            .unwrap()
        );

        let mut test_a = _parse_content("test.txt");
        assert_eq!(test_a.steps(2), 35);
        assert_eq!(
            test_a,
            MyTwoDimArray::from_str(
                "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
            )
            .unwrap()
        );

        let mut test_a = _parse_content("test.txt");
        assert_eq!(test_a.steps(10), 204);
        assert_eq!(
            test_a,
            MyTwoDimArray::from_str(
                "0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000"
            )
            .unwrap()
        );

        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 1656);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 1729);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 195);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 237);
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
