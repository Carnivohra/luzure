mod column;
mod layout;

pub(crate) use column::{ErasedColumn, ErasedComponent, create_column};
pub(crate) use layout::ArchetypeLayout;

use crate::{Entity, component::ComponentId};

pub(crate) type ColumnFactory = fn() -> Box<dyn ErasedColumn>;

pub(crate) struct Table {
    layout: ArchetypeLayout,
    entities: Vec<Entity>,
    columns: Vec<Box<dyn ErasedColumn>>,
}

impl Table {
    pub(crate) fn new(layout: ArchetypeLayout, factories: &[ColumnFactory]) -> Self {
        let mut columns = Vec::with_capacity(layout.component_ids().len());

        for component_id in layout.component_ids() {
            let factory = factories.get(component_id.index())
                .expect("registered component must have a column factory");

            columns.push(factory());
        }

        Self {
            layout,
            entities: vec![],
            columns,
        }
    }

    pub(crate) fn component_ids(&self) -> &[ComponentId] {
        self.layout.component_ids()
    }

    pub(crate) fn contains(&self, component_id: ComponentId) -> bool {
        self.layout.contains(component_id)
    }

    pub(crate) fn components<T: Send + Sync + 'static>(&self, component_id: ComponentId) -> Option<&[T]> {
        self.columns.get(self.layout.column(component_id)?)?
            .as_any()
            .downcast_ref::<Vec<T>>()
            .map(Vec::as_slice)
    }

    pub(crate) fn components_mut<T: Send + Sync + 'static>(&mut self, component_id: ComponentId) -> Option<&mut [T]> {
        let column = self.layout.column(component_id)?;

        self.columns.get_mut(column)?
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .map(Vec::as_mut_slice)
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self, component_id: ComponentId, row: usize) -> Option<&T> {
        self.components::<T>(component_id)?.get(row)
    }

    pub(crate) fn get_mut<T: Send + Sync + 'static>(&mut self, component_id: ComponentId, row: usize) -> Option<&mut T> {
        self.components_mut::<T>(component_id)?.get_mut(row)
    }

    pub(crate) fn replace<T: Send + Sync + 'static>(&mut self, component_id: ComponentId, row: usize, component: T) -> T {
        let column = self.layout.column(component_id)
            .expect("table component must exist");

        *self.columns[column]
            .replace(row, Box::new(component))
            .downcast::<T>()
            .expect("table component type mismatch")
    }

    pub(crate) fn push_component<T: Send + Sync + 'static>(&mut self, component_id: ComponentId, component: T) {
        let column = self.layout.column(component_id)
            .expect("table component must exist");

        self.columns[column]
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .expect("table component type mismatch")
            .push(component);
    }

    pub(crate) fn push_entity(&mut self, entity: Entity) -> usize {
        let row = self.entities.len();

        debug_assert!(self.columns.iter().all(|column| column.len() == row + 1));

        self.entities.push(entity);
        row
    }

    pub(crate) fn push(&mut self, entity: Entity, components: Vec<(ComponentId, ErasedComponent)>) -> usize {
        debug_assert_eq!(components.len(), self.columns.len());

        for (component_id, component) in components {
            let column = self.layout.column(component_id)
                .expect("table component must exist");

            self.columns[column].push(component);
        }

        self.push_entity(entity)
    }

    pub(crate) fn swap_remove(&mut self, row: usize)
        -> (Entity, Vec<(ComponentId, ErasedComponent)>, Option<Entity>)
    {
        let entity = self.entities.swap_remove(row);
        let moved_entity = self.entities.get(row).copied();
        let mut components = Vec::with_capacity(self.columns.len());

        for (component_id, column) in self.layout.component_ids().iter().copied().zip(&mut self.columns) {
            components.push((component_id, column.swap_remove(row)));
        }

        (entity, components, moved_entity)
    }

    pub(crate) fn iter<T: Send + Sync + 'static>(&self, component_id: ComponentId)
        -> Option<impl Iterator<Item = (Entity, &T)>>
    {
        Some(self.entities.iter().copied().zip(self.components::<T>(component_id)?))
    }

    pub(crate) fn iter_mut<T: Send + Sync + 'static>(&mut self, component_id: ComponentId)
        -> Option<impl Iterator<Item = (Entity, &mut T)>>
    {
        let column = self.layout.column(component_id)?;
        let Self { entities, columns, .. } = self;
        let components = columns.get_mut(column)?
            .as_any_mut()
            .downcast_mut::<Vec<T>>()?;

        Some(entities.iter().copied().zip(components))
    }

    pub(crate) fn iter_pair<A: Send + Sync + 'static, B: Send + Sync + 'static>(&self, first_id: ComponentId, second_id: ComponentId)
        -> Option<impl Iterator<Item = (Entity, &A, &B)>>
    {
        let first = self.components::<A>(first_id)?;
        let second = self.components::<B>(second_id)?;

        Some(self.entities.iter().copied()
            .zip(first)
            .zip(second)
            .map(|((entity, first), second)| (entity, first, second)))
    }

    pub(crate) fn iter_pair_mut<A: Send + Sync + 'static, B: Send + Sync + 'static>(&mut self, first_id: ComponentId, second_id: ComponentId)
        -> Option<impl Iterator<Item = (Entity, &mut A, &B)>>
    {
        let first_column = self.layout.column(first_id)?;
        let second_column = self.layout.column(second_id)?;

        debug_assert_ne!(first_column, second_column);

        let Self { entities, columns, .. } = self;
        let (first, second) = if first_column < second_column {
            let (left, right) = columns.split_at_mut(second_column);

            (&mut left[first_column], &mut right[0])
        } else {
            let (left, right) = columns.split_at_mut(first_column);

            (&mut right[0], &mut left[second_column])
        };

        let first = first.as_any_mut().downcast_mut::<Vec<A>>()?;
        let second = second.as_any().downcast_ref::<Vec<B>>()?;

        Some(entities.iter().copied()
            .zip(first)
            .zip(second.iter())
            .map(|((entity, first), second)| (entity, first, second)))
    }
}
