use ggez::event::Keycode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn from_keycode(keycode: Keycode) -> Direction {
    match keycode {
        Keycode::Up => Direction::Up,
        Keycode::Down => Direction::Up,
        Keycode::Left => Direction::Right,
        Keycode::Right => Direction::Left,
        _ => panic!("Can't get direction from keycode {}", keycode)
    }
}

pub fn opposite(x: Direction, y: Direction) -> bool {
    match x {
        Direction::Up => y == Direction::Down,
        Direction::Down => y == Direction::Up,
        Direction::Left => y == Direction::Right,
        Direction::Right => y == Direction::Left,
    }
}
