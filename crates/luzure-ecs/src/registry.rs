mod error;

pub use error::RegistryError;

use crate::{Entity, entity::EntityAllocator, storage::{ErasedStorage, SparseSet}};

use std::{any::TypeId, collections::{HashMap, hash_map::Entry}};

pub struct Registry {
    entities: EntityAllocator,
    storages: HashMap<TypeId, Box<dyn ErasedStorage>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entities: EntityAllocator::new(),
            storages: HashMap::new(),
        }
    }

    pub fn register<T: Send + Sync + 'static>(&mut self) -> bool {
        match self.storages.entry(TypeId::of::<T>()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(Box::new(SparseSet::<T>::new()));
                true
            },
        }
    }

    pub fn is_registered<T: 'static>(&self) -> bool {
        self.storages.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T)
        -> Result<Option<T>, RegistryError>
    {
        if !self.entities.contains(entity) {
            return Err(RegistryError::EntityNotFound { entity });
        }

        let storage = self.storage_mut_or_register::<T>();

        Ok(storage.insert(entity, component))
    }

    pub fn remove<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<T> {
        self.storage_mut::<T>()?.remove(entity)
    }

    pub fn get<T: Send + Sync + 'static>(&self, entity: Entity) -> Option<&T> {
        self.storage::<T>()?.get(entity)
    }

    pub fn get_mut<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.storage_mut::<T>()?.get_mut(entity)
    }

    pub fn contains_component<T: Send + Sync + 'static>(&self, entity: Entity) -> bool {
        self.storage::<T>()
            .is_some_and(|storage| storage.contains(entity))
    }

    pub fn spawn_empty(&mut self) -> Entity {
        self.entities.allocate()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.release(entity) {
            return false;
        }

        for storage in self.storages.values_mut() {
            storage.remove_entity(entity);
        }

        true
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    fn storage<T: Send + Sync + 'static>(&self) -> Option<&SparseSet<T>> {
        self.storages.get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref()
    }

    fn storage_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut SparseSet<T>> {
        self.storages.get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut()
    }

    fn storage_mut_or_register<T: Send + Sync + 'static>(&mut self) -> &mut SparseSet<T> {
        self.storages.entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(SparseSet::<T>::new()))
            .as_any_mut()
            .downcast_mut()
            .expect("registered component storage type mismatch")
    }
}
