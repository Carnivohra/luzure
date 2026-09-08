use crate::{WgpuCamera, WgpuPipeline};

use luzure_render::Camera;
use wgpu::{Adapter, BindGroupLayout, Device, Queue, TextureFormat};

pub(super) struct WgpuRendererState {
    adapter: Adapter,
    camera: WgpuCamera,
    camera_bind_group_layout: BindGroupLayout,
    device: Device,
    pipelines: Vec<(TextureFormat, WgpuPipeline)>,
    queue: Queue,
}

impl WgpuRendererState {
    pub(super) fn new(adapter: Adapter, device: Device, queue: Queue) -> Self {
        let camera_bind_group_layout = WgpuCamera::create_bind_group_layout(&device);
        let camera = WgpuCamera::new(&device, &camera_bind_group_layout, &Camera::IDENTITY);

        Self {
            adapter,
            camera,
            camera_bind_group_layout,
            device,
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

    pub(super) fn update_camera(&self, camera: &Camera) {
        self.camera.update(&self.queue, camera);
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
