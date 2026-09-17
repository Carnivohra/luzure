mod uniform;

use uniform::CameraUniform;
use crate::buffer::buffer_capacity;

use bytemuck::bytes_of;
use luzure_render::{MeshPipelineContract, RenderView, render::RenderError};

use std::mem::size_of;

use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};

pub(crate) struct WgpuCamera {
    bind_group: BindGroup,
    buffer: Buffer,
    capacity: usize,
    staging: Vec<u8>,
    stride: usize,
}

impl WgpuCamera {
    pub(crate) fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("luzure-wgpu camera bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: MeshPipelineContract::CAMERA_BINDING,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(size_of::<CameraUniform>() as u64),
                },
                count: None,
            }],
        })
    }

    pub(crate) fn new(device: &Device, bind_group_layout: &BindGroupLayout) -> Self {
        let alignment = device.limits().min_uniform_buffer_offset_alignment as usize;
        let stride = size_of::<CameraUniform>().div_ceil(alignment) * alignment;
        let buffer = Self::create_buffer(device, stride as u64);
        let bind_group = Self::create_bind_group(device, bind_group_layout, &buffer);

        Self {
            bind_group,
            buffer,
            capacity: 1,
            staging: vec![0; stride],
            stride,
        }
    }

    pub(crate) fn update(&mut self, device: &Device, queue: &Queue, bind_group_layout: &BindGroupLayout, views: &[RenderView])
        -> Result<(), RenderError>
    {
        if views.is_empty() {
            return Ok(());
        }

        if views.len() > self.capacity {
            let limit = device.limits().max_buffer_size.min(u64::from(u32::MAX) + self.stride as u64);
            let (capacity, size) = buffer_capacity(views.len(), self.stride, limit)
                .ok_or(RenderError::ViewCapacityExceeded)?;

            self.buffer = Self::create_buffer(device, size);
            self.bind_group = Self::create_bind_group(device, bind_group_layout, &self.buffer);
            self.staging.resize(size as usize, 0);
            self.capacity = capacity;
        }

        for (index, view) in views.iter().enumerate() {
            let offset = index * self.stride;
            let uniform = CameraUniform::new(view.camera_matrices());
            let bytes = bytes_of(&uniform);

            self.staging[offset..offset + bytes.len()].copy_from_slice(bytes);
        }

        let size = views.len() * self.stride;

        queue.write_buffer(&self.buffer, 0, &self.staging[..size]);

        Ok(())
    }

    pub(crate) fn dynamic_offset(&self, index: usize) -> Result<u32, RenderError> {
        index.checked_mul(self.stride)
            .and_then(|offset| u32::try_from(offset).ok())
            .ok_or(RenderError::ViewCapacityExceeded)
    }

    pub(crate) const fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    fn create_buffer(device: &Device, size: u64) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("luzure-wgpu camera buffer"),
            size,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_bind_group(device: &Device, bind_group_layout: &BindGroupLayout, buffer: &Buffer) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("luzure-wgpu camera bind group"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: MeshPipelineContract::CAMERA_BINDING,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer,
                    offset: 0,
                    size: BufferSize::new(size_of::<CameraUniform>() as u64),
                }),
            }],
        })
    }
}
