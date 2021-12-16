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

type BitSliceType = BitSlice<Msb0, u8>;

#[derive(Debug, PartialEq, Eq, FromPrimitive)]
enum OP {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

impl OP {
    fn apply(&self, sub_values: impl Iterator<Item = usize>) -> usize {
        match self {
            OP::Sum => sub_values.sum(),
            OP::Product => sub_values.product(),
            OP::Min => sub_values.min().unwrap(),
            OP::Max => sub_values.max().unwrap(),
            OP::GreaterThan | OP::LessThan | OP::Equal => {
                let (a, b) = sub_values.collect_tuple().unwrap();
                let result = match self {
                    OP::GreaterThan => a > b,
                    OP::LessThan => a < b,
                    OP::Equal => a == b,
                    _ => unreachable!(),
                };
                match result {
                    true => 1,
                    false => 0,
                }
            }
        }
    }
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
            Package::Op(_, op, sub_packages) => op.apply(sub_packages.iter().map(Package::calc)),
        }
    }

    fn create_package_from_str(s: &str) -> Package {
        parse_package(&create_bit_vec(s), &mut 0)
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

fn parse_package(bit_slice: &BitSliceType, curr_index: &mut usize) -> Package {
    let package_version = bit_slice_to_value(bit_slice, curr_index, 3);
    let package_type = bit_slice_to_value(bit_slice, curr_index, 3);
    if package_type == 4 {
        // literal value
        let mut result: usize = 0;
        loop {
            let is_last_group = bit_slice_to_value(bit_slice, curr_index, 1) == 0;
            let value = bit_slice_to_value(bit_slice, curr_index, 4);
            result = result * 2usize.pow(4) + value;
            if is_last_group {
                // found end
                break;
            }
        }
        Package::Value(package_version, result)
    } else {
        // package_type != 4
        // operator
        let length_type_id = bit_slice_to_value(bit_slice, curr_index, 1);
        let mut subpackages = Vec::new();
        if length_type_id == 0 {
            // 15-bit number representing the number of bits in the sub-packets
            let subpackages_bit_len = bit_slice_to_value(bit_slice, curr_index, 15);
            let end_index = *curr_index + subpackages_bit_len;
            while *curr_index != end_index {
                //println!("{} > {}", curr_index, end_index);
                assert!(*curr_index < end_index);
                subpackages.push(parse_package(bit_slice, curr_index));
            }
        } else {
            // 11-bit number representing the number of sub-packets
            let subpackages_amount = bit_slice_to_value(bit_slice, curr_index, 11);
            for _ in 0..subpackages_amount {
                subpackages.push(parse_package(bit_slice, curr_index));
            }
        }

        let op = FromPrimitive::from_usize(package_type).unwrap();
        Package::Op(package_version, op, subpackages)
    }
}

fn bit_slice_to_value(bit_slice: &BitSliceType, curr_index: &mut usize, length: usize) -> usize {
    let end_index = *curr_index + length;
    let bit_slice: &BitSliceType = &bit_slice[*curr_index..end_index];
    let mut result: usize = 0;
    for bit in bit_slice.iter() {
        result *= 2;
        if *bit {
            result += 1;
        }
    }
    *curr_index += length;
    result
}

fn create_bit_vec(content: &str) -> BitVec<Msb0, u8> {
    let mut io_buf = BitVec::<Msb0, u8>::new();
    io_buf.resize(content.len() / 2 * 8, false);
    (0..content.len())
        .step_by(2)
        .map(|index| {
            (
                index * 8 / 2,
                u8::from_str_radix(&content[index..=index + 1], 16).unwrap(),
            )
        })
        .for_each(|(start_index_bits, from_str_radix)| {
            io_buf[start_index_bits..(start_index_bits + 8)].store(from_str_radix);
        });
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
                    OP::LessThan,
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
                    OP::Max,
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
                    OP::Min,
                    vec![Package::Op(
                        1,
                        OP::Min,
                        vec![Package::Op(5, OP::Min, vec![Package::Value(6, 15)])]
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
                    OP::Sum,
                    vec![
                        Package::Op(
                            0,
                            OP::Sum,
                            vec![Package::Value(0, 10), Package::Value(5, 11)]
                        ),
                        Package::Op(
                            1,
                            OP::Sum,
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
                    OP::Sum,
                    vec![
                        Package::Op(
                            0,
                            OP::Sum,
                            vec![Package::Value(0, 10), Package::Value(6, 11)]
                        ),
                        Package::Op(
                            4,
                            OP::Sum,
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
                    OP::Sum,
                    vec![Package::Op(
                        1,
                        OP::Sum,
                        vec![Package::Op(
                            3,
                            OP::Sum,
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
