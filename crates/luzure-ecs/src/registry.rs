mod error;
mod location;

pub use error::RegistryError;
use location::EntityLocation;

use crate::{Bundle, Entity, component::ComponentId, entity::EntityAllocator, query::{QueryCache, QueryCacheKey}, resource::ResourceStorage, storage::{ArchetypeLayout, ColumnFactory, Table, create_column}};

use std::{any::TypeId, collections::{HashMap, hash_map::Entry}};

pub struct Registry {
    entities: EntityAllocator,
    components: HashMap<TypeId, ComponentId>,
    component_factories: Vec<ColumnFactory>,
    query_cache: QueryCache,
    resources: ResourceStorage,
    tables: Vec<Table>,
    table_indices: HashMap<ArchetypeLayout, usize>,
    locations: Vec<Option<EntityLocation>>,
}

impl Registry {
    pub fn new() -> Self {
        let empty_layout = ArchetypeLayout::new(vec![]);
        let tables = vec![Table::new(empty_layout.clone(), &[])];
        let table_indices = HashMap::from([(empty_layout, 0)]);

        Self {
            entities: EntityAllocator::new(),
            components: HashMap::new(),
            component_factories: vec![],
            query_cache: QueryCache::new(),
            resources: ResourceStorage::new(),
            tables,
            table_indices,
            locations: vec![],
        }
    }

