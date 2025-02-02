pub mod components;
pub mod resources;
pub mod systems;

use components::{
    CameraComponent, ControlComponent, PlayerComponent, PositionComponent, ScoreComponent,
    VisualComponent,
};
use resources::Resources;
use std::collections::HashMap;

/// Unique identifier for entities within the ECS
pub type EntityId = u64;

pub struct Components {
    storages: ComponentStorages,
}

macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        struct ComponentStorages {
            $(
                $name: ComponentMap<$type>,
            )*
        }

        impl ComponentStorages {
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
            pub fn new() -> Self {
                Self {
                    storages: ComponentStorages::new()
                }
            }

            pub fn remove_all_components_with_entity(&mut self, entity: EntityId) {
                $(
                    self.storages.$name.entries.remove(&entity);
                )*
            }

            $(
                pub fn $name(&self, entity: EntityId) -> Option<&$type> {
                    self.storages.$name.get(entity)
                }

                paste::paste! {
                    pub fn [<$name _mut>](&mut self, entity: EntityId) -> Option<&mut $type> {
                        self.storages.$name.get_mut(entity)
                    }

                    pub fn [<expect_ $name>](&self, entity: EntityId) -> &$type {
                        self.$name(entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    pub fn [<expect_ $name _mut>](&mut self, entity: EntityId) -> &mut $type {
                        self.[<$name _mut>](entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    pub fn [<add_ $name>](&mut self, entity: EntityId, component: $type) {
                        self.storages.$name.insert(entity, component);
                    }

                    pub fn [<has_ $name>](&self, entity: EntityId) -> bool {
                        self.$name(entity).is_some()
                    }

                    pub fn [<remove_ $name>](&mut self, entity: EntityId) {
                        self.storages.$name.entries.remove(&entity);
                    }
                }
            )*
        }
    }
}

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

// Generate the component storage and methods
define_component_storages! {
    camera: CameraComponent,
    controls: ControlComponent,
    player: PlayerComponent,
    position: PositionComponent,
    score: ScoreComponent,
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
