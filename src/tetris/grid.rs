use super::graphics;
use piston_window::types::Color;
use piston_window::{Context, G2d};

pub struct Grid {
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
}

pub fn create_empty(rows: usize, cols: usize) -> Grid {
    let mut cells = Vec::new();
    for _ in 0..rows {
        let mut grid_row = Vec::new();
        for _ in 0..cols {
            grid_row.push(Value::Empty);
        }
        cells.push(grid_row);
    }
    Grid { cells }
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
