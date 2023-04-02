use std::{collections::HashSet, time::Instant};

use glam::Vec2;
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::ball::Ball;

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

    pub fn update(
        &mut self,
        last_update: Instant,
        pressed_keys: &HashSet<VirtualKeyCode>,
        window_size: PhysicalSize<u32>,
    ) {
        self.direction = Vec2::ZERO;

        if pressed_keys.contains(&self.up) {
            self.direction += Vec2::NEG_Y;
        }

        if pressed_keys.contains(&self.down) {
            self.direction += Vec2::Y;
        }

        self.position += self.direction * (Instant::now() - last_update).as_secs_f32() * 300.;

        self.position.y = self.position.y.clamp(
            self.height / 2.,
            window_size.height as f32 - self.height / 2.,
        );
    }

    pub fn check_intersection(&self, ball: &Ball) -> bool {
        let ball_left = ball.position.x - ball.radius;
        let ball_right = ball.position.x + ball.radius;
        let ball_top = ball.position.y + ball.radius;
        let ball_bottom = ball.position.y - ball.radius;

        let w = self.width / 2.;
        let h = self.height / 2.;

        if (ball_left < self.position.x + w && ball_right > self.position.x - w)
            && (ball_bottom > self.position.y - h && ball_top < self.position.y + h)
        {
            return true;
        }

        false
    }
}
