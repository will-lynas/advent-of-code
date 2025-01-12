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

    #[test]
    fn test_try_join_success_end_to_start() {
        let mut edge1 = Edge {
            start: Point::new(0, 0),
            end: Point::new(1, 0),
        };
        let edge2 = Edge {
            start: Point::new(1, 0),
            end: Point::new(2, 0),
        };
        assert!(edge1.try_join(&edge2));
        assert_eq!(
            edge1,
            Edge {
                start: Point::new(0, 0),
                end: Point::new(2, 0),
            }
        );
    }

    #[test]
    fn test_try_join_success_start_to_end() {
        let mut edge1 = Edge {
            start: Point::new(1, 0),
            end: Point::new(2, 0),
        };
        let edge2 = Edge {
            start: Point::new(0, 0),
            end: Point::new(1, 0),
        };
        assert!(edge1.try_join(&edge2));
        assert_eq!(
            edge1,
            Edge {
                start: Point::new(0, 0),
                end: Point::new(2, 0),
            }
        );
    }

    #[test]
    fn test_try_join_fail_different_directions() {
        let mut edge1 = Edge {
            start: Point::new(0, 0),
            end: Point::new(1, 0),
        };
        let edge2 = Edge {
            start: Point::new(1, 0),
            end: Point::new(1, 1),
        };
        assert!(!edge1.try_join(&edge2));
        assert_eq!(
            edge1,
            Edge {
                start: Point::new(0, 0),
                end: Point::new(1, 0),
            }
        );
    }

    #[test]
    fn test_try_join_fail_not_connected() {
        let mut edge1 = Edge {
            start: Point::new(0, 0),
            end: Point::new(1, 0),
        };
        let edge2 = Edge {
            start: Point::new(2, 0),
            end: Point::new(3, 0),
        };
        assert!(!edge1.try_join(&edge2));
        assert_eq!(
            edge1,
            Edge {
                start: Point::new(0, 0),
                end: Point::new(1, 0),
            }
        );
    }
}
