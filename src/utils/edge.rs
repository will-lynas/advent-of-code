use std::{
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use super::point::{
    Point,
    DOWN,
    DOWN_RIGHT,
    LEFT,
    RIGHT,
    UP,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
        let (p1, p2) = match dir {
            UP => (point, point + RIGHT),
            LEFT => (point, point + DOWN),
            DOWN => (point + DOWN, point + DOWN_RIGHT),
            RIGHT => (point + RIGHT, point + DOWN_RIGHT),
            _ => panic!("Invalid dir: {dir:?}"),
        };
        Self::new(p1, p2)
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        let dir = p2 - p1;
        Self::from_point_dir(p1, dir)
    }

    pub fn dir(&self) -> Point {
        (self.p2 - self.p1).normalized()
    }

    pub fn merge(e1: Edge, e2: Edge) -> Option<Self> {
        if e1.dir() != e2.dir() {
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

    pub fn connected(&self, other: Edge) -> bool {
        self.p1 == other.p1 || self.p1 == other.p2 || self.p2 == other.p1 || self.p2 == other.p2
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.p1, self.p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_right() {
        let edge = Edge::new(Point::new(0, 0), Point::new(1, 0));
        assert_eq!(edge.dir(), RIGHT);
    }

    #[test]
    fn test_dir_left() {
        let edge = Edge::new(Point::new(1, 0), Point::new(0, 0));
        // The points get flipped
        assert_eq!(edge.dir(), RIGHT);
    }

    #[test]
    fn test_dir_down() {
        let edge = Edge::new(Point::new(0, 0), Point::new(0, 1));
        assert_eq!(edge.dir(), DOWN);
    }

    #[test]
    fn test_dir_up() {
        let edge = Edge::new(Point::new(0, 1), Point::new(0, 0));
        // The points get flipped
        assert_eq!(edge.dir(), DOWN);
    }

    #[test]
    fn test_connected_same_point() {
        let edge1 = Edge::new(Point::new(0, 0), Point::new(1, 0));
        let edge2 = Edge::new(Point::new(1, 0), Point::new(2, 0));
        assert!(edge1.connected(edge2));
    }

    #[test]
    fn test_connected_no_connection() {
        let edge1 = Edge::new(Point::new(0, 0), Point::new(1, 0));
        let edge2 = Edge::new(Point::new(2, 0), Point::new(3, 0));
        assert!(!edge1.connected(edge2));
    }

    #[test]
    fn test_connected_reversed() {
        let edge1 = Edge::new(Point::new(0, 0), Point::new(1, 0));
        let edge2 = Edge::new(Point::new(0, 0), Point::new(-1, 0));
        assert!(edge1.connected(edge2));
    }
}
