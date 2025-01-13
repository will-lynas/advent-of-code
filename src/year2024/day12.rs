use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::{
    edge::Edge,
    grid::Grid,
    point::Point,
};

type Input = Vec<(usize, HashSet<Edge>)>;

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
        out.push((area, edges));
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
            edges.insert(Edge::from_points(point, adjacent));
        }
    }
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|(area, edges)| area * edges.len()).sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|(area, edges)| {
            let mut edges: Vec<_> = edges.iter().copied().collect();
            let mut len = 0;
            while let Some(first) = edges.pop() {
                let mut new_edges = vec![];
                let mut current = first;
                while let Some(pos) = edges.iter().position(|&edge| edge.connected(current)) {
                    let next = edges.remove(pos);
                    if let Some(merged) = Edge::merge(current, next) {
                        current = merged;
                    } else {
                        new_edges.push(current);
                        current = next;
                    }
                }
                len += new_edges.len();
                if Edge::merge(current, first).is_none() {
                    len += 1;
                }
            }
            area * len
        })
        .sum()
}
