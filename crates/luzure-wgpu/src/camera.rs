mod uniform;

use uniform::CameraUniform;

use bytemuck::bytes_of;
use luzure_render::{RenderView, render::RenderError};

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
                binding: 0,
                visibility: ShaderStages::VERTEX,
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
        if views.len() > self.capacity {
            self.capacity = views.len().checked_next_power_of_two()
                .ok_or(RenderError::ViewCapacityExceeded)?;
            let size = self.stride.checked_mul(self.capacity)
                .and_then(|size| u64::try_from(size).ok())
                .ok_or(RenderError::ViewCapacityExceeded)?;

            self.buffer = Self::create_buffer(device, size);
            self.bind_group = Self::create_bind_group(device, bind_group_layout, &self.buffer);
            self.staging.resize(size as usize, 0);
        }

        for (index, view) in views.iter().enumerate() {
            let offset = index.checked_mul(self.stride)
                .ok_or(RenderError::ViewCapacityExceeded)?;
            let uniform = CameraUniform::new(view.camera_matrices());
            let bytes = bytes_of(&uniform);

            self.staging[offset..offset + bytes.len()].copy_from_slice(bytes);
        }

        let size = views.len().checked_mul(self.stride)
            .ok_or(RenderError::ViewCapacityExceeded)?;

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
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer,
                    offset: 0,
                    size: BufferSize::new(size_of::<CameraUniform>() as u64),
                }),
            }],
        })
    }
}
