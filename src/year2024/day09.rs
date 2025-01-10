type FileSystem = Vec<Option<u64>>;

pub fn parse(input: &str) -> FileSystem {
    let nums = input.trim().chars().map(|c| c.to_string().parse().unwrap());
    let mut out = Vec::new();
    for (i, n) in nums.enumerate() {
        let val = (i % 2 == 0).then_some((i / 2) as u64);
        for _ in 0..n {
            out.push(val);
        }
    }
    out
}

pub fn part1(nums: &FileSystem) -> u64 {
    let mut nums = nums.clone();
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

pub fn part2(_nums: &FileSystem) -> u64 {
    0
}
