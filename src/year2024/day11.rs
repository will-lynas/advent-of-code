type Stones = Vec<u64>;

pub fn parse(input: &str) -> Stones {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    let n = n.to_string();
    let l = n.len();
    (l % 2 == 0).then(|| {
        let (a, b) = n.split_at(l / 2);
        (a.parse().unwrap(), b.parse().unwrap())
    })
}

pub fn part1(stones: &Stones) -> usize {
    let mut stones = stones.clone();
    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for &stone in &stones {
            if stone == 0 {
                new_stones.push(1);
            } else if let Some(res) = split_digits(stone) {
                new_stones.push(res.0);
                new_stones.push(res.1);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    stones.len()
}

pub fn part2(stones: &Stones) -> usize {
    stones.len()
}
