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
    pub right_paddle: Paddle,
    pub ball: Ball,
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

        GameState {
            start_time: Instant::now(),
            last_update: Instant::now(),
            pressed_keys: HashSet::new(),
            left_paddle: Paddle::new(Vec2::new(-1., 0.), Vec2::X),
            right_paddle: Paddle::new(Vec2::new(1., 0.), Vec2::NEG_X),
            ball: Ball {},
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        }
    }

    pub fn update(&mut self) {}
}
