extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_f64;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30.0, 30.0);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_f64(width), to_coord_f64(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width as i32, height as i32);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
    println!("Hello");
}
