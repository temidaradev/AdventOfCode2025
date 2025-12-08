use day6::part1;
use std::fs;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&raw));
}
