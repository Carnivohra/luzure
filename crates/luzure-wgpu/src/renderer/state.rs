use crate::{camera::WgpuCamera, instance::WgpuInstances, mesh::WgpuMesh, pipeline::WgpuPipeline};

use luzure_render::{CameraMatrices, MeshDescriptor, MeshHandle, MeshInstance, render::RenderError};
use wgpu::{Adapter, BindGroupLayout, Device, Queue, TextureFormat};

pub(super) struct WgpuRendererState {
    adapter: Adapter,
    camera: WgpuCamera,
    camera_bind_group_layout: BindGroupLayout,
    device: Device,
    instances: WgpuInstances,
    meshes: Vec<Option<WgpuMesh>>,
    pipelines: Vec<(TextureFormat, WgpuPipeline)>,
    queue: Queue,
}

impl WgpuRendererState {
    pub(super) fn new(adapter: Adapter, device: Device, queue: Queue) -> Self {
        let camera_bind_group_layout = WgpuCamera::create_bind_group_layout(&device);
        let camera = WgpuCamera::new(&device, &camera_bind_group_layout, &CameraMatrices::IDENTITY);
        let instances = WgpuInstances::new(&device);

        Self {
            adapter,
            camera,
            camera_bind_group_layout,
            device,
            instances,
            meshes: vec![],
            pipelines: Vec::new(),
            queue,
        }
    }

    pub(super) const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub(super) const fn camera(&self) -> &WgpuCamera {
        &self.camera
    }

    pub(super) fn update_camera(&self, camera_matrices: &CameraMatrices) {
        self.camera.update(&self.queue, camera_matrices);
    }

    pub(super) fn update_instances(&mut self, instances: &[MeshInstance]) -> Result<(), RenderError> {
        self.instances.update(&self.device, &self.queue, instances)
    }

    pub(super) const fn instances(&self) -> &WgpuInstances {
        &self.instances
    }

    pub(super) fn create_mesh(&mut self, handle: MeshHandle, descriptor: MeshDescriptor) -> Result<(), RenderError> {
        let index = usize::try_from(handle.value())
            .map_err(|_| RenderError::InvalidMeshHandle)?;

        if index != self.meshes.len() {
            return Err(RenderError::InvalidMeshHandle);
        }

        let mesh = WgpuMesh::new(&self.device, descriptor)?;

        self.meshes.push(Some(mesh));

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

    pub(super) fn mesh(&self, handle: MeshHandle) -> Option<&WgpuMesh> {
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

    pub(super) fn pipeline(&self, surface_format: TextureFormat) -> Option<&WgpuPipeline> {
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
}
