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
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

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

    pub fn merge(e1: Edge, e2: Edge) -> Option<Self> {
        if e1.dir() != e2.dir() && e1.dir() != e2.dir() * -1 {
            return None;
        }
        if e1.start == e2.start {
            Some(Self::new(e1.end, e2.end))
        } else if e1.start == e2.end {
            Some(Self::new(e1.end, e2.start))
        } else if e1.end == e2.start {
            Some(Self::new(e1.start, e2.end))
        } else if e1.end == e2.end {
            Some(Self::new(e1.start, e2.start))
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
            start: Point::new(0, 0),
            end: Point::new(1, 0),
        };
        assert_eq!(edge.dir(), RIGHT);
    }

    #[test]
    fn test_dir_left() {
        let edge = Edge {
            start: Point::new(1, 0),
            end: Point::new(0, 0),
        };
        assert_eq!(edge.dir(), LEFT);
    }

    #[test]
    fn test_dir_up() {
        let edge = Edge {
            start: Point::new(0, 1),
            end: Point::new(0, 0),
        };
        assert_eq!(edge.dir(), UP);
    }

    #[test]
    fn test_dir_down() {
        let edge = Edge {
            start: Point::new(0, 0),
            end: Point::new(0, 1),
        };
        assert_eq!(edge.dir(), DOWN);
    }
}
