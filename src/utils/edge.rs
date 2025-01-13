use std::hash::Hash;

use super::point::{
    Point,
    DOWN,
    DOWN_RIGHT,
    LEFT,
    RIGHT,
    UP,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
        Self { p1, p2 }
    }

    pub fn from_point_dir(point: Point, dir: Point) -> Self {
        let (start, end) = match dir {
            UP => (point, point + RIGHT),
            LEFT => (point, point + DOWN),
            DOWN => (point + DOWN, point + DOWN_RIGHT),
            RIGHT => (point + RIGHT, point + DOWN_RIGHT),
            _ => panic!("Invalid dir: {dir:?}"),
        };
        Edge { p1: start, p2: end }
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        let dir = p2 - p1;
        Self::from_point_dir(p1, dir)
    }

    pub fn dir(&self) -> Point {
        (self.p2 - self.p1).normalized()
    }

    pub fn merge(e1: Edge, e2: Edge) -> Option<Self> {
        if e1.dir() != e2.dir() && e1.dir() != e2.dir() * -1 {
            return None;
        }
        if e1.p1 == e2.p1 {
            Some(Self::new(e1.p2, e2.p2))
        } else if e1.p1 == e2.p2 {
            Some(Self::new(e1.p2, e2.p1))
        } else if e1.p2 == e2.p1 {
            Some(Self::new(e1.p1, e2.p2))
        } else if e1.p2 == e2.p2 {
            Some(Self::new(e1.p1, e2.p1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_right() {
        let edge = Edge {
            p1: Point::new(0, 0),
            p2: Point::new(1, 0),
        };
        assert_eq!(edge.dir(), RIGHT);
    }

    #[test]
    fn test_dir_left() {
        let edge = Edge {
            p1: Point::new(1, 0),
            p2: Point::new(0, 0),
        };
        assert_eq!(edge.dir(), LEFT);
    }

    #[test]
    fn test_dir_up() {
        let edge = Edge {
            p1: Point::new(0, 1),
            p2: Point::new(0, 0),
        };
        assert_eq!(edge.dir(), UP);
    }

    #[test]
    fn test_dir_down() {
        let edge = Edge {
            p1: Point::new(0, 0),
            p2: Point::new(0, 1),
        };
        assert_eq!(edge.dir(), DOWN);
    }
}
