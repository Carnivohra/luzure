mod slot;

use std::num::NonZeroU32;

use super::{AssetHandle, AssetId};
use slot::AssetSlot;

pub struct AssetStorage<T> {
    free_head: u32,
    len: usize,
    slots: Vec<AssetSlot<T>>,
}

impl<T> AssetStorage<T> {
    const FREE_NONE: u32 = u32::MAX;

    pub const fn new() -> Self {
        Self {
            free_head: Self::FREE_NONE,
            len: 0,
            slots: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity <= Self::FREE_NONE as usize, "asset capacity exceeded");

        Self {
            free_head: Self::FREE_NONE,
            len: 0,
            slots: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, asset: T) -> AssetHandle<T> {
        let (index, generation) = if self.free_head == Self::FREE_NONE {
            let index = u32::try_from(self.slots.len())
                .expect("asset capacity exceeded");

            assert!(index != Self::FREE_NONE, "asset capacity exceeded");

            let generation = NonZeroU32::MIN;
            self.slots.push(AssetSlot::occupied(generation, asset));

            (index, generation)
        } else {
            let index = self.free_head;
            let slot = &mut self.slots[index as usize];

            self.free_head = slot.occupy(asset);

            (index, slot.generation())
        };

        self.len += 1;

        AssetHandle::new(AssetId::new(index, generation))
    }

    pub fn get(&self, handle: AssetHandle<T>) -> Option<&T> {
        let slot = self.slots.get(handle.index() as usize)?;

        if slot.generation().get() != handle.generation() {
            return None;
        }

        slot.get()
    }

    pub fn get_mut(&mut self, handle: AssetHandle<T>) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.index() as usize)?;

        if slot.generation().get() != handle.generation() {
            return None;
        }

        slot.get_mut()
    }

    pub fn remove(&mut self, handle: AssetHandle<T>) -> Option<T> {
        let index = handle.index();
        let slot = self.slots.get_mut(index as usize)?;

        if slot.generation().get() != handle.generation() {
            return None;
        }

        let (asset, reusable) = slot.vacate(self.free_head)?;

        if reusable {
            self.free_head = index;
        }

        self.len -= 1;

        Some(asset)
    }

    pub fn contains(&self, handle: AssetHandle<T>) -> bool {
        self.get(handle).is_some()
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        let required = self.slots.len().checked_add(additional)
            .expect("asset capacity exceeded");

        assert!(required <= Self::FREE_NONE as usize, "asset capacity exceeded");

        self.slots.reserve(additional);
    }
}
