use std::fs;

use day3::{part1, part2};

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let out = part2(&raw);
    println!("Part 1: {}", part1(&raw));
    println!("Part 2: {}", out);
}
