pub fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

pub fn part2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(example), 161);
    }

    #[test]
    fn part2_test() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(example), 48);
    }
}
