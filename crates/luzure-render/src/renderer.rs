use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::{MeshDescriptor, MeshHandle, render::{RenderError, RenderFrame}};

pub trait Renderer {
    type Surface;

    fn create_surface<W: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static>(&mut self, window: W, size: (u32, u32))
        -> Result<Self::Surface, RenderError>;
    fn resize_surface(&mut self, surface: &mut Self::Surface, size: (u32, u32))
        -> Result<(), RenderError>;
    fn create_mesh(&mut self, descriptor: MeshDescriptor) -> Result<MeshHandle, RenderError>;
    fn destroy_mesh(&mut self, mesh: MeshHandle) -> Result<(), RenderError>;
    fn render(&mut self, surface: &Self::Surface, frame: &RenderFrame) -> Result<(), RenderError>;
}
