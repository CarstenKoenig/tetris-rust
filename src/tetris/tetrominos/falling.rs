use super::super::primitives::point::{point, Point};
use super::*;

pub struct FallingTetromino {
    pub coord: Point,
    pub tetromino: Tetromino,
}

impl FallingTetromino {
    pub fn rotate(&mut self) {
        self.tetromino.rotate();
    }

    pub fn drop(&mut self) {
        self.coord += point(0, 1)
    }

    pub fn move_left(&mut self) {
        self.coord += point(-1, 0)
    }

    pub fn move_right(&mut self) {
        self.coord += point(1, 0)
    }

    pub fn draw(&self, cfg: &graphics::Graphics, c: &Context, g: &mut G2d) {
        self.tetromino.draw(cfg, self.coord, &c, g);
    }
}

pub fn create(pt: Point, t: Tetromino) -> FallingTetromino {
    FallingTetromino {
        coord: pt,
        tetromino: t,
    }
}

pub fn create_rnd() -> FallingTetromino {
    let mut rng = thread_rng();
    match rng.gen_range(0, 7) {
        0 => create(point(4, -3), create_l()),
        1 => create(point(4, -2), create_t()),
        2 => create(point(4, -2), create_j()),
        3 => create(point(4, -4), create_i()),
        4 => create(point(4, -2), create_s()),
        5 => create(point(4, -2), create_z()),
        _ => create(point(4, -2), create_o()),
    }
}
