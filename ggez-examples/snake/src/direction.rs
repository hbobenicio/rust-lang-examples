use ggez::event::Keycode;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Keycode> for Direction {
    fn from(keycode: Keycode) -> Self {
        match keycode {
            Keycode::Up => Direction::Up,
            Keycode::Down => Direction::Down,
            Keycode::Left => Direction::Left,
            Keycode::Right => Direction::Right,
            _ => panic!("Can't get direction from keycode {}", keycode)
        }
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

#[cfg(test)]
mod tests {

    use super::{Direction, opposite};
    use ggez::event::Keycode;

    #[test]
    fn opposite_should_work() {
        let up = Direction::Up;
        let down = Direction::Down;
        let left = Direction::Left;
        let right = Direction::Right;

        assert_eq!(opposite(up, down), true);
        assert_eq!(opposite(down, up), true);
        assert_eq!(opposite(left, right), true);
        assert_eq!(opposite(right, left), true);
    }

    #[test]
    fn opposite_should_fail() {
        let up = Direction::Up;
        let down = Direction::Down;
        let left = Direction::Left;
        let right = Direction::Right;

        assert_eq!(opposite(up, up), false);
        assert_eq!(opposite(up, left), false);
        assert_eq!(opposite(up, right), false);

        assert_eq!(opposite(down, down), false);
        assert_eq!(opposite(down, left), false);
        assert_eq!(opposite(down, right), false);

        assert_eq!(opposite(left, left), false);
        assert_eq!(opposite(left, up), false);
        assert_eq!(opposite(left, down), false);

        assert_eq!(opposite(right, right), false);
        assert_eq!(opposite(right, up), false);
        assert_eq!(opposite(right, down), false);
    }

    #[test]
    fn from_keycode_should_work() {
        assert_eq!(Direction::from(Keycode::Up), Direction::Up);
        assert_eq!(Direction::from(Keycode::Down), Direction::Down);
        assert_eq!(Direction::from(Keycode::Left), Direction::Left);
        assert_eq!(Direction::from(Keycode::Right), Direction::Right);
    }
}
