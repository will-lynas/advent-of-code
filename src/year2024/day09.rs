pub fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

pub fn part1(input: &[u64]) -> u64 {
    let mut nums = Vec::new();
    for (i, n) in input.iter().enumerate() {
        let val = (i % 2 == 0).then_some((i / 2) as u64);
        for _ in 0..*n {
            nums.push(val);
        }
    }
    let mut front = 0;
    let mut back = nums.len() - 1;
    while front < back {
        if nums[front].is_some() {
            front += 1;
            continue;
        }
        if nums[back].is_none() {
            back -= 1;
            continue;
        }
        nums.swap(front, back);
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|n| n * i as u64))
        .sum()
}

#[derive(Debug)]
struct File {
    number: u64,
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Space {
    start: u64,
    length: u64,
}

pub fn part2(input: &[u64]) -> u64 {
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut pos = 0;
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            files.push(File {
                number: i as u64 / 2,
                start: pos,
                length: *n,
            });
        } else {
            spaces.push(Space {
                start: pos,
                length: *n,
            });
        }
        pos += n;
    }

    for file in files.iter_mut().rev() {
        for space in &mut spaces {
            if space.start > file.start {
                break;
            }
            if space.length >= file.length {
                file.start = space.start;
                space.length -= file.length;
                space.start += file.length;
                break;
            }
        }
    }

    files
        .iter()
        .map(|file| file.number * file.length * (2 * file.start + file.length - 1) / 2)
        .sum()
}
