use advent_of_code::year2024::day12 as solution;
use indoc::indoc;

const EXAMPLE: &str = indoc! {"
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
"};

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 1930);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 1206);
}
