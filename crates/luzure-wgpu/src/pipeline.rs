use crate::shader::MESH_SOURCE;

use luzure_render::{MeshInstance, MeshVertex};
use std::mem::size_of;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Device, FragmentState,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

const VERTEX_ATTRIBUTES: [VertexAttribute; 3] = [
    VertexAttribute {
        format: VertexFormat::Float32x3,
        offset: 0,
        shader_location: 0,
    },
    VertexAttribute {
        format: VertexFormat::Float32x3,
        offset: size_of::<[f32; 3]>() as u64,
        shader_location: 1,
    },
    VertexAttribute {
        format: VertexFormat::Float32x2,
        offset: size_of::<[f32; 6]>() as u64,
        shader_location: 2,
    },
];

const INSTANCE_ATTRIBUTES: [VertexAttribute; 4] = [
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 0,
        shader_location: 3,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 4]>() as u64,
        shader_location: 4,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 8]>() as u64,
        shader_location: 5,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 12]>() as u64,
        shader_location: 6,
    },
];

pub(crate) struct WgpuPipeline {
    pipeline: RenderPipeline,
}

impl WgpuPipeline {
    pub(crate) fn new(device: &Device, surface_format: TextureFormat, camera_bind_group_layout: &BindGroupLayout) -> Self {
        let shader = device.create_shader_module(
            ShaderModuleDescriptor {
                label: Some("luzure-wgpu mesh shader"),
                source: ShaderSource::Wgsl(MESH_SOURCE.into()),
            },
        );

        let layout = device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: Some("luzure-wgpu mesh pipeline layout"),
                bind_group_layouts: &[Some(camera_bind_group_layout)],
                immediate_size: 0,
            },
        );

        let pipeline = device.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: Some("luzure-wgpu mesh pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &[
                        Some(VertexBufferLayout {
                            array_stride: size_of::<MeshVertex>() as u64,
                            step_mode: VertexStepMode::Vertex,
                            attributes: &VERTEX_ATTRIBUTES,
                        }),
                        Some(VertexBufferLayout {
                            array_stride: size_of::<MeshInstance>() as u64,
                            step_mode: VertexStepMode::Instance,
                            attributes: &INSTANCE_ATTRIBUTES,
                        }),
                    ],
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: surface_format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            },
        );

        Self {
            pipeline,
        }
    }

    pub(crate) const fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
