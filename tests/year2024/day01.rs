use advent_of_code::year2024::day01 as solution;

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
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 11);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 31);
}
