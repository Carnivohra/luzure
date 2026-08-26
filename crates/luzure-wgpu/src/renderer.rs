mod state;

use state::WgpuRendererState;

use luzure_render::{render::RenderError, Renderer};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{DeviceDescriptor, Instance, InstanceDescriptor, RequestAdapterOptions};

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

        let state = self.state.as_ref()
            .ok_or(RenderError::DeviceRequest)?;

        let config = surface.get_default_config(state.adapter(), size.0, size.1)
            .ok_or(RenderError::SurfaceUnsupported)?;

        surface.configure(state.device(), &config);

        Ok(WgpuSurface::new(surface, config))
    }

    fn render(&self, _surface: &Self::Surface) -> Result<(), RenderError> {
        todo!()
    }
}
