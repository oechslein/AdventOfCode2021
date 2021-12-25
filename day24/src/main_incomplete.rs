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
use std::fs::File;
use std::io::BufRead;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;

use fxhash::FxHashMap;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("input.txt"));
    utils::with_measure("Part 2", || solve_part2("test.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

fn create_one_value_interval(x: i64) -> IntervalSet<i64> {
    IntervalSet::new(x, x)
}

fn create_interval(lb: i64, ub: i64) -> IntervalSet<i64> {
    IntervalSet::new(lb, ub)
}

fn calc_part1() {
    let mut x = create_one_value_interval(0);
    let mut y = create_one_value_interval(0);
    let mut z = create_one_value_interval(0);
    let mut w = create_one_value_interval(0);
    let input_01 = create_interval(5, 9);
    let input_02 = create_interval(5, 9);
    let input_03 = create_interval(5, 9);
    let input_04 = create_interval(5, 9);
    let input_05 = create_interval(5, 9);
    let input_06 = create_interval(5, 9);
    let input_07 = create_interval(5, 9);
    let input_08 = create_interval(1, 9);
    let input_09 = create_interval(1, 9);
    let input_10 = create_interval(1, 9);
    let input_11 = create_interval(1, 9);
    let input_12 = create_interval(1, 9);
    let input_13 = create_interval(1, 9);
    let input_14 = create_interval(1, 9);
    w = input_01.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 10);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 2);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_02.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 14);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 13);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_03.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 14);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 13);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_04.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -13);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 9);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_05.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 10);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 15);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_06.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -13);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 3);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_07.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -7);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 6);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_08.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 11);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 5);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_09.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 10);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 16);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    w = input_10.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    // z = z; // z = alu_div(&z, 1);
    x = alu_add(&x, 13);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 1);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    println!("Read input_11");
    w = input_11.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -4);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 6);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    println!("Read input_12");
    w = input_12.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -9);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 3);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    println!("Read input_13");
    w = input_13.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -13);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 7);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    println!("Read input_14");
    w = input_14.clone();
    x = create_one_value_interval(0); // x = alu_mul(&x, 0);
    x = alu_add_complex(&x, &z);
    x = alu_mod(&x, 26);
    z = alu_div(&z, 26);
    x = alu_add(&x, -9);
    x = alu_eql_complex(&x, &w);
    x = alu_eql(&x, 0);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add(&y, 25);
    y = alu_mul_complex(&y, &x);
    y = alu_add(&y, 1);
    z = alu_mul_complex(&z, &y);
    y = create_one_value_interval(0); // y = alu_mul(&y, 0);
    y = alu_add_complex(&y, &w);
    y = alu_add(&y, 9);
    y = alu_mul_complex(&y, &x);
    z = alu_add_complex(&z, &y);
    println!("input_1: {}", input_01);
    println!("input_2: {}", input_02);
    println!("input_3: {}", input_03);
    println!("input_4: {}", input_04);
    println!("input_5: {}", input_05);
    println!("input_6: {}", input_06);
    println!("input_7: {}", input_07);
    println!("input_8: {}", input_08);
    println!("input_9: {}", input_09);
    println!("input_10: {}", input_10);
    println!("input_11: {}", input_11);
    println!("input_12: {}", input_12);
    println!("input_13: {}", input_13);
    println!("input_14: {}", input_14);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("w: {}", w);

    println!("0 in z: {}", z.contains(&0));
}

fn alu_add(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    let result = arg1 + arg2;
    // println!("{}+{}={}", arg1, arg2, result);
    result
}

fn alu_add_complex(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    let result = arg1 + arg2;
    // println!("{}+{}={}", arg1, arg2, result);
    result
}

// not used
fn alu_mul(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    arg1 * arg2
}

