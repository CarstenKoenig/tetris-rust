use crate::tetrominos::*;

pub struct FallingTetromino {
    coord: Point,
    tetromino: Tetromino,
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

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        self.tetromino.draw(self.coord, &c, g);
    }
}

pub fn create(pt: Point, t: Tetromino) -> FallingTetromino {
    FallingTetromino {
        coord: pt,
        tetromino: t,
    }
}
