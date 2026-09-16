use luzure_render::Viewport;
use wgpu::RenderPass;

pub(crate) struct WgpuViewport {
    height: f32,
    scissor_height: u32,
    scissor_width: u32,
    scissor_x: u32,
    scissor_y: u32,
    width: f32,
    x: f32,
    y: f32,
}

impl WgpuViewport {
    pub(crate) fn new(viewport: Viewport, size: (u32, u32)) -> Option<Self> {
        let values = [viewport.x(), viewport.y(), viewport.width(), viewport.height()];

        if values.iter().any(|value| !value.is_finite()) {
            return None;
        }

        let left = viewport.x().clamp(0.0, 1.0);
        let top = viewport.y().clamp(0.0, 1.0);
        let right = (viewport.x() + viewport.width()).clamp(0.0, 1.0);
        let bottom = (viewport.y() + viewport.height()).clamp(0.0, 1.0);

        if right <= left || bottom <= top {
            return None;
        }

        let surface_width = size.0 as f32;
        let surface_height = size.1 as f32;
        let x = left * surface_width;
        let y = top * surface_height;
        let width = (right - left) * surface_width;
        let height = (bottom - top) * surface_height;
        let scissor_x = x.floor() as u32;
        let scissor_y = y.floor() as u32;
        let scissor_right = (x + width).ceil().min(surface_width) as u32;
        let scissor_bottom = (y + height).ceil().min(surface_height) as u32;
        let scissor_width = scissor_right.saturating_sub(scissor_x);
        let scissor_height = scissor_bottom.saturating_sub(scissor_y);

        if scissor_width == 0 || scissor_height == 0 {
            return None;
        }

        Some(Self {
            height,
            scissor_height,
            scissor_width,
            scissor_x,
            scissor_y,
            width,
            x,
            y,
        })
    }

    pub(crate) fn apply(&self, pass: &mut RenderPass<'_>) {
        pass.set_viewport(self.x, self.y, self.width, self.height, 0.0, 1.0);
        pass.set_scissor_rect(
            self.scissor_x,
            self.scissor_y,
            self.scissor_width,
            self.scissor_height,
        );
    }
}
