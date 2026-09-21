use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum AssetReadError {
    #[error("asset read offset overflowed")]
    OffsetOverflow,

    #[error("asset ended during read at offset {offset}: expected {expected} bytes, read {actual}")]
    UnexpectedEnd {
        offset: u64,
        expected: usize,
        actual: usize,
    },

    #[error("asset reader returned {actual} bytes for a {requested}-byte buffer")]
    InvalidRead {
        requested: usize,
        actual: usize,
    },

    #[error("{0}")]
    Failed(Box<str>),
}

impl AssetReadError {
    pub fn failed(message: impl Into<Box<str>>) -> Self {
        Self::Failed(message.into())
    }
}
