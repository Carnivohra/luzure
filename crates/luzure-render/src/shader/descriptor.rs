use crate::ShaderSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderDescriptor {
    source: ShaderSource,
}

impl ShaderDescriptor {
    pub const fn new(source: ShaderSource) -> Self {
        Self { source }
    }

    pub const fn wgsl(source: &'static str) -> Self {
        Self::new(ShaderSource::wgsl(source))
    }

    pub fn wgsl_owned(source: String) -> Self {
        Self::new(ShaderSource::wgsl_owned(source))
    }

    pub const fn source(&self) -> &ShaderSource {
        &self.source
    }
}
