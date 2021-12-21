#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]

extern crate test;

use counter::Counter;
use itertools::Itertools;

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pawn {
    pos: u64,
    score: u64,
    win_amount: u64,
    track_length: u64,
}

impl Pawn {
    fn new(win_amount: u64, track_length: u64, pos: u64) -> Pawn {
        Pawn {
            win_amount,
            track_length,
            pos,
            score: 0,
        }
    }

    fn update_pos_wins_p(&mut self, dice_result: u64) -> bool {
        self.pos = (((self.pos - 1) + dice_result) % self.track_length) + 1;
        self.score += self.pos;
        self.score >= self.win_amount
    }
}

trait Dice {
    fn multi_role(&mut self, amount: u64) -> Counter<u64, u64>;
}

#[derive(Debug, Clone)]
struct DeterministicDice {
    sides: u64,
    _next_roll: u64,
    _total_roles: u64,
}

impl DeterministicDice {
    fn new(sides: u64) -> DeterministicDice {
        DeterministicDice {
            sides,
            _next_roll: 1,
            _total_roles: 0,
        }
    }

    fn role(&mut self) -> u64 {
        let result = self._next_roll;
        self._next_roll += 1;
        if self._next_roll > self.sides {
            self._next_roll = 1;
        }
        self._total_roles += 1;
        result
    }

    fn get_total_roles(&self) -> u64 {
        self._total_roles
    }
}

impl Dice for DeterministicDice {
    fn multi_role(&mut self, amount: u64) -> Counter<u64, u64> {
        let mut result = Counter::new();
        result.insert((0..amount).map(|_| self.role()).sum(), 1);
        result
    }
}

#[derive(Debug, Clone)]
struct DiracDice {
    sides: u64,
    _cache: HashMap<u64, Counter<u64, u64>>,
}

impl Dice for DiracDice {
    fn multi_role(&mut self, amount: u64) -> Counter<u64, u64> {
        self._cache
            .entry(amount)
            .or_insert_with(|| {
                let mut result: Vec<u64> = vec![0];
                for _ in 0..amount {
                    result = result
                        .into_iter()
                        .cartesian_product(1..=self.sides)
                        .map(|(a, b)| a + b)
                        .collect_vec();
                }
                result.into_iter().collect()
            })
            .clone()
    }
}

impl DiracDice {
    fn new(sides: u64) -> DiracDice {
        DiracDice {
            sides,
            _cache: HashMap::new(),
        }
    }
}

fn solve(
    win_amount: u64,
    track_length: u64,
    pos1: u64,
    pos2: u64,
    dice: &mut dyn Dice,
) -> (u64, u64, u64, u64) {
    let mut possible_pawns: Counter<(Pawn, Pawn), u64> = Counter::new();
    let pawn1 = Pawn::new(win_amount, track_length, pos1);
    let pawn2 = Pawn::new(win_amount, track_length, pos2);
    possible_pawns.insert((pawn1, pawn2), 1);
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let mut player1_loses_scores = 0;
    let mut player2_loses_scores = 0;
    while !possible_pawns.is_empty() {
        let mut new_possible_pawns: Counter<(Pawn, Pawn), u64> = Counter::new();
        for ((pawn1, pawn2), pawns_amount) in possible_pawns.iter() {
            for (dice_result_p1, dice_result_p1_amount) in dice.multi_role(3).into_iter() {
                let mut new_pawn1 = pawn1.clone();
                if new_pawn1.update_pos_wins_p(*dice_result_p1) {
                    player1_wins += pawns_amount * dice_result_p1_amount;
                    player2_loses_scores += pawn2.score;
                    continue;
                }

                for (dice_result_p2, dice_result_p2_amount) in dice.multi_role(3).into_iter() {
                    let mut new_pawn2 = pawn2.clone();
                    if new_pawn2.update_pos_wins_p(*dice_result_p2) {
                        player2_wins +=
                            pawns_amount * dice_result_p2_amount * dice_result_p1_amount;
                        player1_loses_scores += new_pawn1.score;
                    } else {
                        new_possible_pawns[&(new_pawn1.clone(), new_pawn2)] +=
                            pawns_amount * dice_result_p2_amount * dice_result_p1_amount;
                    }
                }
            }
        }
        possible_pawns = new_possible_pawns;
    }
    (
        player1_wins,
        player2_wins,
        player1_loses_scores,
        player2_loses_scores,
    )
}

fn parse_input(file_name: &str) -> (u64, u64) {
    if file_name == "input.txt" {
        (1, 3)
    } else {
        (4, 8)
    }
}

fn solve_part1(file_name: &str) -> u64 {
    let (pos1, pos2) = parse_input(file_name);

    let mut dice = DeterministicDice::new(100);
    let (player1_wins, player2_wins, player1_loses_scores, player2_loses_scores) =
        solve(1000, 10, pos1, pos2, &mut dice);

    (if player1_wins >= player2_wins {
        player2_loses_scores
    } else {
        player1_loses_scores
    }) * dice.get_total_roles()
}

fn solve_part2(file_name: &str) -> u64 {
    let (pos1, pos2) = parse_input(file_name);

    let (player1_wins, player2_wins, _, _) = solve(21, 10, pos1, pos2, &mut DiracDice::new(3));

    player1_wins.max(player2_wins)
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
        assert_eq!(solve_part1("test.txt"), 739785);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 897798);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 444356092776315);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 48868319769358);
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
