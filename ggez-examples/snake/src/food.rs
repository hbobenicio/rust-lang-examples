use ggez::{self, Context};
use ggez::graphics::{self, Color, Point2, Rect};

use constants;
use game::GameResult;

#[derive(Debug)]
pub struct Food {
    pub pos: Point2,
    pub color: Color
}

impl Food {
    pub fn new(pos: Point2) -> Food {
        Food {
            pos,
            color: Color::from_rgb(255, 0, 0)
        }
    }
}

impl ggez::event::EventHandler for Food {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_color(ctx, self.color)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, Rect::new(self.pos.x, self.pos.y, constants::BLOCK_SIZE, constants::BLOCK_SIZE))?;

        Ok(())
    }
}