fn alu_mul_complex(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    let mut result = Vec::new();
    for arg1_elem in arg1.iter() {
        if arg1_elem.size() > 20 {
            println!("BIG");
        }
        for arg1_elem in arg1_elem.lower()..=arg1_elem.upper() {
            for arg2_elem in arg2.iter() {
                if arg2_elem.size() > 20 {
                    println!("BIG");
                }
                for arg2_elem in arg2_elem.lower()..=arg2_elem.upper() {
                    result.push((arg1_elem * arg2_elem, arg1_elem * arg2_elem));
                }
            }
        }
    }
    result.sort_unstable();
    let result = result.to_interval_set();
    // println!("{}*{}={}", arg1, arg2, result);
    result
}

fn alu_mod(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    let mut result = HashSet::new();
    for arg1_elem in arg1.iter() {
        if arg1_elem.size() as i64 > arg2 {
            // that means we have to visit all numbers
            return create_interval(0, arg2);
        }
        if arg1_elem.size() > 20 {
            println!("BIG");
        }
        for arg1_elem in arg1_elem.lower()..=arg1_elem.upper() {
            result.insert((arg1_elem % arg2, arg1_elem % arg2));
            if result.len() == arg2 as usize {
                break;
            }
        }
    }
    let result = result.into_iter().sorted().collect_vec().to_interval_set();
    // println!("{}%{}={}", arg1, arg2, result);
    result
}

//
fn alu_div(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
   let mut result = Vec::new();
    for arg1_elem in arg1.iter() {
        result.push((arg1_elem.lower() / arg2, arg1_elem.upper() / arg2));
    }
    result.sort_unstable();
    let result = result.to_interval_set();
    // println!("{}/{}={}", arg1, arg2, result2);

    result
}

fn alu_eql(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    let result = if !arg1.contains(&arg2) {
        create_one_value_interval(0)
    } else if arg1.size() == 1 {
        create_one_value_interval(1)
    } else {
        create_interval(0, 1)
    };
    println!("EQL({},{})={}", arg1, arg2, result);
    result
}

fn alu_eql_complex(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    let result = if arg1.is_disjoint(arg2) {
        create_one_value_interval(0)
    } else if arg1 == arg2 {
        create_one_value_interval(1)
    } else {
        create_interval(0, 1)
    };
    println!("EQL({},{})={}", arg1, arg2, result);
    result
}

// --------------------------

/*
// inverse z = z + w (what was z before => z - w)
fn alu_add_inverse(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    arg1 - arg2
}

fn alu_add_complex_inverse(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    arg1 - arg2
}

// inverse z = z * w (what was z before => z / w)
fn alu_mul_inverse(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    alu_div(arg1, arg2)
}

// inverse z = z * w (what was z before => z / w)
fn alu_mul_complex_inverse(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    let result = Interval::new(arg1.lower() / arg2.lower(), arg1.upper() / arg2.upper());
    debug_assert_eq!(result, alu_mul_complex(arg1, arg2));
    result
}

// TODO
fn alu_mod_inverse(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    Interval::new(arg1.lower() % arg2, arg1.upper() % arg2)
}

// TODO
// inverse z = z / w (what was z before => z * w)  => 2 = 7 / 3 but 3*2 = 6!!
fn alu_div_inverse(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    let result = alu_mul(arg1, arg2);
    debug_assert_eq!(result, alu_div(arg1, arg2));
    result
}

// TODO
// inverse z = eql(z, w) (what was z before) => if result_z = 0 z != w if result_z = 1 z == w
fn alu_eql_inverse(arg1: &IntervalSet<i64>, arg2: i64) -> IntervalSet<i64> {
    if arg1.contains(&arg2) {
        Interval::new(0, 1)
    } else {
        create_one_value_interval(0, 0)
    }
}

// TODO
fn alu_eql_complex_inverse(arg1: &IntervalSet<i64>, arg2: &IntervalSet<i64>) -> IntervalSet<i64> {
    if !arg1.is_disjoint(&arg2) {
        Interval::new(0, 1)
    } else {
        create_one_value_interval(0, 0)
    }
}
*/

