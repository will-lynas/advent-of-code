use advent_of_code::year2024::day23 as solution;

const EXAMPLE: &str = "Hello, World!";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 13);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 13);
}
