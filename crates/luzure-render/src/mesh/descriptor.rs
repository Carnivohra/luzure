use crate::MeshVertex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeshDescriptor<'a> {
    vertices: &'a [MeshVertex],
    indices: &'a [u32],
}

impl<'a> MeshDescriptor<'a> {
    pub const fn new(vertices: &'a [MeshVertex], indices: &'a [u32]) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    pub const fn vertices(&self) -> &[MeshVertex] {
        self.vertices
    }

    pub const fn indices(&self) -> &[u32] {
        self.indices
    }
}
