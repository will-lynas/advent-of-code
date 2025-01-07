use std::{
    collections::HashSet,
    thread::{self, available_parallelism},
};

type Grid = Vec<Vec<char>>;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

    let (mut x, mut y) = find_guard(grid);
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

pub fn part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let (gx, gy) = find_guard(grid);

    let (mut x, mut y) = (gx, gy);
    let mut di = 0;
    let mut path = HashSet::new();
    'outer: loop {
        path.insert((x, y));
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
    path.remove(&(gx, gy));

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
                for (ox, oy) in chunk {
                    grid[ox][oy] = '#';

                    let (mut x, mut y) = (gx, gy);
                    let mut di = 0;

                    let mut visited = HashSet::new();
                    'outer: loop {
                        if !visited.insert((x, y, di)) {
                            local_count += 1;
                            break;
                        }
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

                    grid[ox][oy] = '.';
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
