mod target;

pub(crate) use target::WgpuSurfaceTarget;

use luzure_render::render::RenderError;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureFormat};

use std::rc::Rc;

pub struct WgpuSurface {
    target: Rc<WgpuSurfaceTarget>,
    config: Option<SurfaceConfiguration>,
    device_generation: Option<u64>,
    size: (u32, u32),
}

impl WgpuSurface {
    pub(crate) fn new<W: HasDisplayHandle + HasWindowHandle + 'static>(instance: &Instance, window: W, size: (u32, u32))
        -> Result<Self, RenderError>
    {
        Ok(Self {
            target: Rc::new(WgpuSurfaceTarget::new(instance, window)?),
            config: None,
            device_generation: None,
            size,
        })
    }

    pub(crate) fn target(&self) -> Rc<WgpuSurfaceTarget> {
        Rc::clone(&self.target)
    }

    pub(crate) fn surface(&self) -> &Surface<'static> {
        self.target.surface()
    }

    pub(crate) fn recreate(&mut self, instance: &Instance) -> Result<(), RenderError> {
        let target = Rc::get_mut(&mut self.target)
            .ok_or(RenderError::SurfaceCreation)?;

        target.recreate(instance)?;
        self.config = None;
        self.device_generation = None;

        Ok(())
    }

    pub(crate) fn format(&self) -> Option<TextureFormat> {
        self.config.as_ref().map(|config| config.format)
    }

    pub(crate) const fn device_generation(&self) -> Option<u64> {
        self.device_generation
    }

    pub(crate) const fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub(crate) fn configure(&mut self, adapter: &Adapter, device: &Device, device_generation: u64)
        -> Result<(), RenderError>
    {
        let config = self.surface().get_default_config(adapter, self.size.0, self.size.1)
            .ok_or(RenderError::SurfaceUnsupported)?;

        self.surface().configure(device, &config);
        self.config = Some(config);
        self.device_generation = Some(device_generation);

        Ok(())
    }
}
