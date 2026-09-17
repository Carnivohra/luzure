use std::any::Any;

pub(crate) trait ErasedColumn: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn move_component(&mut self, row: usize, target: &mut dyn ErasedColumn);
    fn remove(&mut self, row: usize);
}

impl<T: Send + Sync + 'static> ErasedColumn for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn move_component(&mut self, row: usize, target: &mut dyn ErasedColumn) {
        let target = target.as_any_mut().downcast_mut::<Vec<T>>()
            .expect("table component type mismatch");

        target.push(self.swap_remove(row));
    }

    fn remove(&mut self, row: usize) {
        drop(self.swap_remove(row));
    }
}

pub(crate) fn create_column<T: Send + Sync + 'static>() -> Box<dyn ErasedColumn> {
    Box::new(Vec::<T>::new())
}
