use std::fs;

pub fn part1() {
    let content = fs::read_to_string("src/day03/input.txt").unwrap();
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(&content) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    println!("{sum}")
}

pub fn part2() {}
