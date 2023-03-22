use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::{
    instance::Instance,
    mesh::{Mesh, Vertex},
};

pub struct Ball {
    pub center: Vec2,
    pub radius: f32,
    pub mesh: Mesh,
}

impl Ball {
    pub fn new(device: &Device, center: Vec2, radius: f32) -> Self {
        let vertices = &[
            Vertex {
                position: [radius, radius, 0.],
                color: [0., 0., 0.],
            },
            Vertex {
                position: [radius, -radius, 0.],
                color: [0., 0., 0.],
            },
            Vertex {
                position: [-radius, -radius, 0.],
                color: [0., 0., 0.],
            },
            Vertex {
                position: [-radius, radius, 0.],
                color: [0., 0., 0.],
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
            mesh,
        }
    }
}
