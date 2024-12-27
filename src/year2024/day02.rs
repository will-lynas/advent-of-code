use itertools::Itertools;

fn lines(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(nums: &[i32]) -> bool {
    let mut nums = nums.iter().tuple_windows().map(|(a, b)| a - b);
    nums.clone().all(|n| (1..=3).contains(&n)) || nums.all(|n| (-3..=-1).contains(&n))
}

pub fn part1(input: &str) -> usize {
    lines(input).iter().filter(|line| is_safe(line)).count()
}

pub fn part2(input: &str) -> usize {
    lines(input)
        .iter()
        .filter(|&line| {
            (0..line.len()).any(|i| {
                let mut v = line.clone();
                v.remove(i);
                is_safe(&v)
            })
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! { "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}
