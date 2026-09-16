use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderSource {
    Wgsl(Cow<'static, str>),
}

impl ShaderSource {
    pub const fn wgsl(source: &'static str) -> Self {
        Self::Wgsl(Cow::Borrowed(source))
    }

    pub fn wgsl_owned(source: String) -> Self {
        Self::Wgsl(Cow::Owned(source))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Wgsl(source) => source,
        }
    }
}
