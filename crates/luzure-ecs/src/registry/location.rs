#[derive(Clone, Copy)]
pub(super) struct EntityLocation {
    pub(super) table: usize,
    pub(super) row: usize,
}

impl EntityLocation {
    pub(super) const fn new(table: usize, row: usize) -> Self {
        Self {
            table,
            row,
        }
    }
}
