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

fn valid(goal: u64, nums: &[u64]) -> bool {
    if nums.len() == 1 {
        return goal == nums[0];
    }
    let last = nums.last().unwrap();
    let nums = &nums[0..nums.len() - 1];
    if &goal >= last && valid(goal - last, nums) {
        return true;
    }
    if goal % last == 0 && valid(goal / last, nums) {
        return true;
    }
    false
}

pub fn part1(lines: &[Line]) -> u64 {
    lines
        .iter()
        .filter_map(|line| {
            if valid(line.0, &line.1) {
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
