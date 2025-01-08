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
    lines
        .iter()
        .filter_map(|line| {
            if valid(line.0, 0, &line.1) {
                Some(line.0)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(_lines: &[Line]) -> usize {
    0
}
