#![warn(clippy::all)]
extern crate piston_window;
extern crate rand;

use piston_window::*;

// connect to draw.rs
mod tetris;

use tetris::*;

fn main() {
    let mut game = game::Game::new(20, 10);

    let mut window: PistonWindow = WindowSettings::new("rusty TETRIS", [800, 1024])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // Update-Loop
        if let Some(UpdateArgs { dt }) = e.update_args() {
            game.update_time(dt);
        }

        // Render-Loop
        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                game.draw(&c, g);
            });
        }

        // Input-Loop
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Left) => game.move_left(),
                    Button::Keyboard(Key::Right) => game.move_right(),
                    Button::Keyboard(Key::Up) => game.rotate(),
                    Button::Keyboard(Key::Down) => game.drop(),
                    _ => (),
                }
            }
        }
    }
}
