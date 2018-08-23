extern crate ggez;

mod direction;
mod snake;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event;
use snake::Snake;

pub struct Game {
    snake: Snake
}

impl Game {
    pub fn new() -> Game {
        let velocity = 1.0;
        let initial_pos = graphics::Rect::new(10.0, 10.0, 25.0, 25.0);
        let snake = Snake::new(initial_pos, velocity);

        Game {
            snake
        }
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.snake.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::Color::from_rgb(0, 255, 0))?;
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
                let new_direction = direction::Direction::from(keycode);
                self.snake.change_direction(new_direction);
            },
            _ => {},
        }
    }
}

fn main() {
    let mut game = Game::new();
    let conf = ggez::conf::Conf::new();
    let mut ctx = Context::load_from_conf("Snake", "Hugo Benício", conf)
        .expect("Unable to create game context");

    graphics::set_background_color(&mut ctx, graphics::BLACK);

    if let Err(e) = event::run(&mut ctx, &mut game) {
        println!("Error: {}", e);
    };
}
