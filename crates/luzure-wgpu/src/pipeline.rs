use crate::{shader::MESH_SOURCE, surface::WgpuDepth};

use luzure_render::{MeshInstance, MeshPipelineContract, MeshVertex};
use std::mem::size_of;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Device, FragmentState,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StencilState, TextureFormat,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

const VERTEX_ATTRIBUTES: [VertexAttribute; 3] = [
    VertexAttribute {
        format: VertexFormat::Float32x3,
        offset: 0,
        shader_location: MeshPipelineContract::POSITION_LOCATION,
    },
    VertexAttribute {
        format: VertexFormat::Float32x3,
        offset: size_of::<[f32; 3]>() as u64,
        shader_location: MeshPipelineContract::NORMAL_LOCATION,
    },
    VertexAttribute {
        format: VertexFormat::Float32x2,
        offset: size_of::<[f32; 6]>() as u64,
        shader_location: MeshPipelineContract::TEXTURE_COORDINATE_LOCATION,
    },
];

const INSTANCE_ATTRIBUTES: [VertexAttribute; 4] = [
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 0,
        shader_location: MeshPipelineContract::MODEL_COLUMN_0_LOCATION,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 4]>() as u64,
        shader_location: MeshPipelineContract::MODEL_COLUMN_1_LOCATION,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 8]>() as u64,
        shader_location: MeshPipelineContract::MODEL_COLUMN_2_LOCATION,
    },
    VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: size_of::<[f32; 12]>() as u64,
        shader_location: MeshPipelineContract::MODEL_COLUMN_3_LOCATION,
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
                    entry_point: Some(MeshPipelineContract::VERTEX_ENTRY),
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
                depth_stencil: Some(DepthStencilState {
                    format: WgpuDepth::FORMAT,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(CompareFunction::Less),
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some(MeshPipelineContract::FRAGMENT_ENTRY),
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
