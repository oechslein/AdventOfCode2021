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

use std::error;
use std::str::FromStr;

use bitvec::prelude::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

//#[macro_use]
//extern crate lazy_static;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("test.txt"));
    utils::with_measure("Part 2", || solve_part2("test.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, FromPrimitive)]
enum OP {
    SUM = 0,
    PRODUCT = 1,
    MIN = 2,
    MAX = 3,
    GREATER_THAN = 5,
    LESS_THAN = 6,
    EQUAL = 7,
}

#[derive(Debug, PartialEq, Eq)]
enum Package {
    Value(usize, usize),
    Op(usize, OP, Vec<Package>),
}

impl Package {
    fn version_sum(&self) -> usize {
        match self {
            Package::Value(version, _) => *version,
            Package::Op(version, _, sub_packages) => {
                *version + sub_packages.iter().map(Package::version_sum).sum::<usize>()
            }
        }
    }

    fn calc(&self) -> usize {
        match self {
            Package::Value(_, value) => *value,
            Package::Op(_, op, sub_packages) => {
                let sub_values = sub_packages.iter().map(Package::calc);
                match op {
                    OP::SUM => sub_values.sum(),
                    OP::PRODUCT => sub_values.product(),
                    OP::MIN => sub_values.min().unwrap(),
                    OP::MAX => sub_values.max().unwrap(),
                    OP::GREATER_THAN | OP::LESS_THAN | OP::EQUAL => {
                        let (a, b) = sub_values.collect_tuple().unwrap();
                        let result = match op {
                            OP::GREATER_THAN => a > b,
                            OP::LESS_THAN => a < b,
                            OP::EQUAL => a == b,
                            _ => panic!(),
                        };
                        match result {
                            true => 1,
                            false => 0,
                        }
                    }
                }
            }
        }
    }

    fn create_package_from_str(s: &str) -> Package {
        let (p, _) = parse_package(&create_bit_vec(s), 0);
        p
    }

    fn solve_part1(s: &str) -> (Package, usize) {
        let p = Package::create_package_from_str(s);
        let version_sum = p.version_sum();
        (p, version_sum)
    }

    fn solve_part2(s: &str) -> (Package, usize) {
        let p = Package::create_package_from_str(s);
        let result = p.calc();
        (p, result)
    }    
}

fn solve_part1(file_name: &str) -> Result<usize, String> {
    let (_, version_sum) = Package::solve_part1(&utils::file_to_string(file_name));
    Ok(version_sum)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let (_, result) = Package::solve_part2(&utils::file_to_string(file_name));
    Ok(result)
}

fn parse_package(bit_slice: &BitSlice<Msb0, u8>, curr_index: usize) -> (Package, usize) {
    let package_version = bit_slice_to_value(&bit_slice[curr_index..curr_index + 3]);
    let package_type = bit_slice_to_value(&bit_slice[curr_index + 3..curr_index + 6]);
    if package_type == 4 {
        // literal value
        let mut curr_index = curr_index + 6;
        let mut result: usize = 0;
        loop {
            let value = bit_slice_to_value(&bit_slice[curr_index + 1..curr_index + 5]);
            result = result * 16 + value;
            if !bit_slice[curr_index] {
                // found end
                curr_index += 5;
                break;
            }
            curr_index += 5;
        }
        let result = (Package::Value(package_version, result), curr_index);
        //println!("result: {:?}", result);
        result
    } else {
        // package_type != 4
        // operator
        let mut curr_index = curr_index + 6;
        let length_type_id = bit_slice_to_value(&bit_slice[curr_index..curr_index + 1]);
        curr_index += 1;
        let mut subpackages = Vec::new();
        if length_type_id == 0 {
            // 15-bit number representing the number of bits in the sub-packets
            let subpackages_bit_len = bit_slice_to_value(&bit_slice[curr_index..curr_index + 15]);
            curr_index += 15;
            let end_index = curr_index + subpackages_bit_len;
            while curr_index != end_index {
                //println!("{} > {}", curr_index, end_index);
                assert!(curr_index < end_index);
                let (parsed_package, next_index) = parse_package(bit_slice, curr_index);
                subpackages.push(parsed_package);
                curr_index = next_index;
            }
        } else {
            // 11-bit number representing the number of sub-packets
            let subpackages_amount = bit_slice_to_value(&bit_slice[curr_index..curr_index + 11]);
            curr_index += 11;
            for i in 0..subpackages_amount {
                //println!( "{} of {}, curr_index: {}",i, subpackages_amount, curr_index);
                let (parsed_package, next_index) = parse_package(bit_slice, curr_index);
                subpackages.push(parsed_package);
                curr_index = next_index;
            }
        }

        let op: OP = FromPrimitive::from_usize(package_type).unwrap();
        let result = (Package::Op(package_version, op, subpackages), curr_index);
        //println!("result: {:?}", result);
        result
    }
}

fn bit_slice_to_value(bit_slice: &BitSlice<Msb0, u8>) -> usize {
    let mut result: usize = 0;
    for bit in bit_slice.iter() {
        if *bit {
            result = 2 * result + 1;
        } else {
            result *= 2;
        }
    }
    result
}

fn create_bit_vec(content: &str) -> BitVec<Msb0, u8> {
    let mut io_buf: BitVec<Msb0, u8> = BitVec::new();
    io_buf.resize(content.len() / 2 * 8, false);
    let mut index = 0;
    while index < content.len() {
        let start_index = index;
        let end_index = index + 1;
        let start_index_bits = start_index * 8 / 2;
        let from_str_radix = u8::from_str_radix(&content[start_index..=end_index], 16);
        if from_str_radix.is_err() {
            panic!(
                "start_index {} end_index {} content {:?}",
                start_index,
                end_index,
                &content[start_index..=end_index]
            );
        }
        io_buf[start_index_bits..(start_index_bits + 8)].store(from_str_radix.unwrap());
        index += 2;
    }
    io_buf
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(Package::solve_part1("D2FE28"), (Package::Value(6, 2021), 6));
        assert_eq!(
            Package::solve_part1("38006F45291200"),
            (
                Package::Op(
                    1,
                    OP::LESS_THAN,
                    vec![Package::Value(6, 10), Package::Value(2, 20)]
                ),
                1 + 6 + 2
            )
        );
        assert_eq!(
            Package::solve_part1("EE00D40C823060"),
            (
                Package::Op(
                    7,
                    OP::MAX,
                    vec![
                        Package::Value(2, 1),
                        Package::Value(4, 2),
                        Package::Value(1, 3)
                    ]
                ),
                7 + 2 + 4 + 1
            )
        );

        assert_eq!(
            Package::solve_part1("8A004A801A8002F478"),
            (
                Package::Op(
                    4,
                    OP::MIN,
                    vec![Package::Op(
                        1,
                        OP::MIN,
                        vec![Package::Op(5, OP::MIN, vec![Package::Value(6, 15)])]
                    )]
                ),
                4 + 1 + 5 + 6
            )
        );

        assert_eq!(
            Package::solve_part1("620080001611562C8802118E34"),
            (
                Package::Op(
                    3,
                    OP::SUM,
                    vec![
                        Package::Op(
                            0,
                            OP::SUM,
                            vec![Package::Value(0, 10), Package::Value(5, 11)]
                        ),
                        Package::Op(
                            1,
                            OP::SUM,
                            vec![Package::Value(0, 12), Package::Value(3, 13)]
                        )
                    ]
                ),
                3 + 5 + 1 + 3
            )
        );
        assert_eq!(
            Package::solve_part1("C0015000016115A2E0802F182340"),
            (
                Package::Op(
                    6,
                    OP::SUM,
                    vec![
                        Package::Op(
                            0,
                            OP::SUM,
                            vec![Package::Value(0, 10), Package::Value(6, 11)]
                        ),
                        Package::Op(
                            4,
                            OP::SUM,
                            vec![Package::Value(7, 12), Package::Value(0, 13)]
                        )
                    ]
                ),
                6 + 6 + 4 + 7
            )
        );
        assert_eq!(
            Package::solve_part1("A0016C880162017C3686B18A3D4780"),
            (
                Package::Op(
                    5,
                    OP::SUM,
                    vec![Package::Op(
                        1,
                        OP::SUM,
                        vec![Package::Op(
                            3,
                            OP::SUM,
                            vec![
                                Package::Value(7, 6),
                                Package::Value(6, 6),
                                Package::Value(5, 12),
                                Package::Value(2, 15),
                                Package::Value(2, 15)
                            ]
                        )]
                    )]
                ),
                5 + 1 + 3 + 7 + 6 + 5 + 2 + 2
            )
        );
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 945);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 945);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 10637009915279);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 10637009915279);
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
