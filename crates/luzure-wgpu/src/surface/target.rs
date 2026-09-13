use luzure_render::render::RenderError;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Instance, Surface, SurfaceTargetUnsafe};

use std::any::Any;

pub(crate) struct WgpuSurfaceTarget {
    surface: Surface<'static>,
    _window: Box<dyn Any>,
}

impl WgpuSurfaceTarget {
    pub(crate) fn new<W: HasDisplayHandle + HasWindowHandle + 'static>(instance: &Instance, window: W)
        -> Result<Self, RenderError>
    {
        let target = unsafe { SurfaceTargetUnsafe::from_display_and_window(&window, &window) }
            .map_err(|_| RenderError::SurfaceCreation)?;
        let surface = unsafe { instance.create_surface_unsafe(target) }
            .map_err(|_| RenderError::SurfaceCreation)?;

        Ok(Self {
            surface,
            _window: Box::new(window),
        })
    }

    pub(crate) const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}
