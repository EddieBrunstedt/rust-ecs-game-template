pub mod components;
pub mod systems;
use components::{
    CameraComponent, ControlComponent, PlayerComponent, PositionComponent, VisualComponent,
};
use std::collections::HashMap;

pub type EntityId = u64;
pub type PlayerId = u64;

macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        pub struct World {
            // Non-component fields
            entity_ids: Vec<EntityId>,
            next_entity_id: EntityId,
            next_player_id: PlayerId,

            // Component storages
            $(
                $name: ComponentMap<$type>,
            )*
        }

        impl World {
            pub fn new() -> Self {
                Self {
                    entity_ids: Vec::new(),
                    next_entity_id: 0,
                    next_player_id: 0,
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
        }
    }
}

define_component_storages! {
    cameras: CameraComponent,
    controls: ControlComponent,
    players: PlayerComponent,
    positions: PositionComponent,
    visuals: VisualComponent
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

impl World {
    pub fn add_camera(&mut self, entity: EntityId, component: CameraComponent) {
        self.cameras.insert(entity, component);
    }

    pub fn add_controls(&mut self, entity: EntityId, component: ControlComponent) {
        self.controls.insert(entity, component);
    }

    pub fn add_player(&mut self, entity: EntityId, mut component: PlayerComponent) {
        component.id = self.next_player_id;
        self.players.insert(entity, component);
        self.next_player_id += 1;
    }

    pub fn add_position(&mut self, entity: EntityId, component: PositionComponent) {
        self.positions.insert(entity, component);
    }

    pub fn add_visual(&mut self, entity: EntityId, component: VisualComponent) {
        self.visuals.insert(entity, component);
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn expect_control(&self, entity: EntityId) -> &ControlComponent {
        self.expect_component(entity, World::get_control, "ControlComponent")
    }

    pub fn expect_control_mut(&mut self, entity: EntityId) -> &mut ControlComponent {
        self.expect_component_mut(entity, World::get_control_mut, "ControlComponent")
    }

    pub fn expect_position(&self, entity: EntityId) -> &PositionComponent {
        self.expect_component(entity, World::get_position, "PositionComponent")
    }

    pub fn expect_position_mut(&mut self, entity: EntityId) -> &mut PositionComponent {
        self.expect_component_mut(entity, World::get_position_mut, "PositionComponent")
    }

    pub fn get_camera(&self, entity: EntityId) -> Option<&CameraComponent> {
        self.cameras.get(entity)
    }

    pub fn get_control(&self, entity: EntityId) -> Option<&ControlComponent> {
        self.controls.get(entity)
    }

    pub fn get_control_mut(&mut self, entity: EntityId) -> Option<&mut ControlComponent> {
        self.controls.get_mut(entity)
    }

    pub fn get_player(&self, entity: EntityId) -> Option<&PlayerComponent> {
        self.players.get(entity)
    }

    pub fn get_position(&self, entity: EntityId) -> Option<&PositionComponent> {
        self.positions.get(entity)
    }

    pub fn get_position_mut(&mut self, entity: EntityId) -> Option<&mut PositionComponent> {
        self.positions.get_mut(entity)
    }

    pub fn get_visual(&self, entity: EntityId) -> Option<&VisualComponent> {
        self.visuals.get(entity)
    }

    pub fn is_player(&self, entity: EntityId) -> bool {
        self.players.get(entity).is_some()
    }

    fn expect_component<T>(
        &self,
        entity: EntityId,
        getter: impl Fn(&World, EntityId) -> Option<&T>,
        name: &str,
    ) -> &T {
        getter(self, entity)
            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, name))
    }

    fn expect_component_mut<T>(
        &mut self,
        entity: EntityId,
        getter: impl Fn(&mut World, EntityId) -> Option<&mut T>,
        name: &str,
    ) -> &mut T {
        getter(self, entity)
            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, name))
    }
}
