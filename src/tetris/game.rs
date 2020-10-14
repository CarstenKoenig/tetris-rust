use piston_window::{Context, G2d};
use super::graphics::{Graphics};
use super::grid::{Grid};
use super::tetrominos::falling::{FallingTetromino};

pub struct Game {
    cfg: Graphics,
    grid: Grid,
    falling: FallingTetromino,
    next_drop: f64
}

impl Game {
    pub fn new(rows: i32, cols: i32) -> Game {
        let grid = Grid::new(rows, cols);
        let cfg = Graphics { rows, cols };
        let falling = super::tetrominos::falling::create_rnd();
        Game {
            cfg: cfg,
            grid: grid,
            falling: falling,
            next_drop: 1.0
        }
    }

    pub fn update_time(&mut self, dt: f64) {
        self.next_drop -= dt;
        while self.next_drop <= 0.0 {
            self.drop();
            self.next_drop += 1.0;
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        self.grid.draw(&self.cfg, c, g);
        self.falling.draw(&self.cfg, c, g);
    }

    pub fn rotate(&mut self) {
        let rotated = self.falling.rotate();
        if self.grid.is_valid_tetromino(&rotated) {
            self.falling = rotated;
        }
    }

    pub fn drop(&mut self) {
        let dropped = self.falling.drop();
        if !self.grid.is_valid_tetromino(&dropped) {
            self.grid.add_tetromino(&self.falling);
            self.falling = super::tetrominos::falling::create_rnd();
            self.grid.remove_full_rows();
        } else {
            self.falling = dropped
        }
    }

    pub fn move_left(&mut self) {
        let moved = self.falling.move_left();
        if self.grid.is_valid_tetromino(&moved) {
            self.falling = moved;
        }
    }

    pub fn move_right(&mut self) {
        let moved = self.falling.move_right();
        if self.grid.is_valid_tetromino(&moved) {
            self.falling = moved;
        }
    }
}