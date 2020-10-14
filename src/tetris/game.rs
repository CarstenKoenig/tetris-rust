use piston_window::{Context, G2d};
use super::graphics::{Graphics};
use super::grid::{Grid};
use super::tetrominos::falling::{FallingTetromino};

const INITIAL_SPEED : f64 = 0.8;
const SPEEDUP_EVERY_TETROMINOS : i32 = 5;
const SPEEDUP_FACTOR : f64 = 0.8;

pub struct Game {
    cfg: Graphics,
    pub score: i32,
    pub game_over: bool,
    grid: Grid,
    falling: FallingTetromino,
    speed: f64,
    next_drop: f64,
    next_speed: i32
}

impl Game {
    pub fn new(rows: i32, cols: i32) -> Game {
        let grid = Grid::new(rows, cols);
        let cfg = Graphics { rows, cols };
        let falling = super::tetrominos::falling::create_rnd();
        Game {
            cfg: cfg,
            score: 0,
            game_over: false,
            grid: grid,
            falling: falling,
            speed: INITIAL_SPEED,
            next_drop: INITIAL_SPEED,
            next_speed: SPEEDUP_EVERY_TETROMINOS
        }
    }

    pub fn update_time(&mut self, dt: f64) {
        if self.game_over {
            return ();
        }
        self.next_drop -= dt;
        while self.next_drop <= 0.0 {
            let new_tetromino = self.drop();
            self.next_drop += self.speed;

            if new_tetromino {
                self.next_speed -= 1;
                while self.next_speed <= 0 {
                    self.speed *= SPEEDUP_FACTOR;
                    self.next_speed += SPEEDUP_EVERY_TETROMINOS;
                }
            }
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        self.grid.draw(&self.cfg, c, g);
        self.falling.draw(&self.cfg, c, g);
    }

    pub fn rotate(&mut self) {
        if self.game_over {
            return ();
        }
        let rotated = self.falling.rotate();
        if self.grid.is_valid_tetromino(&rotated) {
            self.falling = rotated;
        } else {
            // try wall-kick-left
            let kick_left = self.falling.move_left().rotate();
            if self.grid.is_valid_tetromino(&kick_left) {
                self.falling = kick_left;
            } else {
                // try wall-kick-right
                let kick_right = self.falling.move_right().rotate();
                if self.grid.is_valid_tetromino(&kick_right) {
                    self.falling = kick_right;
                }
            }
        }
    }

    pub fn drop(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        let dropped = self.falling.drop();
        if !self.grid.is_valid_tetromino(&dropped) {
            self.grid.add_tetromino(&self.falling);

            self.falling = super::tetrominos::falling::create_rnd();

            if !self.grid.is_valid_tetromino(&self.falling) {
                self.game_over = true;
                return true;
            }

            let removed = self.grid.remove_full_rows();
            self.update_score(removed);
            return true;
        } else {
            self.falling = dropped;
            return false;
        }
    }

    pub fn move_left(&mut self) {
        if self.game_over {
            return ();
        }
        let moved = self.falling.move_left();
        if self.grid.is_valid_tetromino(&moved) {
            self.falling = moved;
        }
    }

    pub fn move_right(&mut self) {
        if self.game_over {
            return ();
        }
        let moved = self.falling.move_right();
        if self.grid.is_valid_tetromino(&moved) {
            self.falling = moved;
        }
    }

    fn update_score(&mut self, removed_rows: u32) {
        if removed_rows > 0 {
            self.score += 4i32.pow(removed_rows);
        }
    }
}