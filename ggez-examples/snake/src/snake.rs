use std::mem;
use ggez::graphics::{self, Rect, Color, Point2};
use ggez::Context;
use ggez::GameResult;
use direction::{self, Direction};
use segment::Segment;

const WIDTH: f32 = 25.0;
const HEIGHT: f32 = 25.0;

#[derive(Debug)]
pub struct Snake {
    pub segments: Vec<Segment>,
    pub color: Color,
    pub velocity: f32,
}

impl Snake {
    pub fn new(initial_pos: Point2, velocity: f32, color: Color) -> Self {
        let segments = vec![
            Segment::new(initial_pos, Direction::Right),
            Segment::new(
                Point2::new(initial_pos.coords.x - WIDTH, initial_pos.coords.y),
                Direction::Right
            )
        ];

        Snake {
            segments,
            velocity,
            color
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        for segment in &self.segments {
            let x = segment.pos.coords.x;
            let y = segment.pos.coords.y;
            
            graphics::set_color(ctx, self.color)?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, Rect::new(x, y, WIDTH, HEIGHT))?;
            graphics::set_color(ctx, Color::from_rgb(0, 100, 0))?;
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), Rect::new(x, y, WIDTH, HEIGHT))?;
        }

        Ok(())
    }

    pub fn update(&mut self) {
        let mut next_direction = self.segments[0].direction.clone();

        for segment in &mut self.segments {
            let next_pos = match next_direction {
                Direction::Up => (segment.pos.x, segment.pos.y - self.velocity),
                Direction::Down => (segment.pos.x, segment.pos.y + self.velocity),
                Direction::Left => (segment.pos.x - self.velocity, segment.pos.y),
                Direction::Right => (segment.pos.x + self.velocity, segment.pos.y)
            };

            segment.pos.coords.x = next_pos.0;
            segment.pos.coords.y = next_pos.1;

            // TODO Problema aqui... Todos os segmentos mudarão de direção em poucos updates...
            mem::swap(&mut segment.direction, &mut next_direction);
        }

        // TODO Problema deste: pulo imediato para as próximas posições
        // for segment in &mut self.segments {
        //     mem::swap(&mut next_pos.0, &mut segment.coords.x);
        //     mem::swap(&mut next_pos.1, &mut segment.coords.y);
        // }
    }

    pub fn update2(&mut self) {
        // let mut next_pos = match self.direction {
        //     Direction::Up => (self.segments[0].x, self.segments[0].y - self.velocity),
        //     Direction::Down => (self.segments[0].x, self.segments[0].y + self.velocity),
        //     Direction::Left => (self.segments[0].x - self.velocity, self.segments[0].y),
        //     Direction::Right => (self.segments[0].x + self.velocity, self.segments[0].y)
        // };

        // for segment in &mut self.segments {
        //     mem::swap(&mut next_pos.0, &mut segment.coords.x);
        //     mem::swap(&mut next_pos.1, &mut segment.coords.y);
        // }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !direction::opposite(self.segments[0].direction, new_direction) {
            self.segments[0].direction = new_direction;
        }
    }
}
