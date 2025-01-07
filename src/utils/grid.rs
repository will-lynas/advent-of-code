use std::ops::{Index, IndexMut};

use super::point::Point;

#[derive(Clone)]
pub struct Grid {
    pub body: Vec<Vec<char>>,
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        Grid {
            body: input
                .trim()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.y >= 0
            && point.y < self.body.len() as i32
            && point.x >= 0
            && point.x < self.body[0].len() as i32
    }

    pub fn find(&self, goal: &char) -> Option<Point> {
        for (i, line) in self.body.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if c == goal {
                    return Some(Point::new(j as i32, i as i32));
                }
            }
        }
        None
    }
}

impl Index<Point> for Grid {
    type Output = char;

    fn index(&self, point: Point) -> &Self::Output {
        &self.body[point.y as usize][point.x as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.body[point.y as usize][point.x as usize]
    }
}
