use indoc::indoc;

use advent_of_code::year2024::day04 as solution;

const EXAMPLE: &str = indoc! {"
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
"};

#[test]
fn part1() {
    assert_eq!(solution::part1(EXAMPLE), 18);
}

#[test]
fn part2() {
    assert_eq!(solution::part2(EXAMPLE), 9);
}
