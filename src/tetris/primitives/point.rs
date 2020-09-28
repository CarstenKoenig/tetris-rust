use std::ops;

use super::coord::Coord;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn to_coord(self) -> Coord {
        Coord {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}

pub fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}
