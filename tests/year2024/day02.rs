use indoc::indoc;

use advent_of_code::year2024::day02 as solution;

const EXAMPLE: &str = indoc! { "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
"};

#[test]
fn part1() {
    assert_eq!(solution::part1(EXAMPLE), 2);
}

#[test]
fn part2() {
    assert_eq!(solution::part2(EXAMPLE), 4);
}
