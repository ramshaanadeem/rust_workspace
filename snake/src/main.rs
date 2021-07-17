extern crate piston_window;
extern crate rand;
// use rand::{Rng};

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::{Game};
// use crate::game::Block;
// #[derive(Copy)]
// pub struct Block {
//     x:i32,
//     y:i32,
// }
const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
// const BORDER_COLOR: Color = [0.5, 0.50, 0.50, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();


    let mut game = Game::new(width, height);
    // creating obstacles
    // let no_of_obstacles = rand::thread_rng().gen_range(1..5);
    // println!("{}",no_of_obstacles);
    // let mut obstacles : Vec<game::Block> = Vec::new();
    // for _i in 0..no_of_obstacles {
    //     let x_coord = rand::thread_rng().gen_range(1..19);
    //     let y_coord = rand::thread_rng().gen_range(1..19);
        
    //     // draw_block(BORDER_COLOR, x_coord, y_coord, con, g);
    //     obstacles.push( Block {
    //         x: x_coord,
    //         y: y_coord,
    //     });
    
    // }
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);

        });
        // for i in 0..no_of_obstacles - 1 {
        //     window.draw_2d(&event, |c, g, _| {
        //         game.draw_obstacles(obstacles[i].x, obstacles[i].y, &c, g)
        //     } );
        // }
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
