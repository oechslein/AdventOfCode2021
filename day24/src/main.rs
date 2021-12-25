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

use gcollections::ops::*;
use interval::interval_set::ToIntervalSet;
use interval::ops::Range;
use interval::{Interval, IntervalSet};
use itertools::{cloned, Itertools};

use core::num;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error;
use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;

use std::fs;

use fxhash::{FxHashMap, FxHashSet};

mod utils;
mod created_code;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("input.txt"));
    utils::with_measure("Part 2", || solve_part2("test.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

fn reverse_number(mut u: u64) -> u64 {
    let mut r = 0;
    while u != 0 {
        r = r * 10 + u % 10;
        u /= 10;
    }
    r
}

fn calc_part1_step_s_rec(
    step: usize,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    known_not_working: &mut FxHashSet<(usize, i64, i64, i64, i64)>,
) -> Option<u64> {
    for input in (1..=9).rev() {
        let (x, y, z, w) = calc_part1_step_n(step, input, x, y, z, w);
        if known_not_working.contains(&(step, x, y, z, w)) {
            //println!("Already known to do not lead to a solution {:?}!", (step,x,y,z,w));
            continue;
        }
        if step == 14 {
            if z == 0 {
                return Some(input as u64);
            }
        } else if let Some(n) = calc_part2_step_s_rec(step + 1, x, y, z, w, known_not_working) {
            return Some(n * 10 + input as u64);
        }
        known_not_working.insert((step, x, y, z, w));
    }
    None
}

fn calc_part1_step_n(
    step: usize,
    input: i64,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
) -> (i64, i64, i64, i64) {
    match step {
        1 => calc_part1_step_01(input, x, y, z, w),
        2 => calc_part1_step_02(input, x, y, z, w),
        3 => calc_part1_step_03(input, x, y, z, w),
        4 => calc_part1_step_04(input, x, y, z, w),
        5 => calc_part1_step_05(input, x, y, z, w),
        6 => calc_part1_step_06(input, x, y, z, w),
        7 => calc_part1_step_07(input, x, y, z, w),
        8 => calc_part1_step_08(input, x, y, z, w),
        9 => calc_part1_step_09(input, x, y, z, w),
        10 => calc_part1_step_10(input, x, y, z, w),
        11 => calc_part1_step_11(input, x, y, z, w),
        12 => calc_part1_step_12(input, x, y, z, w),
        13 => calc_part1_step_13(input, x, y, z, w),
        14 => calc_part1_step_14(input, x, y, z, w),
        _ => unreachable!(),
    }
}

fn calc_part2_step_s_rec(
    step: usize,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    known_not_working: &mut FxHashSet<(usize, i64, i64, i64, i64)>,
) -> Option<u64> {
    for input in 1..=9 {
        let (x, y, z, w) = calc_part1_step_n(step, input, x, y, z, w);
        if known_not_working.contains(&(step, x, y, z, w)) {
            //println!("Already known to do not lead to a solution {:?}!", (step,x,y,z,w));
            continue;
        }
        if step == 14 {
            if z == 0 {
                return Some(input as u64);
            }
        } else if let Some(n) = calc_part2_step_s_rec(step + 1, x, y, z, w, known_not_working) {
            return Some(n * 10 + input as u64);
        }
        known_not_working.insert((step, x, y, z, w));
    }
    None
}


fn solve_part1(file_name: &str) -> i64 {
    create_code(1, file_name);

    created_code::calc_part1().unwrap()
}

fn solve_part2(file_name: &str) -> i64 {
    create_code(2, file_name);

    /*
    let mut known_not_working: FxHashSet<(usize, i64, i64, i64, i64)> = FxHashSet::default();
    let result = calc_part2_step_s_rec(1, 0, 0, 0, 0, &mut known_not_working).unwrap();
    reverse_number(result) as i64
    */

    created_code::calc_part2().unwrap()
}

fn create_code(part: usize, file_name: &str) -> String {
    let mut result = String::new();
    result += 
    "
    use fxhash::FxHashSet;

    fn alu_add(arg1: i64, arg2: i64) -> i64 {
        arg1 + arg2
    }
    
    fn alu_mul(arg1: i64, arg2: i64) -> i64 {
        arg1 * arg2
    }
    
    fn alu_mod(arg1: i64, arg2: i64) -> i64 {
        arg1 % arg2
    }
    
    //
    fn alu_div(arg1: i64, arg2: i64) -> i64 {
        arg1 / arg2
    }
    
    fn alu_eql(arg1: i64, arg2: i64) -> i64 {
        if arg1 == arg2 {
            1
        } else {
            0
        }
    }

    ";

    result += &create_fns(file_name);
    result += "\n";
    result += &create_calls(part);


    result 
}

fn create_fns(file_name: &str) -> String {
    let lines = utils::file_to_lines(file_name).collect_vec();
    let mut index = 0;
    let mut result = String::new();
    for step in 1..=14 {
        let (new_index, new_result) = create_step_fns(&lines[index..], step);
        index += new_index;
        result += &new_result;
    }
    result
}

fn create_calls(part: usize) -> String {
    let mut result = String::new();
    result += &format!("pub fn calc_part{}() -> Option<i64> {{\n", part);
    for step in 1..14 {
        result += &format!("    let mut known_not_working_{:#02}: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();\n", step);
    }
    result += "    let (x, y, z, w) = (0, 0, 0, 0);";
    result += &create_calls_rec(1, part == 1);
    result += "    None\n";
    result += "}}\n";
    result
}

fn create_calls_rec(step: usize, reverse: bool) -> String {
    let mut result = String::new();
    let mut range_str = "1..=9".to_string();
    if reverse {
        range_str = format!("({}).rev()", range_str);
    }
    result += &format!(
        "{}for input_{:#02} in {} {{\n",
        " ".repeat(4 * step),
        step,
        range_str,
    );
    result += &format!(
        "{}let (x, y, z, w) = calc_part1_step_{:#02}(input_{:#02}, x, y, z, w);\n",
        " ".repeat(4 + 4 * step),
        step,
        step,
    );
    if step == 14 {
        let all_inputs = (1..=14)
            .map(|step| format!("input_{:#02} * 10_i64.pow({})", step, 14 - step))
            .join("+");
        result += &format!(
            "{}if z == 0 {{ return Some({}); }}\n",
            " ".repeat(4 + 4 * step),
            all_inputs,
        );
    } else {
        result += &format!(
            "{}if known_not_working_{:#02}.contains(&(x,y,z,w)) {{ continue; }}\n",
            " ".repeat(4 + 4 * step),
            step,
        );
        result += &create_calls_rec(step + 1, reverse);
        result += &format!(
            "{}known_not_working_{:#02}.insert((x, y, z, w));\n",
            " ".repeat(4 + 4 * step),
            step,
        );
    }
    result += &format!("{}}}", " ".repeat(4 * step));
    result
}

fn create_step_fns(lines: &[String], step: usize) -> (usize, String) {
    let mut result = String::new();
    result += &format!("fn calc_part1_step_{:#02}(input: i64, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> (i64,i64,i64,i64) {{\n", step);
    let mut input_seen = false;
    let mut last_index = 0;
    for (index, line) in lines.iter().enumerate() {
        let x = line.split(' ').collect_vec();
        let function = format!("alu_{}", x[0]);
        let first_argument = x[1];
        if x.len() == 2 {
            if !input_seen {
                result += &format!("    {} = input;\n", first_argument);
                input_seen = true;
            } else {
                last_index = index;
                break;
            }
        } else {
            let second_argument = x[2];
            let fun_call = format!(
                "{} = {}({}, {});",
                first_argument, function, first_argument, second_argument
            );

            match function.as_str() {
                "alu_mul" if second_argument == "0" => {
                    result += &format!("    {} = 0; // {}\n", first_argument, fun_call);
                }
                "alu_add" if second_argument == "0" => {
                    result += &format!(
                        "    // {} = {}; // {}\n",
                        first_argument, first_argument, fun_call
                    );
                }
                "alu_div" if second_argument == "1" => {
                    result += &format!(
                        "    // {} = {}; // {}\n",
                        first_argument, first_argument, fun_call
                    );
                }
                "alu_mod" if second_argument == "0" => {
                    result += &format!("    {} = 0; // {}\n", first_argument, fun_call);
                }
                _ => {
                    result += &format!("    {}\n", fun_call);
                }
            }
        }
    }
    result += "    (x,y,z,w)\n";
    result += "}}\n";
    (last_index, result)

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
        assert_eq!(solve_part1("test.txt"), 93997999296912);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 93997999296912);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test2.txt"), 81111379141811);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 81111379141811);
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
