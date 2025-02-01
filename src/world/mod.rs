pub mod components;
pub mod systems;
use components::{CameraComponent, ControlComponent, PlayerComponent, Position, VisualComponent};
use std::collections::HashMap;

pub type EntityId = u64;
pub type PlayerId = u64;

pub struct World {
    // Entities
    entity_ids: Vec<EntityId>,
    next_entity_id: EntityId,
    next_player_id: PlayerId,

    // Components
    cameras: ComponentMap<CameraComponent>,
    controls: ComponentMap<ControlComponent>,
    players: ComponentMap<PlayerComponent>,
    positions: ComponentMap<Position>,
    visuals: ComponentMap<VisualComponent>,
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
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_ids: Vec::new(),
            next_entity_id: 0,
            next_player_id: 0,
            cameras: ComponentMap::new(),
            controls: ComponentMap::new(),
            players: ComponentMap::new(),
            positions: ComponentMap::new(),
            visuals: ComponentMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entity_ids.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn add_position(&mut self, entity: EntityId, component: Position) {
        self.positions.insert(entity, component);
    }

    pub fn add_camera(&mut self, entity: EntityId, component: CameraComponent) {
        self.cameras.insert(entity, component);
    }

    pub fn add_controls(&mut self, entity: EntityId, component: ControlComponent) {
        self.controls.insert(entity, component);
    }

    pub fn add_visual(&mut self, entity: EntityId, component: VisualComponent) {
        self.visuals.insert(entity, component);
    }

    pub fn add_player(&mut self, entity: EntityId, mut component: PlayerComponent) {
        component.id = self.next_player_id;
        self.players.insert(entity, component);
        self.next_player_id += 1;
    }

    pub fn get_position(&self, entity: EntityId) -> Option<&Position> {
        self.positions.get(entity)
    }

    pub fn get_visual(&self, entity: EntityId) -> Option<&VisualComponent> {
        self.visuals.get(entity)
    }

    pub fn get_player(&self, entity: EntityId) -> Option<&PlayerComponent> {
        self.players.get(entity)
    }

    pub fn get_camera(&self, entity: EntityId) -> Option<&CameraComponent> {
        self.cameras.get(entity)
    }

    pub fn get_control(&self, entity: EntityId) -> Option<&ControlComponent> {
        self.controls.get(entity)
    }

    pub fn expect_control(&self, entity: EntityId) -> &ControlComponent {
        self.expect_component(entity, World::get_control, "ControlComponent")
    }

    pub fn expect_position(&self, entity: EntityId) -> &Position {
        self.expect_component(entity, World::get_position, "Position")
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
}
