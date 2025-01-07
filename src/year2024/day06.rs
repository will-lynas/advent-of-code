use std::{
    collections::HashSet,
    thread::{self, available_parallelism},
};

use crate::utils::point::{Point, DIRS};

type Grid = Vec<Vec<char>>;

pub fn parse(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut pos = find_guard(grid);
    let mut di = 0;

    let mut visited = HashSet::new();
    'outer: loop {
        visited.insert(pos);
        loop {
            let new_pos = pos + DIRS[di];
            if new_pos.y < 0
                || new_pos.y >= rows as i32
                || new_pos.x < 0
                || new_pos.x >= cols as i32
            {
                break 'outer;
            }
            if grid[new_pos.y as usize][new_pos.x as usize] != '#' {
                pos = new_pos;
                break;
            }
            di += 1;
            di %= 4;
        }
    }
    visited.len()
}

pub fn part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let original_pos = find_guard(grid);

    let mut pos = original_pos;
    let mut di = 0;
    let mut path = HashSet::new();
    'outer: loop {
        path.insert(pos);
        loop {
            let new_pos = pos + DIRS[di];
            if new_pos.y < 0
                || new_pos.y >= rows as i32
                || new_pos.x < 0
                || new_pos.x >= cols as i32
            {
                break 'outer;
            }
            if grid[new_pos.y as usize][new_pos.x as usize] != '#' {
                pos = new_pos;
                break;
            }
            di += 1;
            di %= 4;
        }
    }
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
                    grid[obstacle_pos.y as usize][obstacle_pos.x as usize] = '#';

                    let mut pos = original_pos;
                    let mut di = 0;

                    let mut visited = HashSet::new();
                    'outer: loop {
                        if !visited.insert((pos, di)) {
                            local_count += 1;
                            break;
                        }
                        loop {
                            let new_pos = pos + DIRS[di];
                            if new_pos.y < 0
                                || new_pos.y >= rows as i32
                                || new_pos.x < 0
                                || new_pos.x >= cols as i32
                            {
                                break 'outer;
                            }
                            if grid[new_pos.y as usize][new_pos.x as usize] != '#' {
                                pos = new_pos;
                                break;
                            }
                            di += 1;
                            di %= 4;
                        }
                    }

                    grid[obstacle_pos.y as usize][obstacle_pos.x as usize] = '.';
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

fn find_guard(grid: &Grid) -> Point {
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'^' {
                return Point::new(j as i32, i as i32);
            }
        }
    }
    unreachable!()
}