    pub fn register<T: Send + Sync + 'static>(&mut self) -> bool {
        match self.components.entry(TypeId::of::<T>()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                let index = u32::try_from(self.component_factories.len())
                    .expect("component capacity exceeded");

                entry.insert(ComponentId::new(index));
                self.component_factories.push(create_column::<T>);
                true
            },
        }
    }

    pub fn is_registered<T: 'static>(&self) -> bool {
        self.component_id::<T>().is_some()
    }

    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, resource: T) -> Option<T> {
        self.resources.insert(resource)
    }

    pub fn resource<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.resources.get()
    }

    pub fn resource_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut()
    }

    pub fn remove_resource<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.resources.remove()
    }

    pub fn contains_resource<T: Send + Sync + 'static>(&self) -> bool {
        self.resources.contains::<T>()
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T)
        -> Result<Option<T>, RegistryError>
    {
        if !self.entities.contains(entity) {
            return Err(RegistryError::EntityNotFound { entity });
        }

        self.register::<T>();
        let component_id = self.component_id::<T>()
            .expect("registered component must have an id");

        let location = self.location(entity)
            .expect("existing entity must have a table location");

        if self.tables[location.table].contains(component_id) {
            return Ok(Some(self.tables[location.table].replace(component_id, location.row, component)));
        }

        let mut component_ids = self.tables[location.table].component_ids().to_vec();

        component_ids.push(component_id);

        let table = self.table_or_create(component_ids);
        let (_, mut components, moved_entity) = self.tables[location.table].swap_remove(location.row);

        if let Some(moved_entity) = moved_entity {
            self.locations[moved_entity.index() as usize] = Some(location);
        }

        components.push((component_id, Box::new(component)));

        let row = self.tables[table].push(entity, components);
        self.locations[entity.index() as usize] = Some(EntityLocation::new(table, row));

        Ok(None)
    }

    pub fn remove<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<T> {
        let location = self.location(entity)?;
        let component_id = self.component_id::<T>()?;

        if !self.tables[location.table].contains(component_id) {
            return None;
        }

        let component_ids = self.tables[location.table].component_ids().iter().copied()
            .filter(|current_id| *current_id != component_id)
            .collect();

        let table = self.table_or_create(component_ids);
        let (_, components, moved_entity) = self.tables[location.table].swap_remove(location.row);

        if let Some(moved_entity) = moved_entity {
            self.locations[moved_entity.index() as usize] = Some(location);
        }

        let mut removed = None;
        let mut retained = Vec::with_capacity(components.len().saturating_sub(1));

        for (current_id, component) in components {
            if current_id == component_id {
                removed = component.downcast::<T>().ok().map(|component| *component);
            } else {
                retained.push((current_id, component));
            }
        }

        let row = self.tables[table].push(entity, retained);
        self.locations[entity.index() as usize] = Some(EntityLocation::new(table, row));

        removed
    }

    pub fn get<T: Send + Sync + 'static>(&self, entity: Entity) -> Option<&T> {
        let location = self.location(entity)?;
        let component_id = self.component_id::<T>()?;

        self.tables[location.table].get(component_id, location.row)
    }

    pub fn get_mut<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let location = self.location(entity)?;
        let component_id = self.component_id::<T>()?;

        self.tables[location.table].get_mut(component_id, location.row)
    }

    pub fn query<T: Send + Sync + 'static>(&self) -> impl Iterator<Item = (Entity, &T)> {
        let component_id = self.component_id::<T>();

        self.tables.iter()
            .filter_map(move |table| table.iter::<T>(component_id?))
            .flatten()
    }

    pub fn query_mut<T: Send + Sync + 'static>(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        let component_id = self.component_id::<T>();

        self.tables.iter_mut()
            .filter_map(move |table| table.iter_mut::<T>(component_id?))
            .flatten()
    }

    pub fn query_pair<A: Send + Sync + 'static, B: Send + Sync + 'static>(&self)
        -> impl Iterator<Item = (Entity, &A, &B)>
    {
        let first_id = self.component_id::<A>();
        let second_id = self.component_id::<B>();

        self.tables.iter()
            .filter_map(move |table| table.iter_pair::<A, B>(first_id?, second_id?))
            .flatten()
    }

    pub fn query_pair_mut<A: Send + Sync + 'static, B: Send + Sync + 'static>(&mut self)
        -> impl Iterator<Item = (Entity, &mut A, &B)>
    {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "mutable query component types must be unique");

        let first_id = self.component_id::<A>();
        let second_id = self.component_id::<B>();

        self.tables.iter_mut()
            .filter_map(move |table| table.iter_pair_mut::<A, B>(first_id?, second_id?))
            .flatten()
    }

    pub fn for_each<T: Send + Sync + 'static, F: FnMut(Entity, &T)>(&mut self, mut each: F) {
        let Some(component_id) = self.component_id::<T>() else {
            return;
        };

        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::single(component_id), tables);

        for index in table_indices {
            let components = tables[*index].iter::<T>(component_id)
                .expect("cached table must contain query component");

            for (entity, component) in components {
                each(entity, component);
            }
        }
    }

    pub fn for_each_mut<T: Send + Sync + 'static, F: FnMut(Entity, &mut T)>(&mut self, mut each: F) {
        let Some(component_id) = self.component_id::<T>() else {
            return;
        };

        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::single(component_id), tables);

        for index in table_indices {
            let components = tables[*index].iter_mut::<T>(component_id)
                .expect("cached table must contain query component");

            for (entity, component) in components {
                each(entity, component);
            }
        }
    }

    pub fn for_each_pair<A: Send + Sync + 'static, B: Send + Sync + 'static, F: FnMut(Entity, &A, &B)>(&mut self, mut each: F) {
        let Some(first_id) = self.component_id::<A>() else {
            return;
        };
        let Some(second_id) = self.component_id::<B>() else {
            return;
        };

        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::pair(first_id, second_id), tables);

        for index in table_indices {
            let components = tables[*index].iter_pair::<A, B>(first_id, second_id)
                .expect("cached table must contain query components");

            for (entity, first, second) in components {
                each(entity, first, second);
            }
        }
    }

    pub fn for_each_pair_mut<A: Send + Sync + 'static, B: Send + Sync + 'static, F: FnMut(Entity, &mut A, &B)>(&mut self, mut each: F) {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "mutable query component types must be unique");

        let Some(first_id) = self.component_id::<A>() else {
            return;
        };
        let Some(second_id) = self.component_id::<B>() else {
            return;
        };

        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::pair(first_id, second_id), tables);

        for index in table_indices {
            let components = tables[*index].iter_pair_mut::<A, B>(first_id, second_id)
                .expect("cached table must contain query components");

            for (entity, first, second) in components {
                each(entity, first, second);
            }
        }
    }

    pub fn contains_component<T: Send + Sync + 'static>(&self, entity: Entity) -> bool {
        let Some(location) = self.location(entity) else {
            return false;
        };
        let Some(component_id) = self.component_id::<T>() else {
            return false;
        };

        self.tables[location.table].contains(component_id)
    }

    pub fn spawn<T: Send + Sync + 'static>(&mut self, component: T) -> Entity {
        self.register::<T>();
        let component_id = self.component_id::<T>()
            .expect("registered component must have an id");

        let table = self.table_or_create(vec![component_id]);

        self.spawn_in_table(table, |table| table.push_component(component_id, component))
    }

    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        bundle.spawn(self)
    }

    pub(crate) fn spawn_two<A: Send + Sync + 'static, B: Send + Sync + 'static>(&mut self, first: A, second: B)
        -> Entity {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "bundle component types must be unique");

        self.register::<A>();
        self.register::<B>();

        let first_id = self.component_id::<A>()
            .expect("registered component must have an id");
        let second_id = self.component_id::<B>()
            .expect("registered component must have an id");

        let table = self.table_or_create(vec![first_id, second_id]);

        self.spawn_in_table(table, |table| {
            table.push_component(first_id, first);
            table.push_component(second_id, second);
        })
    }

    pub(crate) fn spawn_three<A: Send + Sync + 'static, B: Send + Sync + 'static, C: Send + Sync + 'static>(&mut self, first: A, second: B, third: C)
        -> Entity {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "bundle component types must be unique");
        assert_ne!(TypeId::of::<A>(), TypeId::of::<C>(), "bundle component types must be unique");
        assert_ne!(TypeId::of::<B>(), TypeId::of::<C>(), "bundle component types must be unique");

        self.register::<A>();
        self.register::<B>();
        self.register::<C>();

        let first_id = self.component_id::<A>()
            .expect("registered component must have an id");
        let second_id = self.component_id::<B>()
            .expect("registered component must have an id");
        let third_id = self.component_id::<C>()
            .expect("registered component must have an id");

        let table = self.table_or_create(vec![first_id, second_id, third_id]);

        self.spawn_in_table(table, |table| {
            table.push_component(first_id, first);
            table.push_component(second_id, second);
            table.push_component(third_id, third);
        })
    }

    fn spawn_in_table<F: FnOnce(&mut Table)>(&mut self, table: usize, insert: F) -> Entity {
        let entity = self.entities.allocate();

        insert(&mut self.tables[table]);

        let row = self.tables[table].push_entity(entity);

        self.set_location(entity, EntityLocation::new(table, row));
        entity
    }

    pub fn spawn_empty(&mut self) -> Entity {
        let entity = self.entities.allocate();
        let row = self.tables[0].push(entity, vec![]);

        self.set_location(entity, EntityLocation::new(0, row));
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        let Some(location) = self.location(entity) else {
            return false;
        };

        let (_, _, moved_entity) = self.tables[location.table].swap_remove(location.row);

        if let Some(moved_entity) = moved_entity {
            self.locations[moved_entity.index() as usize] = Some(location);
        }

        self.locations[entity.index() as usize] = None;
        self.entities.release(entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    fn component_id<T: 'static>(&self) -> Option<ComponentId> {
        self.components.get(&TypeId::of::<T>()).copied()
    }

    fn location(&self, entity: Entity) -> Option<EntityLocation> {
        if !self.entities.contains(entity) {
            return None;
        }

        self.locations.get(entity.index() as usize).copied().flatten()
    }

    fn set_location(&mut self, entity: Entity, location: EntityLocation) {
        let index = entity.index() as usize;

        if self.locations.len() <= index {
            self.locations.resize(index + 1, None);
        }

        self.locations[index] = Some(location);
    }

    fn table_or_create(&mut self, component_ids: Vec<ComponentId>) -> usize {
        let layout = ArchetypeLayout::new(component_ids);

        if let Some(table) = self.table_indices.get(&layout) {
            return *table;
        }

        let table = self.tables.len();

        self.tables.push(Table::new(layout.clone(), &self.component_factories));
        self.table_indices.insert(layout, table);

        table
    }
}
