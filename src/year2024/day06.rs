use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { row: x, col: y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_guard(grid: &Grid) -> Point {
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'^' {
                return Point::new(i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut pos = find_guard(&grid);

    let directions = [
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
    ];
    let mut dir_index = 0;

    let mut visited = HashSet::new();
    'outer: loop {
        visited.insert(pos);
        loop {
            let new_pos = pos + directions[dir_index % directions.len()];
            if new_pos.row < 0 || new_pos.row >= rows || new_pos.col < 0 || new_pos.col >= cols {
                break 'outer;
            }
            if grid[new_pos.row as usize][new_pos.col as usize] != '#' {
                pos = new_pos;
                break;
            }
            dir_index += 1;
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
