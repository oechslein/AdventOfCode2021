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
type NumberType = u32;

#[derive(Debug)]
struct Board {
    cols: Vec<HashSet<NumberType>>,
    rows: Vec<HashSet<NumberType>>,
}

impl FromStr for Board {
    type Err = String;

    fn from_str(board_str: &str) -> Result<Self, Self::Err> {
        let rows_vec: Vec<Vec<NumberType>> = board_str
            .split("\r\n")
            .map(|row| row.split(' ').filter(|n|!n.is_empty()).map(|n| n.trim().parse::<NumberType>().unwrap()).collect())
            .collect();

        let mut cols = Vec::<HashSet<NumberType>>::new();
        let mut rows = Vec::<HashSet<NumberType>>::new();

        for x in 0..rows_vec.len() {
            let mut row = HashSet::<NumberType>::new();
            for y in 0..rows_vec[0].len() {
                row.insert(rows_vec[x][y]);
            }
            rows.push(row);
        }

        for y in 0..rows_vec[0].len() {
            let mut col = HashSet::<NumberType>::new();
            for x in 0..rows_vec.len() {
                col.insert(rows_vec[x][y]);
            }
            cols.push(col);
        }

        Ok(Board {cols, rows})
    }
}

impl Board {
    fn boards_wins(&self, numbers: &HashSet<NumberType>) -> bool {
        itertools::chain!(self.cols.iter(), self.rows.iter()).any(|row_col| row_col.is_subset(&numbers))
    }

    fn board_sum_of_all_unmarked_numbers(&self, numbers: &HashSet<NumberType>) -> NumberType {
        self.rows.iter().flatten().filter(|x| !numbers.contains(&x)).sum()
    }
}

fn parse_first_line(line: &str) -> Vec<NumberType> {
    line.split(',').map(|n| n.parse::<NumberType>().unwrap()).collect()
}

fn parse_board_lines(input: &mut Split<&str>) -> Vec<Board> {
    input.map(|board_str| Board::from_str(board_str).unwrap()).collect()
}

/// The part1 function calculates the result for part1 // 10110
fn solve_part1(content: String) -> Result<NumberType, String> {
    let mut input = content.split("\r\n\r\n");
    let numbers: Vec<NumberType> = parse_first_line(input.next().unwrap());
    let boards: Vec<Board> = parse_board_lines(&mut input);

    let mut current_number_set = HashSet::<NumberType>::new();

    for number_index in 0..numbers.len() {
        current_number_set.insert(numbers[number_index]);
        for board in boards.iter() {
            if board.boards_wins(&current_number_set) {
                let curr_number = numbers[number_index];
                let sum_of_all_unmarked_numbers = board.board_sum_of_all_unmarked_numbers(&current_number_set);
                return Ok(curr_number * sum_of_all_unmarked_numbers)
            }
        }
    }

    Err(String::from("No boards wins?!"))
}

/// The part2 function calculates the result for part2
fn solve_part2(content: String) -> Result<NumberType, String> {
    let mut input = content.split("\r\n\r\n");
    let numbers: Vec<NumberType> = parse_first_line(input.next().unwrap());
    let boards: Vec<Board> = parse_board_lines(&mut input);

    let mut current_number_set = HashSet::<NumberType>::new();

    let mut winning_boards = HashSet::<usize>::new();

    for number_index in 0..numbers.len() {
        current_number_set.insert(numbers[number_index]);
        for (board_index, board) in boards.iter().enumerate() {
            if !winning_boards.contains(&board_index) && board.boards_wins(&current_number_set){
                winning_boards.insert(board_index);
                if winning_boards.len() == boards.len() {
                    let curr_number = numbers[number_index];
                    let sum_of_all_unmarked_numbers = board.board_sum_of_all_unmarked_numbers(&current_number_set);
                    return Ok(curr_number * sum_of_all_unmarked_numbers)
                }
            }
        }
    }

    Err(String::from("No boards wins?!"))
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
            4512
        );
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("test.txt")).unwrap(),
            1924
        );
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part1(utils::file_to_string("input.txt")).unwrap(),
            8442
        );
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            solve_part2(utils::file_to_string("input.txt")).unwrap(),
            4590
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
