use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let moves = file
        .lines()
        .map(|line| {
            let n = line[1..].parse::<i64>().unwrap();
            if line.starts_with('R') {
                n
            } else {
                -n
            }
        })
        .collect::<Vec<i64>>();

    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    for step in moves {
        pos += step;
        let rotations = if step < 0 {
            (((step - pos + 100) % 100) - step) / 100
        } else {
            pos / 100
        };
        count += rotations;
        pos = pos.rem_euclid(100);
    }

    println!("{}", count);
}
