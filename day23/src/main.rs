#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]
#![feature(hash_drain_filter)]

extern crate test;

use itertools::{cloned, Itertools};

use core::num;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::{cmp, cmp::Ordering, fmt};

use std::error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;

use fxhash::{FxHashMap, FxHashSet};

use std::ops::{Range, RangeInclusive};

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Location {
    Empty,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl Location {
    fn get_symbol(&self) -> String {
        (match self {
            Location::Amber => "A",
            Location::Bronze => "B",
            Location::Copper => "C",
            Location::Desert => "D",
            Location::Empty => ".",
        })
        .to_string()
    }

    fn value(&self) -> usize {
        match self {
            Location::Amber => 1,
            Location::Bronze => 10,
            Location::Copper => 100,
            Location::Desert => 1000,
            Location::Empty => 0,
        }
    }

    fn door_index(&self) -> usize {
        match self {
            Location::Amber => 0,
            Location::Bronze => 1,
            Location::Copper => 2,
            Location::Desert => 3,
            Location::Empty => panic!("Tried to get invalid door_index."),
        }
    }

    fn by_door_index(i: usize) -> Self {
        match i {
            0 => Location::Amber,
            1 => Location::Bronze,
            2 => Location::Copper,
            3 => Location::Desert,
            _ => panic!("Tried to get invalid location (index: {}).", i),
        }
    }
}

#[derive(Clone)]
struct World {
    hallway: [Location; 11],
    rooms: [[Location; 4]; 4],
    score: usize,
}

impl Hash for World {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..self.hallway.len() {
            self.hallway[i].hash(state);
        }
        for i in 0..self.rooms.len() {
            self.rooms[i].hash(state);
        }
        self.score.hash(state);
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=============\nScore: {}\n#############\n#{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#  \n  #{}#{}#{}#{}#  \n  #{}#{}#{}#{}#  \n  #########  ", 
        self.score,
        self.hallway.map(| l | l.get_symbol()).join(""),
        self.rooms[0][0].get_symbol(),
        self.rooms[1][0].get_symbol(),
        self.rooms[2][0].get_symbol(),
        self.rooms[3][0].get_symbol(),
        self.rooms[0][1].get_symbol(),
        self.rooms[1][1].get_symbol(),
        self.rooms[2][1].get_symbol(),
        self.rooms[3][1].get_symbol(),
        self.rooms[0][2].get_symbol(),
        self.rooms[1][2].get_symbol(),
        self.rooms[2][2].get_symbol(),
        self.rooms[3][2].get_symbol(),
        self.rooms[0][3].get_symbol(),
        self.rooms[1][3].get_symbol(),
        self.rooms[2][3].get_symbol(),
        self.rooms[3][3].get_symbol())
    }
}
impl Ord for World {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for World {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for World {}

impl World {
    fn new(rooms: [[Location; 4]; 4]) -> Self {
        World {
            hallway: [
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
                Location::Empty,
            ],
            rooms,
            score: 0,
        }
    }

    fn finished(&self) -> bool {
        !self
            .rooms
            .iter()
            .enumerate()
            .any(|(i, s)| s.iter().any(|l| *l != Location::by_door_index(i))) // TODO change not using by_door_index?
    }

    fn is_hallway_empty(&self, mut from: usize, mut to: usize, distance: usize) -> bool {
        if from > to {
            let t = to;
            to = from - distance;
            from = t;
        } else {
            from += distance;
        }
        !(from..=to)
            .into_iter()
            .any(|i| self.hallway[i] != Location::Empty)
    }

