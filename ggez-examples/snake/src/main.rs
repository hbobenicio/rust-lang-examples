extern crate ggez;

mod direction;
mod snake;

use ggez::Context;
use ggez::graphics::{self, Color, Point2};
use ggez::event;
use snake::Snake;
use direction::Direction;

type GameResult = ggez::GameResult<()>;

pub struct Game {
    snake: Snake
}

impl Game {
    pub fn new() -> Game {
        let velocity = 1.0;
        let initial_pos = Point2::new(50.0, 50.0);
        let snake = Snake::new(initial_pos, velocity, Color::from_rgb(0, 255, 0));

        Game {
            snake
        }
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.snake.update();
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx);

        self.snake.draw(ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            event::Keycode::Up | 
            event::Keycode::Down |
            event::Keycode::Left |
            event::Keycode::Right => {
                let new_direction = Direction::from(keycode);
                self.snake.change_direction(new_direction);
                // TODO Tratar o caso em mudar de mais de 2 direções antes do update
                // (Adicionar numa lista de movimentos, talvez...)
            },
            _ => {},
        }
    }
}

fn main() {
    let mut conf = ggez::conf::Conf::new();
    conf.window_mode = ggez::conf::WindowMode::default()
        .dimensions(400, 300);
    conf.window_setup = ggez::conf::WindowSetup::default()
        .title("Snake Game");

    let mut ctx = Context::load_from_conf("Snake", "Hugo Benício", conf)
        .expect("Unable to create game context");

    graphics::set_background_color(&mut ctx, graphics::BLACK);

    let mut game = Game::new();
    if let Err(e) = event::run(&mut ctx, &mut game) {
        println!("Error: {}", e);
    };
}
