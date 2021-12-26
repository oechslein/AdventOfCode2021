#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]
#![feature(hash_drain_filter)]
#![feature(generic_arg_infer)]

extern crate test;

use itertools::{cloned, Itertools};

use core::num;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::{cmp, cmp::Ordering, fmt};

use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, Index, IndexMut, RangeFrom, SubAssign};
use std::str::FromStr;
use std::{error, mem};

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

type Location = Option<Amphipod>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl Amphipod {
    fn get_symbol(&self) -> String {
        (match self {
            Amphipod::Amber => "A",
            Amphipod::Bronze => "B",
            Amphipod::Copper => "C",
            Amphipod::Desert => "D",
        })
        .to_string()
    }

    fn value(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn room_index(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }

    fn costs(
        &self,
        from_hallway_index: usize,
        to_hallway_index: usize,
        room_location_index: usize,
    ) -> usize {
        self.value() * move_costs(from_hallway_index, to_hallway_index, room_location_index)
    }

    fn estimate_in_room(&self, from_room_index: usize, room_location_index: usize) -> usize {
        let from_hallway_index = room_to_hallway_index(from_room_index);
        let to_hallway_index = room_to_hallway_index(self.room_index());

        self.value() * (move_costs(from_hallway_index, to_hallway_index, room_location_index) + room_location_index)
    }

    fn estimate_in_hallway(&self, from_hallway_index: usize, room_location_index: usize) -> usize {
        let to_hallway_index = room_to_hallway_index(self.room_index());

        self.value() * move_costs(from_hallway_index, to_hallway_index, room_location_index)
    }
}

fn move_costs(
    from_hallway_index: usize,
    to_hallway_index: usize,
    room_location_index: usize,
) -> usize {
    from_hallway_index.max(to_hallway_index) - from_hallway_index.min(to_hallway_index)
        + room_location_index
        + 1
}

#[derive(Clone)]
struct Room<const SIZE: usize> {
    locations: [Location; SIZE],
}

impl<const SIZE: usize> Hash for Room<SIZE> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.locations.hash(state);
    }
}

impl<const SIZE: usize> Index<usize> for Room<SIZE> {
    type Output = Location;

    fn index(&self, index: usize) -> &Self::Output {
        &self.locations[index]
    }
}

impl<const SIZE: usize> IndexMut<usize> for Room<SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.locations[index]
    }
}

impl<const SIZE: usize> Room<SIZE> {
    fn new(locations: [Location; SIZE]) -> Room<SIZE> {
        Room { locations }
    }

    fn iter(&self) -> std::slice::Iter<Option<Amphipod>> {
        self.locations.iter()
    }

    fn finished(&self, room_index: usize) -> bool {
        self.iter()
            .all(|l| l.is_some() && l.as_ref().unwrap().room_index() == room_index)
    }

    fn estimate(&self, room_index: usize) -> usize {
        if self.finished(room_index) {
            0
        } else {
            self.iter()
                .map(|l| {
                    l.as_ref()
                        .map_or(0, |a| a.estimate_in_room(room_index, SIZE - 1))
                })
                .sum()
        }
    }
}

#[derive(Clone)]
struct World<const ROOM_SIZE: usize> {
    hallway: [Location; 11],
    rooms: [Room<ROOM_SIZE>; 4],
    score: usize,
    estimate: usize,
}

impl<const ROOM_SIZE: usize> Hash for World<ROOM_SIZE> {
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

impl<const ROOM_SIZE: usize> fmt::Display for World<ROOM_SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _get_symbol = |loc: &Location| {
            if let Some(a) = loc {
                a.get_symbol()
            } else {
                ".".to_string()
            }
        };
        let room_lines = (0..ROOM_SIZE)
            .map(|i| {
                "###".to_string() + &self.rooms.iter().map(|r| _get_symbol(&r[i])).join("#") + "###"
            })
            .join("\n");
        let hallway_line = self.hallway.iter().map(|l| _get_symbol(l)).join("");

        write!(
            f,
            "=============\nScore: {} Estimate: {}\n#############\n#{}#\n{}\n#############",
            self.score, self.estimate, hallway_line, room_lines
        )
    }
}

// for BinaryHeap
impl<const ROOM_SIZE: usize> Ord for World<ROOM_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .add(other.estimate)
            .cmp(&self.score.add(self.estimate))
    }
}
impl<const ROOM_SIZE: usize> PartialOrd for World<ROOM_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            other
                .score
                .add(other.estimate)
                .cmp(&self.score.add(self.estimate)),
        )
    }
}
impl<const ROOM_SIZE: usize> PartialEq for World<ROOM_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.estimate == other.estimate
    }
}
impl<const ROOM_SIZE: usize> Eq for World<ROOM_SIZE> {}

impl<const ROOM_SIZE: usize> World<ROOM_SIZE> {
    fn new(rooms: [Room<ROOM_SIZE>; 4]) -> Self {
        let mut world = World {
            hallway: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
            rooms,
            score: 0,
            estimate: 0,
        };
        world.update_estimate();
        world
    }

    fn update_estimate(&mut self) {
        let hallway_estimate: usize = self
            .hallway
            .iter()
            .enumerate()
            .map(|(from_hallway_index, l)| {
                l.as_ref()
                    .map_or(0, |a| a.estimate_in_hallway(from_hallway_index, ROOM_SIZE))
            })
            .sum();
        let into_correct_rooms_estimate: usize = self
            .rooms
            .iter()
            .enumerate()
            .map(|(from_room_index, r)| ROOM_SIZE * r.estimate(from_room_index))
            .sum();
        self.estimate = hallway_estimate + into_correct_rooms_estimate;
    }

