use std::ops::Add;

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

pub const DIRS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn rotate_clockwise(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }
}

impl Add for Point {
    type Output = Point;

    #[must_use]
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
