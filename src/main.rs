#![warn(clippy::all)]
extern crate piston_window;
extern crate rand;

use piston_window::*;

// connect to draw.rs
mod tetris;

use tetris::*;

fn main() {
    let cfg = graphics::Graphics { rows: 20, cols: 10 };
    let grid = crate::tetris::grid::create_empty(20, 10);
    let mut next_drop: f64 = 1.0;
    let mut test = tetrominos::falling::create_rnd();
    let mut window: PistonWindow = WindowSettings::new("rusty TETRIS", [800, 1024])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // Update-Loop
        if let Some(UpdateArgs { dt }) = e.update_args() {
            next_drop -= dt;
            while next_drop <= 0.0 {
                test.drop();
                next_drop += 1.0;
            }
        }

        // Render-Loop
        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                grid.draw(&cfg, &c, g);
                test.draw(&cfg, &c, g);
            });
        }

        // Input-Loop
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Left) => test.move_left(),
                    Button::Keyboard(Key::Right) => test.move_right(),
                    Button::Keyboard(Key::Up) => test.rotate(),
                    Button::Keyboard(Key::Down) => test.drop(),
                    _ => (),
                }
            }
        }
    }
}
