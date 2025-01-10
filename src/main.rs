use std::{
    fs::read_to_string,
    iter::empty,
    path::{
        Path,
        PathBuf,
    },
    time::{
        Duration,
        Instant,
    },
};

use advent_of_code::utils::ansi::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    year: Option<String>,
    day: Option<String>,
}

fn main() {
    let args = Args::parse();

    let solutions: Vec<_> = empty()
        .chain(template_year())
        .chain(year2015())
        .chain(year2024())
        .filter(|solution| args.year.clone().is_none_or(|y| y == solution.year))
        .filter(|solution| args.day.clone().is_none_or(|d| d == solution.day))
        .filter(|solution| args.year.is_some() || solution.year != "template_year")
        .collect();

    let mut duration = Duration::ZERO;

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in &solutions
    {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            duration += instant.elapsed();

            println!("{BOLD}{YELLOW}{year} {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
        } else {
            eprintln!("{BOLD}{RED}{year} {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!(
                "    Place input file in {BOLD}{WHITE}{}{RESET}",
                path.display()
            );
        }
    }

    println!(
        "{BOLD}{WHITE}🕓 {}.{} ms{RESET}",
        duration.as_millis(),
        duration.as_nanos() % 1000
    );
}

struct Solution {
    year: String,
    day: String,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use advent_of_code::$year::$day::*;

                    let parsed = parse(&data);
                    let part1 = part1(&parsed);
                    let part2 = part2(&parsed);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year: year.to_string(), day: day.to_string(), path, wrapper }
            },)*]
        }
    }
}

run!(template_year
    template_day
);

run!(year2015
    day01
);

run!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);
