mod error;
mod location;

pub use error::RegistryError;
use location::EntityLocation;

use crate::{Bundle, Entity, entity::EntityAllocator, query::{QueryCache, QueryCacheKey}, storage::{ColumnFactory, Table, create_column}};

use std::{any::TypeId, collections::{HashMap, hash_map::Entry}};

pub struct Registry {
    entities: EntityAllocator,
    components: HashMap<TypeId, ColumnFactory>,
    query_cache: QueryCache,
    tables: Vec<Table>,
    table_indices: HashMap<Vec<TypeId>, usize>,
    locations: Vec<Option<EntityLocation>>,
}

impl Registry {
    pub fn new() -> Self {
        let tables = vec![Table::new(vec![], &HashMap::new())];
        let table_indices = HashMap::from([(vec![], 0)]);

        Self {
            entities: EntityAllocator::new(),
            components: HashMap::new(),
            query_cache: QueryCache::new(),
            tables,
            table_indices,
            locations: vec![],
        }
    }

    pub fn register<T: Send + Sync + 'static>(&mut self) -> bool {
        match self.components.entry(TypeId::of::<T>()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(create_column::<T>);
                true
            },
        }
    }

    pub fn is_registered<T: 'static>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, entity: Entity, component: T)
        -> Result<Option<T>, RegistryError>
    {
        if !self.entities.contains(entity) {
            return Err(RegistryError::EntityNotFound { entity });
        }

        self.register::<T>();

        let location = self.location(entity)
            .expect("existing entity must have a table location");

        if self.tables[location.table].contains(TypeId::of::<T>()) {
            return Ok(Some(self.tables[location.table].replace(location.row, component)));
        }

        let mut component_ids = self.tables[location.table].component_ids().to_vec();

        component_ids.push(TypeId::of::<T>());
        component_ids.sort_unstable();

        let table = self.table_or_create(component_ids);
        let (_, mut components, moved_entity) = self.tables[location.table].swap_remove(location.row);

        if let Some(moved_entity) = moved_entity {
            self.locations[moved_entity.index() as usize] = Some(location);
        }

        components.push((TypeId::of::<T>(), Box::new(component)));

        let row = self.tables[table].push(entity, components);
        self.locations[entity.index() as usize] = Some(EntityLocation::new(table, row));

        Ok(None)
    }

    pub fn remove<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<T> {
        let location = self.location(entity)?;

        if !self.tables[location.table].contains(TypeId::of::<T>()) {
            return None;
        }

        let component_ids = self.tables[location.table].component_ids().iter().copied()
            .filter(|component_id| *component_id != TypeId::of::<T>())
            .collect();

        let table = self.table_or_create(component_ids);
        let (_, components, moved_entity) = self.tables[location.table].swap_remove(location.row);

        if let Some(moved_entity) = moved_entity {
            self.locations[moved_entity.index() as usize] = Some(location);
        }

        let mut removed = None;
        let mut retained = Vec::with_capacity(components.len().saturating_sub(1));

        for (component_id, component) in components {
            if component_id == TypeId::of::<T>() {
                removed = component.downcast::<T>().ok().map(|component| *component);
            } else {
                retained.push((component_id, component));
            }
        }

        let row = self.tables[table].push(entity, retained);
        self.locations[entity.index() as usize] = Some(EntityLocation::new(table, row));

        removed
    }

    pub fn get<T: Send + Sync + 'static>(&self, entity: Entity) -> Option<&T> {
        let location = self.location(entity)?;

        self.tables[location.table].get(location.row)
    }

    pub fn get_mut<T: Send + Sync + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let location = self.location(entity)?;

        self.tables[location.table].get_mut(location.row)
    }

    pub fn query<T: Send + Sync + 'static>(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.tables.iter()
            .filter_map(|table| table.iter::<T>())
            .flatten()
    }

    pub fn query_mut<T: Send + Sync + 'static>(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.tables.iter_mut()
            .filter_map(|table| table.iter_mut::<T>())
            .flatten()
    }

    pub fn query_pair<A: Send + Sync + 'static, B: Send + Sync + 'static>(&self)
        -> impl Iterator<Item = (Entity, &A, &B)>
    {
        self.tables.iter()
            .filter_map(|table| table.iter_pair::<A, B>())
            .flatten()
    }

    pub fn query_pair_mut<A: Send + Sync + 'static, B: Send + Sync + 'static>(&mut self)
        -> impl Iterator<Item = (Entity, &mut A, &B)>
    {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "mutable query component types must be unique");

        self.tables.iter_mut()
            .filter_map(|table| table.iter_pair_mut::<A, B>())
            .flatten()
    }

    pub fn for_each<T: Send + Sync + 'static, F: FnMut(Entity, &T)>(&mut self, mut each: F) {
        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::single::<T>(), tables);

        for index in table_indices {
            let components = tables[*index].iter::<T>()
                .expect("cached table must contain query component");

            for (entity, component) in components {
                each(entity, component);
            }
        }
    }

    pub fn for_each_mut<T: Send + Sync + 'static, F: FnMut(Entity, &mut T)>(&mut self, mut each: F) {
        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::single::<T>(), tables);

        for index in table_indices {
            let components = tables[*index].iter_mut::<T>()
                .expect("cached table must contain query component");

            for (entity, component) in components {
                each(entity, component);
            }
        }
    }

    pub fn for_each_pair<A: Send + Sync + 'static, B: Send + Sync + 'static, F: FnMut(Entity, &A, &B)>(&mut self, mut each: F) {
        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::pair::<A, B>(), tables);

        for index in table_indices {
            let components = tables[*index].iter_pair::<A, B>()
                .expect("cached table must contain query components");

            for (entity, first, second) in components {
                each(entity, first, second);
            }
        }
    }

    pub fn for_each_pair_mut<A: Send + Sync + 'static, B: Send + Sync + 'static, F: FnMut(Entity, &mut A, &B)>(&mut self, mut each: F) {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "mutable query component types must be unique");

        let Self { query_cache, tables, .. } = self;
        let table_indices = query_cache.tables(QueryCacheKey::pair::<A, B>(), tables);

        for index in table_indices {
            let components = tables[*index].iter_pair_mut::<A, B>()
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

        self.tables[location.table].contains(TypeId::of::<T>())
    }

    pub fn spawn<T: Send + Sync + 'static>(&mut self, component: T) -> Entity {
        self.register::<T>();

        let table = self.table_or_create(vec![TypeId::of::<T>()]);

        self.spawn_in_table(table, |table| table.push_component(component))
    }

    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        bundle.spawn(self)
    }

    pub(crate) fn spawn_two<A: Send + Sync + 'static, B: Send + Sync + 'static>(&mut self, first: A, second: B)
        -> Entity {
        assert_ne!(TypeId::of::<A>(), TypeId::of::<B>(), "bundle component types must be unique");

        self.register::<A>();
        self.register::<B>();

        let mut component_ids = vec![TypeId::of::<A>(), TypeId::of::<B>()];

        component_ids.sort_unstable();

        let table = self.table_or_create(component_ids);

        self.spawn_in_table(table, |table| {
            table.push_component(first);
            table.push_component(second);
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

        let mut component_ids = vec![TypeId::of::<A>(), TypeId::of::<B>(), TypeId::of::<C>()];

        component_ids.sort_unstable();

        let table = self.table_or_create(component_ids);

        self.spawn_in_table(table, |table| {
            table.push_component(first);
            table.push_component(second);
            table.push_component(third);
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

    fn table_or_create(&mut self, component_ids: Vec<TypeId>) -> usize {
        if let Some(table) = self.table_indices.get(&component_ids) {
            return *table;
        }

        let table = self.tables.len();

        self.tables.push(Table::new(component_ids.clone(), &self.components));
        self.table_indices.insert(component_ids, table);

        table
    }
}
