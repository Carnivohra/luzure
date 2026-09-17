use super::ErasedColumn;

pub(super) struct TableRow<'a> {
    columns: &'a mut [Box<dyn ErasedColumn>],
    row: usize,
}

impl<'a> TableRow<'a> {
    pub(super) fn new(columns: &'a mut [Box<dyn ErasedColumn>], row: usize) -> Self {
        Self { columns, row }
    }

    pub(super) fn remove(&mut self) {
        while let Some((column, remaining)) = std::mem::take(&mut self.columns).split_first_mut() {
            self.columns = remaining;
            column.remove(self.row);
        }
    }
}

impl Drop for TableRow<'_> {
    fn drop(&mut self) {
        self.remove();
    }
}
