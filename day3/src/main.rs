use itertools::Itertools;
use std::fs;

fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut best = 0u64;

        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                let n = (chars[i] as u8 - b'0') as u64 * 10 + (chars[j] as u8 - b'0') as u64;
                if n > best {
                    best = n;
                }
            }
        }

        total += best;
    }

    total
}

pub fn part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|bank| {
            let mut batteries: Vec<char> = vec![];

            let mut current_index = 0;
            for i in 0..11 {
                let (index, first_max) = &bank[current_index..(bank.len() - 11 + i)]
                    .chars()
                    .enumerate()
                    .max_set_by_key(|(_index, battery)| *battery)
                    .first()
                    .cloned()
                    .unwrap();

                batteries.push(*first_max);
                current_index = current_index + index + 1;
            }

            let (_second_index, second_max) = &bank[(current_index)..]
                .chars()
                .enumerate()
                .max_by_key(|(_index, battery)| *battery)
                .unwrap();

            batteries.push(*second_max);

            batteries.iter().collect::<String>().parse::<u64>().unwrap()
        })
        .sum::<u64>();

    result.to_string()
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let out = part2(&raw);
    println!("{out}");
}
