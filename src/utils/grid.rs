use std::ops::{Index, IndexMut};

use super::point::Point;

#[derive(Clone)]
pub struct Grid<T> {
    pub body: Vec<Vec<T>>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        Grid {
            body: input
                .trim()
                .lines()
                .map(|line| line.bytes().collect())
                .collect(),
        }
    }
}

impl<T> Grid<T> {
    pub fn contains(&self, point: Point) -> bool {
        point.y >= 0
            && point.y < self.body.len() as i32
            && point.x >= 0
            && point.x < self.body[0].len() as i32
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, goal: &T) -> Option<Point> {
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

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.body[point.y as usize][point.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.body[point.y as usize][point.x as usize]
    }
}
