extern crate ggez;

mod direction;
mod snake;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event;
use direction::Direction;
use snake::Snake;

pub struct State {
    snake: Snake
}

impl State {
    pub fn new() -> State {
        State {
            snake: Snake::new(graphics::Rect::new(10.0, 10.0, 25.0, 25.0))
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.snake.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 0.0, 0.0))?;
        self.snake.draw(ctx);

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        match keycode {
            event::Keycode::Up | 
            event::Keycode::Down |
            event::Keycode::Left |
            event::Keycode::Right =>
                self.snake.change_direction(direction::from_keycode(keycode)),
            _ => {},
        }
    }
}

fn main() {
    let mut state = State::new();
    let conf = ggez::conf::Conf::new();
    let mut ctx = Context::load_from_conf("Snake", "Hugo Benício", conf)
        .expect("Unable to create game context");

    if let Err(e) = event::run(&mut ctx, &mut state) {
        println!("Error: {}", e);
    };
}
