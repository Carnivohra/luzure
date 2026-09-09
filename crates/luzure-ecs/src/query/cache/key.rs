use crate::storage::Table;

use std::any::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct QueryCacheKey {
    first: TypeId,
    second: Option<TypeId>,
}

impl QueryCacheKey {
    pub(crate) fn single<T: 'static>() -> Self {
        Self {
            first: TypeId::of::<T>(),
            second: None,
        }
    }

    pub(crate) fn pair<A: 'static, B: 'static>() -> Self {
        let first = TypeId::of::<A>();
        let second = TypeId::of::<B>();

        if first <= second {
            Self {
                first,
                second: Some(second),
            }
        } else {
            Self {
                first: second,
                second: Some(first),
            }
        }
    }

    pub(crate) fn matches(self, table: &Table) -> bool {
        table.contains(self.first)
            && self.second.is_none_or(|second| table.contains(second))
    }
}
