pub mod component;
pub mod resource;
pub mod system;

use component::Components;
use resource::Resources;

/// Unique identifier for entities within the ECS
pub type EntityId = u64;

pub struct World {
    entity_ids: Vec<EntityId>,
    next_entity_id: EntityId,
    pub resources: Resources,
    pub components: Components,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_ids: Vec::new(),
            next_entity_id: 0,
            resources: Resources::new(),
            components: Components::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        if let Some(pos) = self.entity_ids.iter().position(|&id| id == entity) {
            self.entity_ids.swap_remove(pos);
        }

        self.components.remove_all_components_with_entity(entity);
    }
}
