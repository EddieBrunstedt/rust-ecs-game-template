mod definitions;

use super::EntityId;
use std::collections::HashMap;

pub use definitions::*;

pub struct Components {
    storages: ComponentStorages,
}

/// Generates a complete component storage system from a list of component types.
///
/// This macro creates:
/// - A ComponentStorages struct containing a ComponentMap for each component type
/// - A complete API for managing each component type including:
///   - Getters (immutable and mutable)
///   - Expectation methods that panic if component is missing
///   - Add/remove/has methods
///   - Entity-based component management
///
/// # Example
/// ```rust
/// define_component_storages! {
///     position: PositionComponent,
///     velocity: VelocityComponent,
/// }
/// ```
macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        /// Internal storage containing all registered component types
        struct ComponentStorages {
            $(
                $name: ComponentMap<$type>,
            )*
        }

        impl ComponentStorages {
            /// Creates a new ComponentStorages with empty maps for all components
            fn new() -> Self {
                Self {
                    $(
                        $name: ComponentMap::new(),
                    )*
                }
            }
        }

        #[allow(dead_code)]
        impl Components {
            /// Creates a new empty Components container
            pub fn new() -> Self {
                Self {
                    storages: ComponentStorages::new()
                }
            }

            /// Removes all components associated with the given entity
            pub fn remove_all_components_with_entity(&mut self, entity: EntityId) {
                $(
                    self.storages.$name.entries.remove(&entity);
                )*
            }

            $(
                /// Gets an immutable reference to the component if it exists on the entity
                pub fn $name(&self, entity: EntityId) -> Option<&$type> {
                    self.storages.$name.get(entity)
                }

                paste::paste! {
                    /// Gets a mutable reference to the component if it exists on the entity
                    pub fn [<$name _mut>](&mut self, entity: EntityId) -> Option<&mut $type> {
                        self.storages.$name.get_mut(entity)
                    }

                    /// Gets an immutable reference to the component, panicking if it doesn't exist
                    pub fn [<expect_ $name>](&self, entity: EntityId) -> &$type {
                        self.$name(entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    /// Gets a mutable reference to the component, panicking if it doesn't exist
                    pub fn [<expect_ $name _mut>](&mut self, entity: EntityId) -> &mut $type {
                        self.[<$name _mut>](entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    /// Adds a component to an entity, replacing any existing one
                    pub fn [<add_ $name>](&mut self, entity: EntityId, component: $type) {
                        self.storages.$name.insert(entity, component);
                    }

                    /// Checks if the entity has this component
                    pub fn [<has_ $name>](&self, entity: EntityId) -> bool {
                        self.$name(entity).is_some()
                    }

                    /// Removes this component from the entity if it exists
                    pub fn [<remove_ $name>](&mut self, entity: EntityId) {
                        self.storages.$name.entries.remove(&entity);
                    }
                }
            )*
        }
    }
}

// Generate the component storage and methods
define_component_storages! {
    camera: CameraComponent,
    controls: ControlComponent,
    player: PlayerComponent,
    position: PositionComponent,
    score: ScoreComponent,
    visual: VisualComponent
}

/// A generic storage container for component data associated with entities.
pub struct ComponentMap<T> {
    entries: HashMap<EntityId, T>,
}

impl<T> ComponentMap<T> {
    /// Creates a new empty component storage.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Associates a component with an entity.
    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.entries.insert(entity, component);
    }

    /// Retrieves a reference to an entity's component if it exists.
    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.entries.get(&entity)
    }

    /// Retrieves a mutable reference to an entity's component if it exists.
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.entries.get_mut(&entity)
    }
}
