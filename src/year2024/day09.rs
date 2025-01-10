pub fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

pub fn part1(input: &[u64]) -> u64 {
    let mut nums = Vec::new();
    for (i, n) in input.iter().enumerate() {
        let val = (i % 2 == 0).then_some((i / 2) as u64);
        for _ in 0..*n {
            nums.push(val);
        }
    }
    let mut front = 0;
    let mut back = nums.len() - 1;
    while front < back {
        if nums[front].is_some() {
            front += 1;
            continue;
        }
        if nums[back].is_none() {
            back -= 1;
            continue;
        }
        nums.swap(front, back);
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|n| n * i as u64))
        .sum()
}

pub fn part2(_nums: &[u64]) -> u64 {
    0
}
