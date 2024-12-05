use std::fs;

use itertools::Itertools;

fn is_safe(nums: Vec<i32>) -> bool {
    let mut nums = nums.iter().tuple_windows().map(|(a, b)| a - b);
    nums.clone().all(|n| (1..=3).contains(&n)) || nums.all(|n| (-3..=-1).contains(&n))
}

pub fn part1() {
    let content = fs::read_to_string("src/day02/input.txt").unwrap();
    let count: usize = content
        .trim()
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            is_safe(nums)
        })
        .count();
    println!("{count:?}")
}

pub fn part2() {}
