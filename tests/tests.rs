// Template:

// use indoc::indoc;
//
// use advent_of_code::year2024::day01 as solution;
//
// const EXAMPLE: &str = indoc! {"
//     TESTING
// "};
//
// #[test]
// fn part1() {
//     assert_eq!(solution::part1(EXAMPLE), 69);
// }
//
// #[test]
// fn part2() {
//     assert_eq!(solution::part2(EXAMPLE), 420);
// }

macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2024
    day06
);
