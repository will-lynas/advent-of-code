#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        mod $year {$(
            mod $day {
                use advent_of_code::$year::$day as solution;
                use std::fs::read_to_string;
                use std::path::Path;
                use std::sync::LazyLock;
                use test::Bencher;

                static INPUT: LazyLock<String> = LazyLock::new(|| {
                    let year = stringify!($year);
                    let day = stringify!($day);
                    let path = Path::new("input").join(year).join(day).with_extension("txt");
                    read_to_string(path).unwrap()
                });

                #[bench]
                fn part1(b: &mut Bencher) {
                    b.iter(|| {
                        let input = solution::parse(&INPUT);
                        solution::part1(&input)
                    });
                }

                #[bench]
                fn part2(b: &mut Bencher) {
                    b.iter(|| {
                        let input = solution::parse(&INPUT);
                        solution::part2(&input)
                    });
                }
            }
        )*}
    }
}

benchmark!(template_year
    template_day
);

benchmark!(year2015
    day01
);

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);
