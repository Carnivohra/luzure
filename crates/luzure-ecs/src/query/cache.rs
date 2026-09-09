mod entry;
mod key;

use entry::QueryCacheEntry;
pub(crate) use key::QueryCacheKey;

use crate::storage::Table;

use std::collections::{HashMap, hash_map::Entry};

pub(crate) struct QueryCache {
    entries: HashMap<QueryCacheKey, QueryCacheEntry>,
}

impl QueryCache {
    pub(crate) fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub(crate) fn tables<'a>(&'a mut self, key: QueryCacheKey, tables: &[Table]) -> &'a [usize] {
        let entry = match self.entries.entry(key) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(QueryCacheEntry::new()),
        };

        entry.update(key, tables);
        entry.tables()
    }
}
