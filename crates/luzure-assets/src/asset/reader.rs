mod error;
mod memory;

pub use error::AssetReadError;
pub use memory::MemoryAssetReader;

pub trait AssetReader {
    fn len(&self) -> u64;

    fn read_at(
        &self,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<usize, AssetReadError>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn read_exact_at(
        &self,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<(), AssetReadError> {
        let expected = buffer.len();
        let mut read = 0;

        while read < expected {
            let current_offset = offset.checked_add(read as u64)
                .ok_or(AssetReadError::OffsetOverflow)?;
            let remaining = expected - read;
            let amount = self.read_at(current_offset, &mut buffer[read..])?;

            if amount == 0 {
                return Err(AssetReadError::UnexpectedEnd {
                    offset,
                    expected,
                    actual: read,
                });
            }

            if amount > remaining {
                return Err(AssetReadError::InvalidRead {
                    requested: remaining,
                    actual: amount,
                });
            }

            read += amount;
        }

        Ok(())
    }
}
