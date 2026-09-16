use crate::CameraMatrices;

use super::{RenderTarget, Viewport};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderView {
    camera_matrices: CameraMatrices,
    order: i32,
    target: RenderTarget,
    viewport: Viewport,
}

impl RenderView {
    pub const fn new(target: RenderTarget, camera_matrices: CameraMatrices, viewport: Viewport, order: i32) -> Self {
        Self {
            camera_matrices,
            order,
            target,
            viewport,
        }
    }

    pub const fn camera_matrices(&self) -> &CameraMatrices {
        &self.camera_matrices
    }

    pub const fn target(&self) -> RenderTarget {
        self.target
    }

    pub const fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub const fn order(&self) -> i32 {
        self.order
    }
}
