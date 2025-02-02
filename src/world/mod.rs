pub mod components;
pub mod systems;
use components::{
    CameraComponent, ControlComponent, PlayerComponent, PositionComponent, VisualComponent,
};
use std::collections::HashMap;

pub type EntityId = u64;

/// This macro generates a complete Entity Component System (ECS) storage implementation.
/// It automatically creates all necessary methods for component management, reducing
/// boilerplate code and ensuring consistent API across all component types.
///
/// # Arguments
/// Takes a list of component definitions in the format: `name: type`
/// where `name` becomes the base for all generated method names
/// and `type` is the component's data type.
///
/// # Generated Structure
/// - Creates a World struct with:
///   - Vector of entity IDs
///   - Next available entity ID
///   - ComponentMap storage for each component type
///
/// # Generated Methods Per Component
/// - Regular getter: `name(&self, entity) -> Option<&Type>`
/// - Mutable getter: `name_mut(&mut self, entity) -> Option<&mut Type>`
/// - Expect getter: `expect_name(&self, entity) -> &Type` (panics if missing)
/// - Expect mutable: `expect_name_mut(&mut self, entity) -> &mut Type` (panics if missing)
/// - Component adder: `add_name(&mut self, entity, component)`
/// - Component check: `has_name(&self, entity) -> bool`
///
/// # Benefits
/// 1. Eliminates repetitive code for each component type
/// 2. Ensures consistent method naming and behavior
/// 3. Reduces potential for errors in component management
/// 4. Makes adding new component types as simple as adding one line
macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        pub struct World {
            entity_ids: Vec<EntityId>,
            next_entity_id: EntityId,
            $(
                $name: ComponentMap<$type>,
            )*
        }

        #[allow(dead_code)]
        impl World {
            pub fn new() -> Self {
                Self {
                    entity_ids: Vec::new(),
                    next_entity_id: 0,
                    $(
                        $name: ComponentMap::new(),
                    )*
                }
            }

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
    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }
}

define_component_storages! {
    camera: CameraComponent,
    controls: ControlComponent,
    player: PlayerComponent,
    position: PositionComponent,
    visual: VisualComponent
}

pub struct ComponentMap<T> {
    entries: HashMap<EntityId, T>,
}

impl<T> ComponentMap<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.entries.insert(entity, component);
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.entries.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.entries.get_mut(&entity)
    }
}
