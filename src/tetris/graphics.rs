use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 1024.0;

fn darker(color: Color) -> Color {
    let mut darker_color = color;
    darker_color[0] = 0.7 * color[0];
    darker_color[1] = 0.7 * color[1];
    darker_color[2] = 0.7 * color[2];
    darker_color
}

pub struct Graphics {
    pub rows: i32,
    pub cols: i32,
}
impl Graphics {
    fn block_size(&self) -> f64 {
        WINDOW_HEIGHT / self.rows as f64
    }

    pub fn draw_block(&self, color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
        let block_sz = self.block_size();
        let screen_x = ((2 * x - self.cols) as f64 * block_sz + WINDOW_WIDTH) / 2.0;
        let screen_y = (y as f64) * block_sz;

        rectangle(
            darker(color),
            [
                screen_x + 1.0,
                screen_y + 1.0,
                block_sz - 2.0,
                block_sz - 2.0,
            ],
            con.transform,
            g,
        );

        rectangle(
            color,
            [
                screen_x + 3.0,
                screen_y + 3.0,
                block_sz - 6.0,
                block_sz - 6.0,
            ],
            con.transform,
            g,
        );
    }
}
