use crate::component::ComponentId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ArchetypeLayout {
    component_ids: Box<[ComponentId]>,
}

impl ArchetypeLayout {
    pub(crate) fn new(mut component_ids: Vec<ComponentId>) -> Self {
        component_ids.sort_unstable();

        debug_assert!(!component_ids.windows(2)
            .any(|component_ids| component_ids[0] == component_ids[1]));

        Self { component_ids: component_ids.into_boxed_slice() }
    }

    pub(crate) fn component_ids(&self) -> &[ComponentId] {
        &self.component_ids
    }

    pub(crate) fn column(&self, component_id: ComponentId) -> Option<usize> {
        self.component_ids.binary_search(&component_id).ok()
    }

    pub(crate) fn contains(&self, component_id: ComponentId) -> bool {
        self.column(component_id).is_some()
    }
}
