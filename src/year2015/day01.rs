pub fn parse(input: &str) -> String {
    input.to_string()
}

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
