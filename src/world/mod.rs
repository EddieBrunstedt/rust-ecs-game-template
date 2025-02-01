pub mod components;
pub mod systems;
use components::{Control, Player, Position, Visual};
use std::collections::HashMap;

pub type EntityId = u64;
pub type PlayerId = u64;

pub struct World {
    // Entities
    entity_ids: Vec<EntityId>,
    next_entity_id: EntityId,
    next_player_id: PlayerId,

    // Components
    controls: ComponentMap<Control>,
    players: ComponentMap<Player>,
    positions: ComponentMap<Position>,
    visuals: ComponentMap<Visual>,
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
            players: ComponentMap::new(),
            visuals: ComponentMap::new(),
            positions: ComponentMap::new(),
            controls: ComponentMap::new(),
            next_entity_id: 0,
            next_player_id: 0,
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

    pub fn add_controls(&mut self, entity: EntityId, component: Control) {
        self.controls.insert(entity, component);
    }

    pub fn add_visual(&mut self, entity: EntityId, component: Visual) {
        self.visuals.insert(entity, component);
    }

    pub fn add_player(&mut self, entity: EntityId, mut component: Player) {
        component.id = self.next_player_id;
        self.players.insert(entity, component);
        self.next_player_id += 1;
    }

    pub fn get_position(&self, entity: EntityId) -> Option<&Position> {
        self.positions.get(entity)
    }

    pub fn get_visual(&self, entity: EntityId) -> Option<&Visual> {
        self.visuals.get(entity)
    }

    pub fn get_player(&self, entity: EntityId) -> Option<&Player> {
        self.players.get(entity)
    }

    pub fn get_control(&self, entity: EntityId) -> Option<&Control> {
        self.controls.get(entity)
    }
}
