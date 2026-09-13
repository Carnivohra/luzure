mod initialization;
mod state;

use initialization::{WgpuRendererInitialization, initialize};
use state::WgpuRendererState;

use luzure_render::{MeshDescriptor, MeshHandle, Renderer, RendererStatus, render::{RenderError, RenderFrame}};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Instance, InstanceDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor};

use crate::WgpuSurface;

use std::task::{Context, Poll, Waker};

pub struct WgpuRenderer {
    instance: Instance,
    initialization: Option<WgpuRendererInitialization>,
    state: Option<WgpuRendererState>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::new_without_display_handle()),
            initialization: None,
            state: None,
        }
    }
}

impl Renderer for WgpuRenderer {
    type Surface = WgpuSurface;

    fn update(&mut self) -> Result<RendererStatus, RenderError> {
        if self.state.is_some() {
            return Ok(RendererStatus::Ready);
        }

        let Some(initialization) = self.initialization.as_mut() else {
            return Ok(RendererStatus::Uninitialized);
        };

        let mut context = Context::from_waker(Waker::noop());

        match initialization.as_mut().poll(&mut context) {
            Poll::Pending => Ok(RendererStatus::Initializing),
            Poll::Ready(result) => {
                self.initialization = None;
                self.state = Some(result?);

                Ok(RendererStatus::Ready)
            },
        }
    }

    fn suspend(&mut self) {
        self.initialization = None;
    }

    fn create_surface<W: HasDisplayHandle + HasWindowHandle + 'static>(&mut self, window: W, size: (u32, u32))
        -> Result<Self::Surface, RenderError>
    {
        if size.0 == 0 || size.1 == 0 {
            return Err(RenderError::InvalidSurfaceSize);
        }

        let mut surface = WgpuSurface::new(&self.instance, window, size)?;

        if self.state.is_none() && self.initialization.is_none() {
            self.initialization = Some(initialize(self.instance.clone(), surface.target()));
        }

        if let Some(state) = &mut self.state {
            surface.configure(state.adapter(), state.device())?;
            state.ensure_pipeline(surface.format()
                .ok_or(RenderError::SurfaceUnsupported)?);
        }

        Ok(surface)
    }

    fn resize_surface(&mut self, surface: &mut Self::Surface, size: (u32, u32))
        -> Result<(), RenderError>
    {
        if size.0 == 0 || size.1 == 0 {
            return Ok(());
        }

        surface.set_size(size);

        if let Some(state) = &mut self.state {
            surface.configure(state.adapter(), state.device())?;
            state.ensure_pipeline(surface.format()
                .ok_or(RenderError::SurfaceUnsupported)?);
        }

        Ok(())
    }

    fn create_mesh(&mut self, mesh: MeshHandle, descriptor: MeshDescriptor) -> Result<(), RenderError> {
        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.create_mesh(mesh, descriptor)
    }

    fn destroy_mesh(&mut self, mesh: MeshHandle) -> Result<(), RenderError> {
        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.destroy_mesh(mesh)
    }

    fn render(&mut self, surface: &mut Self::Surface, render_frame: &RenderFrame) -> Result<(), RenderError> {
        let Some(state) = self.state.as_mut() else {
            return Ok(());
        };

        if surface.format().is_none() {
            surface.configure(state.adapter(), state.device())?;
        }

        let surface_format = surface.format()
            .ok_or(RenderError::SurfaceUnsupported)?;

        state.ensure_pipeline(surface_format);

        state.update_camera(render_frame.camera_matrices());
        state.update_instances(render_frame.instances())?;

        let pipeline = state.pipeline(surface_format)
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
            label: Some("luzure-wgpu render pass"), color_attachments: &[Some(RenderPassColorAttachment {
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
