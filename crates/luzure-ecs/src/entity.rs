mod allocator;

pub(crate) use allocator::EntityAllocator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    const fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
        }
    }

    pub const fn index(self) -> u32 {
        self.index
    }

    pub const fn generation(self) -> u32 {
        self.generation
    }
}
