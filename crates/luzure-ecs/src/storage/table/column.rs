use std::any::Any;

pub(crate) type ErasedComponent = Box<dyn Any + Send + Sync>;

pub(crate) trait ErasedColumn: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn len(&self) -> usize;
    fn push(&mut self, component: ErasedComponent);
    fn replace(&mut self, row: usize, component: ErasedComponent) -> ErasedComponent;
    fn swap_remove(&mut self, row: usize) -> ErasedComponent;
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

    fn push(&mut self, component: ErasedComponent) {
        self.push(*component.downcast::<T>()
            .expect("table component type mismatch"));
    }

    fn replace(&mut self, row: usize, component: ErasedComponent) -> ErasedComponent {
        let component = *component.downcast::<T>()
            .expect("table component type mismatch");

        Box::new(std::mem::replace(&mut self[row], component))
    }

    fn swap_remove(&mut self, row: usize) -> ErasedComponent {
        Box::new(self.swap_remove(row))
    }
}

pub(crate) fn create_column<T: Send + Sync + 'static>() -> Box<dyn ErasedColumn> {
    Box::new(Vec::<T>::new())
}
