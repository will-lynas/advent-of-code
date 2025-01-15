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

pub fn part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|claw| {
            let (a, b, goal) = claw;

            // | a.x b.x | | ap |   | goal.x |
            // | a.y b.y | | bp | = | goal.y |
            //
            // | ap |           | b.y  -b.x | | goal.x |
            // | bp | = (1/det) | -a.y a.x  | | goal.y |

            let det = a.x * b.y - a.y * b.x;
            if det == 0 {
                return 0;
            }

            let mut ap = goal.x * b.y - goal.y * b.x;
            let mut bp = goal.y * a.x - goal.x * a.y;

            if ap % det != 0 || bp % det != 0 {
                return 0;
            }

            ap /= det;
            bp /= det;

            ap * 3 + bp
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
