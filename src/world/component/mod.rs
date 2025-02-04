mod definitions;
mod macros;

use super::Entity;
pub use definitions::*;
use std::collections::HashMap;

/// A container for all component storages in the ECS.
/// Provides high-level access to component data and entity management.
pub struct Components {
    storages: ComponentStorages,
}

impl<T> ComponentMap<T> {
    /// Creates a new empty component storage.
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Retrieves a reference to an entity's component if it exists.
    fn get(&self, entity: Entity) -> Option<&T> {
        self.entries.get(&entity)
    }

    /// Retrieves a mutable reference to an entity's component if it exists.
    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.entries.get_mut(&entity)
    }

    /// Associates a component with an entity.
    fn insert(&mut self, entity: Entity, component: T) {
        self.entries.insert(entity, component);
    }
}

/// Defines the interface for adding components to entities.
/// Implemented automatically by the component related macros.
pub trait ComponentStorage<T> {
    fn add_component(&mut self, entity: Entity, component: T);
}

/// Generic storage container for component data, mapping entities to their components.
/// Provides basic CRUD operations for component management.
struct ComponentMap<T> {
    entries: HashMap<Entity, T>,
}
