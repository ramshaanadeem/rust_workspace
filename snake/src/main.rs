extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
use crate::draw::to_coord_u32;
use crate::game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    let mut glyphs = Glyphs::from_bytes(
        include_bytes!("Roboto-Black.ttf"),
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&mut glyphs, &c, g);
            glyphs.factory.encoder.flush(device);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
