use ggez::graphics::{self, Rect};
use ggez::Context;
use ggez::GameResult;
use direction::{self, Direction};

pub struct Snake {
    pub head: Rect,
    pub body: Vec<Rect>,
    pub direction: Direction,
    pub velocity: f32
}

impl Snake {
    pub fn new(initial_pos: Rect, velocity: f32) -> Self {
        Snake {
            head: initial_pos,
            body: vec![],
            direction: Direction::Right,
            velocity
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.head)?;

        Ok(())
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.head.y -= self.velocity,
            Direction::Down => self.head.y += self.velocity,
            Direction::Left => self.head.x -= self.velocity,
            Direction::Right => self.head.x += self.velocity
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !direction::opposite(self.direction, new_direction) {
            self.direction = new_direction;
        }
    }
}
