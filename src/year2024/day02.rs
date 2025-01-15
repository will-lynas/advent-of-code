use itertools::Itertools;

use crate::utils::parsing::StringNumberParsing;

type Lines = Vec<Vec<i64>>;

pub fn parse(input: &str) -> Lines {
    input
        .trim()
        .lines()
        .map(StringNumberParsing::signed_nums)
        .collect()
}

pub fn part1(lines: &Lines) -> usize {
    lines.iter().filter(|line| is_safe(line)).count()
}

pub fn part2(lines: &Lines) -> usize {
    lines
        .iter()
        .filter(|&line| {
            (0..line.len()).any(|i| {
                let mut v = line.clone();
                v.remove(i);
                is_safe(&v)
            })
        })
        .count()
}

fn is_safe(nums: &[i64]) -> bool {
    let mut nums = nums.iter().tuple_windows().map(|(a, b)| a - b);
    nums.clone().all(|n| (1..=3).contains(&n)) || nums.all(|n| (-3..=-1).contains(&n))
}
