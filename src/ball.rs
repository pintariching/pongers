use std::time::Instant;

use glam::Vec2;

use crate::paddle::Paddle;

pub struct Ball {
    pub center: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            velocity: Vec2::new(-100., 0.),
        }
    }

    pub fn update(&mut self, last_update: Instant, left_paddle: &Paddle, right_paddle: &Paddle) {
        self.center += self.velocity * (Instant::now() - last_update).as_secs_f32();

        let left = self.center.x - self.radius;
        let right = self.center.x + self.radius;
        let top = self.center.y + self.radius;
        let bottom = self.center.y - self.radius;

        if left < left_paddle.position.x + left_paddle.width / 2. {
            self.velocity = Vec2::new(100., 0.);
        }

        if right > right_paddle.position.x - right_paddle.width / 2. {
            self.velocity = Vec2::new(-100., 0.);
        }
    }
}
