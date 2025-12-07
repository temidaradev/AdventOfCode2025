use std::collections::HashSet;
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

pub fn part1(input: &str) -> String {
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

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let mut positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, value)| {
                (value == '@').then_some(IVec2::new(x as i32, y as i32))
            })
        })
        .collect::<HashSet<IVec2>>();

    let mut removed_count = 0;
    loop {
        let rolls_to_remove: HashSet<IVec2> = positions
            .iter()
            .filter(|&position| {
                NEIGHBORS
                    .iter()
                    .filter(|&offset| positions.contains(&(position + offset)))
                    .count()
                    < 4
            })
            .cloned()
            .collect();
        if rolls_to_remove.len() == 0 {
            break;
        } else {
            removed_count += rolls_to_remove.len();
        }
        positions = positions.difference(&rolls_to_remove).cloned().collect();
    }
    removed_count.to_string()
}
