use super::graphics;
use super::primitives::colors;
use super::primitives::coord::{coord, Coord};
use super::primitives::point::Point;
use piston_window::*;
use rand::{thread_rng, Rng};

pub mod falling;

#[derive(Debug, Clone)]
pub struct Tetromino {
    blocks: Vec<Coord>,
    pub color: colors::Color,
    center: Coord,
}

impl Tetromino {
    pub fn rotate(&mut self) -> Tetromino {
        let mut new_blocks = self.blocks.clone();
        for coord in new_blocks.iter_mut() {
            coord.rotate90_at(self.center);
        };
        Tetromino {
            color: self.color,
            center: self.center,
            blocks: new_blocks
        }
    }

    pub fn points(&self, pt: Point) -> Vec<Point> {
        self.blocks
            .iter()
            .map(|&c| (c + pt.to_coord()).to_point())
            .collect()
    }

    pub fn draw(&self, cfg: &graphics::Graphics, at: Point, c: &Context, g: &mut G2d) {
        for pt in self.points(at).iter() {
            cfg.draw_block(self.color, pt.x, pt.y, c, g)
        }
    }
}

pub fn create_t() -> Tetromino {
    Tetromino {
        color: colors::BLUE,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(-1.0, 0.0),
            coord(0.0, 0.0),
            coord(1.0, 0.0),
            coord(0.0, 1.0),
        ],
    }
}

pub fn create_o() -> Tetromino {
    Tetromino {
        color: colors::GREEN,
        center: coord(0.5, 0.5),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(1.0, 0.0),
            coord(1.0, 1.0),
        ],
    }
}

pub fn create_l() -> Tetromino {
    Tetromino {
        color: colors::RED,
        center: coord(0.0, 1.0),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(0.0, 2.0),
            coord(1.0, 2.0),
        ],
    }
}

pub fn create_j() -> Tetromino {
    Tetromino {
        color: colors::PURPLE,
        center: coord(1.0, 1.0),
        blocks: vec![
            coord(1.0, 0.0),
            coord(1.0, 1.0),
            coord(1.0, 2.0),
            coord(0.0, 2.0),
        ],
    }
}

pub fn create_i() -> Tetromino {
    Tetromino {
        color: colors::YELLOW,
        center: coord(0.0, 2.0),
        blocks: vec![
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(0.0, 2.0),
            coord(0.0, 3.0),
        ],
    }
}

pub fn create_s() -> Tetromino {
    Tetromino {
        color: colors::CYAN,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(-1.0, 0.0),
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(1.0, 1.0),
        ],
    }
}

pub fn create_z() -> Tetromino {
    Tetromino {
        color: colors::ORANGE,
        center: coord(0.0, 0.0),
        blocks: vec![
            coord(1.0, 0.0),
            coord(0.0, 0.0),
            coord(0.0, 1.0),
            coord(-1.0, 1.0),
        ],
    }
}
