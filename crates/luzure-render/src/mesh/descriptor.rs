use crate::MeshVertex;

#[derive(Debug, Clone, PartialEq)]
pub struct MeshDescriptor {
    vertices: Vec<MeshVertex>,
    indices: Vec<u32>,
}

impl MeshDescriptor {
    pub const fn new(vertices: Vec<MeshVertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    pub fn vertices(&self) -> &[MeshVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}
