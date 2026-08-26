use wgpu::{Surface, SurfaceConfiguration};

pub struct WgpuSurface {
    _surface: Surface<'static>,
    _config: SurfaceConfiguration,
}

impl WgpuSurface {
    pub(crate) fn new(_surface: Surface<'static>, _config: SurfaceConfiguration) -> Self {
        Self {
            _surface,
            _config,
        }
    }
}
