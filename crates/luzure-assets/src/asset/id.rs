use std::num::{NonZeroU32, NonZeroU64};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AssetId(NonZeroU64);

impl AssetId {
    pub const fn new(index: u32, generation: NonZeroU32) -> Self {
        let value = (generation.get() as u64) << 32 | index as u64;

        match NonZeroU64::new(value) {
            Some(value) => Self(value),
            None => unreachable!(),
        }
    }

    pub const fn index(self) -> u32 {
        self.0.get() as u32
    }

    pub const fn generation(self) -> u32 {
        (self.0.get() >> 32) as u32
    }
}

const _: () = assert!(size_of::<AssetId>() == size_of::<u64>());
const _: () = assert!(size_of::<Option<AssetId>>() == size_of::<u64>());
