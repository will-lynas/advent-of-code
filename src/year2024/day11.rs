type Input = Vec<u64>;

pub fn parse(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    println!("{input:?}");
    input.len()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
