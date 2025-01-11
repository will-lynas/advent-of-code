use advent_of_code::year2024::day10 as solution;
use indoc::indoc;

const EXAMPLE: &str = indoc! {"
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
"};

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 36);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 81);
}
