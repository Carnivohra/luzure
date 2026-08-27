use crate::Entity;

#[derive(Default)]
pub(crate) struct EntityAllocator {
    generations: Vec<u32>,
    free_indices: Vec<u32>,
}

impl EntityAllocator {
    pub(crate) const fn new() -> Self {
        Self {
            generations: vec![],
            free_indices: vec![],
        }
    }

    pub(crate) fn allocate(&mut self) -> Entity {
        if let Some(index) = self.free_indices.pop() {
            return Entity::new(index, self.generations[index as usize]);
        }

        let index = u32::try_from(self.generations.len())
            .expect("entity capacity exceeded");

        self.generations.push(0);
        Entity::new(index, 0)
    }

    pub(crate) fn release(&mut self, entity: Entity) -> bool {
        let Some(generation) = self.generations.get_mut(entity.index as usize) else {
            return false;
        };

        if *generation != entity.generation {
            return false;
        }

        *generation = generation.wrapping_add(1);
        self.free_indices.push(entity.index);

        true
    }

    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.generations.get(entity.index as usize)
            .is_some_and(|generation| *generation == entity.generation)
    }
}
