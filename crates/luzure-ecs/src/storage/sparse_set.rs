use crate::{Entity, storage::ErasedStorage};

use std::any::Any;

const EMPTY: u32 = u32::MAX;

pub(crate) struct SparseSet<T> {
    sparse: Vec<u32>,
    entities: Vec<Entity>,
    components: Vec<T>,
}

impl<T> SparseSet<T> {
    pub(crate) const fn new() -> Self {
        Self {
            sparse: vec![],
            entities: vec![],
            components: vec![],
        }
    }

    pub(crate) fn insert(&mut self, entity: Entity, component: T) -> Option<T> {
        let sparse_index = entity.index() as usize;

        if self.sparse.len() <= sparse_index {
            self.sparse.resize(sparse_index + 1, EMPTY);
        }

        let dense_index = self.sparse[sparse_index];

        if dense_index != EMPTY {
            let dense_index = dense_index as usize;

            self.entities[dense_index] = entity;
            return Some(std::mem::replace(&mut self.components[dense_index], component));
        }

        let dense_index = u32::try_from(self.entities.len())
            .expect("component capacity exceeded");

        self.sparse[sparse_index] = dense_index;
        self.entities.push(entity);
        self.components.push(component);

        None
    }

    pub(crate) fn remove(&mut self, entity: Entity) -> Option<T> {
        let dense_index = self.dense_index(entity)?;
        let component = self.components.swap_remove(dense_index);

        self.entities.swap_remove(dense_index);
        self.sparse[entity.index() as usize] = EMPTY;

        if dense_index < self.entities.len() {
            let moved_entity = self.entities[dense_index];
            self.sparse[moved_entity.index() as usize] = dense_index as u32;
        }

        Some(component)
    }

    pub(crate) fn get(&self, entity: Entity) -> Option<&T> {
        let dense_index = self.dense_index(entity)?;
        self.components.get(dense_index)
    }

    pub(crate) fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let dense_index = self.dense_index(entity)?;
        self.components.get_mut(dense_index)
    }

    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.dense_index(entity).is_some()
    }

    fn dense_index(&self, entity: Entity) -> Option<usize> {
        let dense_index = *self.sparse.get(entity.index() as usize)?;

        if dense_index == EMPTY {
            return None;
        }

        let dense_index = dense_index as usize;

        if self.entities.get(dense_index).copied() != Some(entity) {
            return None;
        }

        Some(dense_index)
    }
}


impl<T: Send + Sync + 'static> ErasedStorage for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.remove(entity);
    }
}