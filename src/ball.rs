use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use winit::dpi::PhysicalSize;

use crate::{
    mesh::{Mesh, Vertex},
    paddle::Paddle,
};

pub struct Ball {
    pub center: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
    pub mesh: Mesh,
}

impl Ball {
    pub fn new(device: &Device, center: Vec2, radius: f32) -> Self {
        let vertices = &[
            Vertex {
                position: [radius, radius, 0.],
            },
            Vertex {
                position: [radius, -radius, 0.],
            },
            Vertex {
                position: [-radius, -radius, 0.],
            },
            Vertex {
                position: [-radius, radius, 0.],
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
            center,
            radius,
            velocity: Vec2::new(-100., 0.),
            mesh,
        }
    }

    pub fn to_raw(&self) -> BallRaw {
        BallRaw {
            center: self.center,
            radius: self.radius,
        }
    }

    pub fn update(
        &mut self,
        last_update: Instant,
        window_size: &PhysicalSize<u32>,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
    ) {
        self.center += self.velocity * (Instant::now() - last_update).as_secs_f32();

        let left = self.center.x - self.radius;
        let right = self.center.x + self.radius;
        let top = self.center.y + self.radius;
        let bottom = self.center.y - self.radius;

        if left < left_paddle.position.x + left_paddle.width / 2. {
            self.velocity = Vec2::new(100., 0.);
        }

        if right > right_paddle.position.x - right_paddle.width / 2. {
            self.velocity = Vec2::new(-100., 0.);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BallRaw {
    pub center: Vec2,
    pub radius: f32,
}

impl BallRaw {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<BallRaw>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32,
                },
            ],
        }
    }
}
