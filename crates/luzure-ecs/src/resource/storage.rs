use std::{any::{Any, TypeId}, collections::{HashMap, hash_map::Entry}};

pub(crate) struct ResourceStorage {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceStorage {
    pub(crate) fn new() -> Self {
        Self { resources: HashMap::new() }
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(&mut self, resource: T) -> Option<T> {
        match self.resources.entry(TypeId::of::<T>()) {
            Entry::Occupied(mut entry) => {
                let current = entry.get_mut().downcast_mut::<T>()
                    .expect("resource type must match its identifier");

                Some(std::mem::replace(current, resource))
            },
            Entry::Vacant(entry) => {
                entry.insert(Box::new(resource));
                None
            },
        }
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub(crate) fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub(crate) fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        let resource = self.resources.remove(&TypeId::of::<T>())?
            .downcast::<T>()
            .expect("resource type must match its identifier");

        Some(*resource)
    }

    pub(crate) fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }
}
