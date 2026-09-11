use crate::{component::ComponentId, storage::Table};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct QueryCacheKey {
    first: ComponentId,
    second: Option<ComponentId>,
}

impl QueryCacheKey {
    pub(crate) const fn single(component_id: ComponentId) -> Self {
        Self {
            first: component_id,
            second: None,
        }
    }

    pub(crate) fn pair(first: ComponentId, second: ComponentId) -> Self {
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
