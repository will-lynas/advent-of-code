use advent_of_code::year2015::day01 as solution;

#[test]
fn part1() {
    let input = solution::parse("))(((((");
    assert_eq!(solution::part1(&input), 3);
}

#[test]
fn part2() {
    let input = solution::parse("()())");
    assert_eq!(solution::part2(&input), 5);
}
