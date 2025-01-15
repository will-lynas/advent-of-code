use crate::utils::parsing::StringNumberParsing;

type Claw = [i64; 6];
type Input = Vec<Claw>;

pub fn parse(input: &str) -> Input {
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

fn solve([ax, ay, bx, by, mut gx, mut gy]: Claw, part2: bool) -> i64 {
    if part2 {
        gx += 10_000_000_000_000;
        gy += 10_000_000_000_000;
    }

    // | ax bx | | ap |   | gx |
    // | ay by | | bp | = | gy |
    //
    // | ap |           |  by -bx | | gx |
    // | bp | = (1/det) | -ay  ax | | gy |

    let det = ax * by - ay * bx;
    if det == 0 {
        return 0;
    }

    let mut ap = gx * by - gy * bx;
    let mut bp = gy * ax - gx * ay;

    if ap % det != 0 || bp % det != 0 {
        return 0;
    }

    ap /= det;
    bp /= det;

    ap * 3 + bp
}

pub fn part1(input: &Input) -> i64 {
    input.iter().copied().map(|claw| solve(claw, false)).sum()
}

pub fn part2(input: &Input) -> i64 {
    input.iter().copied().map(|claw| solve(claw, true)).sum()
}
