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

//use counter::Counter;
use itertools::Itertools;
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

type DotsType = Vec<(usize, usize)>;
type FoldingsType = Vec<(char, usize)>;

struct FoldProblem {
    dots: DotsType,
    folds: FoldingsType,
}

impl FromStr for FoldProblem {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let (dots_content, folds_content) = content.split("\r\n\r\n").collect_tuple().unwrap();

        let dots = dots_content
            .split("\r\n")
            .map(|line| {
                line.split(',')
                    .map(|sub_line| sub_line.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<DotsType>();

        let folds = folds_content
            .split("\r\n")
            .map(|line| {
                let line = line.to_string().replace("fold along ", "");
                let (axis, position) = line.split('=').collect_tuple().unwrap();
                let axis_char = axis.chars().next().unwrap();
                let fold_position = position.parse::<usize>().unwrap();
                (axis_char, fold_position)
            })
            .collect::<FoldingsType>();

        Ok(FoldProblem {
            dots: dots,
            folds: folds,
        })
    }
}

impl FoldProblem {
    fn apply_foldings_to_dot(&mut self, index: usize) -> bool {
        let dot = &mut self.dots[index];
        for (fold_axis, fold_pos) in self.folds.iter() {
            let dot_coor_ref: &mut usize;
            if *fold_axis == 'x' {
                dot_coor_ref = &mut dot.0;
            } else {
                assert_eq!(*fold_axis, 'y');
                dot_coor_ref = &mut dot.1;
            }

            let dot_corr = *dot_coor_ref;
            match (dot_corr).cmp(fold_pos) {
                Ordering::Less => {}
                Ordering::Equal => {
                    // Break from for loop, indicating that item needs to be removed
                    return false; 
                }
                Ordering::Greater => {
                    *dot_coor_ref = 2 * fold_pos - dot_corr;
                }
            }
        }
        return true;
    }

    fn apply_foldings(&mut self) {
        let mut index = 0;
        while index < self.dots.len() {
            if !self.apply_foldings_to_dot(index) {
                self.dots.remove(index);
                // skip index += 1, since we removed current item
            } else {
                index += 1;
            }
        }
    }

    fn get_unique_dots_count(&self) -> usize {
        self.dots.iter().collect::<HashSet<_>>().len()
    }

    fn print_dots(&self) {
        let dots: &DotsType = &self.dots;
        let max_x = dots.iter().map(|(dot_x, _)| dot_x).max().unwrap();
        let max_y = dots.iter().map(|(_, dot_y)| dot_y).max().unwrap();
        for y in 0..=*max_y {
            for x in 0..=*max_x {
                if dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }    
}

/// The part1 function calculates the result for part2
fn solve_part1(file_name: &str) -> Result<usize, String> {
    // read dots in Vec<(usize, usize)>
    // fold on y=y_fold means
    // all (x,y) with y < y_fold remains same
    // all (x,y) with y = y_fold will be erased
    // all (x,y) with y > y_fold will be changed to (x,y_fold*2-y)
    //
    // Assumption: fold position is always half of width/height
    //
    // all folds can be done per position!

    let mut fold_problem =
        FoldProblem::from_str(utils::file_to_string(file_name).as_str()).unwrap();
    //println!("folds: {:?}", folds);
    //fold_problem.print_dots();

    // just first fold
    fold_problem.folds.drain(1..fold_problem.folds.len());
    fold_problem.apply_foldings();
    //fold_problem.print_dots();

    Ok(fold_problem.get_unique_dots_count())
}

/// The part2 function calculates the result for part2
fn solve_part2(file_name: &str) -> Result<usize, String> {
    let mut fold_problem =
        FoldProblem::from_str(utils::file_to_string(file_name).as_str()).unwrap();
    //println!("folds: {:?}", folds);
    //fold_problem.print_dots();

    fold_problem.apply_foldings();
    fold_problem.print_dots();

    Ok(fold_problem.get_unique_dots_count())
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 17);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 706);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 16);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 95); // LRFJBJEH
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
