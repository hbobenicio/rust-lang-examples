use ggez::graphics::{self, Rect};
use ggez::Context;
use ggez::GameResult;
use direction::{self, Direction};

pub struct Snake {
    pub head: Rect,
    pub direction: Direction
}

impl Snake {
    pub fn new(initial_pos: Rect) -> Self {
        Snake {
            head: initial_pos,
            direction: Direction::Right
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.head)?;

        Ok(())
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.head.y += 0.1,
            Direction::Down => self.head.y += 0.1,
            Direction::Left => self.head.x -= 0.1,
            Direction::Right => self.head.x += 0.1
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !direction::opposite(self.direction, new_direction) {
            self.direction = new_direction;
        }
    }
}
