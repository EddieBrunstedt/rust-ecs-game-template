pub mod component;
pub mod resource;
pub mod system;

use std::{collections::VecDeque, fmt};

use component::{ComponentStorage, Components};
use resource::Resources;

/// A generational identifier that prevents the ABA problem by combining
/// an index with a generation counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationalId {
    /// Index into the entity pool
    index: u32,
    /// Generation counter to prevent ABA problems
    generation: u32,
}

impl fmt::Display for GenerationalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format as "idx:gen" - e.g., "123:2"
        write!(f, "{}:{}", self.index, self.generation)
    }
}

/// Central container for all ECS elements including entities, components, and resources.
/// Manages entity lifecycle and provides access to component and resource storage.
pub struct World {
    pub resources: Resources,
    pub components: Components,
    // Stores active entity generations
    generations: Vec<u32>,
    // Queue of freed indices for reuse
    free_indices: VecDeque<u32>,
    // Count of all entities ever created (used for new indices)
    entity_count: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            free_indices: VecDeque::new(),
            generations: Vec::new(),
            components: Components::new(),
            resources: Resources::new(),
        }
    }

    pub fn remove_entity(&mut self, entity: GenerationalId) {
        // Verify the generation matches before removal
        if let Some(current_gen) = self.generations.get(entity.index as usize) {
            if *current_gen == entity.generation {
                // Increment generation and add index to free list
                self.generations[entity.index as usize] += 1;
                self.free_indices.push_back(entity.index);
                // Remove all components
                self.components.remove_all_components_with_entity(entity);
            }
        }
    }

    /// Start building an entity with a fluent interface
    pub fn build_entity(&mut self) -> EntityBuilder {
        let entity = self.create_entity();
        EntityBuilder {
            world: self,
            entity,
        }
    }

    /// Returns an iterator over all active entities
    pub fn iter_entities(&self) -> impl Iterator<Item = GenerationalId> + '_ {
        self.generations
            .iter()
            .enumerate()
            .filter(|(index, _)| !self.free_indices.contains(&(*index as u32)))
            .map(|(index, gen)| GenerationalId {
                index: index as u32,
                generation: *gen,
            })
    }

    fn create_entity(&mut self) -> GenerationalId {
        // Try to reuse an index from the free list
        if let Some(index) = self.free_indices.pop_front() {
            let generation = self.generations[index as usize];
            GenerationalId { index, generation }
        } else {
            // Create a new entity index
            let index = self.entity_count;
            self.entity_count += 1;
            let generation = 0;
            self.generations.push(generation);
            GenerationalId { index, generation }
        }
    }

    pub fn is_entity_alive(&self, entity: GenerationalId) -> bool {
        self.generations
            .get(entity.index as usize)
            .is_some_and(|&current_gen| current_gen == entity.generation)
    }
}

/// Builder for fluent entity creation with components
pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: GenerationalId,
}

impl EntityBuilder<'_> {
    pub fn with<T>(self, component: T) -> Self
    where
        Components: ComponentStorage<T>,
    {
        self.world.components.add_component(self.entity, component);
        self
    }

    pub fn id(&self) -> GenerationalId {
        self.entity
    }
}
