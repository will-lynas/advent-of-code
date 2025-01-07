use gxhash::{HashSet, HashSetExt};
use std::thread::{self, available_parallelism};

use crate::utils::{grid::Grid, point::UP};

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

    let path_vec: Vec<_> = visited.into_iter().collect();
    let threads: usize = available_parallelism().unwrap().into();
    let chunk_size = path_vec.len().div_ceil(threads);

    let path_chunks: Vec<_> = path_vec.chunks(chunk_size).map(<[_]>::to_vec).collect();

    let handles: Vec<_> = path_chunks
        .into_iter()
        .map(|chunk| {
            let mut grid = grid.clone();
            thread::spawn(move || {
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
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}
