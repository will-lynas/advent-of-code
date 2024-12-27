use clap::Parser;
use std::{
    fs::read_to_string,
    iter::empty,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use utils::ansi::*;

pub mod utils;

pub mod year2024;

#[derive(Parser, Debug)]
struct Args {
    year: Option<String>,
    day: Option<String>,
}

fn main() {
    let args = Args::parse();

    let solutions: Vec<_> = empty()
        .chain(year2024())
        .filter(|solution| args.year.clone().is_none_or(|y| y == solution.year))
        .filter(|solution| args.day.clone().is_none_or(|d| d == solution.day))
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

    println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
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
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year: year.to_string(), day: day.to_string(), path, wrapper }
            },)*]
        }
    }
}

run!(year2024
    day01
);
