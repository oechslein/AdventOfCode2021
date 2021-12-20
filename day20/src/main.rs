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
use std::collections::{HashMap, HashSet, VecDeque};
use std::error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::ops::{Add, RangeFrom, SubAssign};

mod utils;

use image::gif::GifEncoder;
use image::{Delay, Frame, Frames, GenericImage, GenericImageView, ImageBuffer, RgbImage};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("input.txt"));
    utils::with_measure("Part 2", || solve_part2("input.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

//#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[derive(Debug, Clone)]
struct Image {
    _data: HashSet<(isize, isize)>,
}

impl Image {
    fn new() -> Image {
        Image {
            _data: HashSet::new(),
        }
    }
    fn get(&self, x: isize, y: isize) -> bool {
        self._data.get(&(x, y)).is_some()
    }

    fn set(&mut self, x: isize, y: isize, new_value: bool) {
        if new_value {
            self._data.insert((x, y));
        }
    }

    fn get_minmax(&self) -> ((isize, isize), (isize, isize)) {
        let mut min_x_y = (isize::MAX, isize::MAX);
        let mut max_x_y = (isize::MIN, isize::MIN);
        for (x, y) in self._data.iter() {
            min_x_y = (min_x_y.0.min(*x), min_x_y.1.min(*y));
            max_x_y = (max_x_y.0.max(*x), max_x_y.1.max(*y));
        }
        (min_x_y, max_x_y)
    }

    fn get_minmax_for_steps(&self, i: isize, steps: isize) -> ((isize, isize), (isize, isize)) {
        let (min_x_y, max_x_y) = self.get_minmax();
        let (min_x_y, max_x_y) = if i == 0 {
            (
                (min_x_y.0 - 2 * steps, min_x_y.1 - 2 * steps),
                (max_x_y.0 + 2 * steps, max_x_y.1 + 2 * steps),
            )
        } else {
            (
                (min_x_y.0 - 1, min_x_y.1 - 1),
                (max_x_y.0 + 1, max_x_y.1 + 1),
            )
        };
        (min_x_y, max_x_y)
    }

    fn get_neighbors_as_number(&self, x: isize, y: isize) -> usize {
        let mut result = 0;
        for (new_y, new_x) in (y - 1..=y + 1).cartesian_product(x - 1..=x + 1) {
            result *= 2;
            if self.get(new_x, new_y) {
                result += 1;
            }
        }
        result
    }

    fn iter(&self) -> impl Iterator<Item = &(isize, isize)> {
        self._data.iter()
    }

    fn display(&self) {
        let (min_x_y, max_x_y) = self.get_minmax();
        for y in min_x_y.1..max_x_y.1 {
            for x in min_x_y.0..max_x_y.0 {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn save_pgn(&self, file_name: &str, (min_x_y, max_x_y): ((isize, isize), (isize, isize))) {
        let width = (max_x_y.0 - min_x_y.0) as u32;
        let height = (max_x_y.1 - min_x_y.1) as u32;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            if self.get(x as isize + min_x_y.0, y as isize + min_x_y.1) {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        });
        img.save(file_name).unwrap();
    }

    fn create_frame(&self, (min_x_y, max_x_y): ((isize, isize), (isize, isize))) -> Frame {
        let width = (max_x_y.0 - min_x_y.0) as u32;
        let height = (max_x_y.1 - min_x_y.1) as u32;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            if self.get(x as isize + min_x_y.0, y as isize + min_x_y.1) {
                image::Rgba([0u8, 0u8, 0u8, 255u8])
            } else {
                image::Rgba([255u8, 255u8, 255u8, 255u8])
            }
        });
        Frame::from_parts(img, width, height, Delay::from_numer_denom_ms(100, 1))
    }
}

fn parse(content: &str) -> (HashMap<usize, bool>, Image) {
    let content = content.replace('\r', "");
    let (transform_str, image_str) = content.trim().split("\n\n").collect_tuple().unwrap();

    let transform = transform_str
        .chars()
        .enumerate()
        .map(|(index, c)| (index, c == '#'))
        .collect();
    let mut image = Image::new();

    for (y, line) in image_str.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            image.set(x as isize, y as isize, c == '#')
        }
    }

    (transform, image)
}

fn solve(file_name: &str, steps: isize, gif_file_name: &str) -> usize {
    let (transform_to_pixel, mut image) = parse(&utils::file_to_string(file_name));

    //println!("{} {} {}", empty_result, full_result, 1 << 8);
    let (start_min_x_y, start_max_x_y) = image.get_minmax();
    let final_size = (
        (start_min_x_y.0 - steps, start_min_x_y.1 - steps),
        (start_max_x_y.0 + steps, start_max_x_y.1 + steps),
    );
    //let mut frames = Vec::new();
    for i in 0..steps {
        let mut new_image = Image::new();
        let (min_x_y, max_x_y) = image.get_minmax_for_steps(i, steps);
        //println!("Step: {} ({:?})", i + 1, (min_x_y, max_x_y));
        //image.display();
        //image.save_pgn(format!("{}_{}.png", gif_file_name, i).as_str(), final_size);
        //frames.push(image.create_frame(final_size));

        for y in min_x_y.1..=max_x_y.1 {
            for x in min_x_y.0..=max_x_y.0 {
                new_image.set(
                    x,
                    y,
                    transform_to_pixel[&image.get_neighbors_as_number(x, y)],
                );
            }
        }
        image = new_image;
    }

    let (min_x_y, max_x_y) = image.get_minmax();
    //println!("Result: ({:?})", (min_x_y, max_x_y));
    //image.display();
    //image.save_pgn(
    //    format!("{}_FULL.png", gif_file_name).as_str(),
    //    image.get_minmax(),
    //);
    //frames.push(image.create_frame(final_size));
    //let file_out = File::create(gif_file_name).unwrap();
    //let mut encoder = GifEncoder::new(file_out);
    //encoder.encode_frames(frames.into_iter());

    let result = image
        .iter()
        .filter(|(x, y)| {
            *x >= start_min_x_y.0 - steps
                && *y >= start_min_x_y.1 - steps
                && *x <= start_max_x_y.0 + steps
                && *y <= start_max_x_y.1 + steps
        })
        .count();
    result
}

fn solve_part1(file_name: &str) -> usize {
    let result = solve(file_name, 2, format!("part1_{}.gif", file_name).as_str());
    result
}

fn solve_part2(file_name: &str) -> usize {
    let result = solve(file_name, 50, format!("part2_{}.gif", file_name).as_str());
    //solve(file_name, 1000, format!("partX_{}.gif", file_name).as_str());
    result
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
        assert_eq!(solve_part1("test.txt"), 35);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 5464);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 3351);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 19228);
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
