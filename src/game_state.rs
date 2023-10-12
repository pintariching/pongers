use std::{collections::HashSet, time::Instant};

use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    ShaderStages,
};
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::{
    ball::Ball,
    paddle::{Paddle, PaddleRaw},
};

pub struct GameState {
    pub start_time: Instant,
    pub last_update: Instant,
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub paddles: [Paddle; 8],
    pub paddle_storage_buffer: Buffer,
    pub paddle_storage_bind_group_layout: BindGroupLayout,
    pub paddle_storage_bind_group: BindGroup,
    pub ball: Ball,
    pub ball_storage_buffer: Buffer,
    pub ball_storage_bind_group_layout: BindGroupLayout,
    pub ball_storage_bind_group: BindGroup,
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

        let raw_ball = ball.to_raw();

        let ball_storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Ball Storage Buffer"),
            contents: bytemuck::cast_slice(&[raw_ball]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let ball_storage_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Ball Storage Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let ball_storage_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &ball_storage_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: ball_storage_buffer.as_entire_binding(),
            }],
            label: Some("Ball Storage Bind Group"),
        });

        let paddles = [
            Paddle::new(
                Vec2::new(50., window_size.height as f32 / 2.),
                Vec2::X,
                VirtualKeyCode::W,
                VirtualKeyCode::S,
                20.,
                100.,
                true,
                0.,
            ),
            // Paddle::new(
            //     Vec2::new(
            //         window_size.width as f32 - 20.,
            //         window_size.height as f32 / 2.,
            //     ),
            //     Vec2::NEG_X,
            //     VirtualKeyCode::Up,
            //     VirtualKeyCode::Down,
            //     20.,
            //     100.,
            //     true,
            //     0.,
            // ),
            Paddle::default(),
            Paddle::default(),
            Paddle::default(),
            Paddle::default(),
            Paddle::default(),
            Paddle::default(),
            Paddle::default(),
        ];

        let raw_paddles: [PaddleRaw; 8] = [
            paddles[0].to_raw(),
            paddles[1].to_raw(),
            paddles[2].to_raw(),
            paddles[3].to_raw(),
            paddles[4].to_raw(),
            paddles[5].to_raw(),
            paddles[6].to_raw(),
            paddles[7].to_raw(),
        ];

        let paddle_storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Paddle Storage Buffer"),
            contents: bytemuck::cast_slice(&[raw_paddles]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let paddle_storage_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Paddle Storage Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let paddle_storage_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &paddle_storage_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: paddle_storage_buffer.as_entire_binding(),
            }],
            label: Some("Paddle Storage Bind Group"),
        });

        GameState {
            start_time: Instant::now(),
            last_update: Instant::now(),
            pressed_keys: HashSet::new(),
            paddles,
            paddle_storage_buffer,
            paddle_storage_bind_group,
            paddle_storage_bind_group_layout,
            ball,
            ball_storage_buffer,
            ball_storage_bind_group,
            ball_storage_bind_group_layout,
            window_size: window_size.clone(),
        }
    }

    pub fn update(&mut self) {
        for p in &mut self.paddles {
            if !p.active {
                continue;
            }

            p.update(self.last_update, &self.pressed_keys, &self.window_size);

            if let Some(normal) = p.check_intersection(&self.ball) {
                println!("Intersection normal: {:?}", normal);

                let v = calculate_reflection(normal, &self.ball, &p);
                self.ball.velocity = v;
            }
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

        self.ball.update(self.last_update, &self.pressed_keys);
    }
}

fn calculate_reflection(normal: Vec2, ball: &Ball, paddle: &Paddle) -> Vec2 {
    let proj = ball.velocity.project_onto_normalized(normal);
    let i = proj * -2.;
    let mut new_v = i + ball.velocity;

    let diff = (ball.position.y - paddle.position.y) / ((paddle.height - ball.radius) / 2.);

    new_v.y = diff * 100.;

    new_v.clamp_length_max(100.);
    new_v
}

fn calculate_wall_reflection(ball: &Ball, normal: Vec2) -> Vec2 {
    let proj = ball.velocity.project_onto_normalized(normal);
    let i = proj * -2.;
    let new_v = i + ball.velocity;

    new_v
}
