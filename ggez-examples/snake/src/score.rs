use ggez::{self, Context};
use ggez::graphics::{self, Font, Text, Point2, Color};

use game::GameResult;

pub struct Score {
    value: u32,
    position: Point2,
    font: Font,
    color: Color
}

impl Score {
    pub fn new(position: Point2) -> Score {
        Score {
            value: 0,
            position,
            font: Font::default_font().expect("Não foi possível obter fonte padrão."),
            color: graphics::WHITE
        }
    }

    pub fn _score_up(&mut self) {
        self.value += 1;
    }

    pub fn _reset(&mut self) {
        self.value = 0;
    }

    pub fn _set_score(&mut self, value: u32) {
        self.value = value;
    }
}

impl ggez::event::EventHandler for Score {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_color(ctx, self.color)?;

        let text_value = format!("Pontuação: {}", self.value);
        let text = Text::new(ctx, &text_value, &self.font)?;
        graphics::draw(ctx, &text, self.position, 0.0)?;

        Ok(())
    }
}