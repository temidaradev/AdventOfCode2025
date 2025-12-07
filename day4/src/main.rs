use std::collections::HashSet;
use std::fs;

use glam::*;
use nom::*;

fn part1(input: &str) -> String {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, value)| {
                (value == '@').then_some(IVec2::new(x as i32, y as i32))
            })
        })
        .collect::<HashSet<IVec2>>();

    println!("{:?}", positions);

    todo!()
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    part1(&raw);
}
