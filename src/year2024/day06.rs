use std::{
    collections::HashSet,
    thread::{self, available_parallelism},
};

use crate::utils::{grid::Grid, point::UP};

pub fn parse(input: &str) -> Grid {
    Grid::parse(input)
}

pub fn part1(grid: &Grid) -> usize {
    let mut pos = grid.find(&'^').unwrap();
    let mut dir = UP;

    let mut visited = HashSet::new();
    'outer: loop {
        visited.insert(pos);
        loop {
            let new_pos = pos + dir;
            if !grid.contains(new_pos) {
                break 'outer;
            }
            if grid[new_pos] != '#' {
                pos = new_pos;
                break;
            }
            dir.rotate_clockwise();
        }
    }
    visited.len()
}

pub fn part2(grid: &Grid) -> usize {
    let original_pos = grid.find(&'^').unwrap();

    let mut pos = original_pos;
    let mut dir = UP;

    let mut path = HashSet::new();
    'outer: loop {
        path.insert(pos);
        loop {
            let new_pos = pos + dir;
            if !grid.contains(new_pos) {
                break 'outer;
            }
            if grid[new_pos] != '#' {
                pos = new_pos;
                break;
            }
            dir.rotate_clockwise();
        }
    }
    path.len();
    path.remove(&original_pos);

    let path_vec: Vec<_> = path.into_iter().collect();
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
                    grid[obstacle_pos] = '#';

                    let mut pos = original_pos;
                    let mut dir = UP;

                    let mut visited = HashSet::new();
                    'outer: loop {
                        if !visited.insert((pos, dir)) {
                            local_count += 1;
                            break;
                        }
                        loop {
                            let new_pos = pos + dir;
                            if !grid.contains(new_pos) {
                                break 'outer;
                            }
                            if grid[new_pos] != '#' {
                                pos = new_pos;
                                break;
                            }
                            dir.rotate_clockwise();
                        }
                    }

                    grid[obstacle_pos] = '.';
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
