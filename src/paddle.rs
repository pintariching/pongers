use std::{collections::HashSet, time::Instant};

use bytemuck::{Pod, Zeroable};
use glam::{Mat2, Vec2};
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::{ball::Ball, game_state::Intersection};

pub struct Paddle {
    pub position: Vec2,
    pub direction: Vec2,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub width: f32,
    pub height: f32,
    pub active: bool,

    /// The angle of the paddle in radians
    pub rotation: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            position: Default::default(),
            direction: Default::default(),
            up: VirtualKeyCode::Up,
            down: VirtualKeyCode::Down,
            width: Default::default(),
            height: Default::default(),
            active: false,
            rotation: 0.,
        }
    }
}

impl Paddle {
    /// `rotation` the angle of the paddle in degrees
    pub fn new(
        position: Vec2,
        direction: Vec2,
        up: VirtualKeyCode,
        down: VirtualKeyCode,
        width: f32,
        height: f32,
        active: bool,
        rotation: f32,
    ) -> Self {
        Self {
            position,
            direction,
            up,
            down,
            width,
            height,
            active,
            rotation: rotation.to_radians(),
        }
    }

    pub fn update(
        &mut self,
        last_update: Instant,
        pressed_keys: &HashSet<VirtualKeyCode>,
        window_size: &PhysicalSize<u32>,
    ) {
        self.direction = Vec2::ZERO;

        if pressed_keys.contains(&self.up) {
            self.direction += Vec2::NEG_Y;
        }

        if pressed_keys.contains(&self.down) {
            self.direction += Vec2::Y;
        }

        if pressed_keys.contains(&VirtualKeyCode::Q) {
            self.rotation += 0.5 * (Instant::now() - last_update).as_secs_f32();
        }

        if pressed_keys.contains(&VirtualKeyCode::E) {
            self.rotation -= 0.5 * (Instant::now() - last_update).as_secs_f32();
        }

        self.position += self.direction * (Instant::now() - last_update).as_secs_f32() * 300.;

        self.position.y = self.position.y.clamp(
            self.height / 2.,
            window_size.height as f32 - self.height / 2.,
        );
    }

    pub fn check_intersection(&self, ball: &Ball) -> Option<Intersection> {
        let rotation_matrix = Mat2::from_angle(self.rotation);
        let w = self.width / 2.;
        let h = self.height / 2.;

        let a = Vec2::new(-w, -h);
        let b = Vec2::new(w, -h);
        let c = Vec2::new(w, h);
        let d = Vec2::new(-w, h);

        let rot_a = self.position + rotation_matrix * a;
        let rot_b = self.position + rotation_matrix * b;
        let rot_c = self.position + rotation_matrix * c;
        let rot_d = self.position + rotation_matrix * d;

        if (ball.position - rot_a).length() < ball.radius {
            return Some(Intersection::Point(a));
        }

        if (ball.position - rot_b).length() < ball.radius {
            return Some(Intersection::Point(b));
        }

        if (ball.position - rot_c).length() < ball.radius {
            return Some(Intersection::Point(c));
        }

        if (ball.position - rot_d).length() < ball.radius {
            return Some(Intersection::Point(d));
        }

        let ball_relative = Mat2::from_angle(-self.rotation) * (ball.position - self.position);

        let ball_left = ball_relative.x - ball.radius;
        let ball_right = ball_relative.x + ball.radius;
        let ball_top = ball_relative.y - ball.radius;
        let ball_bottom = ball_relative.y + ball.radius;

        if ball_left < w && ball_right > -w && ball_bottom < h && ball_top > -h {
            return Some(Intersection::Edge);
        }

        None
    }

    pub fn corners(&self) -> [Vec2; 4] {
        let rotation_matrix = Mat2::from_angle(self.rotation);
        let w = self.width / 2.;
        let h = self.height / 2.;

        let a = Vec2::new(-w, -h);
        let b = Vec2::new(w, -h);
        let c = Vec2::new(w, h);
        let d = Vec2::new(-w, h);

        let rot_a = self.position + rotation_matrix * a;
        let rot_b = self.position + rotation_matrix * b;
        let rot_c = self.position + rotation_matrix * c;
        let rot_d = self.position + rotation_matrix * d;

        [rot_a, rot_b, rot_c, rot_d]
    }

    pub fn to_raw(&self) -> PaddleRaw {
        let corners = self.corners();

        PaddleRaw {
            a: corners[0],
            b: corners[1],
            c: corners[2],
            d: corners[3],
            active: if self.active { 1 } else { 0 },
            _padding: 0.,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PaddleRaw {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
    pub d: Vec2,
    pub active: i32,
    _padding: f32,
}
