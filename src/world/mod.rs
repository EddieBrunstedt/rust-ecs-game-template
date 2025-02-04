mod component;
mod entity;
mod resource;
mod system;

use std::collections::VecDeque;

pub use component::*;
pub use entity::*;
pub use resource::*;
pub use system::*;

use crate::config::GameConfig;

/// Central container for all ECS elements including entities, components, and resources.
/// Manages entity lifecycle and provides access to component and resource storage.
pub struct World {
    pub resources: Resources,
    pub components: Components,
    // Stores active entity generations
    // TODO: Change to usize
    entity_generations: Vec<u32>,
    // Queue of freed indices for reuse
    // TODO: Change to usize
    free_entity_indices: VecDeque<u32>,
    // Count of all entities ever created (used for new indices)
    // TODO: Change to usize
    entity_count: u32,
}

impl World {
    pub fn new() -> Self {
        Self::with_capacity(GameConfig::default().max_entities)
    }

    pub fn with_capacity(max_entities: usize) -> Self {
        Self {
            entity_count: 0,
            free_entity_indices: VecDeque::with_capacity(max_entities),
            entity_generations: Vec::with_capacity(max_entities),
            components: Components::new(),
            resources: Resources::new(),
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        // Verify the generation matches before removal
        // TODO: Don't cast to usize once index is of type usize
        if let Some(current_gen) = self.entity_generations.get(entity.index as usize) {
            if *current_gen == entity.generation {
                // Increment generation and add index to free list
                // TODO: Don't cast to usize once index is of type usize
                self.entity_generations[entity.index as usize] += 1;
                self.free_entity_indices.push_back(entity.index);
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
    pub fn iter_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entity_generations
            .iter()
            .enumerate()
            // TODO: Don't cast to u32 once index is of type usize
            .filter(|(index, _)| !self.free_entity_indices.contains(&(*index as u32)))
            .map(|(index, gen)| Entity {
                // TODO: Don't cast to usize once index is of type usize
                index: index as u32,
                generation: *gen,
            })
    }

    fn create_entity(&mut self) -> Entity {
        // Try to reuse an index from the free list
        if let Some(index) = self.free_entity_indices.pop_front() {
            // TODO: Don't cast to usize once index is of type usize
            let generation = self.entity_generations[index as usize];
            Entity { index, generation }
        } else {
            // Create a new entity index
            let index = self.entity_count;
            assert!(
                // TODO: Don't cast to u32 once index is of type usize
                index < GameConfig::default().max_entities as u32,
                "Maximum entity limit of {} reached. Cannot create more entities.",
                GameConfig::default().max_entities
            );
            self.entity_count += 1;
            let generation = 0;
            self.entity_generations.push(generation);
            Entity { index, generation }
        }
    }

    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        self.entity_generations
            // TODO: Don't cast to usize once index is of type usize
            .get(entity.index as usize)
            .is_some_and(|&current_gen| current_gen == entity.generation)
    }
}
