use std::{
    fmt::{
        self,
        Debug,
        Formatter,
    },
    ops::{
        Index,
        IndexMut,
    },
};

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

    pub fn print_with_points(&self, points: impl IntoIterator<Item = Point>) {
        let mut grid = self.clone();
        points.into_iter().for_each(|point| grid[point] = b'#');
        println!("{grid:?}");
    }
}

impl Debug for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x, y)] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> GridIter<T> {
        <&Self as IntoIterator>::into_iter(self)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.y >= 0 && point.y < self.height && point.x >= 0 && point.x < self.width
    }

    pub fn point(&self, index: usize) -> Point {
        let x = (index as i32) % self.width;
        let y = (index as i32) / self.width;
        Point::new(x, y)
    }

    pub fn points(&self) -> Vec<Point> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Point::new(x, y)))
            .collect()
    }

    pub fn adjacent(&self, point: Point) -> Vec<Point> {
        point
            .adjacent()
            .into_iter()
            .filter(|point| self.contains(*point))
            .collect()
    }

    pub fn zero_grid(&self) -> Grid<usize> {
        Grid {
            width: self.width,
            height: self.height,
            body: vec![0; (self.width * self.height) as usize],
        }
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    pub fn find(&self, goal: &T) -> Option<Point> {
        self.body
            .iter()
            .position(|b| b == goal)
            .map(|index| self.point(index))
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

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, index: 0 }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.grid.body.len()).then(|| {
            let point = self.grid.point(self.index);
            let value = &self.grid.body[self.index];
            self.index += 1;
            (point, value)
        })
    }
}
