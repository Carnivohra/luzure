use luzure_render::{MeshInstance, render::RenderError};
use crate::buffer::buffer_capacity;

use std::{mem::{size_of, size_of_val}, slice};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

pub(crate) struct WgpuInstances {
    buffer: Buffer,
    capacity: usize,
}

impl WgpuInstances {
    pub(crate) fn new(device: &Device) -> Self {
        Self {
            buffer: Self::create_buffer(device, size_of::<MeshInstance>() as u64),
            capacity: 1,
        }
    }

    pub(crate) fn update(&mut self, device: &Device, queue: &Queue, instances: &[MeshInstance])
        -> Result<(), RenderError>
    {
        if instances.is_empty() {
            return Ok(());
        }

        if instances.len() > self.capacity {
            let (capacity, size) = buffer_capacity(instances.len(), size_of::<MeshInstance>(), device.limits().max_buffer_size)
                .ok_or(RenderError::InstanceCapacityExceeded)?;

            self.buffer = Self::create_buffer(device, size);
            self.capacity = capacity;
        }

        queue.write_buffer(&self.buffer, 0, Self::bytes(instances));

        Ok(())
    }

    pub(crate) const fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn create_buffer(device: &Device, size: u64) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("luzure-wgpu instance buffer"),
            size,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn bytes(instances: &[MeshInstance]) -> &[u8] {
        const { assert!(size_of::<MeshInstance>() == size_of::<[[f32; 4]; 4]>()); }

        unsafe {
            slice::from_raw_parts(instances.as_ptr().cast(), size_of_val(instances))
        }
    }
}
