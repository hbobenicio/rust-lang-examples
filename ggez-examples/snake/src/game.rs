use ggez::{self, Context};
use ggez::graphics::{self, Color, Point2};
use ggez::event;
use snake::{Snake};
use food::{Food};
use score::{Score};
use direction::Direction;

pub type GameResult = ggez::GameResult<()>;

pub struct Game {
    snake: Snake,
    food: Food,
    score: Score
}

impl Game {
    pub fn new() -> Game {

        // Initializing the Snake
        let velocity = 1.0;
        let initial_pos = Point2::new(50.0, 50.0);
        let snake = Snake::new(initial_pos, velocity, Color::from_rgb(0, 255, 0));

        // Initializing the score
        let score = Score::new(Point2::new(5.0, 5.0));

        // Initializing the Food
        let food_pos = Point2::new(100.0, 100.0);
        let food = Food::new(food_pos);

        Game {
            snake,
            food,
            score
        }
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.snake.update(ctx)?;
        self.score.update(ctx)?;
        self.food.update(ctx)?;
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx);

        self.snake.draw(ctx)?;
        self.score.draw(ctx)?;
        self.food.draw(ctx)?;

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
