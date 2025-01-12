use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    grid::Grid,
    point::Point,
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn dfs(
    grid: &Grid<u8>,
    point: Point,
    plant: u8,
    visited: &mut HashSet<Point>,
    edges: &mut HashSet<(Point, Point)>,
    area: &mut usize,
) {
    if visited.contains(&point) {
        return;
    }
    visited.insert(point);
    *area += 1;
    for adjacent in point.adjacent() {
        if grid.contains(adjacent) && grid[adjacent] == plant {
            dfs(grid, adjacent, plant, visited, edges, area);
        } else {
            edges.insert((point, adjacent));
        }
    }
}

pub fn part1(grid: &Grid<u8>) -> usize {
    let mut visited = HashSet::new();
    let mut total = 0;

    for point in grid.points() {
        let mut edges = HashSet::new();
        let mut area = 0;
        let plant = grid[point];
        dfs(grid, point, plant, &mut visited, &mut edges, &mut area);
        total += edges.len() * area;
    }
    total
}

pub fn part2(_grid: &Grid<u8>) -> usize {
    0
}
