use std::collections::VecDeque;

use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    grid::Grid,
    point::Point,
};

type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

pub fn part1(grid: &Input) -> usize {
    let mut count = 0;
    for point in grid.points() {
        if grid[point] == b'0' {
            count += score(grid, point);
        }
    }
    count
}

fn score(grid: &Grid<u8>, point: Point) -> usize {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(point);
    while let Some(current) = q.pop_front() {
        let valid_next: Vec<_> = grid
            .adjacent(current)
            .into_iter()
            .filter(|next| grid[*next] == grid[current] + 1 && !seen.contains(next))
            .collect();
        for next in valid_next {
            q.push_back(next);
            seen.insert(next);
        }
    }
    seen.into_iter()
        .filter(|point| grid[*point] == b'9')
        .count()
}

pub fn part2(grid: &Input) -> usize {
    let mut counts = grid.zero_grid();

    for point in grid.points() {
        if grid[point] == b'9' {
            counts[point] = 1;
        }
    }

    for i in (0..=8).rev() {
        for point in grid.points() {
            if grid[point] != i + b'0' {
                continue;
            }
            counts[point] += grid
                .adjacent(point)
                .iter()
                .filter(|&&point| grid[point] == i + 1 + b'0')
                .map(|&point| counts[point])
                .sum::<usize>();
        }
    }

    counts
        .points()
        .iter()
        .filter(|&&point| grid[point] == b'0')
        .map(|&point| counts[point])
        .sum()
}
