#![warn(clippy::all)]
extern crate piston_window;
extern crate rand;
extern crate find_folder;

use piston_window::*;

// connect to draw.rs
mod tetris;

use tetris::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("rusty TETRIS", [800, 1024])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();


    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraCode-Bold.ttf")).unwrap();

    let mut game = game::Game::new(20, 10);

    while let Some(e) = window.next() {
        // Update-Loop
        if let Some(UpdateArgs { dt }) = e.update_args() {
            game.update_time(dt);
        }

        // Render-Loop
        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, device| {
                if game.game_over {
                    clear([0.3, 0.1, 0.1, 1.0], g);
                } else {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                }
                game.draw(&c, g);

                // Draw Score top-left
                let transform = c.transform.trans(10.0, 30.0);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &game.score.to_string(),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g
                ).unwrap();

                // draw game-over if neccessary
                if game.game_over {
                    let transform = c.transform.trans(10.0, 80.0);
                    text::Text::new_color([0.0, 0.0, 1.0, 1.0], 32).draw(
                        "GAME OVER!",
                        &mut glyphs,
                        &c.draw_state,
                        transform,
                        g
                    ).unwrap();
                }


                // Update glyphs before rendering
                glyphs.factory.encoder.flush(device);
            });
        }

        // Input-Loop
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Left) => game.move_left(),
                    Button::Keyboard(Key::Right) => game.move_right(),
                    Button::Keyboard(Key::Up) => game.rotate(),
                    Button::Keyboard(Key::Down) => { game.drop(); () },
                    _ => (),
                }
            }
        }
    }
}
