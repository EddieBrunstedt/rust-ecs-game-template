use std::fmt;

use super::{
    component::{ComponentStorage, Components},
    World,
};

/// A generational identifier that prevents the ABA problem by combining
/// an index with a generation counter.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Index into the entity pool
    pub index: usize,
    /// Generation counter to prevent ABA problems
    pub generation: usize,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format as "idx:gen" - e.g., "123:2"
        write!(f, "{}:{}", self.index, self.generation)
    }
}

/// Builder for fluent entity creation with components
pub struct EntityBuilder<'a> {
    pub world: &'a mut World,
    pub entity: Entity,
}

impl EntityBuilder<'_> {
    pub fn with<T>(self, component: T) -> Self
    where
        Components: ComponentStorage<T>,
    {
        self.world.components.add_component(self.entity, component);
        self
    }

    pub fn id(&self) -> Entity {
        self.entity
    }
}
