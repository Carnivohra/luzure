mod error;

use std::{borrow::Borrow, fmt, str::FromStr};

pub use error::AssetPathError;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssetPath(Box<str>);

impl AssetPath {
    pub fn new(path: impl AsRef<str>) -> Result<Self, AssetPathError> {
        let path = path.as_ref();

        if path.is_empty() {
            return Err(AssetPathError::Empty);
        }

        let bytes = path.as_bytes();
        let has_drive = bytes.len() >= 2
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':';

        if path.starts_with('/') || path.starts_with('\\') || has_drive {
            return Err(AssetPathError::Absolute);
        }

        if path.contains('\0') {
            return Err(AssetPathError::NullByte);
        }

        let mut normalized = String::with_capacity(path.len());

        for component in path.split(['/', '\\']) {
            match component {
                "" | "." => continue,
                ".." => return Err(AssetPathError::ParentTraversal),
                _ => {}
            }

            if !normalized.is_empty() {
                normalized.push('/');
            }

            normalized.push_str(component);
        }

        if normalized.is_empty() {
            return Err(AssetPathError::Empty);
        }

        Ok(Self(normalized.into_boxed_str()))
    }

    pub const fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_boxed_str(self) -> Box<str> {
        self.0
    }
}

impl AsRef<str> for AssetPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for AssetPath {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AssetPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AssetPath {
    type Err = AssetPathError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Self::new(path)
    }
}

impl TryFrom<&str> for AssetPath {
    type Error = AssetPathError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}
