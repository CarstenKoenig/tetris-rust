use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 1024.0;

const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 20;

const BLOCK_SIZE: f64 = WINDOW_HEIGHT / (GRID_HEIGHT as f64);

fn darker(color: Color) -> Color {
    let mut darker_color = color.clone();
    darker_color[0] = 0.7 * color[0];
    darker_color[1] = 0.7 * color[1];
    darker_color[2] = 0.7 * color[2];
    darker_color
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let screen_x = ((2 * x - GRID_WIDTH) as f64 * BLOCK_SIZE + WINDOW_WIDTH) / 2.0;
    let screen_y = (y as f64) * BLOCK_SIZE;

    rectangle(
        darker(color),
        [
            screen_x + 1.0,
            screen_y + 1.0,
            BLOCK_SIZE - 2.0,
            BLOCK_SIZE - 2.0,
        ],
        con.transform,
        g,
    );

    rectangle(
        color,
        [
            screen_x + 3.0,
            screen_y + 3.0,
            BLOCK_SIZE - 6.0,
            BLOCK_SIZE - 6.0,
        ],
        con.transform,
        g,
    );
}
