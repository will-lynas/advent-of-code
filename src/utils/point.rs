use std::{
    fmt::{
        self,
        Debug,
        Formatter,
    },
    ops::{
        Add,
        AddAssign,
        Mul,
        Sub,
    },
};

use num::integer::gcd;

pub const ORIGIN: Point = Point::new(0, 0);

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

pub const UP_LEFT: Point = Point::new(-1, -1);
pub const UP_RIGHT: Point = Point::new(1, -1);
pub const DOWN_RIGHT: Point = Point::new(1, 1);
pub const DOWN_LEFT: Point = Point::new(-1, 1);

pub const ORTHOGONALS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];
pub const DIRS: [Point; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

#[derive(PartialOrd, Ord, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub y: i32,
    pub x: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { y, x }
    }

    pub fn rotate_clockwise(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }

    pub fn orthogonals(&self) -> Vec<Self> {
        ORTHOGONALS.iter().map(move |&dir| *self + dir).collect()
    }

    #[must_use]
    pub fn normalized(&self) -> Self {
        if self == &ORIGIN {
            return ORIGIN;
        }
        let n = gcd(self.x, self.y);
        Self {
            x: self.x / n,
            y: self.y / n,
        }
    }

    pub fn norm1(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn dot(&self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn orthogonal(&self, other: Self) -> bool {
        self.dot(other) == 0
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, n: i32) -> Self {
        Self::new(self.x * n, self.y * n)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, n: usize) -> Self {
        Self::new(self.x * n as i32, self.y * n as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_origin() {
        let origin = Point::new(0, 0);
        assert_eq!(origin.normalized(), origin);
    }

    #[test]
    fn test_normalized_positive() {
        let point = Point::new(6, 8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(3, 4));
    }

    #[test]
    fn test_normalized_negative() {
        let point = Point::new(-6, -8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(-3, -4));
    }

    #[test]
    fn test_normalized_mixed_signs() {
        let point = Point::new(6, -8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(3, -4));
    }

    #[test]
    fn test_normalized_already_normalized() {
        let point = Point::new(3, 4);
        assert_eq!(point.normalized(), point);
    }
}
