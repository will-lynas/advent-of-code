use indoc::indoc;

use advent_of_code::year2024::day06::*;

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
    assert_eq!(part2(EXAMPLE), 6);
}
