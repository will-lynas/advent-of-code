pub fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.trim().chars() {
        if c == '(' {
            floor += 1
        } else {
            floor -= 1
        }
    }

    floor
}

pub fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.trim().chars().enumerate() {
        if c == '(' {
            floor += 1
        } else {
            floor -= 1
        }

        if floor < 0 {
            return i + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    // Most of the test cases are excluded

    #[test]
    fn part1_test() {
        assert_eq!(part1("))((((("), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("()())"), 5);
    }
}