fn solve_part1(file_name: &str) -> usize {
    println!("fn calc_part1() {{");
    println!("    let mut x = create_one_value_interval(0);");
    println!("    let mut y = create_one_value_interval(0);");
    println!("    let mut z = create_one_value_interval(0);");
    println!("    let mut w = create_one_value_interval(0);");
    for i in 1..=14 {
        println!("    let input_{:#02} = create_interval(1,9);", i);
    }
    let lines = utils::file_to_lines(file_name).collect_vec();
    let inp_args = print_lines(&lines);
    print_values();

    /*
    println!();
    println!("/////////////////////////////////////////////////////////////////////");
    println!("        z = create_one_value_interval(0);");

    print_lines_inverse(&lines, inp_args);
    print_values();

    println!("}}");
    println!();
    */

    calc_part1();

    0
}

fn print_values() {
    for i in 1..=14 {
        println!("    println!(\"input_{:#02}: {{}}\", input_{:#02});", i, i);
    }
    println!("    println!(\"x: {{}}\", x);");
    println!("    println!(\"y: {{}}\", y);");
    println!("    println!(\"z: {{}}\", z);");
    println!("    println!(\"w: {{}}\", w);");
}

fn print_lines(lines: &[String]) -> usize {
    let mut inp_arg = 1;
    for line in lines {
        let x = line.split(' ').collect_vec();
        let function = format!("alu_{}", x[0]);
        let first_argument = x[1];
        if x.len() > 2 {
            let second_argument = x[2];
            let second_argument_is_number =
                second_argument.chars().all(|c| c.is_digit(10) || c == '-');
            let fun_suffix = match second_argument_is_number {
                true => "",
                false => "_complex",
            };
            let fun_call;
            if second_argument_is_number {
                fun_call = format!(
                    "{} = {}{}(&{}, {});",
                    first_argument, function, fun_suffix, first_argument, second_argument
                );
            } else {
                fun_call = format!(
                    "{} = {}{}(&{}, &{});",
                    first_argument, function, fun_suffix, first_argument, second_argument
                );
            }

            match function.as_str() {
                "alu_mul" if second_argument == "0" => {
                    println!(
                        "    {} = create_one_value_interval(0); // {}",
                        first_argument, fun_call
                    );
                }
                "alu_add" if second_argument == "0" => {
                    println!(
                        "    // {} = {}; // {}",
                        first_argument, first_argument, fun_call
                    );
                }
                "alu_div" if second_argument == "1" => {
                    println!(
                        "    // {} = {}; // {}",
                        first_argument, first_argument, fun_call
                    );
                }
                "alu_mod" if second_argument == "0" => {
                    println!(
                        "    {} = create_one_value_interval(0); // {}",
                        first_argument, fun_call
                    );
                }
                _ => {
                    println!("    {}", fun_call);
                }
            }
        } else {
            debug_assert_eq!(function, "alu_inp");
            println!("    println!(\"Read input_{:#02}\");", inp_arg);
            println!("    {} = input_{:#02}.clone();", first_argument, inp_arg);
            inp_arg += 1;
        }
    }
    inp_arg
}

fn print_lines_inverse(lines: &[String], inp_args: usize) {
    let mut inp_arg = inp_args;
    for line in lines.iter().rev() {
        let x = line.split(' ').collect_vec();
        let function = format!("alu_{}_inverse", x[0]);
        let first_argument = x[1];
        if x.len() > 2 {
            let second_argument = x[2];
            let second_argument_is_number =
                second_argument.chars().all(|c| c.is_digit(10) || c == '-');
            let fun_suffix = match second_argument_is_number {
                true => "",
                false => "_complex",
            };
            let fun_call = format!(
                "{} = {}{}({}, {});",
                first_argument, function, fun_suffix, first_argument, second_argument
            );

            println!("    {}", fun_call);
        } else {
            debug_assert_eq!(function, "alu_inp_inverse");
            println!("    input_{:#02} = {};", inp_arg, first_argument);
            inp_arg -= 1;
        }
    }
}

fn solve_part2(file_name: &str) -> usize {
    42
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
