use crate::utils::parsing::StringNumberParsing;

type Claw = [i64; 6];

pub fn parse(input: &str) -> Vec<Claw> {
    input
        .split("\n\n")
        .map(|block| {
            let mut nums = block.signed_nums().into_iter();
            [
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ]
        })
        .collect()
}

fn solve(&[ax, ay, bx, by, mut gx, mut gy]: &Claw, part2: bool) -> i64 {
    if part2 {
        gx += 10_000_000_000_000;
        gy += 10_000_000_000_000;
    }

    // | ax bx | | a |   | gx |
    // | ay by | | b | = | gy |
    //
    // | a |           |  by -bx | | gx |
    // | b | = (1/det) | -ay  ax | | gy |

    let det = ax * by - ay * bx;
    if det == 0 {
        return 0;
    }

    let mut a = gx * by - gy * bx;
    let mut b = gy * ax - gx * ay;

    if a % det != 0 || b % det != 0 {
        return 0;
    }

    a /= det;
    b /= det;

    a * 3 + b
}

pub fn part1(input: &[Claw]) -> i64 {
    input.iter().map(|claw| solve(claw, false)).sum()
}

pub fn part2(input: &[Claw]) -> i64 {
    input.iter().map(|claw| solve(claw, true)).sum()
}