    fn finished(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(room_index, room)| room.finished(room_index))
    }

    fn is_hallway_empty(&self, from: usize, to: usize, skip_from: bool) -> bool {
        let distance = if skip_from { 1 } else { 0 };
        if from > to {
            (to..=from - distance)
                .into_iter()
                .all(|i| self.hallway[i].is_none())
        } else {
            (from + distance..=to)
                .into_iter()
                .all(|i| self.hallway[i].is_none())
        }
    }

    fn _not_empty_but_in_wrong_room(l: &Location, room_index: usize) -> bool {
        l.as_ref()
            .map_or(false, |a: &Amphipod| a.room_index() != room_index)
    }

    fn move_into_rooms(
        &self,
        visited: &mut FxHashSet<World<ROOM_SIZE>>,
        stack: &mut BinaryHeap<World<ROOM_SIZE>>,
    ) -> bool {
        let mut moved_into_room = false;
        for (curr_hallway_index, l) in self.hallway.iter().enumerate() {
            if let Some(a) = l {
                let room_index = a.room_index();
                let to_hallway_index = room_to_hallway_index(room_index);
                if self.is_hallway_empty(curr_hallway_index, to_hallway_index, true)
                    && self.rooms[room_index]
                        .iter()
                        .all(|l| !World::<ROOM_SIZE>::_not_empty_but_in_wrong_room(l, room_index))
                {
                    if let Some(room_location_index) =
                        self.rooms[room_index].iter().rposition(|l| l.is_none())
                    {
                        let mut new_world = self.clone();
                        mem::swap(
                            &mut new_world.hallway[curr_hallway_index],
                            &mut new_world.rooms[room_index][room_location_index],
                        );
                        new_world.score +=
                            a.costs(curr_hallway_index, to_hallway_index, room_location_index);
                        new_world.update_estimate();
                        if !visited.contains(&new_world) {
                            visited.insert(new_world.clone());
                            stack.push(new_world);
                        }
                        moved_into_room = true;
                    }
                }
            }
        }
        moved_into_room
    }

    fn move_into_hallway(
        &self,
        visited: &mut FxHashSet<World<ROOM_SIZE>>,
        stack: &mut BinaryHeap<World<ROOM_SIZE>>,
    ) {
        for (from_room_index, room) in self.rooms.iter().enumerate().filter(|(room_index, room)| {
            room.iter()
                .any(|l| World::<ROOM_SIZE>::_not_empty_but_in_wrong_room(l, *room_index))
        }) {
            if let Some(room_location_index) = room.iter().position(|a| a.is_some()) {
                static POSSIBLE_HALLWAY_INDEXES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
                let from_hallway_index = room_to_hallway_index(from_room_index);
                for to_hallway_index in POSSIBLE_HALLWAY_INDEXES {
                    if self.is_hallway_empty(to_hallway_index, from_hallway_index, false) {
                        let to_be_moved = room[room_location_index].as_ref().unwrap();
                        let mut new_world = self.clone();
                        mem::swap(
                            &mut new_world.rooms[from_room_index][room_location_index],
                            &mut new_world.hallway[to_hallway_index],
                        );
                        new_world.score += to_be_moved.costs(
                            from_hallway_index,
                            to_hallway_index,
                            room_location_index,
                        );
                        if !visited.contains(&new_world) {
                            visited.insert(new_world.clone());
                            stack.push(new_world);
                        }
                    }
                }
            }
        }
    }

    fn get_least_energy(self) -> Option<usize> {
        println!("{}", self);
        let mut visited = FxHashSet::default();
        visited.insert(self.clone());
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

fn room_to_hallway_index(room_index: usize) -> usize {
    room_index * 2 + 2
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

fn solve_part1(_file_name: &str) -> usize {
    let s = World::new([
        Room::new([Some(Amphipod::Copper), Some(Amphipod::Bronze)]),
        Room::new([Some(Amphipod::Desert), Some(Amphipod::Amber)]),
        Room::new([Some(Amphipod::Amber), Some(Amphipod::Desert)]),
        Room::new([Some(Amphipod::Bronze), Some(Amphipod::Copper)]),
    ]);
    s.get_least_energy().unwrap()
}

fn solve_part2(_file_name: &str) -> usize {
    let s = World::new([
        Room::new([
            Some(Amphipod::Copper),
            Some(Amphipod::Desert),
            Some(Amphipod::Desert),
            Some(Amphipod::Bronze),
        ]),
        Room::new([
            Some(Amphipod::Desert),
            Some(Amphipod::Copper),
            Some(Amphipod::Bronze),
            Some(Amphipod::Amber),
        ]),
        Room::new([
            Some(Amphipod::Amber),
            Some(Amphipod::Bronze),
            Some(Amphipod::Amber),
            Some(Amphipod::Desert),
        ]),
        Room::new([
            Some(Amphipod::Bronze),
            Some(Amphipod::Amber),
            Some(Amphipod::Copper),
            Some(Amphipod::Copper),
        ]),
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
        assert_eq!(solve_part1("test.txt"), 13556);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 13556);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test2.txt"), 54200);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 54200);
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
