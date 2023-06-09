use std::time::Instant;

use bytemuck::{Pod, Zeroable};
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

    pub fn to_raw(&self) -> BallRaw {
        BallRaw {
            position: self.position,
            radius: self.radius,
            _padding: 0.,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BallRaw {
    pub position: Vec2,
    pub radius: f32,
    _padding: f32,
}
