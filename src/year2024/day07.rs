use crate::utils::threading::run_threads;

type Line = (u64, Vec<u64>);

pub fn parse(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (goal, nums) = line.split_once(':').unwrap();
            let goal: u64 = goal.parse().unwrap();
            let nums: Vec<u64> = nums.trim().split(' ').map(|n| n.parse().unwrap()).collect();
            (goal, nums)
        })
        .collect()
}

fn valid(goal: u64, current: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return goal == current;
    }
    let first = nums.first().unwrap();
    let nums = &nums[1..nums.len()];
    valid(goal, current + first, nums) || valid(goal, current * first, nums)
}

pub fn part1(lines: &[Line]) -> u64 {
    let fun = |chunk: Vec<Line>| {
        chunk
            .iter()
            .filter_map(|line| valid(line.0, 0, &line.1).then_some(line.0))
            .sum::<u64>()
    };
    run_threads(lines.to_vec(), fun).into_iter().sum()
}

fn valid2(goal: u64, current: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return goal == current;
    }
    let first = nums.first().unwrap();
    let nums = &nums[1..nums.len()];
    valid2(goal, current + first, nums)
        || valid2(goal, current * first, nums)
        || valid2(
            goal,
            (current.to_string() + &first.to_string()).parse().unwrap(),
            nums,
        )
}

pub fn part2(lines: &[Line]) -> u64 {
    let fun = |chunk: Vec<Line>| {
        chunk
            .iter()
            .filter_map(|line| valid2(line.0, 0, &line.1).then_some(line.0))
            .sum::<u64>()
    };
    run_threads(lines.to_vec(), fun).into_iter().sum()
}
