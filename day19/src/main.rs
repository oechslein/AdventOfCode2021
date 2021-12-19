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
use std::ops::{Add, RangeFrom};

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
    fn all_orientations() -> Vec<Orientation> {
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

        // TODO IF have 48, assert_eq!(all_orientations.len(), 24);

        all_orientations
    }
}

type NumberType = isize;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point3D {
    _data: [NumberType; 3],
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Point3D<{},{},{}>",
            self._data[0], self._data[3], self._data[2]
        ))
    }
}

impl Point3D {
    fn new(x: NumberType, y: NumberType, z: NumberType) -> Point3D {
        Point3D { _data: [x, y, z] }
    }

    fn x(&self) -> NumberType {
        self._data[0]
    }
    fn y(&self) -> NumberType {
        self._data[1]
    }
    fn z(&self) -> NumberType {
        self._data[2]
    }

    fn apply_orientation(&self, orientation: &Orientation) -> Point3D {
        let mut x = self._data[orientation.x_index];
        let mut y = self._data[orientation.y_index];
        let mut z = self._data[orientation.z_index];
        if orientation.x_inverted {
            x = -x;
        }
        if orientation.y_inverted {
            y = -y;
        }
        if orientation.z_inverted {
            z = -z;
        }
        Point3D::new(x, y, z)
    }

    fn get_translation(&self, point_to: &Point3D) -> Point3D {
        Point3D::new(
            point_to.x() - self.x(),
            point_to.y() - self.y(),
            point_to.z() - self.z(),
        )
    }

    fn apply_translation(&self, point_by: &Point3D) -> Point3D {
        Point3D::new(
            self.x() + point_by.x(),
            self.y() + point_by.y(),
            self.z() + point_by.z(),
        )
    }

    fn manhattan_distance(&self, other: &Point3D) -> usize {
        (self.x() - other.x()).abs() as usize
            + (self.y() - other.y()).abs() as usize
            + (self.z() - other.z()).abs() as usize
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ScannerReadings {
    scanner_position: Point3D,
    beacon_positions: Vec<Point3D>,
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
    fn new(beacon_positions: Vec<Point3D>) -> ScannerReadings {
        ScannerReadings {
            scanner_position: Point3D::new(0, 0, 0),
            beacon_positions,
        }
    }

    fn apply_orientation(&self, orientation: &Orientation) -> ScannerReadings {
        ScannerReadings {
            scanner_position: self.scanner_position.apply_orientation(orientation),
            beacon_positions: self
                .beacon_positions
                .iter()
                .map(|beacon_position| beacon_position.apply_orientation(orientation))
                .collect_vec(),
        }
    }

    fn apply_translation(&self, point_by: &Point3D) -> ScannerReadings {
        ScannerReadings {
            scanner_position: self.scanner_position.apply_translation(point_by),
            beacon_positions: self
                .beacon_positions
                .iter()
                .map(|beacon_position| beacon_position.apply_translation(point_by))
                .collect_vec(),
        }
    }

    /* 25! is too big
    fn all_orderings(&self) -> Vec<ScannerReadings> {
        println!("y: {}", self.beacon_positions.len());
        println!(
            "y: {}",
            self.beacon_positions
                .iter()
                .permutations(self.beacon_positions.len())
                .count()
        );
        self.beacon_positions
            .iter()
            .permutations(self.beacon_positions.len())
            .map(|beacon_positions| ScannerReadings {
                beacon_positions: beacon_positions.into_iter().copied().collect_vec(),
            })
            .collect_vec()
    }
    */

    fn matching_positions(
        &self,
        full_list_of_correct_beacon_positions: &HashSet<Point3D>,
    ) -> HashSet<Point3D> {
        let beacon_positions_set: HashSet<Point3D> =
            self.beacon_positions.iter().copied().collect();
        let other_beacon_positions_set = full_list_of_correct_beacon_positions;

        beacon_positions_set
            .intersection(other_beacon_positions_set)
            .copied()
            .collect()
    }
}

fn parse(content: &str) -> VecDeque<ScannerReadings> {
    content
        .replace("\r", "")
        .trim()
        .split("\n\n")
        .map(|scanner_reading_block| {
            let mut scanner_reading_block = scanner_reading_block.split('\n');
            let _ = scanner_reading_block.next(); // ignore --- scanner x ----
            let beacon_positions = scanner_reading_block
                .map(|scanner_reading_line| {
                    //println!("scanner_reading_line: {:?}", scanner_reading_line);
                    let (x, y, z) = scanner_reading_line
                        .split(',')
                        .map(|point_str| point_str.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Point3D::new(x, y, z)
                })
                .collect_vec();
            ScannerReadings::new(beacon_positions)
        })
        .collect()
}

fn solve(file_name: &str) -> (HashSet<Point3D>, Vec<Point3D>) {
    let mut open_scanner_readings = parse(&utils::file_to_string(file_name));
    let mut done_scanner_readings = VecDeque::new();
    let scanner_0 = open_scanner_readings.pop_front().unwrap();
    let mut full_list_of_correct_beacon_positions: HashSet<Point3D> = scanner_0
        .beacon_positions
        .iter()
        .copied()
        .collect::<HashSet<Point3D>>();
    done_scanner_readings.push_back(scanner_0);
    while let Some(scanner) = open_scanner_readings.pop_front() {
        if let Some(scanner_modified) = get_possible_oriented_translated_scanner_readings(
            &scanner,
            &full_list_of_correct_beacon_positions,
        ) {
            full_list_of_correct_beacon_positions.extend(&scanner_modified.beacon_positions);
            done_scanner_readings.push_back(scanner_modified);
        } else {
            open_scanner_readings.push_back(scanner);
        }
    }
    let scanner_positions = done_scanner_readings
        .iter()
        .map(|scanner_reading| scanner_reading.scanner_position)
        .collect_vec();
    (full_list_of_correct_beacon_positions, scanner_positions)
}

fn get_possible_oriented_translated_scanner_readings(
    scanner_1: &ScannerReadings,
    full_list_of_correct_beacon_positions: &HashSet<Point3D>,
) -> Option<ScannerReadings> {
    for scanner_0_point_0 in full_list_of_correct_beacon_positions {
        for orientation in Orientation::all_orientations() {
            let scanner_1_orientation = scanner_1.apply_orientation(&orientation);
            for beacon_position_s1 in scanner_1_orientation.beacon_positions.iter() {
                let translation = beacon_position_s1.get_translation(scanner_0_point_0);
                let scanner_1_translated = scanner_1_orientation.apply_translation(&translation);
                let matching_positions = scanner_1_translated
                    .matching_positions(full_list_of_correct_beacon_positions)
                    .len();
                if matching_positions >= 12 {
                    // TODO the remaining ones need to be 1000 away from scanner (both)
                    return Some(scanner_1_translated);
                }
            }
        }
    }
    None
}

fn solve_part1(file_name: &str) -> Result<usize, String> {
    let (full_list_of_correct_beacon_positions, _) = solve(file_name);

    Ok(full_list_of_correct_beacon_positions.len())
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let (_, scanner_positions) = solve(file_name);

    let result = scanner_positions
        .iter()
        .cartesian_product(scanner_positions.iter())
        .map(|(scanner_pos_a, scanner_pos_b)| scanner_pos_a.manhattan_distance(scanner_pos_b))
        .max()
        .unwrap();

    Ok(result)
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
