#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]
#![feature(destructuring_assignment)]

extern crate test;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::mem;
use std::ops::Sub;
use std::path::Path;
use std::str::{FromStr, Split};
use std::time::{Duration, Instant};

use counter::Counter;
use itertools::Itertools;
use test::Bencher;

#[macro_use]
extern crate lazy_static;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            _ if s.to_uppercase() == s => Ok(Node::BigCave(s.to_string())),
            _ => Ok(Node::SmallCave(s.to_string())),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let elem =
            match self {
                Node::Start => {"start"}
                Node::End => {"end"}
                Node::BigCave(s) => {s.as_str()}
                Node::SmallCave(s) => {s.as_str()}
            };
        write!(f, "{}", elem)
    }
}

type NodePath<'a> = Vec<&'a Node>;

#[derive(Debug)]
struct Edge {
    from: Box<Node>,
    to: Box<Node>,
}

#[derive(Debug)]
struct UndirectedGraph {
    edges: Vec<Edge>,
}

impl UndirectedGraph {
    fn _parse_content(file_name: &str) -> UndirectedGraph {
        let mut g = UndirectedGraph::new();
        for line in utils::file_to_lines(file_name) {
            let (from_str, to_str) = line.split('-').collect_tuple().unwrap();
            g.add_edge(Node::from_str(from_str).unwrap(), Node::from_str(to_str).unwrap())
        }
        g
    }

    fn new() -> UndirectedGraph {
        let edges = Vec::<Edge>::new();
        UndirectedGraph { edges }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.edges.push(Edge {
            from: Box::from(from.clone()),
            to: Box::from(to.clone()),
        });
    }

    fn get_nodes_from(&self, from: &Node) -> HashSet<&Node> {
        let mut nodes = HashSet::<&Node>::new();
        for edge in self.edges.iter() {
            if edge.from.as_ref() == from {
                nodes.insert(edge.to.as_ref());
            }
            if edge.to.as_ref() == from {
                nodes.insert(edge.from.as_ref());
            }
        }
        nodes
    }

    fn get_nodes(&self) -> HashSet<&Node> {
        let mut nodes = HashSet::<&Node>::new();
        for edge in self.edges.iter() {
            nodes.insert(edge.from.as_ref());
            nodes.insert(edge.to.as_ref());
        }
        nodes
    }

    fn search_valid_pathes(&self, is_invalid_fn: fn(&NodePath) -> bool) -> Vec<NodePath> {
        let start_node = &Node::Start;
        let goal_node = &Node::End;

        let mut all_pathes: Vec<NodePath> = Vec::new();
        let mut queue: VecDeque<NodePath> = VecDeque::new();
        queue.push_back(vec![start_node]);

        while !queue.is_empty() {
            let curr_path = queue.pop_front().unwrap();
            let curr_node = *curr_path.last().unwrap();
            //println!("Removed from queue {:?}", curr_path);

            if curr_node == goal_node {
                //println!("Found {:?}", curr_path);
                all_pathes.push(curr_path);
            } else {
                for child_node in &self.get_nodes_from(&curr_node) {
                    let mut new_path = curr_path.clone();
                    new_path.push(child_node);
                    if !is_invalid_fn(&new_path) {
                        //println!("Added to queue {:?}", new_path);
                        queue.push_back(new_path);
                    } else {
                        //println!("Excluded {:?}", new_path);
                    }
                }
            }
        }
        all_pathes
    }
}

fn path_nok_part1(new_path: &NodePath) -> bool {
    let (start_count, how_many_above_1, _) = get_path_metrics(new_path);
    return start_count > 1 || how_many_above_1 != 0;
}

fn path_nok_part2(new_path: &NodePath) -> bool {
    let (start_count, how_many_above_1, how_many_above_2) = get_path_metrics(new_path);
    start_count > 1 || how_many_above_1 > 1 || how_many_above_2 != 0
}

fn get_path_metrics(new_path: &NodePath) -> (i32, usize, usize) {
    let mut counter: Counter<&String, usize> = Counter::new();
    let mut start_count = 0;
    for node in new_path.iter() {
        match node {
            Node::SmallCave(s) => {
                counter[&s] += 1;
            }
            Node::Start => {
                start_count += 1;
            }
            _ => {}
        }
    }

    let how_many_above_1 = counter.iter().filter(|(_, value)| value >= &&2).count();
    let how_many_above_2 = counter.iter().filter(|(_, value)| value >= &&3).count();
    (start_count, how_many_above_1, how_many_above_2)
}

fn print_pathnodes(all_pathes: &Vec<NodePath>) {
    for path in all_pathes.iter().sorted() {
        for node in path {
            print!("{},", node);
        }
        println!();
    }
}


/// The part1 function calculates the result for part2
fn solve_part1(file_name: &str) -> Result<usize, String> {
    let g = UndirectedGraph::_parse_content(file_name);
    //println!("g: {:?}", g);

    let all_pathes = g.search_valid_pathes(path_nok_part1);
    //print_pathnodes(&all_pathes);

    Ok(all_pathes.len())
}

/// The part2 function calculates the result for part2
fn solve_part2(file_name: &str) -> Result<usize, String> {
    let g = UndirectedGraph::_parse_content(file_name);
    //println!("g: {:?}", g);

    let all_pathes = g.search_valid_pathes(path_nok_part2);
    //print_pathnodes(&all_pathes);

    Ok(all_pathes.len())
}


////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 226);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 4749);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 3509);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 123054);
        Ok(())
    }

    /*
    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
     */
}
