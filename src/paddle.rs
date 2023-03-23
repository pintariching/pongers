use std::{collections::HashSet, time::Instant};

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::mesh::{Mesh, Vertex};

pub struct Paddle {
    pub position: Vec2,
    pub direction: Vec2,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub width: f32,
    pub height: f32,
    pub mesh: Mesh,
}

impl Paddle {
    pub fn new(
        device: &Device,
        position: Vec2,
        direction: Vec2,
        up: VirtualKeyCode,
        down: VirtualKeyCode,
        width: f32,
        height: f32,
    ) -> Self {
        let w = width / 2.;
        let h = height / 2.;

        let vertices = &[
            Vertex {
                position: [w, h, 0.],
            },
            Vertex {
                position: [w, -h, 0.],
            },
            Vertex {
                position: [-w, -h, 0.],
            },
            Vertex {
                position: [-w, h, 0.],
            },
        ];

        let indices = &[0, 1, 2, 2, 3, 0];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        let mesh = Mesh {
            name: "Mesh".into(),
            vertex_buffer,
            index_buffer,
            num_indices: 6,
        };

        Self {
            position,
            direction,
            up,
            down,
            width,
            height,
            mesh,
        }
    }

    pub fn to_raw(&self) -> PaddleRaw {
        PaddleRaw {
            position: self.position,
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

        self.position += self.direction * (Instant::now() - last_update).as_secs_f32() * 300.;

        let limit = window_size.height as f32 / 2. - 50.;
        self.position.y = self.position.y.clamp(-limit, limit);
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PaddleRaw {
    pub position: Vec2,
}

impl PaddleRaw {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<PaddleRaw>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 2,
                format: VertexFormat::Float32x2,
            }],
        }
    }
}
