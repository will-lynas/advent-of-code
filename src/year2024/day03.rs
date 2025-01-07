pub fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    let mut parts = input.split("don't()");
    let mut enable_parts = vec![parts.next().unwrap()]; // Start enabled
    for part in parts {
        if let Some((_, good_part)) = part.split_once("do()") {
            enable_parts.push(good_part);
        };
    }
    enable_parts.iter().map(|part| part1(part)).sum()
}
