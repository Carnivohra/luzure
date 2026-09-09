mod state;

use state::WgpuRendererState;

use luzure_render::{MeshDescriptor, MeshHandle, render::{RenderError, RenderFrame}, Renderer};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Color, CommandEncoderDescriptor, CurrentSurfaceTexture, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, TextureViewDescriptor};

use crate::WgpuSurface;

pub struct WgpuRenderer {
    instance: Instance,
    state: Option<WgpuRendererState>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::new_without_display_handle()),
            state: None,
        }
    }
}

impl Renderer for WgpuRenderer {
    type Surface = WgpuSurface;

    fn create_surface<W: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static>(&mut self, window: W, size: (u32, u32))
        -> Result<Self::Surface, RenderError>
    {
        if size.0 == 0 || size.1 == 0 {
            return Err(RenderError::InvalidSurfaceSize);
        }

        let surface = self.instance.create_surface(window)
            .map_err(|_| RenderError::SurfaceCreation)?;

        if self.state.is_none() {
            let adapter = pollster::block_on(self.instance.request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface), ..Default::default()
            })).map_err(|_| RenderError::AdapterRequest)?;

            let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
                label: Some("luzure-wgpu device"), ..Default::default()
            })).map_err(|_| RenderError::DeviceRequest)?;

            self.state = Some(WgpuRendererState::new(adapter, device, queue));
        }

        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        let config = surface.get_default_config(state.adapter(), size.0, size.1)
            .ok_or(RenderError::SurfaceUnsupported)?;

        state.ensure_pipeline(config.format);
        surface.configure(state.device(), &config);

        Ok(WgpuSurface::new(surface, config))
    }

    fn resize_surface(&mut self, surface: &mut Self::Surface, size: (u32, u32))
        -> Result<(), RenderError>
    {
        if size.0 == 0 || size.1 == 0 {
            return Ok(());
        }

        let state = self.state.as_ref()
            .ok_or(RenderError::DeviceRequest)?;

        surface.resize(state.device(), size);

        Ok(())
    }

    fn create_mesh(&mut self, descriptor: MeshDescriptor) -> Result<MeshHandle, RenderError> {
        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.create_mesh(descriptor)
    }

    fn destroy_mesh(&mut self, mesh: MeshHandle) -> Result<(), RenderError> {
        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.destroy_mesh(mesh)
    }

    fn render(&mut self, surface: &Self::Surface, render_frame: &RenderFrame) -> Result<(), RenderError> {
        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.update_camera(render_frame.camera());
        state.update_instances(render_frame.instances())?;

        let pipeline = state.pipeline(surface.format())
            .ok_or(RenderError::PipelineUnavailable)?;

        let frame = match surface.surface().get_current_texture() {
            CurrentSurfaceTexture::Success(frame) | CurrentSurfaceTexture::Suboptimal(frame) => frame,
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
            _ => return Err(RenderError::SurfaceAcquisition),
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = state.device().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("luzure-wgpu frame encoder"),
        });

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("luzure-wgpu clear pass"), color_attachments: &[Some(RenderPassColorAttachment {
                view: &view, depth_slice: None, resolve_target: None, ops: Operations {
                    load: LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), store: StoreOp::Store
                },
            })], ..Default::default()
        });

        pass.set_pipeline(pipeline.pipeline());
        pass.set_bind_group(0, state.camera().bind_group(), &[]);

        if !render_frame.mesh_batches().is_empty() {
            pass.set_vertex_buffer(1, state.instances().buffer().slice(..));
        }

        for batch in render_frame.mesh_batches() {
            let mesh = state.mesh(batch.mesh())
                .ok_or(RenderError::InvalidMeshHandle)?;
            let first_instance = batch.first_instance();
            let instance_count = batch.instance_count();
            let end_instance = first_instance.checked_add(instance_count)
                .ok_or(RenderError::InvalidInstanceRange)?;
            let end_index = usize::try_from(end_instance)
                .map_err(|_| RenderError::InvalidInstanceRange)?;

            if end_index > render_frame.instances().len() {
                return Err(RenderError::InvalidInstanceRange);
            }

            if instance_count == 0 {
                continue;
            }

            pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            pass.set_index_buffer(mesh.index_buffer().slice(..), mesh.index_format());
            pass.draw_indexed(0..mesh.index_count(), 0, first_instance..end_instance);
        }

        drop(pass);
        state.queue().submit([encoder.finish()]);
        state.queue().present(frame);

        Ok(())
    }
}
