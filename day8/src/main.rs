use day8::{part1, part2};
use std::fs;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&raw));
    println!("Part 2: {}", part2(&raw));
}
