use std::{
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use super::point::Point;

#[derive(PartialOrd, Ord, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Option<Self> {
        ((p1 - p2).norm1() == 1).then(|| {
            let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
            Self { p1, p2 }
        })
    }

    pub fn dir(&self) -> Point {
        self.p1 - self.p2
    }

    pub fn connected(&self, other: Self) -> bool {
        if self.dir() == other.dir() {
            let diff = self.p1 - other.p1;
            self.dir().orthogonal(diff) && diff.norm1() == 1
        } else {
            self.p1 == other.p1 || self.p1 == other.p2 || self.p2 == other.p1 || self.p2 == other.p2
        }
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.p1, self.p2)
    }
}
