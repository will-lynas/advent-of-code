use super::point::{
    Point,
    DOWN,
    DOWN_RIGHT,
    LEFT,
    RIGHT,
    UP,
};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
}

impl Edge {
    pub fn from_point_dir(point: Point, dir: Point) -> Self {
        let (start, end) = match dir {
            UP => (point, point + RIGHT),
            LEFT => (point, point + DOWN),
            DOWN => (point + DOWN, point + DOWN_RIGHT),
            RIGHT => (point + RIGHT, point + DOWN_RIGHT),
            _ => panic!("Invalid dir: {dir:?}"),
        };
        Edge { start, end }
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        let dir = p2 - p1;
        Self::from_point_dir(p1, dir)
    }

    pub fn dir(&self) -> Point {
        (self.end - self.start).normalized()
    }

    pub fn try_join(&mut self, other: &Self) -> bool {
        if self.dir() != other.dir() {
            return false;
        }
        if self.end == other.start {
            self.end = other.end;
            true
        } else if self.start == other.end {
            self.start = other.start;
            true
        } else {
            false
        }
    }
}
