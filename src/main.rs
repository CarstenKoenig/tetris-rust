extern crate piston_window;
extern crate rand;

use piston_window::Event;
use piston_window::*;

// connect to draw.rs
mod colors;
mod draw;
mod point;
mod tetrominos;

fn main() {
    let mut next_rotate: f64 = 1.0;
    let mut test = tetrominos::j();
    let mut window: PistonWindow = WindowSettings::new("rusty TETRIS", [800, 1024])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        if let Event::Loop(Loop::Update(a)) = e {
            next_rotate -= a.dt;
            while next_rotate <= 0.0 {
                test.rotate();
                next_rotate += 1.0;
            }
        }
        if let Event::Loop(Loop::Render(_)) = e {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                draw_board(&c, g);

                test.draw(2, 4, &c, g);
            });
        }
    }
}

fn draw_board(c: &Context, g: &mut G2d) {
    for y in 0..20 {
        for x in 0..10 {
            draw::draw_block([0.1, 0.1, 0.1, 1.0], x, y, &c, g);
        }
    }
}
