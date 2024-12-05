use std::fs;

use itertools::Itertools;

pub fn part1() {
    let content = fs::read_to_string("src/day02/input.txt").unwrap();
    let count: usize = content
        .trim()
        .lines()
        .filter(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| a - b);
            nums.clone().all(|n| (1..=3).contains(&n)) || nums.all(|n| (-3..=-1).contains(&n))
        })
        .count();
    print!("{count:?}")
}

pub fn part2() {}
