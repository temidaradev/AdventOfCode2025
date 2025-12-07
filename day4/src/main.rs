use std::collections::HashSet;
use std::fs;

use glam::*;

const NEIGHBORS: [IVec2; 8] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

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

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| positions.contains(&(position + offset)))
                .count()
                < 4
        })
        .count();

    println!("{:?}", count);
    count.to_string()
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    part1(&raw);
}
