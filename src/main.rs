extern crate piston_window;
extern crate rand;

use piston_window::*;

// connect to draw.rs
mod colors;
mod draw;
mod point;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("rusty TETRIS", [800, 1024])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            draw_board(&c, g);

            draw::draw_block(colors::RED, 0, 19, &c, g);
            draw::draw_block(colors::GREEN, 1, 19, &c, g);
            draw::draw_block(colors::BLUE, 2, 19, &c, g);
            draw::draw_block(colors::CYAN, 3, 19, &c, g);
            draw::draw_block(colors::YELLOW, 4, 19, &c, g);
            draw::draw_block(colors::ORANGE, 5, 19, &c, g);
            draw::draw_block(colors::PURPLE, 6, 19, &c, g);
            draw::draw_block(colors::RED, 7, 19, &c, g);
            draw::draw_block(colors::GREEN, 8, 19, &c, g);
            draw::draw_block(colors::BLUE, 9, 19, &c, g);
            draw::draw_block(colors::YELLOW, 0, 0, &c, g);
            draw::draw_block(colors::ORANGE, 9, 0, &c, g);
        });
    }
}

fn draw_board(c: &Context, g: &mut G2d) {
    for y in 0..20 {
        for x in 0..10 {
            draw::draw_block([0.1, 0.1, 0.1, 1.0], x, y, &c, g);
        }
    }
}
