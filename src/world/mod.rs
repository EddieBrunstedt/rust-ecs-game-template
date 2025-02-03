pub mod component;
pub mod resource;
pub mod system;

use component::{ComponentStorage, Components};
use resource::Resources;

/// Unique identifier for entities within the ECS.
pub type EntityId = u64;

/// Central container for all ECS elements including entities, components, and resources.
/// Manages entity lifecycle and provides access to component and resource storage.
pub struct World {
    pub resources: Resources,
    pub components: Components,
    entity_ids: Vec<EntityId>,
    next_entity_id: EntityId,
}

impl World {
    // TODO: Add documentation
    pub fn new() -> Self {
        Self {
            entity_ids: Vec::new(),
            next_entity_id: 0,
            resources: Resources::new(),
            components: Components::new(),
        }
    }

    // TODO: Add documentation
    pub fn remove_entity(&mut self, entity: EntityId) {
        if let Some(pos) = self.entity_ids.iter().position(|&id| id == entity) {
            self.entity_ids.swap_remove(pos);
        }

        self.components.remove_all_components_with_entity(entity);
    }

    /// Start building an entity with a fluent interface
    pub fn build_entity(&mut self) -> EntityBuilder {
        let entity = self.create_entity();
        EntityBuilder {
            world: self,
            entity,
        }
    }

    fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }
}

/// Builder for fluent entity creation with components
pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: EntityId,
}

impl EntityBuilder<'_> {
    /// Add a component to the entity being built
    pub fn with<T>(self, component: T) -> Self
    where
        Components: ComponentStorage<T>,
    {
        self.world.components.add_component(self.entity, component);
        self
    }

    /// Get the EntityId of the built entity
    pub fn id(&self) -> EntityId {
        self.entity
    }
}
