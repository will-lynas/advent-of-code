use crate::utils::{
    parsing::StringNumberParsing,
    point::Point,
};

type Input = Vec<(Point, Point, Point)>;

fn parse_line(line: &str) -> Point {
    let nums = line.unsigned_nums();
    Point::new(nums[0] as i32, nums[1] as i32)
}

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            (
                parse_line(lines.next().unwrap()),
                parse_line(lines.next().unwrap()),
                parse_line(lines.next().unwrap()),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    dbg!(input);
    input.len()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
