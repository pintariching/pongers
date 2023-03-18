use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};
use winit::{dpi::PhysicalSize, event::VirtualKeyCode};

use crate::{
    instance::Instance,
    mesh::{Mesh, Vertex},
};

pub struct Paddle {
    pub instance: Instance,
    pub instance_buffer: Buffer,
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
        window_size: &PhysicalSize<u32>,
    ) -> Self {
        let vertices = &[
            Vertex {
                position: [0., 0., 0.],
                color: [1., 1., 1.],
            },
            Vertex {
                position: [0.2, 0., 0.],
                color: [1., 1., 1.],
            },
            Vertex {
                position: [0.2, -1., 0.],
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

        let instance = Instance {
            position: Vec2::new(window_size.width as f32 * -0.48, 0.),
            rotation: 0.,
            scale: 100.,
        };

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[instance.to_raw()]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self {
            direction,
            up,
            down,
            mesh,
            instance,
            instance_buffer,
        }
    }
}
