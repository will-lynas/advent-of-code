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

test!(template_year
    template_day
);

test!(year2015
    day01
);

test!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);
