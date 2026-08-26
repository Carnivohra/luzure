use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::render::RenderError;

pub trait Renderer {
    type Surface;

    fn create_surface<W: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static>(&mut self, window: W, size: (u32, u32))
        -> Result<Self::Surface, RenderError>;
    fn render(&self, surface: &Self::Surface) -> Result<(), RenderError>;
}
