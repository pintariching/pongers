use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device,
};
use winit::event::VirtualKeyCode;

use crate::mesh::{Mesh, Vertex};

pub struct Paddle {
    pub position: Vec2,
    pub direction: Vec2,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub mesh: Mesh,
}

impl Paddle {
    pub fn new(
        device: &Device,
        position: Vec2,
        direction: Vec2,
        up: VirtualKeyCode,
        down: VirtualKeyCode,
    ) -> Self {
        let vertices = &[
            Vertex {
                position: [0., 0., 0.],
                color: [1., 1., 1.],
            },
            Vertex {
                position: [1., 0., 0.],
                color: [1., 1., 1.],
            },
            Vertex {
                position: [1., -1., 0.],
                color: [1., 1., 1.],
            },
            Vertex {
                position: [0., -1., 0.],
                color: [1., 1., 1.],
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
            mesh,
        }
    }
}
