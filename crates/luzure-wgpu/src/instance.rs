use luzure_render::{MeshInstance, render::RenderError};

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
            self.capacity = instances.len().checked_next_power_of_two()
                .ok_or(RenderError::InstanceCapacityExceeded)?;
            let size = u64::try_from(self.capacity).ok()
                .and_then(|capacity| capacity.checked_mul(size_of::<MeshInstance>() as u64))
                .ok_or(RenderError::InstanceCapacityExceeded)?;

            self.buffer = Self::create_buffer(device, size);
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
