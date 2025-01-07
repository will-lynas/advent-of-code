use indoc::indoc;

use advent_of_code::year2024::day06 as solution;

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
fn part1() {
    assert_eq!(solution::part1(EXAMPLE), 41);
}

#[test]
fn part2() {
    assert_eq!(solution::part2(EXAMPLE), 6);
}
