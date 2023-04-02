use std::time::Instant;

use glam::Vec2;

pub struct Ball {
    pub position: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            position: center,
            radius,
            velocity: Vec2::new(-200., 10.).clamp_length_max(200.),
        }
    }

    pub fn update(&mut self, last_update: Instant) {
        self.position += self.velocity * (Instant::now() - last_update).as_secs_f32();
    }
}
