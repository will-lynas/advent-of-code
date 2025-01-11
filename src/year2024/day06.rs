use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    grid::Grid,
    point::UP,
    threading::run_threads,
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> usize {
    let mut pos = grid.find(&b'^').unwrap();
    let mut dir = UP;

    let mut visited = HashSet::new();
    while grid.contains(pos + dir) {
        if grid[pos + dir] == b'#' {
            dir.rotate_clockwise();
            continue;
        }
        visited.insert(pos);
        pos += dir;
    }
    visited.insert(pos);
    visited.len()
}

pub fn part2(grid: &Grid<u8>) -> usize {
    let original_pos = grid.find(&b'^').unwrap();

    let mut pos = original_pos;
    let mut dir = UP;

    let mut visited = HashSet::new();
    while grid.contains(pos + dir) {
        if grid[pos + dir] == b'#' {
            dir.rotate_clockwise();
            continue;
        }
        visited.insert(pos);
        pos += dir;
    }
    visited.insert(pos);
    visited.remove(&original_pos);

    let fun = |chunk| {
        let mut grid = grid.clone();
        let mut local_count = 0;
        for obstacle_pos in chunk {
            grid[obstacle_pos] = b'#';

            let mut pos = original_pos;
            let mut dir = UP;

            let mut visited = HashSet::new();
            while grid.contains(pos + dir) {
                if grid[pos + dir] == b'#' {
                    dir.rotate_clockwise();
                    continue;
                }
                if !visited.insert((pos, dir)) {
                    local_count += 1;
                    break;
                }
                pos += dir;
            }

            grid[obstacle_pos] = b'.';
        }
        local_count
    };

    let path_vec: Vec<_> = visited.into_iter().collect();
    run_threads(path_vec, fun).iter().sum()
}
