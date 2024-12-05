use std::fs;

use itertools::Itertools;

fn lines() -> Vec<Vec<i32>> {
    let content = fs::read_to_string("src/day02/input.txt").unwrap();
    content
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(nums: &[i32]) -> bool {
    let mut nums = nums.iter().tuple_windows().map(|(a, b)| a - b);
    nums.clone().all(|n| (1..=3).contains(&n)) || nums.all(|n| (-3..=-1).contains(&n))
}

pub fn part1() {
    let count = lines().iter().filter(|line| is_safe(line)).count();
    println!("{count:?}")
}

pub fn part2() {
    let count = lines()
        .iter()
        .filter(|&line| {
            (0..line.len()).any(|i| {
                let mut v = line.clone();
                v.remove(i);
                is_safe(&v)
            })
        })
        .count();
    println!("{count:?}")
}
