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
    pub p1: Point,
    pub p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}->{:?}", self.p1, self.p2)
    }
}
