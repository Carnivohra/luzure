use luzure_render::render::RenderError;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use wgpu::{Instance, Surface, SurfaceTargetUnsafe};

use std::any::Any;

pub(crate) struct WgpuSurfaceTarget {
    raw_display_handle: RawDisplayHandle,
    raw_window_handle: RawWindowHandle,
    surface: Surface<'static>,
    _window: Box<dyn Any>,
}

impl WgpuSurfaceTarget {
    pub(crate) fn new<W: HasDisplayHandle + HasWindowHandle + 'static>(instance: &Instance, window: W)
        -> Result<Self, RenderError>
    {
        let raw_display_handle = window.display_handle()
            .map_err(|_| RenderError::SurfaceCreation)?;
        let raw_window_handle = window.window_handle()
            .map_err(|_| RenderError::SurfaceCreation)?;
        let raw_display_handle = raw_display_handle.as_raw();
        let raw_window_handle = raw_window_handle.as_raw();
        let surface = Self::create_surface(instance, raw_display_handle, raw_window_handle)?;

        Ok(Self {
            raw_display_handle,
            raw_window_handle,
            surface,
            _window: Box::new(window),
        })
    }

    pub(crate) fn recreate(&mut self, instance: &Instance) -> Result<(), RenderError> {
        self.surface = Self::create_surface(instance, self.raw_display_handle, self.raw_window_handle)?;

        Ok(())
    }

    pub(crate) const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    fn create_surface(instance: &Instance, raw_display_handle: RawDisplayHandle, raw_window_handle: RawWindowHandle)
        -> Result<Surface<'static>, RenderError>
    {
        let target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(raw_display_handle),
            raw_window_handle,
        };

        unsafe { instance.create_surface_unsafe(target) }
            .map_err(|_| RenderError::SurfaceCreation)
    }
}
