mod initialization;
mod state;

use initialization::{WgpuRendererInitialization, initialize};
use state::WgpuRendererState;

use luzure_render::{MeshDescriptor, MeshHandle, Renderer, RendererStatus, render::{RenderError, RenderFrame}};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, Instance, InstanceDescriptor, TextureViewDescriptor};

use crate::WgpuSurface;

use std::task::{Context, Poll, Waker};

pub struct WgpuRenderer {
    instance: Instance,
    initialization: Option<WgpuRendererInitialization>,
    device_generation: u64,
    meshes: Vec<Option<MeshDescriptor>>,
    state: Option<WgpuRendererState>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::new_without_display_handle()),
            initialization: None,
            device_generation: 0,
            meshes: Vec::new(),
            state: None,
        }
    }

    fn configure_surface(state: &mut WgpuRendererState, surface: &mut WgpuSurface, device_generation: u64)
        -> Result<(), RenderError>
    {
        surface.configure(state.adapter(), state.device(), device_generation)?;
        state.ensure_pipeline(surface.format()
            .ok_or(RenderError::SurfaceUnsupported)?);

        Ok(())
    }

    fn discard_lost_state(&mut self) {
        if self.state.as_ref().is_some_and(|state| state.is_lost()) {
            self.state = None;
            self.initialization = None;
        }
    }

    fn initialize_from_surface(&mut self, surface: &WgpuSurface) {
        if self.state.is_none() && self.initialization.is_none() {
            self.initialization = Some(initialize(self.instance.clone(), surface.target()));
        }
    }
}

impl Renderer for WgpuRenderer {
    type Surface = WgpuSurface;

    fn update(&mut self) -> Result<RendererStatus, RenderError> {
        self.discard_lost_state();

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
                let mut state = result?;

                state.restore_meshes(&self.meshes)?;

                self.device_generation = self.device_generation.wrapping_add(1);
                self.state = Some(state);

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

        self.discard_lost_state();
        self.initialize_from_surface(&surface);

        if let Some(state) = &mut self.state {
            Self::configure_surface(state, &mut surface, self.device_generation)?;
        }

        Ok(surface)
    }

    fn resize_surface(&mut self, surface: &mut Self::Surface, size: (u32, u32))
        -> Result<(), RenderError>
    {
        if size.0 == 0 || size.1 == 0 {
            return Ok(());
        }

        self.discard_lost_state();

        if surface.size() == size && surface.device_generation() == Some(self.device_generation) && self.state.is_some() {
            return Ok(());
        }

        surface.set_size(size);

        if let Some(state) = &mut self.state {
            Self::configure_surface(state, surface, self.device_generation)?;
        }

        Ok(())
    }

    fn create_mesh(&mut self, mesh: MeshHandle, descriptor: MeshDescriptor) -> Result<(), RenderError> {
        self.discard_lost_state();

        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.create_mesh(mesh, &descriptor)?;
        self.meshes.push(Some(descriptor));

        Ok(())
    }

    fn destroy_mesh(&mut self, mesh: MeshHandle) -> Result<(), RenderError> {
        self.discard_lost_state();

        let index = usize::try_from(mesh.value())
            .map_err(|_| RenderError::InvalidMeshHandle)?;
        let descriptor = self.meshes.get_mut(index)
            .ok_or(RenderError::InvalidMeshHandle)?;

        if descriptor.is_none() {
            return Err(RenderError::InvalidMeshHandle);
        }

        let state = self.state.as_mut()
            .ok_or(RenderError::DeviceRequest)?;

        state.destroy_mesh(mesh)?;
        descriptor.take();

        Ok(())
    }

    fn render(&mut self, surface: &mut Self::Surface, render_frame: &RenderFrame) -> Result<(), RenderError> {
        self.discard_lost_state();
        self.initialize_from_surface(surface);

        let Some(state) = self.state.as_mut() else {
            return Ok(());
        };

        if surface.device_generation() != Some(self.device_generation) {
            Self::configure_surface(state, surface, self.device_generation)?;
        }

        let (frame, reconfigure) = match surface.surface().get_current_texture() {
            CurrentSurfaceTexture::Success(frame) => (frame, false),
            CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
            CurrentSurfaceTexture::Outdated => {
                Self::configure_surface(state, surface, self.device_generation)?;
                return Ok(());
            },
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
            CurrentSurfaceTexture::Lost => {
                if state.poll_device_loss() {
                    return Ok(());
                }

                surface.recreate(&self.instance)?;
                Self::configure_surface(state, surface, self.device_generation)?;
                return Ok(());
            },
            CurrentSurfaceTexture::Validation => {
                if state.poll_device_loss() {
                    return Ok(());
                }

                return Err(RenderError::SurfaceAcquisition);
            },
        };

        if !render_frame.views().is_empty() {
            state.update_cameras(render_frame.views())?;
            state.update_instances(render_frame.instances())?;
        }

        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let depth = surface.depth()
            .ok_or(RenderError::SurfaceUnsupported)?;
        let mut encoder = state.device().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("luzure-wgpu frame encoder"),
        });

        state.encode(&mut encoder, &view, depth, render_frame)?;

        state.queue().submit([encoder.finish()]);
        state.queue().present(frame);

        if reconfigure {
            Self::configure_surface(state, surface, self.device_generation)?;
        }

        Ok(())
    }
}
