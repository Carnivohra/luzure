use crate::{camera::WgpuCamera, instance::WgpuInstances, mesh::WgpuMesh, pipeline::WgpuPipeline, surface::WgpuDepth, viewport::WgpuViewport};

use luzure_render::{MeshDescriptor, MeshHandle, MeshInstance, MeshPipelineContract, RenderFrame, RenderView, render::RenderError};
use wgpu::{Adapter, BindGroupLayout, Color, CommandEncoder, Device, LoadOp, Operations, PollType, Queue, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, TextureFormat, TextureView};

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub(super) struct WgpuRendererState {
    adapter: Adapter,
    camera: WgpuCamera,
    camera_bind_group_layout: BindGroupLayout,
    device: Device,
    device_lost: Arc<AtomicBool>,
    instances: WgpuInstances,
    meshes: Vec<Option<WgpuMesh>>,
    pipelines: Vec<(TextureFormat, WgpuPipeline)>,
    queue: Queue,
}

impl WgpuRendererState {
    pub(super) fn new(adapter: Adapter, device: Device, queue: Queue) -> Self {
        let camera_bind_group_layout = WgpuCamera::create_bind_group_layout(&device);
        let camera = WgpuCamera::new(&device, &camera_bind_group_layout);
        let device_lost = Arc::new(AtomicBool::new(false));
        let device_lost_callback = Arc::clone(&device_lost);
        let instances = WgpuInstances::new(&device);

        device.set_device_lost_callback(move |_, _| {
            device_lost_callback.store(true, Ordering::Relaxed);
        });

        Self {
            adapter,
            camera,
            camera_bind_group_layout,
            device,
            device_lost,
            instances,
            meshes: vec![],
            pipelines: Vec::new(),
            queue,
        }
    }

    pub(super) const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub(super) fn is_lost(&self) -> bool {
        self.device_lost.load(Ordering::Relaxed)
    }

    pub(super) fn poll_device_loss(&self) -> bool {
        let _ = self.device.poll(PollType::Poll);

        self.is_lost()
    }

    pub(super) fn update_cameras(&mut self, views: &[RenderView]) -> Result<(), RenderError> {
        self.camera.update(&self.device, &self.queue, &self.camera_bind_group_layout, views)
    }

    pub(super) fn update_instances(&mut self, instances: &[MeshInstance]) -> Result<(), RenderError> {
        self.instances.update(&self.device, &self.queue, instances)
    }

    pub(super) fn create_mesh(&mut self, handle: MeshHandle, descriptor: &MeshDescriptor) -> Result<(), RenderError> {
        let index = usize::try_from(handle.value())
            .map_err(|_| RenderError::InvalidMeshHandle)?;

        if index != self.meshes.len() {
            return Err(RenderError::InvalidMeshHandle);
        }

        let mesh = WgpuMesh::new(&self.device, descriptor)?;

        self.meshes.push(Some(mesh));

        Ok(())
    }

    pub(super) fn restore_meshes(&mut self, descriptors: &[Option<MeshDescriptor>]) -> Result<(), RenderError> {
        self.meshes.reserve(descriptors.len());

        for descriptor in descriptors {
            let mesh = match descriptor {
                Some(descriptor) => Some(WgpuMesh::new(&self.device, descriptor)?),
                None => None,
            };

            self.meshes.push(mesh);
        }

        Ok(())
    }

    pub(super) fn destroy_mesh(&mut self, handle: MeshHandle) -> Result<(), RenderError> {
        let index = usize::try_from(handle.value())
            .map_err(|_| RenderError::InvalidMeshHandle)?;
        let mesh = self.meshes.get_mut(index)
            .ok_or(RenderError::InvalidMeshHandle)?;

        if mesh.take().is_none() {
            return Err(RenderError::InvalidMeshHandle);
        }

        Ok(())
    }

    fn mesh(&self, handle: MeshHandle) -> Option<&WgpuMesh> {
        let index = usize::try_from(handle.value()).ok()?;

        self.meshes.get(index)?.as_ref()
    }

    pub(super) fn ensure_pipeline(&mut self, surface_format: TextureFormat) {
        if self.pipelines.iter().any(|entry| entry.0 == surface_format) {
            return;
        }

        let pipeline = WgpuPipeline::new(&self.device, surface_format, &self.camera_bind_group_layout);
        self.pipelines.push((surface_format, pipeline));
    }

    fn pipeline(&self, surface_format: TextureFormat) -> Option<&WgpuPipeline> {
        self.pipelines
            .iter()
            .find(|entry| entry.0 == surface_format)
            .map(|entry| &entry.1)
    }

    pub(super) const fn device(&self) -> &Device {
        &self.device
    }

    pub(super) const fn queue(&self) -> &Queue {
        &self.queue
    }

    pub(super) fn encode(&self, encoder: &mut CommandEncoder, color: &TextureView, depth: &WgpuDepth, frame: &RenderFrame)
        -> Result<(), RenderError>
    {
        let size = (color.texture().width(), color.texture().height());
        let pipeline = self.pipeline(color.texture().format())
            .ok_or(RenderError::PipelineUnavailable)?;
        let mut load = LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 });

        for (index, view) in frame.views().iter().enumerate() {
            let Some(viewport) = WgpuViewport::new(view.viewport(), size) else {
                continue;
            };
            let camera_offset = self.camera.dynamic_offset(index)?;
            let mut pass = Self::begin_pass(encoder, color, Some(depth.view()), load);

            pass.set_pipeline(pipeline.pipeline());
            viewport.apply(&mut pass);
            pass.set_bind_group(MeshPipelineContract::CAMERA_GROUP, self.camera.bind_group(), &[camera_offset]);

            if !frame.mesh_batches().is_empty() {
                pass.set_vertex_buffer(1, self.instances.buffer().slice(..));
            }

            for batch in frame.mesh_batches() {
                let mesh = self.mesh(batch.mesh())
                    .ok_or(RenderError::InvalidMeshHandle)?;
                let first_instance = batch.first_instance();
                let end_instance = first_instance + batch.instance_count();

                pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
                pass.set_index_buffer(mesh.index_buffer().slice(..), mesh.index_format());
                pass.draw_indexed(0..mesh.index_count(), 0, first_instance..end_instance);
            }

            drop(pass);
            load = LoadOp::Load;
        }

        if matches!(load, LoadOp::Clear(_)) {
            drop(Self::begin_pass(encoder, color, None, load));
        }

        Ok(())
    }

    fn begin_pass<'a>(encoder: &'a mut CommandEncoder, color: &TextureView, depth: Option<&TextureView>, load: LoadOp<Color>) -> RenderPass<'a> {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("luzure-wgpu render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: color, depth_slice: None, resolve_target: None, ops: Operations {
                    load, store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: depth.map(|view| RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        })
    }
}
