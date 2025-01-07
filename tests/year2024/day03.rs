use advent_of_code::year2024::day03 as solution;

#[test]
fn part1() {
    let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input = solution::parse(example);
    assert_eq!(solution::part1(&input), 161);
}

#[test]
fn part2() {
    let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = solution::parse(example);
    assert_eq!(solution::part2(&input), 48);
}
