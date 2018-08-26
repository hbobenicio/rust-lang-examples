use ggez::graphics::Point2;
use direction::Direction;

#[derive(Debug)]
pub struct Segment {
    pub pos: Point2,
    pub direction: Direction
}

impl Segment {
    pub fn new(pos: Point2, direction: Direction) -> Segment {
        Segment {
            pos,
            direction
        }
    }
}
