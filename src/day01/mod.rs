use std::fs;

pub fn part1() {
    let content = fs::read_to_string("src/day01/input.txt").unwrap();
    let lines = content.trim().lines();
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();
    for line in lines {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    let dist: u32 = left
        .into_iter()
        .zip(right)
        .map(|(x, y)| x.abs_diff(y))
        .sum();
    println!("{dist}")
}
