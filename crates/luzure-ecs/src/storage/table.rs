mod column;

pub(crate) use column::{ErasedColumn, ErasedComponent, create_column};

use crate::Entity;

use std::{any::TypeId, collections::HashMap};

pub(crate) type ColumnFactory = fn() -> Box<dyn ErasedColumn>;

pub(crate) struct Table {
    component_ids: Vec<TypeId>,
    entities: Vec<Entity>,
    columns: HashMap<TypeId, Box<dyn ErasedColumn>>,
}

impl Table {
    pub(crate) fn new(component_ids: Vec<TypeId>, factories: &HashMap<TypeId, ColumnFactory>) -> Self {
        let mut columns = HashMap::with_capacity(component_ids.len());

        for component_id in &component_ids {
            let factory = factories.get(component_id)
                .expect("registered component must have a column factory");

            columns.insert(*component_id, factory());
        }

        Self {
            component_ids,
            entities: vec![],
            columns,
        }
    }

    pub(crate) fn component_ids(&self) -> &[TypeId] {
        &self.component_ids
    }

    pub(crate) fn contains(&self, component_id: TypeId) -> bool {
        self.columns.contains_key(&component_id)
    }

    pub(crate) fn components<T: Send + Sync + 'static>(&self) -> Option<&[T]> {
        self.columns.get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<Vec<T>>()
            .map(Vec::as_slice)
    }

    pub(crate) fn components_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut [T]> {
        self.columns.get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .map(Vec::as_mut_slice)
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self, row: usize) -> Option<&T> {
        self.components::<T>()?.get(row)
    }

    pub(crate) fn get_mut<T: Send + Sync + 'static>(&mut self, row: usize) -> Option<&mut T> {
        self.components_mut::<T>()?.get_mut(row)
    }

    pub(crate) fn replace<T: Send + Sync + 'static>(&mut self, row: usize, component: T) -> T {
        *self.columns.get_mut(&TypeId::of::<T>())
            .expect("table component must exist")
            .replace(row, Box::new(component))
            .downcast::<T>()
            .expect("table component type mismatch")
    }

    pub(crate) fn push_component<T: Send + Sync + 'static>(&mut self, component: T) {
        self.columns.get_mut(&TypeId::of::<T>())
            .expect("table component must exist")
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .expect("table component type mismatch")
            .push(component);
    }

    pub(crate) fn push_entity(&mut self, entity: Entity) -> usize {
        let row = self.entities.len();

        debug_assert!(self.columns.values().all(|column| column.len() == row + 1));

        self.entities.push(entity);
        row
    }

    pub(crate) fn push(&mut self, entity: Entity, components: Vec<(TypeId, ErasedComponent)>) -> usize {
        debug_assert_eq!(components.len(), self.columns.len());

        for (component_id, component) in components {
            self.columns.get_mut(&component_id)
                .expect("table component must exist")
                .push(component);
        }

        self.push_entity(entity)
    }

    pub(crate) fn swap_remove(&mut self, row: usize)
        -> (Entity, Vec<(TypeId, ErasedComponent)>, Option<Entity>)
    {
        let entity = self.entities.swap_remove(row);
        let moved_entity = self.entities.get(row).copied();
        let mut components = Vec::with_capacity(self.columns.len());

        for (component_id, column) in &mut self.columns {
            components.push((*component_id, column.swap_remove(row)));
        }

        (entity, components, moved_entity)
    }

    pub(crate) fn iter<T: Send + Sync + 'static>(&self)
        -> Option<impl Iterator<Item = (Entity, &T)>>
    {
        Some(self.entities.iter().copied().zip(self.components::<T>()?))
    }

    pub(crate) fn iter_mut<T: Send + Sync + 'static>(&mut self)
        -> Option<impl Iterator<Item = (Entity, &mut T)>>
    {
        let Self { entities, columns, .. } = self;
        let components = columns.get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<Vec<T>>()?;

        Some(entities.iter().copied().zip(components))
    }
}
