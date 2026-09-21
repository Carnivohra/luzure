use super::{AssetPath, AssetPathError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum AssetError {
    #[error("invalid asset path: {0}")]
    InvalidPath(#[from] AssetPathError),

    #[error("asset not found: {0}")]
    NotFound(AssetPath),

    #[error("unsupported asset: {0}")]
    Unsupported(AssetPath),

    #[error("failed to read asset {path}: {message}")]
    Read {
        path: AssetPath,
        message: Box<str>,
    },

    #[error("failed to decode asset {path}: {message}")]
    Decode {
        path: AssetPath,
        message: Box<str>,
    },
}

impl AssetError {
    pub fn read(path: AssetPath, message: impl Into<Box<str>>) -> Self {
        Self::Read {
            path,
            message: message.into(),
        }
    }

    pub fn decode(path: AssetPath, message: impl Into<Box<str>>) -> Self {
        Self::Decode {
            path,
            message: message.into(),
        }
    }

    pub const fn path(&self) -> Option<&AssetPath> {
        match self {
            Self::InvalidPath(_) => None,
            Self::NotFound(path)
            | Self::Unsupported(path)
            | Self::Read { path, .. }
            | Self::Decode { path, .. } => Some(path),
        }
    }
}
