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
use regex::Captures;

use core::num;
use std::cell::RefCell;
use std::error;
use std::ops::Add;
use std::rc::Rc;

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

/*
[[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].
[[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4]

explode:
1. At index of "3" (left side) look to the left until first number, increase this number with "3"
2. At index of "2" (right side) look to the right until first number, increase this number with "2"
3. Replace "[3,2]" with 0

To split a regular number, replace it with a pair;
the left element of the pair should be the regular number divided by two and rounded down,
while the right element of the pair should be the regular number divided by two and rounded up.
For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.

split:
replace the number with a pair.

check if "nested inside four pairs":
+1 for "["
if == 4 pair must be reduced (we neeed to find the indexes of left_start, left_end, right_start, right_end)
  left_start = curr + 1
  left_end = next(",") -1
  right_start = next(",") +1
  right_end = next("]") -1
-1 for "]"

add:
X + Y = [X,Y]

*/

type NumberType = usize;

//#[derive(Debug, PartialEq, Eq, FromPrimitive)]

const NUMBER_TO_SPLIT_LIMIT: usize = 10;
const EXPLODE_LEVEL: usize = 5;

fn magnitude(s: &str) -> usize {
    use regex::Regex;

    let mut s = s.to_string();
    let re = Regex::new(r"\[(?P<left>\d+),(?P<right>\d+)\]").unwrap();
    while let Some(cap) = re.captures(&s) {
        let left: usize = cap.name("left").unwrap().as_str().parse().unwrap();
        let right: usize = cap.name("right").unwrap().as_str().parse().unwrap();
        let new_number = 3 * left + 2 * right;
        let x = cap.get(0).unwrap();
        s = format!("{}{}{}", &s[0..x.start()], new_number, &s[x.end()..]);
    }

    s.parse().unwrap()
}

fn add_with_reduce(left: &str, right: &str) -> String {
    reduce(&add(left, right))
}

fn add(left: &str, right: &str) -> String {
    format!("[{},{}]", left, right)
}

fn reduce(s: &str) -> String {
    let mut s = s.to_string();
    loop {
        //println!("s: {}", s);
        let exploded_s = explode(&s);
        //println!("exploded_s: {}", exploded_s);
        let splitted_s = split_once(&exploded_s);
        if splitted_s == s {
            return s;
        }
        //println!("splitted_s: {}", splitted_s);
        s = splitted_s;
    }
}

fn split_once(s: &str) -> String {
    if let Some((start_index, end_index)) = get_split_indexes(s) {
        split_with_index(s, start_index, end_index)
    } else {
        s.to_string()
    }
}

fn explode(s: &str) -> String {
    /*
    1. At index of "3" (left side) look to the left until first number, increase this number with "3"
    2. At index of "2" (right side) look to the right until first number, increase this number with "2"
    3. Replace "[3,2]" with 0
    */
    let mut s = s.to_string();
    while let Some((start_index, comma_index, end_index)) = get_nested_pair_indexes(&s) {
        let (left_side_number, right_side_number) =
            parse_pair(&s, (start_index, comma_index, end_index));

        let left_s = explode_replace_to_left(&s[0..start_index], left_side_number);
        let right_s = explode_replace_to_right(&s[end_index + 1..], right_side_number);
        s = format!("{}0{}", left_s, right_s);
    }
    s
}

///////////

fn explode_nullify_pair(
    s: &str,
    start_index: usize,
    comma_index: usize,
    end_index: usize,
) -> (String, usize, usize) {
    let (left_side_number, right_side_number) =
        parse_pair(s, (start_index, comma_index, end_index));
    let left = s[0..start_index].to_string();
    let right = s[end_index + 1..].to_string();
    (
        format!("{}0{}", left, right),
        left_side_number,
        right_side_number,
    )
}

fn explode_replace_to_right(s: &str, old_number: usize) -> String {
    if let Some((number_start_index, number_end_index)) = get_next_number_to_right(s) {
        let number = parse_number(s, number_start_index, number_end_index);
        let left = s[0..number_start_index].to_string();
        let right = s[number_end_index..].to_string();
        format!("{}{}{}", left, old_number + number, right)
    } else {
        s.to_string()
    }
}

fn get_next_number_to_right(s: &str) -> Option<(usize, usize)> {
    let mut number_start_index_opt = None;
    for (index, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            if number_start_index_opt.is_none() {
                number_start_index_opt = Some(index);
            }
        } else if let Some(number_start_index) = number_start_index_opt {
            let number_end_index = index;
            return Some((number_start_index, number_end_index));
        }
    }
    None
}

fn explode_replace_to_left(s: &str, old_number: usize) -> String {
    if let Some((number_start_index, number_end_index)) = get_next_number_to_left(s) {
        let number = parse_number(s, number_start_index, number_end_index);
        let left = s[0..number_start_index].to_string();
        let right = s[number_end_index..].to_string();
        format!("{}{}{}", left, old_number + number, right)
    } else {
        s.to_string()
    }
}

