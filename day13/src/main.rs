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

fn _parse_content(file_name: &str) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    let content = fs::read_to_string(file_name).unwrap();
    let (dots, folds) = content.split("\r\n\r\n").collect_tuple().unwrap();

    let dots = dots
        .split("\r\n")
        .map(|line| {
            line.split(',')
                .map(|sub_line| sub_line.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<HashSet<(usize, usize)>>();

    let folds = folds
        .split("\r\n")
        .map(|line| {
            let line = line.to_string().replace("fold along ", "");
            let (axis, position) = line.split('=').collect_tuple().unwrap();
            let axis_char = axis.chars().next().unwrap();
            let fold_position = position.parse::<usize>().unwrap();
            (axis_char, fold_position)
        })
        .collect::<Vec<(char, usize)>>();

    (dots, folds)
}

fn print_dots(dots: &HashSet<(usize, usize)>) {
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

fn fold_one(
    dots: HashSet<(usize, usize)>,
    fold_axis: char,
    fold_pos: usize,
) -> HashSet<(usize, usize)> {
    let mut new_dots = HashSet::new();
    if fold_axis == 'y' {
        for (dot_x, dot_y) in dots.iter() {
            if dot_y < &fold_pos {
                new_dots.insert((*dot_x, *dot_y));
            } else if dot_y == &fold_pos {
                // erase
            } else {
                // dot_y > &fold_pos
                let new_dot_y = 2 * fold_pos - dot_y;
                new_dots.insert((*dot_x, new_dot_y));
            }
        }
    } else {
        assert_eq!(fold_axis, 'x');
        for (dot_x, dot_y) in dots.iter() {
            if dot_x < &fold_pos {
                new_dots.insert((*dot_x, *dot_y));
            } else if dot_x == &fold_pos {
                // erase
            } else {
                // dot_x > &fold_pos
                let new_dot_x = 2 * fold_pos - dot_x;
                new_dots.insert((new_dot_x, *dot_y));
            }
        }
    }
    new_dots
}

fn fold_all(dots: HashSet<(usize, usize)>, folds: &Vec<(char, usize)>) -> HashSet<(usize, usize)> {
    let mut new_dots = HashSet::new();
    for (mut dot_x, mut dot_y) in dots.iter() {
        let mut erase = false;
        for (fold_axis, fold_pos) in folds.iter() {
            if *fold_axis == 'y' {
                if dot_y < *fold_pos {
                    // ignore
                } else if dot_y == *fold_pos {
                    erase = true;
                    break;
                } else {
                    // dot_y > &fold_pos
                    dot_y = 2 * fold_pos - dot_y;
                }
            } else {
                assert_eq!(*fold_axis, 'x');
                if dot_x < *fold_pos {
                    // ignore
                } else if dot_x == *fold_pos {
                    erase = true;
                    break;
                } else {
                    // dot_x > &fold_pos
                    dot_x = 2 * fold_pos - dot_x;
                }
            }
        }
        if !erase {
            new_dots.insert((dot_x, dot_y));
        }
    }
    new_dots
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

    let (mut dots, folds) = _parse_content(file_name);
    //println!("folds: {:?}", folds);
    //print_dots(&dots);

    for (fold_axis, fold_pos) in folds {
        dots = fold_one(dots, fold_axis, fold_pos);
        //print_dots(&dots);
        break;
    }

    Ok(dots.len())
}

/// The part2 function calculates the result for part2
fn solve_part2(file_name: &str) -> Result<usize, String> {
    let (mut dots, folds) = _parse_content(file_name);
    //println!("folds: {:?}", folds);
    //print_dots(&dots);

    /*
    for (fold_axis, fold_pos) in folds {
        dots = fold(dots, fold_axis, fold_pos);
    }
    */
    dots = fold_all(dots, &folds);

    print_dots(&dots);
    Ok(dots.len())
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
