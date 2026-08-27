use wgpu::{Surface, SurfaceConfiguration};

pub struct WgpuSurface {
    surface: Surface<'static>,
    _config: SurfaceConfiguration,
}

impl WgpuSurface {
    pub(crate) fn new(surface: Surface<'static>, _config: SurfaceConfiguration) -> Self {
        Self {
            surface,
            _config,
        }
    }

    pub(crate) const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}
