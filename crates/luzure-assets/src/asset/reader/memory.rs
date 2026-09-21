use std::sync::Arc;

use super::{AssetReadError, AssetReader};

#[derive(Clone, Debug)]
pub struct MemoryAssetReader {
    bytes: Arc<[u8]>,
}

impl MemoryAssetReader {
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl AssetReader for MemoryAssetReader {
    fn len(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn read_at(
        &self,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<usize, AssetReadError> {
        let Ok(offset) = usize::try_from(offset) else {
            return Ok(0);
        };
        let Some(bytes) = self.bytes.get(offset..) else {
            return Ok(0);
        };
        let len = buffer.len().min(bytes.len());

        buffer[..len].copy_from_slice(&bytes[..len]);

        Ok(len)
    }
}
