use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    pub fn to_point(self) -> Point {
        Point {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
        }
    }
    pub fn rotate90(self) -> Coord {
        Coord {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn rotate90_at(&mut self, at: Coord) {
        *self -= at;
        *self = self.rotate90();
        *self += at;
    }
}

pub fn coord(x: f64, y: f64) -> Coord {
    Coord { x, y }
}

impl ops::Neg for Coord {
    type Output = Coord;
    fn neg(self) -> Coord {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, _rhs: Coord) -> Coord {
        Coord {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}

impl ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::SubAssign for Coord {
    fn sub_assign(&mut self, _rhs: Self) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
    }
}

impl ops::Mul<Coord> for f64 {
    type Output = Coord;
    fn mul(self, _rhs: Coord) -> Coord {
        Coord {
            x: self * _rhs.x,
            y: self * _rhs.y,
        }
    }
}

impl ops::MulAssign<f64> for Coord {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

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

pub fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}
