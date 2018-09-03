use std::mem;

use ggez::{self, Context};
use ggez::graphics::{self, Rect, Color, Point2};

use constants;
use direction::{self, Direction};
use game::GameResult;

#[derive(Debug)]
pub struct Snake {
    pub segments: Vec<Point2>,
    pub color: Color,
    pub velocity: f32,
    pub direction: Direction,
    update_tick: f32
}

impl Snake {
    pub fn new(initial_pos: Point2, velocity: f32, color: Color) -> Self {
        let segments = vec![
            initial_pos,
            Point2::new(initial_pos.coords.x - constants::BLOCK_SIZE, initial_pos.coords.y)
        ];

        Snake {
            segments,
            velocity,
            color,
            direction: Direction::Right,
            update_tick: 0.0
        }
    }

    fn next_pos(&self) -> Point2 {
        let x: f32;
        let y: f32;

        let head = &self.segments[0];

        match self.direction {
            Direction::Up => {
                x = head.x;
                y = if head.y - constants::BLOCK_SIZE < 0.0 {
                    constants::WINDOW_HEIGHT - constants::BLOCK_SIZE
                } else {
                    head.y - constants::BLOCK_SIZE
                };
            },
            Direction::Down => {
                x = head.x;
                y = (head.y + constants::BLOCK_SIZE) % constants::WINDOW_HEIGHT;
            },
            Direction::Left => {
                x = if head.x - constants::BLOCK_SIZE < 0.0 {
                    constants::WINDOW_WIDTH - constants::BLOCK_SIZE
                } else {
                    head.x - constants::BLOCK_SIZE
                };
                y = head.y;
            },
            Direction::Right => {
                x = (head.x + constants::BLOCK_SIZE) % constants::WINDOW_WIDTH;
                y = head.y;
            }
        };

        Point2::new(x, y)
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !direction::opposite(self.direction, new_direction) {
            self.direction = new_direction;
        }
    }
}

impl ggez::event::EventHandler for Snake {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for segment in &self.segments {
            let x = segment.coords.x;
            let y = segment.coords.y;
            
            graphics::set_color(ctx, self.color)?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, Rect::new(x, y, constants::BLOCK_SIZE, constants::BLOCK_SIZE))?;
            graphics::set_color(ctx, Color::from_rgb(0, 100, 0))?;
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), Rect::new(x, y, constants::BLOCK_SIZE, constants::BLOCK_SIZE))?;
        }

        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_tick += self.velocity;

        if self.update_tick < constants::UPDATE_DELAY {
            return Ok(());
        }

        let mut next_pos = self.next_pos();

        for segment in &mut self.segments {
            mem::swap(&mut next_pos.x, &mut segment.coords.x);
            mem::swap(&mut next_pos.y, &mut segment.coords.y);
        }

        self.update_tick = 0.0;
        Ok(())
    }
}
