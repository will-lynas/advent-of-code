use crate::utils::{
    grid::Grid,
    point::{
        DIRS,
        DOWN_LEFT,
        DOWN_RIGHT,
        UP_LEFT,
        UP_RIGHT,
    },
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    let target = "XMAS".as_bytes();

    let mut count = 0;
    for (start, val) in grid {
        if val != target.iter().next().unwrap() {
            continue;
        }
        for dir in DIRS {
            let mut matched = true;
            for (i, &c) in target.iter().enumerate().skip(1) {
                let new = start + dir * i;
                if !grid.contains(new) || c != grid[new] {
                    matched = false;
                    break;
                }
            }
            if matched {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(grid: &Grid<u8>) -> usize {
    grid.inner_points(1)
        .into_iter()
        .filter(|&point| {
            grid[point] == b'A'
                && ((grid[point + UP_LEFT] == b'M' && grid[point + DOWN_RIGHT] == b'S')
                    || (grid[point + UP_LEFT] == b'S' && grid[point + DOWN_RIGHT] == b'M'))
                && ((grid[point + UP_RIGHT] == b'M' && grid[point + DOWN_LEFT] == b'S')
                    || (grid[point + UP_RIGHT] == b'S' && grid[point + DOWN_LEFT] == b'M'))
        })
        .count()
}
