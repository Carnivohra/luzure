use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AssetPathError {
    #[error("path is empty")]
    Empty,

    #[error("path must be relative")]
    Absolute,

    #[error("path cannot contain a parent component")]
    ParentTraversal,

    #[error("path cannot contain a null byte")]
    NullByte,
}
