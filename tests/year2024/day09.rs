use advent_of_code::year2024::day09 as solution;

const EXAMPLE: &str = "2333133121414131402";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 1928);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 0);
}
