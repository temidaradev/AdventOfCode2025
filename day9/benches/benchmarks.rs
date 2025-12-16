use day9::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() {
    let input = include_str!("../input.txt");
    divan::black_box(part1(input));
}

#[divan::bench]
fn bench_part2() {
    let input = include_str!("../input.txt");
    divan::black_box(part2(input));
}
