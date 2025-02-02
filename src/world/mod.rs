pub mod components;
pub mod systems;
use components::{
    CameraComponent, ControlComponent, PlayerComponent, PositionComponent, VisualComponent,
};
use std::collections::HashMap;

/// Unique identifier for entities within the ECS
pub type EntityId = u64;

/// This macro generates a complete Entity Component System (ECS) storage implementation.
/// For each component type, it generates the following methods:
///
/// - `name(&self, entity) -> Option<&Type>`
///   Gets an immutable reference to the component
///
/// - `name_mut(&mut self, entity) -> Option<&mut Type>`
///   Gets a mutable reference to the component
///
/// - `expect_name(&self, entity) -> &Type`
///   Gets an immutable reference, panics if component doesn't exist
///
/// - `expect_name_mut(&mut self, entity) -> &mut Type`
///   Gets a mutable reference, panics if component doesn't exist
///
/// - `add_name(&mut self, entity, component)`
///   Adds a component to an entity
///
/// - `has_name(&self, entity) -> bool`
///   Checks if an entity has this component
///
/// # Example
/// ```
/// define_component_storages! {
///     position: PositionComponent,
///     visual: VisualComponent
/// }
macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        /// Central ECS container managing all entities and their components.
        /// Generated using the `define_component_storages` macro.
        pub struct World {
            /// List of all active entity IDs
            entity_ids: Vec<EntityId>,
            /// Counter for generating unique entity IDs
            next_entity_id: EntityId,
            $(
                $name: ComponentMap<$type>,
            )*
        }

        #[allow(dead_code)]
        impl World {
            /// Creates a new empty World instance
            pub fn new() -> Self {
                Self {
                    entity_ids: Vec::new(),
                    next_entity_id: 0,
                    $(
                        $name: ComponentMap::new(),
                    )*
                }
            }

            /// Removes an entity and all its components from the world
            pub fn remove_entity(&mut self, entity: EntityId) {
                if let Some(pos) = self.entity_ids.iter().position(|&id| id == entity) {
                    self.entity_ids.swap_remove(pos);
                }

                $(
                    self.$name.entries.remove(&entity);
                )*
            }

            $(
                pub fn $name(&self, entity: EntityId) -> Option<&$type> {
                    self.$name.get(entity)
                }

                paste::paste! {
                    pub fn [<$name _mut>](&mut self, entity: EntityId) -> Option<&mut $type> {
                        self.$name.get_mut(entity)
                    }

                    pub fn [<expect_ $name>](&self, entity: EntityId) -> &$type {
                        self.$name.get(entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    pub fn [<expect_ $name _mut>](&mut self, entity: EntityId) -> &mut $type {
                        self.$name.get_mut(entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    pub fn [<add_ $name>](&mut self, entity: EntityId, component: $type) {
                        self.$name.insert(entity, component);
                    }

                    pub fn [<has_ $name>](&self, entity: EntityId) -> bool {
                        self.$name.get(entity).is_some()
                    }
                }
            )*
        }
    }
}

impl World {
    /// Creates a new entity and returns its unique ID
    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }
}

// Generate the component storage and methods
define_component_storages! {
    camera: CameraComponent,
    controls: ControlComponent,
    player: PlayerComponent,
    position: PositionComponent,
    visual: VisualComponent
}

/// A generic storage container for component data associated with entities
pub struct ComponentMap<T> {
    entries: HashMap<EntityId, T>,
}

impl<T> ComponentMap<T> {
    /// Creates a new empty component storage
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Associates a component with an entity
    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.entries.insert(entity, component);
    }

    /// Retrieves a reference to an entity's component if it exists
    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.entries.get(&entity)
    }

    /// Retrieves a mutable reference to an entity's component if it exists
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.entries.get_mut(&entity)
    }
}
