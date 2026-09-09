use super::QueryCacheKey;

use crate::storage::Table;

pub(super) struct QueryCacheEntry {
    scanned_tables: usize,
    tables: Vec<usize>,
}

impl QueryCacheEntry {
    pub(super) const fn new() -> Self {
        Self {
            scanned_tables: 0,
            tables: Vec::new(),
        }
    }

    pub(super) fn update(&mut self, key: QueryCacheKey, tables: &[Table]) {
        for (index, table) in tables.iter().enumerate().skip(self.scanned_tables) {
            if key.matches(table) {
                self.tables.push(index);
            }
        }

        self.scanned_tables = tables.len();
    }

    pub(super) fn tables(&self) -> &[usize] {
        &self.tables
    }
}
