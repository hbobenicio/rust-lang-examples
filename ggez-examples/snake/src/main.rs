extern crate ggez;

mod constants;
mod direction;
mod food;
mod score;
mod snake;
mod game;

use game::Game;

fn main() {
    let mut conf = ggez::conf::Conf::new();
    conf.window_mode = ggez::conf::WindowMode::default()
        .dimensions(constants::WINDOW_WIDTH as u32, constants::WINDOW_HEIGHT as u32);
    conf.window_setup = ggez::conf::WindowSetup::default()
        .title("Snake Game");

    let mut ctx = ggez::Context::load_from_conf("Snake", "Hugo Benício", conf)
        .expect("Unable to create game context");

    ggez::graphics::set_background_color(&mut ctx, ggez::graphics::BLACK);

    let mut game = Game::new();
    if let Err(e) = ggez::event::run(&mut ctx, &mut game) {
        eprintln!("Error: {}", e);
    };
}
