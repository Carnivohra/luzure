use std::{fmt, hash::{Hash, Hasher}, marker::PhantomData};

use crate::AssetId;

#[repr(transparent)]
pub struct AssetHandle<T> {
    id: AssetId,
    marker: PhantomData<fn() -> T>,
}

impl<T> AssetHandle<T> {
    pub const fn new(id: AssetId) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }

    pub const fn id(self) -> AssetId {
        self.id
    }

    pub const fn index(self) -> u32 {
        self.id.index()
    }

    pub const fn generation(self) -> u32 {
        self.id.generation()
    }
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetHandle<T> {}

impl<T> fmt::Debug for AssetHandle<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("AssetHandle")
            .field(&self.id)
            .finish()
    }
}

impl<T> Eq for AssetHandle<T> {}

impl<T> Hash for AssetHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Ord for AssetHandle<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> PartialEq for AssetHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> PartialOrd for AssetHandle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const _: () = assert!(size_of::<AssetHandle<()>>() == size_of::<u64>());
const _: () = assert!(size_of::<Option<AssetHandle<()>>>() == size_of::<u64>());
