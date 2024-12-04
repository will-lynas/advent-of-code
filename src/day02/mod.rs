use std::fs;

pub fn part1() {
    let content = fs::read_to_string("src/day02/example.txt").unwrap();
    let nums: Vec<Vec<u32>> = content
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    print!("{nums:?}")
}

pub fn part2() {}
