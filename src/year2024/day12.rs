use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    edge::Edge,
    grid::Grid,
    point::Point,
};

type Input = Vec<(u8, usize, HashSet<Edge>)>;

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut out = Vec::new();
    let mut visited = HashSet::new();
    for point in grid.points() {
        if visited.contains(&point) {
            continue;
        }
        let mut edges = HashSet::new();
        let mut area = 0;
        let plant = grid[point];
        dfs(&grid, point, plant, &mut visited, &mut edges, &mut area);
        out.push((plant, area, edges));
    }
    out
}

fn dfs(
    grid: &Grid<u8>,
    point: Point,
    plant: u8,
    visited: &mut HashSet<Point>,
    edges: &mut HashSet<Edge>,
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
            edges.insert(Edge::new(point, adjacent).unwrap());
        }
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(_, area, edges)| area * edges.len())
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input.iter().map(|(_plant, _area, _edges)| 0).sum()
}
