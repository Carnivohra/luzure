use std::num::NonZeroU32;

pub(super) struct AssetSlot<T> {
    generation: NonZeroU32,
    state: AssetSlotState<T>,
}

impl<T> AssetSlot<T> {
    pub(super) const fn occupied(generation: NonZeroU32, asset: T) -> Self {
        Self {
            generation,
            state: AssetSlotState::Occupied(asset),
        }
    }

    pub(super) const fn generation(&self) -> NonZeroU32 {
        self.generation
    }

    pub(super) const fn get(&self) -> Option<&T> {
        match &self.state {
            AssetSlotState::Occupied(asset) => Some(asset),
            AssetSlotState::Vacant { .. } => None,
        }
    }

    pub(super) const fn get_mut(&mut self) -> Option<&mut T> {
        match &mut self.state {
            AssetSlotState::Occupied(asset) => Some(asset),
            AssetSlotState::Vacant { .. } => None,
        }
    }

    pub(super) fn occupy(&mut self, asset: T) -> u32 {
        let next = match &self.state {
            AssetSlotState::Vacant { next } => *next,
            AssetSlotState::Occupied(_) => unreachable!(),
        };

        self.state = AssetSlotState::Occupied(asset);

        next
    }

    pub(super) fn vacate(&mut self, next: u32) -> Option<(T, bool)> {
        if matches!(&self.state, AssetSlotState::Vacant { .. }) {
            return None;
        }

        let generation = self.generation.get().checked_add(1)
            .and_then(NonZeroU32::new);
        let state = std::mem::replace(
            &mut self.state,
            AssetSlotState::Vacant { next },
        );
        let AssetSlotState::Occupied(asset) = state else {
            return None;
        };

        let Some(generation) = generation else {
            return Some((asset, false));
        };

        self.generation = generation;

        Some((asset, true))
    }
}

pub(super) enum AssetSlotState<T> {
    Occupied(T),
    Vacant { next: u32 },
}
