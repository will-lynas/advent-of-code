use advent_of_code::year2024::day11 as solution;

const EXAMPLE: &str = "125 17";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 55312);
}

// Part 2 has no test
