use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    year: Option<u32>,
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();

    match (args.year, args.day) {
        (Some(year), Some(day)) => println!("Year: {}, Day: {}", year, day),
        (Some(year), None) => println!("Year: {}, Day: None", year),
        (None, None) => println!("Year: None, Day: None"),
        (None, Some(_)) => unreachable!(),
    }
}
