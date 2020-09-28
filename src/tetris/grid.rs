use super::graphics;
use super::tetrominos::falling::FallingTetromino;
use piston_window::types::Color;
use piston_window::{Context, G2d};

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: Vec<Vec<Value>>,
}

impl Grid {
    pub fn draw(&self, cfg: &graphics::Graphics, c: &Context, g: &mut G2d) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cell.draw(cfg, x as i32, y as i32, c, g)
            }
        }
    }

    pub fn add_tetromino(&mut self, t: &FallingTetromino) {
        for pt in t.tetromino.points(t.coord) {
            if pt.y < 0 || pt.y >= self.rows || pt.x < 0 || pt.x >= self.cols {
                continue;
            }
            self.cells[pt.y as usize][pt.x as usize] = Value::Block {
                color: t.tetromino.color,
            }
        }
    }
}

pub fn create_empty(rows: i32, cols: i32) -> Grid {
    let mut cells = Vec::new();
    for _ in 0..rows {
        let mut grid_row = Vec::new();
        for _ in 0..cols {
            grid_row.push(Value::Empty);
        }
        cells.push(grid_row);
    }
    Grid { rows, cols, cells }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Empty,
    Block { color: Color },
}

impl Value {
    fn draw(&self, cfg: &graphics::Graphics, x: i32, y: i32, c: &Context, g: &mut G2d) {
        match self {
            Value::Empty => cfg.draw_block([0.1, 0.1, 0.1, 1.0], x, y, &c, g),
            Value::Block { color } => cfg.draw_block(*color, x, y, &c, g),
        }
    }
}
