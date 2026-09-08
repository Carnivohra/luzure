mod uniform;

use uniform::CameraUniform;

use bytemuck::bytes_of;
use luzure_render::Camera;
use std::mem::size_of;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages,
    Device, Queue, ShaderStages,
};

pub struct WgpuCamera {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl WgpuCamera {
    pub(crate) fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("luzure-wgpu camera bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(size_of::<CameraUniform>() as u64),
                    },
                    count: None,
                }],
            },
        )
    }

    pub fn new(device: &Device, bind_group_layout: &BindGroupLayout, camera: &Camera) -> Self {
        let uniform = CameraUniform::new(camera);

        let buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("luzure-wgpu camera buffer"),
                contents: bytes_of(&uniform),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            },
        );

        let bind_group = device.create_bind_group(
            &BindGroupDescriptor {
                label: Some("luzure-wgpu camera bind group"),
                layout: bind_group_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            },
        );

        Self { buffer, bind_group }
    }

    pub fn update(&self, queue: &Queue, camera: &Camera) {
        let uniform = CameraUniform::new(camera);
        queue.write_buffer(&self.buffer, 0, bytes_of(&uniform));
    }

    pub const fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
