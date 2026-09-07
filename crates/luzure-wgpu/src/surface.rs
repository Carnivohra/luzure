use wgpu::{Device, Surface, SurfaceConfiguration};

pub struct WgpuSurface {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
}

impl WgpuSurface {
    pub(crate) fn new(surface: Surface<'static>, config: SurfaceConfiguration) -> Self {
        Self {
            surface,
            config,
        }
    }

    pub(crate) const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub(crate) fn resize(&mut self, device: &Device, size: (u32, u32)) {
        self.config.width = size.0;
        self.config.height = size.1;
        self.surface.configure(device, &self.config);
    }
}
