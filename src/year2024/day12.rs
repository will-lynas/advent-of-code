use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    grid::Grid,
    point::Point,
};

type Input = Vec<(usize, HashSet<(Point, Point)>)>;

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut out = Vec::new();
    let mut visited = HashSet::new();
    for point in grid.points() {
        let mut edges = HashSet::new();
        let mut area = 0;
        let plant = grid[point];
        dfs(&grid, point, plant, &mut visited, &mut edges, &mut area);
        out.push((area, edges));
    }
    out
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

pub fn part1(input: &Input) -> usize {
    input.iter().map(|(area, edges)| area * edges.len()).sum()
}

pub fn part2(_grid: &Input) -> usize {
    0
}
