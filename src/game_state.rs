use std::{collections::HashSet, time::Instant};

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    ShaderStages,
};
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::{ball::Ball, paddle::Paddle};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Uniforms {
    pub left_paddle_position: Vec2,
    pub left_paddle_width: f32,
    pub left_paddle_height: f32,
    pub right_paddle_position: Vec2,
    pub right_paddle_width: f32,
    pub right_paddle_height: f32,
    pub ball_position: Vec2,
    pub ball_radius: f32,
    _padding: f32,
}

impl Uniforms {
    pub fn new(left: &Paddle, right: &Paddle, ball: &Ball) -> Self {
        Self {
            left_paddle_position: left.position,
            left_paddle_width: left.width,
            left_paddle_height: left.height,
            right_paddle_position: right.position,
            right_paddle_width: right.width,
            right_paddle_height: right.height,
            ball_position: ball.position,
            ball_radius: ball.radius,
            _padding: 0.,
        }
    }
}

pub struct GameState {
    pub start_time: Instant,
    pub last_update: Instant,
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub ball: Ball,
    pub uniforms: Uniforms,
    pub uniforms_buffer: Buffer,
    pub uniforms_bind_group_layout: BindGroupLayout,
    pub uniforms_bind_group: BindGroup,
    pub window_size: PhysicalSize<u32>,
}

impl GameState {
    pub fn new(device: &Device, window_size: &PhysicalSize<u32>) -> Self {
        let ball = Ball::new(
            Vec2::new(
                window_size.width as f32 / 2.,
                window_size.height as f32 / 2.,
            ),
            10.,
        );

        let left_paddle = Paddle::new(
            Vec2::new(20., window_size.height as f32 / 2.),
            Vec2::X,
            VirtualKeyCode::W,
            VirtualKeyCode::S,
            20.,
            100.,
        );

        let right_paddle = Paddle::new(
            Vec2::new(
                window_size.width as f32 - 20.,
                window_size.height as f32 / 2.,
            ),
            Vec2::NEG_X,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
            20.,
            100.,
        );

        let uniforms = Uniforms::new(&left_paddle, &right_paddle, &ball);

        let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniforms Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniforms_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Uniforms Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let uniforms_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniforms_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        GameState {
            start_time: Instant::now(),
            last_update: Instant::now(),
            pressed_keys: HashSet::new(),
            left_paddle,
            right_paddle,
            ball,
            uniforms,
            uniforms_buffer,
            uniforms_bind_group_layout,
            uniforms_bind_group,
            window_size: window_size.clone(),
        }
    }

    pub fn update(&mut self) {
        self.left_paddle
            .update(self.last_update, &self.pressed_keys, self.window_size);

        self.right_paddle
            .update(self.last_update, &self.pressed_keys, self.window_size);

        if self.left_paddle.check_intersection(&self.ball) {
            let v = calculate_reflection(&self.ball, &self.left_paddle);
            self.ball.velocity = v;
        }

        if self.right_paddle.check_intersection(&self.ball) {
            let v = calculate_reflection(&self.ball, &self.right_paddle);
            self.ball.velocity = v;
        }

        if self.ball.position.y - self.ball.radius < 0. {
            // Top wall collision
            let v = calculate_wall_reflection(&self.ball, Vec2::Y);
            self.ball.velocity = v;
        }

        if self.ball.position.y + self.ball.radius > self.window_size.height as f32 {
            // Bottom wall collision
            let v = calculate_wall_reflection(&self.ball, Vec2::NEG_Y);
            self.ball.velocity = v;
        }

        self.ball.update(self.last_update);

        // if self.right_paddle.check_intersection(&self.ball) {
        //     println!("Right intersection");
        // }

        self.uniforms = Uniforms::new(&self.left_paddle, &self.right_paddle, &self.ball);
    }
}

fn calculate_reflection(ball: &Ball, paddle: &Paddle) -> Vec2 {
    let n = Vec2::X;
    let proj = ball.velocity.project_onto_normalized(n);
    let i = proj * -2.;
    let mut new_v = i + ball.velocity;

    let diff = (ball.position.y - paddle.position.y) / ((paddle.height - ball.radius) / 2.);

    new_v.y = diff * 100.;

    new_v.clamp_length_max(200.);
    new_v
}

fn calculate_wall_reflection(ball: &Ball, normal: Vec2) -> Vec2 {
    let proj = ball.velocity.project_onto_normalized(normal);
    let i = proj * -2.;
    let new_v = i + ball.velocity;

    new_v
}
