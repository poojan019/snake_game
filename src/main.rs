mod draw;
mod snake;
mod game;

use piston_window::*;
use crate::game::Game;
use crate::draw::to_coord_u32;

const BLACK_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = 
        match WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build() {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Error creating window: {}", e);
                return;
            }
        };

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
    
        window.draw_2d(&event, |c, g, _device| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });
    
        if let Some(update_args) = event.update_args() {
            game.update(update_args.dt);
        }
    }
    
}