fn get_next_number_to_left(s: &str) -> Option<(usize, usize)> {
    let mut number_end_index_opt = None;
    for (index, c) in s.chars().rev().enumerate() {
        let index = s.len() - index - 1;
        if c.is_digit(10) {
            if number_end_index_opt.is_none() {
                number_end_index_opt = Some(index + 1);
            }
        } else if let Some(number_end_index) = number_end_index_opt {
            let number_start_index = index + 1;
            return Some((number_start_index, number_end_index));
        }
    }
    None
}
fn split_with_index(s: &str, start_index: usize, end_index: usize) -> String {
    let number = parse_number(s, start_index + 1, end_index);
    let left_number = (number as f64 / 2.0).floor() as usize;
    let right_number = number - left_number;
    let start = s[0..=start_index].to_string();
    let end = s[end_index..].to_string();
    format!("{}[{},{}]{}", start, left_number, right_number, end)
}

fn get_split_indexes(s: &str) -> Option<(usize, usize)> {
    let mut start_index_opt = None;
    for (index, c) in s.chars().enumerate() {
        match c {
            _ if c.is_digit(10) => {
                if start_index_opt.is_none() {
                    start_index_opt = Some(index - 1);
                }
            }
            _ => {
                if let Some(start_index) = start_index_opt {
                    let end_index = index;
                    let number = parse_number(s, start_index + 1, end_index);
                    if number >= NUMBER_TO_SPLIT_LIMIT {
                        return Some((start_index, end_index));
                    } else {
                        start_index_opt = None;
                    }
                }
            }
        }
    }
    None
}

// Returns index of [ , ]
fn get_nested_pair_indexes(s: &str) -> Option<(usize, usize, usize)> {
    let mut counter = 0;
    let mut start_index = 0;
    let mut comma_index = 0;
    for (index, c) in s.chars().enumerate() {
        match c {
            '[' => {
                start_index = index;
                counter += 1;
            }
            ',' => {
                comma_index = index;
            }
            ']' => {
                if counter >= EXPLODE_LEVEL {
                    let end_index = index;
                    return Some((start_index, comma_index, end_index));
                }
                counter -= 1;
            }
            _ => {}
        }
    }
    None
}

fn parse_number(s: &str, start_index: usize, end_index: usize) -> usize {
    s[start_index..end_index].parse().unwrap()
}

fn parse_pair(s: &str, indexes: (usize, usize, usize)) -> (usize, usize) {
    (
        parse_number(s, indexes.0 + 1, indexes.1),
        parse_number(s, indexes.1 + 1, indexes.2),
    )
}

fn solve_part1(file_name: &str) -> Result<NumberType, String> {
    let result_str = utils::file_to_lines(file_name)
        .reduce(|accum, item| add_with_reduce(&accum, &item))
        .unwrap();
    let result = magnitude(&result_str);

    Ok(result)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let lines = utils::file_to_lines(file_name).collect_vec();
    let result = lines.iter().cartesian_product(lines.iter()).map(|(line1, line2)| magnitude(&add_with_reduce(line1, line2))).max().unwrap();

    Ok(result)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(add("[1,2]", "[[3,4],5]"), "[[1,2],[[3,4],5]]".to_string());

        assert_eq!(
            get_nested_pair_indexes("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"),
            Some((16, 18, 20))
        );

        let content = "[[6,[5,[4,[3,2]]]],1]";
        assert_eq!(
            get_nested_pair_indexes(&content.to_string()),
            Some((10, 12, 14))
        );
        assert_eq!(
            parse_pair(
                &content.to_string(),
                get_nested_pair_indexes(content).unwrap()
            ),
            (3, 2)
        );

        assert_eq!(
            split_with_index("[[[[0,7],4],[15,[0,13]]],[1,1]]", 12, 15),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string()
        );

        assert_eq!(
            get_split_indexes("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
            Some((12, 15))
        );

        assert_eq!(
            explode_nullify_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", 4, 6, 8),
            ("[[[[0,4],4],[7,[[8,4],9]]],[1,1]]".to_string(), 4, 3)
        );

        assert_eq!(
            explode_replace_to_right(",4],4],[7,[[8,4],9]]],[1,1]]", 3),
            ",7],4],[7,[[8,4],9]]],[1,1]]"
        );

        assert_eq!(explode_replace_to_left("[[[[", 4), "[[[[".to_string());

        assert_eq!(
            explode_replace_to_left("[[[[0,7],4],[7,[", 8),
            "[[[[0,7],4],[15,[".to_string()
        );

        let mut content = add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]");
        assert_eq!(content, "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());

        content = explode(&content);
        assert_eq!(content, "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        content = split_once(&content);
        assert_eq!(content, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        content = split_once(&content);
        assert_eq!(content, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        content = explode(&content);
        assert_eq!(content, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        content = reduce(&add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"));
        assert_eq!(content, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        content = add_with_reduce(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        );
        assert_eq!(
            content,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );

        let content_vec = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];

        assert_eq!(
            content_vec
                .into_iter()
                .map(|s| s.to_string())
                .reduce(|accum, item| add_with_reduce(&accum, &item))
                .unwrap(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );

        assert_eq!(magnitude("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
        assert_eq!(
            magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            3488
        );
        assert_eq!(
            magnitude("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"),
            4140
        );

        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 4140);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 3892);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 3993);
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
