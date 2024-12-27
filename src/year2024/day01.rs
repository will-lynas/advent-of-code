use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    let lines = input.trim().lines();
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();
    for line in lines {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    (left, right)
}

pub fn part1(input: &Input) -> u32 {
    let (left, right) = input;
    left.into_iter()
        .zip(right)
        .map(|(x, y)| x.abs_diff(y.clone()))
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let (left, right) = input;
    let counts = right
        .into_iter()
        .fold(HashMap::<u32, u32>::new(), |mut acc, item| {
            *acc.entry(item.clone()).or_insert(0) += 1;
            acc
        });
    left.iter()
        .filter_map(|num| counts.get(num).map(|&val2| num * val2))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! { "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3     
    "};

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 31);
    }
}
