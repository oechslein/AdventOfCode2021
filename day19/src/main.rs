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
use std::collections::{HashSet, VecDeque};
use std::error;
use std::fmt::{Debug, Display};
use std::ops::{Add, RangeFrom, SubAssign};

mod utils;

use glam::IVec3;

#[macro_use]
extern crate lazy_static;

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
struct Orientation {
    x_inverted: bool,
    y_inverted: bool,
    z_inverted: bool,
    x_index: usize,
    y_index: usize,
    z_index: usize,
}

impl Orientation {
    fn identity() -> Orientation {
        Orientation {
            x_inverted: false,
            y_inverted: false,
            z_inverted: false,
            x_index: 0,
            y_index: 1,
            z_index: 2,
        }
    }
    fn all_orientations() -> &'static Vec<Orientation> {
        lazy_static! {
            static ref ALL_ORIENTATIONS: Vec<Orientation> = {
                let mut all_orientations = Vec::new();
                for x_inverted in [true, false] {
                    for y_inverted in [true, false] {
                        for z_inverted in [true, false] {
                            for x_index in 0..3 {
                                for y_index in 0..3 {
                                    if x_index != y_index {
                                        let z_index = 3 - x_index - y_index;
                                        all_orientations.push(Orientation {
                                            x_inverted,
                                            y_inverted,
                                            z_inverted,
                                            x_index,
                                            y_index,
                                            z_index,
                                        })
                                    }
                                }
                            }
                        }
                    }
                }

                all_orientations
            };
        }

        &ALL_ORIENTATIONS
    }
}

fn apply_orientation(point: &IVec3, orientation: &Orientation) -> IVec3 {
    let data = point.to_array();
    let mut x = data[orientation.x_index];
    let mut y = data[orientation.y_index];
    let mut z = data[orientation.z_index];
    if orientation.x_inverted {
        x = -x;
    }
    if orientation.y_inverted {
        y = -y;
    }
    if orientation.z_inverted {
        z = -z;
    }
    IVec3::new(x, y, z)
}

fn apply_translation(point: IVec3, point_by: IVec3) -> IVec3 {
    point + point_by
}

fn get_translation(point: IVec3, point_to: IVec3) -> IVec3 {
    point_to - point
}

fn manhattan_distance(point: IVec3, other: IVec3) -> usize {
    let dist = (point - other).abs();
    dist.x as usize + dist.y as usize + dist.z as usize
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ScannerReadings {
    scanner_position: IVec3,
    beacon_positions: HashSet<IVec3>,
}

impl Display for ScannerReadings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ScannerReadings<S:{},{}>",
            self.scanner_position,
            self.beacon_positions
                .iter()
                .map(|pos| pos.to_string())
                .collect::<String>()
        ))
    }
}

impl ScannerReadings {
    fn new(beacon_positions: HashSet<IVec3>) -> ScannerReadings {
        ScannerReadings {
            scanner_position: IVec3::new(0, 0, 0),
            beacon_positions,
        }
    }

    fn apply_orientation(&self, orientation: &Orientation) -> ScannerReadings {
        ScannerReadings {
            scanner_position: apply_orientation(&self.scanner_position, orientation),
            beacon_positions: self
                .beacon_positions
                .iter()
                .map(|beacon_position| apply_orientation(beacon_position, orientation))
                .collect(),
        }
    }

    fn apply_translation(&self, point_by: &IVec3) -> ScannerReadings {
        ScannerReadings {
            scanner_position: apply_translation(self.scanner_position, *point_by),
            beacon_positions: self
                .beacon_positions
                .iter()
                .map(|beacon_position| apply_translation(*beacon_position, *point_by))
                .collect(),
        }
    }

    fn matching_positions(&self, full_list_of_correct_beacon_positions: &HashSet<IVec3>) -> usize {
        let beacon_positions_set: &HashSet<IVec3> = &self.beacon_positions;
        let other_beacon_positions_set = full_list_of_correct_beacon_positions;

        beacon_positions_set
            .intersection(other_beacon_positions_set)
            .count()
    }

    fn get_possible_oriented_translated_scanner_readings(
        &self,
        full_list_of_correct_beacon_positions: &HashSet<IVec3>,
    ) -> Option<ScannerReadings> {
        for point in full_list_of_correct_beacon_positions {
            for orientation in Orientation::all_orientations() {
                let scanner_orientation = self.apply_orientation(orientation);
                for beacon_position_s1 in scanner_orientation.beacon_positions.iter() {
                    let translation = get_translation(*beacon_position_s1, *point);
                    let scanner_translated = scanner_orientation.apply_translation(&translation);
                    let matching_positions = scanner_translated
                        .matching_positions(full_list_of_correct_beacon_positions);
                    if matching_positions >= 12 {
                        // TODO the remaining ones need to be 1000 away from scanner (both)
                        return Some(scanner_translated);
                    }
                }
            }
        }
        None
    }
}

fn parse(content: &str) -> VecDeque<ScannerReadings> {
    content
        .replace("\r", "")
        .trim()
        .split("\n\n")
        .map(|scanner_reading_block| {
            let beacon_positions = scanner_reading_block
                .split('\n')
                .skip(1)
                .map(|scanner_reading_line| {
                    //println!("scanner_reading_line: {:?}", scanner_reading_line);
                    let (x, y, z) = scanner_reading_line
                        .split(',')
                        .map(|point_str| point_str.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    IVec3::new(x, y, z)
                })
                .collect();
            ScannerReadings::new(beacon_positions)
        })
        .collect()
}

fn solve(file_name: &str) -> (usize, usize) {
    let mut open_scanner_readings = parse(&utils::file_to_string(file_name));
    let mut done_scanner_readings = VecDeque::new();

    let scanner_0 = open_scanner_readings.pop_front().unwrap();
    let mut list_of_correct_beacon_positions: HashSet<IVec3> = scanner_0
        .beacon_positions
        .iter()
        .copied()
        .collect::<HashSet<IVec3>>();
    done_scanner_readings.push_back(scanner_0);

    while let Some(scanner) = open_scanner_readings.pop_front() {
        if let Some(scanner_modified) = scanner
            .get_possible_oriented_translated_scanner_readings(&list_of_correct_beacon_positions)
        {
            list_of_correct_beacon_positions.extend(&scanner_modified.beacon_positions);
            done_scanner_readings.push_back(scanner_modified);

        } else {
            open_scanner_readings.push_back(scanner);
        }
    }

    let max_manhattan_distance = done_scanner_readings
        .iter()
        .map(|scanner_reading| scanner_reading.scanner_position)
        .tuple_combinations()
        .map(|(scanner_pos_a, scanner_pos_b)| manhattan_distance(scanner_pos_a, scanner_pos_b))
        .max()
        .unwrap();

    (list_of_correct_beacon_positions.len(), max_manhattan_distance)
}

fn solve_part1(file_name: &str) -> Result<usize, String> {
    let (beacon_count, _) = solve(file_name);

    Ok(beacon_count)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let (_, max_manhattan_distance) = solve(file_name);

    Ok(max_manhattan_distance)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 79);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 355);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 3621);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 4909);
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
