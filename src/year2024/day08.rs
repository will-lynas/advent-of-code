use gxhash::{
    HashMap,
    HashMapExt,
    HashSet,
    HashSetExt,
};

use crate::utils::{
    grid::Grid,
    point::Point,
};

type Input = (Grid<u8>, HashMap<u8, Vec<Point>>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut map = HashMap::new();
    grid.points().into_iter().for_each(|point| {
        let c = grid[point];
        if c != b'.' {
            map.entry(c).or_insert_with(Vec::new).push(point);
        }
    });
    (grid, map)
}

pub fn part1((grid, map): &Input) -> usize {
    let mut antinodes = HashSet::new();
    for points in map.values() {
        for p1 in points {
            for p2 in points {
                if p1 == p2 {
                    continue;
                }
                let diff = *p1 - *p2;
                antinodes.insert(*p1 + diff);
                antinodes.insert(*p2 - diff);
            }
        }
    }
    antinodes
        .into_iter()
        .filter(|point| grid.contains(point))
        .count()
}

pub fn part2((grid, map): &Input) -> usize {
    let mut antinodes = HashSet::new();
    for points in map.values() {
        for p1 in points {
            for p2 in points {
                if p1 == p2 {
                    continue;
                }
                let diff = *p1 - *p2;
                for i in 0.. {
                    let new = *p1 + diff * i;
                    if !grid.contains(&new) {
                        break;
                    }
                    antinodes.insert(new);
                }
                for i in 0.. {
                    let new = *p2 - diff * i;
                    if !grid.contains(&new) {
                        break;
                    }
                    antinodes.insert(new);
                }
            }
        }
    }
    antinodes.len()
}
