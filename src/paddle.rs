use glam::Vec2;

pub struct Paddle {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Paddle {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }
}
