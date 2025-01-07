use advent_of_code::year2015::day01 as solution;

#[test]
fn part1() {
    assert_eq!(solution::part1("))((((("), 3);
}

#[test]
fn part2() {
    assert_eq!(solution::part2("()())"), 5);
}
