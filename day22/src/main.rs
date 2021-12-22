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
    x_range: Interval<isize>,
    y_range: Interval<isize>,
    z_range: Interval<isize>,
    holes: Vec<Cuboid>,
}

impl Cuboid {
    fn new(x_range: Interval<isize>, y_range: Interval<isize>, z_range: Interval<isize>) -> Cuboid {
        Self {
            x_range,
            y_range,
            z_range,
            holes: vec![],
        }
    }

    fn _parse_interval(s: &str) -> Interval<isize> {
        let (axis_lower, axis_upper): (isize, isize) = s[2..]
            .split("..")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Interval::new(axis_lower, axis_upper)
    }

    fn from(s: &str) -> Cuboid {
        let (x, y, z) = s
            .split(',')
            .map(Cuboid::_parse_interval)
            .collect_tuple()
            .unwrap();
        Cuboid::new(x, y, z)
    }

    fn intersect_axis(&mut self, limit_interval: Interval<isize>) {
        self.x_range = self.x_range.intersection(&limit_interval);
        self.y_range = self.y_range.intersection(&limit_interval);
        self.z_range = self.z_range.intersection(&limit_interval);
    }

    fn contains(&self, x: &isize, y: &isize, z: &isize) -> bool {
        self.x_range.contains(x)
            && self.y_range.contains(y)
            && self.z_range.contains(z)
            && !self.holes.iter().any(|h| h.contains(x, y, z))
    }

    fn size(&self) -> usize {
        let cuboid_size = self.x_range.size() * self.y_range.size() * self.z_range.size();
        let holes_size: usize = self.holes.iter().map(Cuboid::size).sum();
        cuboid_size - holes_size
    }

    fn is_empty(&self) -> bool {
        self.x_range.is_empty() || self.y_range.is_empty() || self.z_range.is_empty()
    }

    fn intersection(&self, other: &Self) -> Self {
        Cuboid::new(
            self.x_range.intersection(&other.x_range),
            self.y_range.intersection(&other.y_range),
            self.z_range.intersection(&other.z_range),
        )
    }

    fn remove_cuboid(&mut self, other: &Self) {
        // remove cuboid from holes below (a hole is a negative /off cuboid, a hole of a negative cuboid is positive / on again)
        for hole in &mut self.holes {
            hole.remove_cuboid(other);
        }

        // add hole only if intersects
        let new_hole = self.intersection(other);
        if !new_hole.is_empty() {
            self.holes.push(new_hole);
        }
    }
}

fn parse(file_name: &str) -> Vec<(bool, Cuboid)> {
    utils::file_to_lines(file_name)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (s1, s2) = s.split(' ').collect_tuple().unwrap();
            let is_on = "on" == s1;
            (is_on, Cuboid::from(s2))
        })
        .collect_vec()
}

fn apply_instructions(instructions: Vec<(bool, Cuboid)>) -> usize {
    let mut cuboids_on = Vec::<Cuboid>::new();

    for (is_on, cuboid) in instructions {
        // remove new cubiod from existing on Buboids
        for cuboid_on in cuboids_on.iter_mut() {
            cuboid_on.remove_cuboid(&cuboid);
        }

        // add all on cubiods
        if is_on {
            cuboids_on.push(cuboid);
        }
    }
    cuboids_on.iter().map(Cuboid::size).sum()
}

fn solve_part1(file_name: &str) -> usize {
    let mut instructions = parse(file_name);
    instructions
        .iter_mut()
        .for_each(|(_, c)| c.intersect_axis(Interval::new(-50, 50)));
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
