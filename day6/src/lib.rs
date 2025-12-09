use itertools::{izip, multizip, Itertools, Position};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

pub fn part1(input: &str) -> String {
    let (_input, (nums, ops)) = parse.parse(input).unwrap();
    let result = ops
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let it = (0..nums.len())
                .into_iter()
                .map(|inner_index| nums[inner_index][index]);
            match *op {
                "*" => it.product(),
                "+" => it.sum::<u64>(),
                _ => {
                    panic!("");
                }
            }
        })
        .sum::<u64>();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut ops = vec![];
    let mut lines_iterators = vec![];
    for (pos, line) in input.lines().with_position() {
        if let Position::Last = pos {
            let (_input, mut output) = operations(line).unwrap();
            output.reverse();
            ops = output;
        }
        lines_iterators.push(line.chars().rev());
    }
    let result = ops
        .iter()
        .map(|op| {
            let mut output = match *op {
                "*" => 1,
                "+" => 0,
                _ => {
                    panic!("");
                }
            };
            loop {
                let result: u64 = lines_iterators
                    .iter_mut()
                    .rev()
                    .filter_map(|line| line.next().and_then(|c| c.to_digit(10)))
                    .enumerate()
                    .map(|(places, digit)| digit as u64 * 10u64.pow(places as u32))
                    .sum();
                if result == 0 {
                    break;
                }
                match *op {
                    "*" => {
                        output *= result;
                    }
                    "+" => {
                        output += result;
                    }
                    _ => {
                        panic!("");
                    }
                }
            }
            output
        })
        .sum::<u64>();
    result.to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<&str>)> {
    separated_pair(
        separated_list1(
            line_ending,
            delimited(space0, separated_list1(space1, complete::u64), space0),
        ),
        line_ending,
        separated_list1(space1, alt((tag("*"), tag("+")))),
    )
    .parse(input)
}

fn operations(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alt((tag("*"), tag("+")))).parse(input)
}
