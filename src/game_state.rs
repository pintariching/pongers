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
    camera::{Camera, CameraUniform},
    paddle::Paddle,
};

pub struct GameState {
    pub start_time: Instant,
    pub last_update: Instant,
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub left_paddle: Paddle,
    pub left_paddle_buffer: Buffer,
    pub right_paddle: Paddle,
    pub right_paddle_buffer: Buffer,
    pub ball: Ball,
    pub ball_buffer: Buffer,
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub camera_bind_group_layout: BindGroupLayout,
}

impl GameState {
    pub fn new(device: &Device, window_size: &PhysicalSize<u32>) -> Self {
        let camera = Camera {
            focus_position: Vec2::new(0., 0.),
            zoom: 1.,
            window_size: window_size.clone(),
            aspect_ratio: 3. / 4.,
        };

        let camera_uniform = CameraUniform::new(&camera);
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let ball = Ball::new(device, Vec2::new(0., 0.), 100.);
        let ball_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Ball Buffer"),
            contents: bytemuck::cast_slice(&[ball.to_raw()]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let left_paddle = Paddle::new(
            device,
            Vec2::new(-(window_size.width as f32 / 2.) + 50., 0.),
            Vec2::X,
            VirtualKeyCode::W,
            VirtualKeyCode::S,
            20.,
            100.,
        );

        let left_paddle_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Left Paddle Buffer"),
            contents: bytemuck::cast_slice(&[left_paddle.to_raw()]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let right_paddle = Paddle::new(
            device,
            Vec2::new((window_size.width as f32 / 2.) - 50., 0.),
            Vec2::NEG_X,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
            20.,
            100.,
        );

        let right_paddle_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Right Paddle Buffer"),
            contents: bytemuck::cast_slice(&[right_paddle.to_raw()]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        GameState {
            start_time: Instant::now(),
            last_update: Instant::now(),
            pressed_keys: HashSet::new(),
            left_paddle,
            left_paddle_buffer,
            right_paddle,
            right_paddle_buffer,
            ball,
            ball_buffer,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        }
    }

    pub fn update(&mut self) {
        self.left_paddle.update(
            self.last_update,
            &self.pressed_keys,
            &self.camera.window_size,
        );

        self.right_paddle.update(
            self.last_update,
            &self.pressed_keys,
            &self.camera.window_size,
        );

        self.ball.update(
            self.last_update,
            &self.camera.window_size,
            &self.left_paddle,
            &self.right_paddle,
        );
    }
}
