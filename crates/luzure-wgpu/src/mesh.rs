use luzure_render::{MeshDescriptor, MeshVertex, render::RenderError};

use std::{mem::{size_of, size_of_val}, slice};
use wgpu::{Buffer, BufferUsages, Device, IndexFormat};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub(crate) struct WgpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl WgpuMesh {
    pub(crate) fn new(device: &Device, descriptor: MeshDescriptor) -> Result<Self, RenderError> {
        if descriptor.vertices().is_empty() || descriptor.indices().is_empty() {
            return Err(RenderError::InvalidMesh);
        }

        let index_count = u32::try_from(descriptor.indices().len())
            .map_err(|_| RenderError::MeshCapacityExceeded)?;

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("luzure-wgpu mesh vertex buffer"),
            contents: Self::vertex_bytes(descriptor.vertices()),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("luzure-wgpu mesh index buffer"),
            contents: bytemuck::cast_slice(descriptor.indices()),
            usage: BufferUsages::INDEX,
        });

        Ok(Self {
            vertex_buffer,
            index_buffer,
            index_count,
        })
    }

    pub(crate) const fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub(crate) const fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub(crate) const fn index_format(&self) -> IndexFormat {
        IndexFormat::Uint32
    }

    pub(crate) const fn index_count(&self) -> u32 {
        self.index_count
    }

    fn vertex_bytes(vertices: &[MeshVertex]) -> &[u8] {
        const { assert!(size_of::<MeshVertex>() == size_of::<[f32; 8]>()); }

        unsafe {
            slice::from_raw_parts(vertices.as_ptr().cast(), size_of_val(vertices))
        }
    }
}