    fn move_into_rooms(
        &self,
        visited: &mut FxHashSet<String>,
        stack: &mut BinaryHeap<World>,
    ) -> bool {
        let mut moved_into_room = false;
        for (i, a) in self
            .hallway
            .iter()
            .enumerate()
            .filter(|(_, a)| *(*a) != Location::Empty)
        {
            let room_door_index = a.door_index();
            if self.is_hallway_empty(i, room_door_index * 2 + 2, 1)
                && !self.rooms[room_door_index].iter().any(|l| {
                    *l != Location::by_door_index(room_door_index) && *l != Location::Empty
                })
            {
                if let Some(r_i) = self.rooms[room_door_index]
                    .iter()
                    .rposition(|l| *l == Location::Empty)
                {
                    let to_be_moved = self.hallway[i];
                    let mut new_situation = self.clone();
                    new_situation.rooms[room_door_index][r_i] = to_be_moved;
                    new_situation.hallway[i] = Location::Empty;
                    new_situation.score += to_be_moved.value()
                        * (cmp::max(i, room_door_index * 2 + 2)
                            - cmp::min(i, room_door_index * 2 + 2)
                            + r_i
                            + 1);
                    if !visited.contains(&new_situation.to_string()) {
                        visited.insert(new_situation.to_string());
                        stack.push(new_situation);
                    }
                    moved_into_room = true;
                }
            }
        }
        moved_into_room
    }

    fn move_into_hallway(&self, visited: &mut FxHashSet<String>, stack: &mut BinaryHeap<World>) {
        let possible_end_pos = [0, 1, 3, 5, 7, 9, 10];

        for (sr_i, sr) in self.rooms.iter().enumerate().filter(|(i, sr)| {
            sr.iter()
                .any(|l| *l != Location::Empty && *l != Location::by_door_index(*i))
        }) {
            if let Some(r_i) = sr.iter().position(|a| *a != Location::Empty) {
                let hallway_index = sr_i * 2 + 2;
                for i in possible_end_pos {
                    if self.is_hallway_empty(i, hallway_index, 0) {
                        let to_be_moved = sr[r_i];
                        let mut new_situation = self.clone();
                        new_situation.rooms[sr_i][r_i] = Location::Empty;
                        new_situation.hallway[i] = to_be_moved;
                        new_situation.score += to_be_moved.value()
                            * (cmp::max(i, hallway_index) - cmp::min(i, hallway_index) + r_i + 1);
                        if !visited.contains(&new_situation.to_string()) {
                            visited.insert(new_situation.to_string());
                            stack.push(new_situation);
                        }
                    }
                }
            }
        }
    }

    fn get_least_energy(self) -> Option<usize> {
        let mut visited = FxHashSet::default();
        visited.insert(self.to_string());
        let mut stack = BinaryHeap::new();
        stack.push(self);
        while let Some(current) = stack.pop() {
            if current.finished() {
                println!("{}", current);
                return Some(current.score);
            }

            if !current.move_into_rooms(&mut visited, &mut stack) {
                current.move_into_hallway(&mut visited, &mut stack);
            }
        }
        None
    }
}

fn get_range(from: usize, to: usize) -> Range<usize> {
    // from is own position, exclude it
    if from < to {
        from + 1..to + 1
    } else {
        assert!(from != to);
        to..from
    }
}

fn solve_part1(file_name: &str) -> usize {
    let s = World::new([
        [
            Location::Copper,
            Location::Bronze,
            Location::Amber,
            Location::Amber,
        ],
        [
            Location::Desert,
            Location::Amber,
            Location::Bronze,
            Location::Bronze,
        ],
        [
            Location::Amber,
            Location::Desert,
            Location::Copper,
            Location::Copper,
        ],
        [
            Location::Bronze,
            Location::Copper,
            Location::Desert,
            Location::Desert,
        ],
    ]);
    s.get_least_energy().unwrap()
}

fn solve_part2(file_name: &str) -> usize {
    let s = World::new([
        [
            Location::Copper,
            Location::Desert,
            Location::Desert,
            Location::Bronze,
        ],
        [
            Location::Desert,
            Location::Copper,
            Location::Bronze,
            Location::Amber,
        ],
        [
            Location::Amber,
            Location::Bronze,
            Location::Amber,
            Location::Desert,
        ],
        [
            Location::Bronze,
            Location::Amber,
            Location::Copper,
            Location::Copper,
        ],
    ]);
    s.get_least_energy().unwrap()
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
