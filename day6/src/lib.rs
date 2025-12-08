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
    todo!()
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
