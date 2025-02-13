mod component;
mod entity;
mod resource;
mod scene;
mod system;

use std::collections::VecDeque;

use crate::config::GameConfig;

pub use component::*;
pub use entity::*;
pub use resource::*;
pub use scene::*;
pub use system::*;

/// Central container for all ECS elements including entities, components, and resources.
/// Manages entity lifecycle and provides access to component and resource storage.
pub struct World {
    pub resources: Resources,
    pub components: Components,
    // Stores active entity generations
    entity_generations: Vec<usize>,
    // Queue of freed indices for reuse
    free_entity_indices: VecDeque<usize>,
    // Count of all entities ever created (used for new indices)
    entity_count: usize,
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
        if let Some(current_gen) = self.entity_generations.get(entity.index) {
            if *current_gen == entity.generation {
                self.entity_generations[entity.index] += 1;
                self.free_entity_indices.push_back(entity.index);
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
            .filter(|(index, _)| !self.free_entity_indices.contains(index))
            .map(|(index, gen)| Entity {
                index,
                generation: *gen,
            })
    }

    fn create_entity(&mut self) -> Entity {
        // Try to reuse an index from the free list
        if let Some(index) = self.free_entity_indices.pop_front() {
            let generation = self.entity_generations[index];
            Entity { index, generation }
        } else {
            // Create a new entity index
            let index = self.entity_count;
            assert!(
                index < GameConfig::default().max_entities,
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
            .get(entity.index)
            .is_some_and(|&current_gen| current_gen == entity.generation)
    }
}
