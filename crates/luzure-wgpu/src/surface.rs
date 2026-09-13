mod target;

pub(crate) use target::WgpuSurfaceTarget;

use luzure_render::render::RenderError;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureFormat};

use std::rc::Rc;

pub struct WgpuSurface {
    target: Rc<WgpuSurfaceTarget>,
    config: Option<SurfaceConfiguration>,
    size: (u32, u32),
}

impl WgpuSurface {
    pub(crate) fn new<W: HasDisplayHandle + HasWindowHandle + 'static>(instance: &Instance, window: W, size: (u32, u32))
        -> Result<Self, RenderError>
    {
        Ok(Self {
            target: Rc::new(WgpuSurfaceTarget::new(instance, window)?),
            config: None,
            size,
        })
    }

    pub(crate) fn target(&self) -> Rc<WgpuSurfaceTarget> {
        Rc::clone(&self.target)
    }

    pub(crate) fn surface(&self) -> &Surface<'static> {
        self.target.surface()
    }

    pub(crate) fn format(&self) -> Option<TextureFormat> {
        self.config.as_ref().map(|config| config.format)
    }

    pub(crate) const fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub(crate) fn configure(&mut self, adapter: &Adapter, device: &Device) -> Result<(), RenderError> {
        let config = self.surface().get_default_config(adapter, self.size.0, self.size.1)
            .ok_or(RenderError::SurfaceUnsupported)?;

        self.surface().configure(device, &config);
        self.config = Some(config);

        Ok(())
    }
}
