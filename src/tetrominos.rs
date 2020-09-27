use super::colors::*;
use super::draw;
use super::point::*;
use piston_window::*;

pub struct Tetromino {
    blocks: Vec<Coord>,
    color: Color,
    center: Coord,
}

impl Tetromino {
    pub fn rotate(&mut self) {
        for coord in self.blocks.iter_mut() {
            coord.rotate90_at(self.center);
        }
    }

    fn points(&self, pt: Point) -> Vec<Point> {
        self.blocks
            .iter()
            .map(|&c| (c + pt.to_coord()).to_point())
            .collect()
    }

    pub fn draw(&self, at: Point, c: &Context, g: &mut G2d) {
        for pt in self.points(at).iter() {
            draw::draw_block(self.color, pt.x, pt.y, c, g)
        }
    }
}

pub fn t() -> Tetromino {
    Tetromino {
        color: BLUE,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(-1.0, 0.0),
            coord(0.0, 0.0),
            coord(1.0, 0.0),
            coord(0.0, 1.0),
        ],
    }
}

pub fn o() -> Tetromino {
    Tetromino {
        color: GREEN,
        center: coord(0.5, 0.5),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(1.0, 0.0),
            coord(1.0, 1.0),
        ],
    }
}

pub fn l() -> Tetromino {
    Tetromino {
        color: RED,
        center: coord(0.0, 1.0),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(0.0, 2.0),
            coord(1.0, 2.0),
        ],
    }
}

pub fn j() -> Tetromino {
    Tetromino {
        color: PURPLE,
        center: coord(1.0, 1.0),
        blocks: vec![
            coord(1.0, 0.0),
            coord(1.0, 1.0),
            coord(1.0, 2.0),
            coord(0.0, 2.0),
        ],
    }
}

pub fn i() -> Tetromino {
    Tetromino {
        color: YELLOW,
        center: coord(0.0, 2.0),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(0.0, 2.0),
            coord(0.0, 3.0),
        ],
    }
}

pub fn s() -> Tetromino {
    Tetromino {
        color: CYAN,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(-1.0, 0.0),
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(1.0, 1.0),
        ],
    }
}

pub fn z() -> Tetromino {
    Tetromino {
        color: ORANGE,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(1.0, 0.0),
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(-1.0, 1.0),
        ],
    }
}
