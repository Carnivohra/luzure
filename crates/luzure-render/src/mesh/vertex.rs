use luzure_math::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MeshVertex {
    position: Vec3,
    normal: Vec3,
    texture_coordinate: Vec2,
}

impl MeshVertex {
    pub const fn new(position: Vec3, normal: Vec3, texture_coordinate: Vec2) -> Self {
        Self {
            position,
            normal,
            texture_coordinate,
        }
    }

    pub const fn position(&self) -> Vec3 {
        self.position
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    pub const fn texture_coordinate(&self) -> Vec2 {
        self.texture_coordinate
    }
}
