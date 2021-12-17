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

//#[derive(Debug, PartialEq, Eq, FromPrimitive)]
type NumberType = i32;
type CoorType = (NumberType, NumberType);

fn solve_part1(_: &str) -> Result<NumberType, String> {
    //let (target_area_start, target_area_end) = ((20, -5), (30, -10));
    let (target_area_start, target_area_end) = ((102, -90), (157, -146));

    if let Some((max_highest_y, _)) = solve(target_area_start, target_area_end) {
        Ok(max_highest_y)
    } else {
        Err("Nothing found".to_string())
    }
}

fn solve_part2(_: &str) -> Result<usize, String> {
    //let (target_area_start, target_area_end) = ((20, -5), (30, -10));
    let (target_area_start, target_area_end) = ((102, -90), (157, -146));

    if let Some((_, all_vecs)) = solve(target_area_start, target_area_end) {
        Ok(all_vecs.len())
    } else {
        Err("Nothing found".to_string())
    }
}

fn solve(
    target_area_start: CoorType,
    target_area_end: CoorType,
) -> Option<(NumberType, Vec<CoorType>)> {
    let mut max_highest_y = NumberType::MIN;
    let mut all_vecs = Vec::new();
    // otherwise it would jump over the target area
    let y_speed_max_heuristic = (target_area_start.1 as NumberType)
        .abs()
        .max((target_area_end.1 as NumberType).abs());
    for (x_speed, y_speed) in (1..target_area_start.0 + target_area_end.0)
        .cartesian_product(-y_speed_max_heuristic..y_speed_max_heuristic)
    {
        let result = calculate_forward_path(x_speed, y_speed, target_area_start, target_area_end);
        //println!("({} {}) => {:?}", x_speed, y_speed, result);
        if let Some(highest_y) = result {
            all_vecs.push((x_speed, y_speed));
            max_highest_y = max_highest_y.max(highest_y);
        }
    }

    if all_vecs.is_empty() {
        None
    } else {
        Some((max_highest_y, all_vecs))
    }
}

fn calculate_forward_path(
    x_speed: NumberType,
    y_speed: NumberType,
    target_area_start: CoorType,
    target_area_end: CoorType,
) -> Option<NumberType> {
    let mut x = 0;
    let mut y = 0;
    let mut x_speed = x_speed;
    let mut y_speed = y_speed;
    let mut highest_y = y;
    //println!("x,y: {},{}; xspeed,yspeed: {},{}", x, y, x_speed, y_speed);

    loop {
        x += x_speed;
        y += y_speed;
        x_speed = match x_speed.cmp(&0) {
            std::cmp::Ordering::Less => x_speed + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => x_speed - 1,
        };
        y_speed -= 1;
        highest_y = highest_y.max(y);
        //println!("x,y: {},{}; xspeed,yspeed: {},{}", x, y, x_speed, y_speed);

        // y axis is inverted
        if x > target_area_end.0 || y < target_area_end.1 {
            return None;
        }
        if x >= target_area_start.0 && y <= target_area_start.1 {
            assert!(x <= target_area_end.0 && y >= target_area_end.1);
            return Some(highest_y);
        }
        // since y_speed get lower and lower, y get lower and lower and will become lower than target_area_end.1
        // so this loop will always end
    }
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
        assert_eq!(solve_part1("test.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 5247);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 5247);
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
