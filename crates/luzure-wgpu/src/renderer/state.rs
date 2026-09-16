use crate::{camera::WgpuCamera, instance::WgpuInstances, mesh::WgpuMesh, pipeline::WgpuPipeline};

use luzure_render::{MeshDescriptor, MeshHandle, MeshInstance, RenderView, render::RenderError};
use wgpu::{Adapter, BindGroupLayout, Device, PollType, Queue, TextureFormat};

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

    pub(super) const fn camera(&self) -> &WgpuCamera {
        &self.camera
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

    pub(super) const fn instances(&self) -> &WgpuInstances {
        &self.instances
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
