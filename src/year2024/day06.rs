use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_guard(grid: &Grid) -> (usize, usize) {
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'^' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let (mut x, mut y) = find_guard(&grid);
    let mut di = 0;

    let mut visited = HashSet::new();
    'outer: loop {
        visited.insert((x, y));
        loop {
            let (dx, dy) = DIRS[di];
            let (tx, ty) = (x as isize + dx, y as isize + dy);
            if tx < 0 || tx >= rows as isize || ty < 0 || ty >= cols as isize {
                break 'outer;
            }
            if grid[tx as usize][ty as usize] != '#' {
                (x, y) = (tx as usize, ty as usize);
                break;
            }
            di += 1;
            di %= 4;
        }
    }
    visited.len()
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
