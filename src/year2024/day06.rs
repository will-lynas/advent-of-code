pub fn part1(_input: &str) -> u32 {
    0
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
