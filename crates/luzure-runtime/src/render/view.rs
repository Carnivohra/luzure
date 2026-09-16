use luzure_ecs::Entity;
use luzure_render::Viewport;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraView {
    camera: Entity,
    order: i32,
    target: Entity,
    viewport: Viewport,
}

impl CameraView {
    pub const fn new(camera: Entity, target: Entity, viewport: Viewport) -> Self {
        Self {
            camera,
            order: 0,
            target,
            viewport,
        }
    }

    pub const fn camera(&self) -> Entity {
        self.camera
    }

    pub const fn target(&self) -> Entity {
        self.target
    }

    pub const fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub const fn order(&self) -> i32 {
        self.order
    }

    pub const fn set_camera(&mut self, camera: Entity) {
        self.camera = camera;
    }

    pub const fn set_target(&mut self, target: Entity) {
        self.target = target;
    }

    pub const fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub const fn set_order(&mut self, order: i32) {
        self.order = order;
    }
}
