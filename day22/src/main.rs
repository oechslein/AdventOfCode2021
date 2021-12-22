#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]

extern crate test;

use itertools::Itertools;
use num_complex::Complex64;

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;

use fxhash::FxHashMap;

use gcollections::ops::*;
use interval::interval_set::*;
use interval::ops::*;
use interval::Interval;

use std::ops::RangeInclusive;

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

//#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]

#[derive(Debug, Clone)]
struct Cuboid {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
    holes: Vec<Cuboid>,
}

fn parse_interval(s: &str) -> RangeInclusive<isize> {
    let x: (isize, isize) = s[2..]
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    x.0..=x.1
}

fn parse(file_name: &str) -> Vec<(bool, Cuboid)> {
    utils::file_to_lines(file_name)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (s1, s2) = s.split(' ').collect_tuple().unwrap();
            let is_on = "on" == s1;
            let (x, y, z) = s2.split(',').map(parse_interval).collect_tuple().unwrap();
            (is_on, Cuboid::new(x, y, z))
        })
        .collect_vec()
}

impl Cuboid {
    fn new(
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
        z_range: RangeInclusive<isize>,
    ) -> Cuboid {
        Self {
            x_range,
            y_range,
            z_range,
            holes: vec![],
        }
    }

    fn trim(&mut self, limit: usize) {
        let limit = limit as isize;
        self.x_range = (*self.x_range.start()).max(-limit)..=(*self.x_range.end()).min(limit);
        self.y_range = (*self.y_range.start()).max(-limit)..=(*self.y_range.end()).min(limit);
        self.z_range = (*self.z_range.start()).max(-limit)..=(*self.z_range.end()).min(limit);
    }

    fn contains(&self, x: &isize, y: &isize, z: &isize) -> bool {
        self.x_range.contains(x)
            && self.y_range.contains(y)
            && self.z_range.contains(z)
            && !self.holes.iter().any(|h| h.contains(x, y, z))
    }

    fn remove_cuboid(&mut self, other: &Self) {
        for hole in &mut self.holes {
            hole.remove_cuboid(other);
        }
        if let Some(new_hole) = self.intersect(other) {
            self.holes.push(new_hole);
        }
    }

    fn intersect_range(
        range: &RangeInclusive<isize>,
        other_range: &RangeInclusive<isize>,
    ) -> RangeInclusive<isize> {
        *range.start().max(other_range.start())..=*range.end().min(other_range.end())
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let x_range = Cuboid::intersect_range(&self.x_range, &other.x_range);
        let y_range = Cuboid::intersect_range(&self.y_range, &other.y_range);
        let z_range = Cuboid::intersect_range(&self.z_range, &other.z_range);

        if x_range.is_empty() || y_range.is_empty() || z_range.is_empty() {
            return None;
        }

        Some(Cuboid::new(x_range, y_range, z_range))
    }

    fn calc_ons(&self) -> usize {
        // this could be removed if ExactSizeIterator is implemented for (u|i)size.
        let calc_size = |range: &RangeInclusive<isize>| {
            if range.is_empty() {
                0
            } else {
                (range.end() - range.start() + 1) as usize
            }
        };
        let hole_offs: usize = self.holes.iter().map(|c| c.calc_ons()).sum();
        (calc_size(&self.x_range) * calc_size(&self.y_range) * calc_size(&self.z_range)) - hole_offs
    }
}

fn apply_instructions(instructions: Vec<(bool, Cuboid)>) -> usize {
    let mut cuboids_on = Vec::<Cuboid>::new();
    for (is_on, c) in instructions {
        let new_cuboid = Cuboid::new(c.x_range, c.y_range, c.z_range);
        for cuboid in cuboids_on.iter_mut() {
            cuboid.remove_cuboid(&new_cuboid)
        }

        if is_on {
            cuboids_on.push(new_cuboid);
        }
    }
    cuboids_on.iter().map(Cuboid::calc_ons).sum()
}


fn solve_part1(file_name: &str) -> usize {
    let mut instructions = parse(file_name);
    instructions.iter_mut().for_each(|(_, c)| c.trim(50));
    apply_instructions(instructions)
}

fn solve_part2(file_name: &str) -> usize {
    let instructions = parse(file_name);
    apply_instructions(instructions)
}


////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() {}

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 590784);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 576028);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test2.txt"), 2758514936282235);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 1387966280636636);
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
