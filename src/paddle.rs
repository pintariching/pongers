use std::{collections::HashSet, time::Instant};

use glam::Vec2;
use winit::event::VirtualKeyCode;

pub struct Paddle {
    pub position: Vec2,
    pub direction: Vec2,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(
        position: Vec2,
        direction: Vec2,
        up: VirtualKeyCode,
        down: VirtualKeyCode,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            position,
            direction,
            up,
            down,
            width,
            height,
        }
    }

    pub fn update(&mut self, last_update: Instant, pressed_keys: &HashSet<VirtualKeyCode>) {
        self.direction = Vec2::ZERO;

        if pressed_keys.contains(&self.up) {
            self.direction += Vec2::NEG_Y;
        }

        if pressed_keys.contains(&self.down) {
            self.direction += Vec2::Y;
        }

        self.position += self.direction * (Instant::now() - last_update).as_secs_f32() * 300.;

        // let limit = window_size.height as f32 / 2. - 50.;
        // self.position.y = self.position.y.clamp(-limit, limit);
    }
}
