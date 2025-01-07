use std::ops::{Index, IndexMut};

use super::point::Point;

#[derive(Clone)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub body: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let bytes: Vec<_> = input.trim().lines().map(str::as_bytes).collect();
        let height = bytes.len() as i32;
        let width = bytes[0].len() as i32;
        let mut body = Vec::with_capacity((width * height) as usize);
        bytes.iter().for_each(|slice| body.extend_from_slice(slice));
        Grid {
            width,
            height,
            body,
        }
    }
}

impl<T> Grid<T> {
    pub fn contains(&self, point: Point) -> bool {
        point.y >= 0 && point.y < self.height && point.x >= 0 && point.x < self.width
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    pub fn find(&self, goal: &T) -> Option<Point> {
        self.body.iter().position(|b| b == goal).map(|index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        })
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.body[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.body[(self.width * point.y + point.x) as usize]
    }
}
