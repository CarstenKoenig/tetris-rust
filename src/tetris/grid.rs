use super::graphics;
use super::primitives::point::Point;
use super::tetrominos::falling::FallingTetromino;
use piston_window::types::Color;
use piston_window::{Context, G2d};

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: Vec<Vec<Value>>,
}

impl Grid {
    pub fn new(rows: i32, cols: i32) -> Grid {
        let mut cells = Vec::new();
        for _ in 0..rows {
            let grid_row = new_row(cols);
            cells.push(grid_row);
        }
        Grid { rows, cols, cells }
    }

    pub fn draw(&self, cfg: &graphics::Graphics, c: &Context, g: &mut G2d) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cell.draw(cfg, x as i32, y as i32, c, g)
            }
        }
    }

    pub fn add_tetromino(&mut self, ft: &FallingTetromino) {
        for pt in ft.points() {
            if !self.in_bounds(pt, true) {
                continue;
            }
            self.cells[pt.y as usize][pt.x as usize] = Value::Block {
                color: ft.tetromino.color,
            }
        }
    }

    pub fn is_valid_tetromino(&self, ft: &FallingTetromino) -> bool {
        for pt in ft.points() {
            if !self.in_bounds(pt, false) || self.is_overlapping(pt) {
                return false;
            }
        };
        true
    }

    pub fn remove_full_rows(&mut self) {
        for row in (0..self.rows).rev() {
            if self.is_row_full(row as usize) {
                self.remove_row(row as usize);
            }
        }
        while self.cells.len() < self.rows as usize {
            self.cells.insert(0, new_row(self.cols))
        }
    }

    fn in_bounds(&self, pt: Point, check_top: bool) -> bool {
        (!check_top || pt.y >= 0) && pt.y < self.rows &&
        pt.x >= 0 && pt.x < self.cols
    }

    fn is_overlapping(&self, pt: Point) -> bool {
        // should only be called with a point inside bounds
        // BUT: this can be called in in_bounds with an invalide pt.y so
        // guard against that
        pt.y >= 0 && self.cells[pt.y as usize][pt.x as usize].is_block()
    }

    fn remove_row(&mut self, row: usize) {
        self.cells.remove(row);
    }

    fn is_row_full(&self, row: usize) -> bool {
        for col in 0..self.cols {
            if !self.cells[row][col as usize].is_block() {
                return false;
            }
        }
        true
    }
}

fn new_row(cols: i32) -> Vec<Value> {
    let mut grid_row = Vec::new();
    for _ in 0..cols {
        grid_row.push(Value::Empty);
    }
    grid_row
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Empty,
    Block { color: Color },
}

impl Value {
    pub fn is_block(self) -> bool {
        match self {
            Value::Empty => false,
            Value::Block { color: _ } => true
        }
    }

    fn draw(&self, cfg: &graphics::Graphics, x: i32, y: i32, c: &Context, g: &mut G2d) {
        match self {
            Value::Empty => cfg.draw_block([0.1, 0.1, 0.1, 1.0], x, y, &c, g),
            Value::Block { color } => cfg.draw_block(*color, x, y, &c, g),
        }
    }
}
