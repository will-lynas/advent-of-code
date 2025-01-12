use gxhash::{
    HashMap,
    HashMapExt,
};

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

fn inner(stones: &Stones, remaining: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if remaining == 0 {
        return stones.len();
    }
    stones
        .iter()
        .map(|&stone| {
            if let Some(&ans) = cache.get(&(stone, remaining - 1)) {
                return ans;
            }

            let new_stones = if stone == 0 {
                vec![1]
            } else if let Some(res) = split_digits(stone) {
                vec![res.0, res.1]
            } else {
                vec![stone * 2024]
            };

            let ans = inner(&new_stones, remaining - 1, cache);
            cache.insert((stone, remaining - 1), ans);
            ans
        })
        .sum()
}

fn answer(stones: &Stones, iterations: usize) -> usize {
    let mut cache = HashMap::new();
    inner(stones, iterations, &mut cache)
}

pub fn part1(stones: &Stones) -> usize {
    answer(stones, 25)
}

pub fn part2(stones: &Stones) -> usize {
    answer(stones, 75)
}
