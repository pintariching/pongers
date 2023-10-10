use std::collections::HashSet;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use winit::event::VirtualKeyCode;

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

    pub fn update(&mut self, last_update: Instant, pressed_keys: &HashSet<VirtualKeyCode>) {
        if pressed_keys.contains(&VirtualKeyCode::Up) {
            self.position += Vec2::NEG_Y * ((Instant::now() - last_update).as_secs_f32() * 100.);
        }

        if pressed_keys.contains(&VirtualKeyCode::Down) {
            self.position += Vec2::Y * ((Instant::now() - last_update).as_secs_f32() * 100.);
        }

        if pressed_keys.contains(&VirtualKeyCode::Left) {
            self.position += Vec2::NEG_X * ((Instant::now() - last_update).as_secs_f32() * 100.);
        }

        if pressed_keys.contains(&VirtualKeyCode::Right) {
            self.position += Vec2::X * ((Instant::now() - last_update).as_secs_f32() * 100.);
        }

        // self.position += self.velocity * (Instant::now() - last_update).as_secs_f32();
    }

    pub fn corners(&self, axis: Vec2) -> [Vec2; 2] {
        let norm_axis = axis.normalize();

        let a = self.position + norm_axis * self.radius;
        let b = self.position - norm_axis * self.radius;

        [a, b]
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
