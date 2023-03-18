use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::instance::Instance;

pub struct Ball {
    pub instance: Instance,
    pub instance_buffer: Buffer,
}

impl Ball {
    pub fn new(device: &Device) -> Self {
        let instance = Instance {
            position: Vec2::new(0., 0.),
            rotation: 0.,
            scale: 100.,
        };

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[instance.to_raw()]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self {
            instance,
            instance_buffer,
        }
    }
}
