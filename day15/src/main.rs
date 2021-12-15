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
use std::collections::{HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::mem;
use std::ops::Sub;
use std::path::Path;
use std::str::{FromStr, Split};
use std::time::{Duration, Instant};

use array2d::Array2D;
use counter::Counter;
//use counter::Counter;
use itertools::{all, Itertools};
use petgraph::algo::{astar, dijkstra, floyd_warshall};
use petgraph::graphmap::GraphMap;
use petgraph::{Directed, Graph, Undirected};
use test::Bencher;

//#[macro_use]
//extern crate lazy_static;

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

type TileType = Array2D<usize>;
type GraphType = GraphMap<(usize, usize), usize, Directed>;

fn solve_part1(file_name: &str) -> Result<usize, String> {
    let tile = parse_content(file_name);
    //print_array2d(&tile);

    let costs = calc_costs_topleft_bottom_right(create_graph(&tile));
    Ok(costs)
}

fn solve_part2(file_name: &str) -> Result<usize, String> {
    let big_tile = create_big_tile(&parse_content(file_name), 5, 5);
    //print_array2d(&big_tile);

    let costs = calc_costs_topleft_bottom_right(create_graph(&big_tile));
    Ok(costs)
}

fn parse_content(file_name: &str) -> TileType {
    let content = &utils::file_to_string(file_name);
    let original_tile = content
        .split("\r\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    Array2D::from_columns(&original_tile)
}

fn create_graph(tile: &TileType) -> GraphType {
    let mut graph: GraphType = GraphType::new();
    let x_size = tile.num_columns() as isize;
    let y_size = tile.num_rows() as isize;
    for x in 0..x_size {
        for y in 0..y_size {
            for (x_neighbor, y_neighbor) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                if x_neighbor >= 0 && x_neighbor < x_size && y_neighbor >= 0 && y_neighbor < y_size
                {
                    let x = x as usize;
                    let y = y as usize;
                    let x_neighbor = x_neighbor as usize;
                    let y_neighbor = y_neighbor as usize;
                    graph.add_edge(
                        (x, y),
                        (x_neighbor, y_neighbor),
                        tile[(x_neighbor, y_neighbor)],
                    );
                }
            }
        }
    }
    graph
}

fn create_big_tile(tile: &TileType, amount_x: usize, amount_y: usize) -> TileType {
    let x_size = tile.num_columns();
    let y_size = tile.num_rows();
    let mut big_tile = Array2D::filled_with(00, 5 * y_size as usize, 5 * x_size as usize);
    for tile_x in 0..amount_x {
        for tile_y in 0..amount_y {
            for x in 0..x_size {
                for y in 0..y_size {
                    big_tile[((tile_x * x_size) + x, (tile_y * y_size) + y)] =
                        ((tile_x + tile_y) + tile[(x, y)] - 1) % 9 + 1;
                }
            }
        }
    }
    big_tile
}

fn calc_costs_topleft_bottom_right(graph: GraphType) -> usize {
    let start = (0, 0);
    let end = graph.nodes().sorted().rev().next().unwrap();
    /*
    let node_map = dijkstra(&graph, start, Some(end), |(_, _, weight)| *weight);
    node_map[&end]
    */
    let (costs, _) = astar(
        &graph,
        start,
        |finish| finish == end,
        |(_, _, weight)| *weight,
        // wrong heuristic, leads to wrong path: |pos| (end.0 - pos.0).pow(2) + (end.1 - pos.1).pow(2),
        |pos| (end.0 - pos.0) + (end.1 - pos.1),
    )
    .unwrap();

    costs
}

fn print_array2d(tile: &Array2D<isize>) {
    for y in 0..tile.num_rows() {
        for x in 0..tile.num_columns() {
            print!("{} ", tile[(x, y)]);
        }
        println!();
    }
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::utils::{file_to_string, parse_input};

    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 40);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 423);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 315);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 2778);
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
